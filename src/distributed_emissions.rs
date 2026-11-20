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

use crate::records::{
    AcousticEmitterSystem, BeamData, EmitterSystem, EntityIdentifier, EventIdentifier,
    FundamentalOperationalData, IFFFundamentalParameterData, LayerHeader, PduHeader,
    PropulsionSystemData, SecondaryOperationalData, SystemIdentifier, TrackJamData, Vector,
    VectoringNozzleSystemData, WorldCoordinates,
};
use crate::siso::enums::{
    DeadReckoningAlgorithm, DesignatorCode, DesignatorSystemName, EEAttributeStateIndicator,
    PduType, ProtocolFamily, UAPassiveParameterIndex, UAStateChangeUpdateIndicator,
};

/// The fundamental parameters of an Electromagnetic Emission beam: its frequency, ERP,
/// pulse rate, pulse width, and scan.
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct FundamentalParameterData {
    pub frequency: f32,
    pub frequency_range: f32,
    pub effective_radiated_power: f32,
    pub pulse_repetition_frequency: f32,
    pub pulse_width: f32,
    pub beam_azimuth_center: f32,
    pub beam_azimuth_sweep: f32,
    pub beam_elevation_center: f32,
    pub beam_elevation_sweep: f32,
    pub beam_sweep_sync: f32,
}

/// One beam of an emitter system in an Electromagnetic Emission PDU.
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ElectromagneticEmissionBeamData {
    /// This beam's length in 32-bit words, or 0 when it is over 255 words.
    /// `serialize` sets it.
    #[deku(update = "crate::wire::length_in_words(crate::wire::encoded_len(self)?)")]
    pub beam_data_length: u8,
    pub beam_id_number: u8,
    pub beam_parameter_index: u16,
    pub fundamental_parameter_data: FundamentalParameterData,
    pub beam_function: u8,
    #[deku(update = "self.track_jam_targets.len()")]
    pub number_of_track_jam_targets: u8,
    pub high_density_track_jam: u8,
    pub pad4: u8,
    pub jamming_mode_sequence: u32,
    #[deku(count = "number_of_track_jam_targets")]
    pub track_jam_targets: Vec<TrackJamData>,
}

/// One emitter system in an Electromagnetic Emission PDU, with its beams.
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ElectromagneticEmissionSystemData {
    /// This emitter system's length in 32-bit words, beams included, or 0
    /// when it is over 255 words. `serialize` sets it
    #[deku(update = "crate::wire::length_in_words(crate::wire::encoded_len(self)?)")]
    pub system_data_length: u8,
    #[deku(update = "crate::wire::update_each(&mut self.beam_data_records)?")]
    pub number_of_beams: u8,
    pub emissionspadding2: u16,
    pub emitter_system: EmitterSystem,
    pub location: Vector,
    #[deku(count = "number_of_beams")]
    pub beam_data_records: Vec<ElectromagneticEmissionBeamData>,
}

/// Implemented according to IEEE 1278.1-2012 §7.6.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct ElectromagneticEmissionsPdu {
    #[deku(skip)]
    header: PduHeader,
    pub emitting_entity_id: EntityIdentifier,
    pub event_id: EventIdentifier,
    pub state_update_indicator: EEAttributeStateIndicator,
    #[deku(update = "crate::wire::update_each(&mut self.systems)?")]
    pub number_of_systems: u8,
    padding: u16,
    #[deku(count = "number_of_systems")]
    pub systems: Vec<ElectromagneticEmissionSystemData>,
}
crate::pdu::impl_pdu!(
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
    pub designating_entity_id: EntityIdentifier,
    pub code_name: DesignatorSystemName,
    pub designated_entity_id: EntityIdentifier,
    pub designator_code: DesignatorCode,
    pub designator_power: f32,
    pub designator_wavelength: f32,
    pub designator_spot_wrt_designated: Vector,
    pub designator_spot_location: WorldCoordinates,
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    padding: u8,
    padding2: u16,
    pub entity_linear_acceleration: Vector,
}
crate::pdu::impl_pdu!(
    DesignatorPdu,
    PduHeader,
    PduType::Designator,
    ProtocolFamily::DistributedEmissionRegeneration
);

/// The fundamental parameters of an acoustic beam.
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AcousticBeamFundamentalParameter {
    pub active_emission_parameter_index: u16,
    pub scan_pattern: u16,
    pub beam_center_azimuth: f32,
    pub azimuthal_beamwidth: f32,
    pub beam_center_de: f32,
    pub de_beamwidth: f32,
}

impl AcousticBeamFundamentalParameter {
    #[must_use]
    pub const fn new(
        active_emission_parameter_index: u16,
        scan_pattern: u16,
        beam_center_azimuth: f32,
        azimuthal_beamwidth: f32,
        beam_center_de: f32,
        de_beamwidth: f32,
    ) -> Self {
        Self {
            active_emission_parameter_index,
            scan_pattern,
            beam_center_azimuth,
            azimuthal_beamwidth,
            beam_center_de,
            de_beamwidth,
        }
    }
}

/// One beam of an acoustic emitter system in an Underwater Acoustic PDU.
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AcousticBeamData {
    /// This beam's length in 32-bit words. `serialize` sets it.
    #[deku(update = "crate::wire::length_in_words(crate::wire::encoded_len(self)?)")]
    pub beam_data_length: u8,
    pub beam_id_number: u8,
    pub pad2: u16,
    pub fundamental_data_parameters: AcousticBeamFundamentalParameter,
}

impl AcousticBeamData {
    #[must_use]
    pub const fn new(
        beam_data_length: u8,
        beam_id_number: u8,
        pad2: u16,
        fundamental_data_parameters: AcousticBeamFundamentalParameter,
    ) -> Self {
        Self {
            beam_data_length,
            beam_id_number,
            pad2,
            fundamental_data_parameters,
        }
    }
}

/// One acoustic emitter system in an Underwater Acoustic PDU with its beams.
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AcousticEmitterSystemData {
    /// This emitter system's length in 32-bit words, beams included.
    /// `serialize` sets it
    #[deku(update = "crate::wire::length_in_words(crate::wire::encoded_len(self)?)")]
    pub emitter_system_data_length: u8,
    #[deku(update = "crate::wire::update_each(&mut self.beam_records)?")]
    pub number_of_beams: u8,
    pub pad2: u16,
    pub acoustic_emitter_system: AcousticEmitterSystem,
    pub emitter_location: Vector,
    #[deku(count = "number_of_beams")]
    pub beam_records: Vec<AcousticBeamData>,
}

impl AcousticEmitterSystemData {
    #[must_use]
    pub const fn new(
        emitter_system_data_length: u8,
        pad2: u16,
        acoustic_emitter_system: AcousticEmitterSystem,
        emitter_location: Vector,
        beam_records: Vec<AcousticBeamData>,
    ) -> Self {
        Self {
            emitter_system_data_length,
            number_of_beams: 0,
            pad2,
            acoustic_emitter_system,
            emitter_location,
            beam_records,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §7.6.4 Table 163
pub struct ApaData {
    pub parameter_index: u16,
    pub parameter_value: i16,
}

impl ApaData {
    #[must_use]
    pub const fn new(parameter_index: u16, parameter_value: i16) -> Self {
        Self {
            parameter_index,
            parameter_value,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §7.6.4(j-l)
pub struct ShaftRPMs {
    pub current_shaft_rpms: i16,
    pub ordered_shaft_rpms: i16,
    pub shaft_rpm_rate_of_change: i32,
}

impl ShaftRPMs {
    #[must_use]
    pub const fn new(
        current_shaft_rpms: i16,
        ordered_shaft_rpms: i16,
        shaft_rpm_rate_of_change: i32,
    ) -> Self {
        Self {
            current_shaft_rpms,
            ordered_shaft_rpms,
            shaft_rpm_rate_of_change,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §7.6.4
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct UnderwaterAcousticPdu {
    #[deku(skip)]
    header: PduHeader,
    pub emitting_entity_id: EntityIdentifier,
    pub event_id: EventIdentifier,
    pub state_change_update_indicator: UAStateChangeUpdateIndicator,
    padding: u8,
    pub passive_parameter_index: UAPassiveParameterIndex,
    pub propulsion_plant_configuration: u8,
    #[deku(update = "self.shaft_rpms.len()")]
    pub number_of_shafts: u8,
    #[deku(update = "self.apa_data.len()")]
    pub number_of_apas: u8,
    #[deku(update = "crate::wire::update_each(&mut self.emitter_systems)?")]
    pub number_of_ua_emitter_systems: u8,
    #[deku(count = "number_of_shafts")]
    pub shaft_rpms: Vec<ShaftRPMs>,
    #[deku(count = "number_of_apas")]
    pub apa_data: Vec<ApaData>,
    #[deku(count = "number_of_ua_emitter_systems")]
    /// Each emitter with its location and beams.
    pub emitter_systems: Vec<AcousticEmitterSystemData>,
}
crate::pdu::impl_pdu!(
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
    pub emitting_entity_id: EntityIdentifier,
    pub event_id: EventIdentifier,
    pub relative_antenna_location: Vector,
    pub system_id: SystemIdentifier,
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
crate::pdu::impl_pdu!(
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
        assert = "crate::wire::fits_in_pdu(secondary_operational_data.number_of_iff_fundamental_parameter_records)"
    )]
    pub secondary_operational_data: SecondaryOperationalData,
    #[deku(count = "secondary_operational_data.number_of_iff_fundamental_parameter_records")]
    pub iff_parameters: Vec<IFFFundamentalParameterData>,
}

impl IffLayer2 {
    fn derived_layer_header(&self) -> Result<LayerHeader, DekuError> {
        let length = u16::try_from(crate::wire::encoded_len(self)?)
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
    let mut rest: &[u8] = &octets;
    let layer_2 = crate::wire::read(&mut rest)?;
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
    pub originating_entity_id: EntityIdentifier,
    pub infrared_signature_representation_index: u16,
    pub acoustic_signature_representation_index: u16,
    pub radar_cross_section_signature_representation_index: u16,
    #[deku(
        update = "self.propulsion_system_data.len()",
        assert = "crate::wire::fits_in_pdu(*number_of_propulsion_systems)"
    )]
    pub number_of_propulsion_systems: u16,
    #[deku(
        update = "self.vectoring_nozzle_system_data.len()",
        assert = "crate::wire::fits_in_pdu(*number_of_vectoring_nozzle_systems)"
    )]
    pub number_of_vectoring_nozzle_systems: u16,
    #[deku(count = "number_of_propulsion_systems")]
    pub propulsion_system_data: Vec<PropulsionSystemData>,
    #[deku(count = "number_of_vectoring_nozzle_systems")]
    pub vectoring_nozzle_system_data: Vec<VectoringNozzleSystemData>,
}
crate::pdu::impl_pdu!(
    SupplementalEmissionPdu,
    PduHeader,
    PduType::SupplementalEmission,
    ProtocolFamily::DistributedEmissionRegeneration
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{pdu::Pdu, pdu::pdu_tests};

    /// Serialize a PDU, failing the test if serialization returns an error.
    pub fn to_bytes<P: Pdu>(pdu: &mut P) -> Vec<u8> {
        pdu.to_bytes().expect("serialize failed")
    }

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
