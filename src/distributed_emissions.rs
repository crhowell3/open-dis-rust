//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Distributed Emission Regeneration protocol family

#![allow(deprecated)]

use deku::{
    DekuError, DekuReader, DekuWriter,
    no_std_io::{Read, Seek, Write},
    reader::Reader,
    writer::Writer,
};

use crate::common::{
    data_types::{
        AcousticEmitterSystemData, ApaData, BeamData, ElectromagneticEmissionSystemData,
        EntityCoordinateVector, EntityId, EventId, FundamentalOperationalData,
        IFFFundamentalParameterData, LayerHeader, LinearAcceleration, PropulsionSystemData,
        SecondaryOperationalData, ShaftRPMs, SystemId, VectoringNozzleSystemData, WorldCoordinate,
    },
    enums::{
        DeadReckoningAlgorithm, DesignatorCode, DesignatorSystemName, EEAttributeStateIndicator,
        PduType, ProtocolFamily, UAPassiveParameterIndex, UAStateChangeUpdateIndicator,
    },
    pdu_header::PduHeader,
};

/// Implemented according to IEEE 1278.1-2012 §7.6.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct ElectromagneticEmissionsPdu {
    #[deku(skip)]
    header: PduHeader,
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub state_update_indicator: EEAttributeStateIndicator,
    #[deku(update = "crate::common::wire::update_each(&mut self.systems)?")]
    pub number_of_systems: u8,
    padding: u16,
    #[deku(count = "number_of_systems")]
    pub systems: Vec<ElectromagneticEmissionSystemData>,
}
crate::impl_pdu!(
    ElectromagneticEmissionsPdu,
    PduHeader,
    PduType::ElectromagneticEmissions,
    ProtocolFamily::DistributedEmissionRegeneration
);

/// Implemented according to IEEE 1278.1-2012 §7.6.3
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct DesignatorPdu {
    #[deku(skip)]
    header: PduHeader,
    pub designating_entity_id: EntityId,
    pub code_name: DesignatorSystemName,
    pub designated_entity_id: EntityId,
    pub designator_code: DesignatorCode,
    pub designator_power: f32,
    pub designator_wavelength: f32,
    pub designator_spot_wrt_designated: EntityCoordinateVector,
    pub designator_spot_location: WorldCoordinate,
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    padding: u8,
    padding2: u16,
    pub entity_linear_acceleration: LinearAcceleration,
}
crate::impl_pdu!(
    DesignatorPdu,
    PduHeader,
    PduType::Designator,
    ProtocolFamily::DistributedEmissionRegeneration
);

/// Implemented according to IEEE 1278.1-2012 §7.6.4
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct UnderwaterAcousticPdu {
    #[deku(skip)]
    header: PduHeader,
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub state_change_update_indicator: UAStateChangeUpdateIndicator,
    padding: u8,
    pub passive_parameter_index: UAPassiveParameterIndex,
    pub propulsion_plant_configuration: u8,
    #[deku(update = "self.shaft_rpms.len()")]
    pub number_of_shafts: u8,
    #[deku(update = "self.apa_data.len()")]
    pub number_of_apas: u8,
    #[deku(update = "crate::common::wire::update_each(&mut self.emitter_systems)?")]
    pub number_of_ua_emitter_systems: u8,
    #[deku(count = "number_of_shafts")]
    pub shaft_rpms: Vec<ShaftRPMs>,
    #[deku(count = "number_of_apas")]
    pub apa_data: Vec<ApaData>,
    #[deku(count = "number_of_ua_emitter_systems")]
    /// Each emitter with its location and beams.
    pub emitter_systems: Vec<AcousticEmitterSystemData>,
}
crate::impl_pdu!(
    UnderwaterAcousticPdu,
    PduHeader,
    PduType::UnderwaterAcoustic,
    ProtocolFamily::DistributedEmissionRegeneration
);

/// Implemented according to IEEE 1278.1-2012 §7.6.5
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct IFFPdu {
    #[deku(skip)]
    header: PduHeader,
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub relative_antenna_location: EntityCoordinateVector,
    pub system_id: SystemId,
    pub system_designator: u8,
    pub system_specific_data: u8,
    pub fundamental_operational_data: FundamentalOperationalData,
    /// Everything after layer 1. Layers are found from the octets that
    /// follow it, not from `information_layers`, which you set yourself.
    #[deku(
        reader = "read_iff_layers(deku::reader)",
        writer = "write_iff_layers(deku::writer, layers)",
        update = "self.layers.updated()?"
    )]
    pub layers: IffLayers,
}
crate::impl_pdu!(
    IFFPdu,
    PduHeader,
    PduType::IFF,
    ProtocolFamily::DistributedEmissionRegeneration
);

/// IFF layer 2, emissions data (IEEE 1278.1-2012 §7.6.5.3).
#[derive(Debug, Default, Clone, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct IffLayer2 {
    /// `serialize` sets the layer number to 2 and the length to the
    /// layer's size in octets, header included.
    #[deku(update = "self.derived_layer_header()?")]
    pub layer_header: LayerHeader,
    pub beam_data: BeamData,
    /// `serialize` sets the record count from `iff_parameters`.
    #[deku(
        update = "self.derived_secondary_operational_data()?",
        assert = "crate::common::wire::fits_in_pdu(secondary_operational_data.number_of_iff_fundamental_parameter_records)"
    )]
    pub secondary_operational_data: SecondaryOperationalData,
    #[deku(count = "secondary_operational_data.number_of_iff_fundamental_parameter_records")]
    pub iff_parameters: Vec<IFFFundamentalParameterData>,
}

impl IffLayer2 {
    fn derived_layer_header(&self) -> Result<LayerHeader, DekuError> {
        let length = u16::try_from(crate::common::wire::encoded_len(self)?)
            .map_err(|_| DekuError::Assertion("IFF layer 2 is too long".into()))?;
        Ok(LayerHeader {
            layer_number: 2,
            length,
            ..self.layer_header
        })
    }

    fn derived_secondary_operational_data(&self) -> Result<SecondaryOperationalData, DekuError> {
        let count = u16::try_from(self.iff_parameters.len()).map_err(|_| {
            DekuError::Assertion("too many IFF fundamental parameter records".into())
        })?;
        Ok(SecondaryOperationalData {
            number_of_iff_fundamental_parameter_records: count,
            ..self.secondary_operational_data
        })
    }
}

/// The layers of an IFF PDU after layer 1.
#[derive(Debug, Default, Clone)]
pub struct IffLayers {
    /// Layer 2, when the octets after layer 1 begin with it.
    pub layer_2: Option<IffLayer2>,
    /// Any further layers, as received: the octets after layer 2, or all of
    /// them when they don't begin with layer 2.
    pub other_layers: Vec<u8>,
}

impl IffLayers {
    /// A copy with layer 2's update rules applied.
    fn updated(&self) -> Result<Self, DekuError> {
        let mut layers = self.clone();
        if let Some(layer_2) = layers.layer_2.as_mut() {
            deku::DekuUpdate::update(layer_2)?;
        }
        Ok(layers)
    }
}

fn read_iff_layers<R: Read + Seek>(reader: &mut Reader<R>) -> Result<IffLayers, DekuError> {
    let mut octets = Vec::new();
    while !reader.end() {
        octets.push(u8::from_reader_with_ctx(reader, ())?);
    }
    if octets.first() != Some(&2) {
        return Ok(IffLayers {
            layer_2: None,
            other_layers: octets,
        });
    }
    let mut rest = bytes::Bytes::from(octets);
    let layer_2 = crate::common::wire::read(&mut rest)?;
    Ok(IffLayers {
        layer_2: Some(layer_2),
        other_layers: rest.to_vec(),
    })
}

fn write_iff_layers<W: Write + Seek>(
    writer: &mut Writer<W>,
    layers: &IffLayers,
) -> Result<(), DekuError> {
    if let Some(layer_2) = &layers.layer_2 {
        layer_2.to_writer(writer, ())?;
    }
    layers.other_layers.to_writer(writer, ())
}

/// Implemented according to IEEE 1278.1-2012 §7.6.6
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct SupplementalEmissionPdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub infrared_signature_representation_index: u16,
    pub acoustic_signature_representation_index: u16,
    pub radar_cross_section_signature_representation_index: u16,
    #[deku(
        update = "self.propulsion_system_data.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_propulsion_systems)"
    )]
    pub number_of_propulsion_systems: u16,
    #[deku(
        update = "self.vectoring_nozzle_system_data.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_vectoring_nozzle_systems)"
    )]
    pub number_of_vectoring_nozzle_systems: u16,
    #[deku(count = "number_of_propulsion_systems")]
    pub propulsion_system_data: Vec<PropulsionSystemData>,
    #[deku(count = "number_of_vectoring_nozzle_systems")]
    pub vectoring_nozzle_system_data: Vec<VectoringNozzleSystemData>,
}
crate::impl_pdu!(
    SupplementalEmissionPdu,
    PduHeader,
    PduType::SupplementalEmission,
    ProtocolFamily::DistributedEmissionRegeneration
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::{
            constants::BITS_PER_BYTE,
            pdu::{Pdu, to_bytes},
        },
        pdu_tests,
    };
    use bytes::Buf;

    pdu_tests!(
        designator_pdu_tests, DesignatorPdu,
        pdu_type: PduType::Designator,
        default_bits: 704,
        wrong_type: ElectromagneticEmissionsPdu,
    );
    pdu_tests!(
        electromagnetic_emissions_pdu_tests, ElectromagneticEmissionsPdu,
        pdu_type: PduType::ElectromagneticEmissions,
        default_bits: 224,
        wrong_type: UnderwaterAcousticPdu,
    );
    pdu_tests!(
        underwater_acoustic_pdu_tests, UnderwaterAcousticPdu,
        pdu_type: PduType::UnderwaterAcoustic,
        default_bits: 256,
        wrong_type: IFFPdu,
    );
    pdu_tests!(
        iff_pdu_tests, IFFPdu,
        pdu_type: PduType::IFF,
        default_bits: 480,
        wrong_type: SupplementalEmissionPdu,
    );
    pdu_tests!(
        supplemental_emission_pdu_tests, SupplementalEmissionPdu,
        pdu_type: PduType::SupplementalEmission,
        default_bits: 224,
        wrong_type: ElectromagneticEmissionsPdu,
    );
}
