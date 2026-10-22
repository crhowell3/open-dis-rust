//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Distributed Emission Regeneration protocol family

#![allow(deprecated)]

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            AcousticEmitterSystem, ApaData, BeamData, ElectromagneticEmissionSystemData,
            EntityCoordinateVector, EntityId, EventId, FundamentalOperationalData,
            IFFFundamentalParameterData, LayerHeader, LinearAcceleration, PropulsionSystemData,
            SecondaryOperationalData, ShaftRPMs, SystemId, VectoringNozzleSystemData,
            WorldCoordinate,
        },
        enums::{
            DeadReckoningAlgorithm, DesignatorCode, DesignatorSystemName,
            EEAttributeStateIndicator, PduType, ProtocolFamily, UAPassiveParameterIndex,
            UAStateChangeUpdateIndicator,
        },
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.6.2
    pub struct ElectromagneticEmissionsPdu {
        header: PduHeader,
        pdu_type: PduType::ElectromagneticEmissions,
        protocol_family: ProtocolFamily::DistributedEmissionRegeneration,
        fields: {
            pub emitting_entity_id: EntityId,
            pub event_id: EventId,
            pub state_update_indicator: EEAttributeStateIndicator,
            pub number_of_systems: u8,
            padding: u16,
            pub systems: Vec<ElectromagneticEmissionSystemData>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.6.3
    pub struct DesignatorPdu {
        header: PduHeader,
        pdu_type: PduType::Designator,
        protocol_family: ProtocolFamily::DistributedEmissionRegeneration,
        fields: {
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
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.6.4
    pub struct UnderwaterAcousticPdu {
        header: PduHeader,
        pdu_type: PduType::UnderwaterAcoustic,
        protocol_family: ProtocolFamily::DistributedEmissionRegeneration,
        fields: {
            pub emitting_entity_id: EntityId,
            pub event_id: EventId,
            pub state_change_update_indicator: UAStateChangeUpdateIndicator,
            padding: u8,
            pub passive_parameter_index: UAPassiveParameterIndex,
            pub propulsion_plant_configuration: u8,
            pub number_of_shafts: u8,
            pub number_of_apas: u8,
            pub number_of_ua_emitter_systems: u8,
            pub shaft_rpms: Vec<ShaftRPMs>,
            pub apa_data: Vec<ApaData>,
            pub emitter_systems: Vec<AcousticEmitterSystem>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.6.5
    pub struct IFFPdu {
        header: PduHeader,
        pdu_type: PduType::IFF,
        protocol_family: ProtocolFamily::DistributedEmissionRegeneration,
        fields: {
            pub emitting_entity_id: EntityId,
            pub event_id: EventId,
            pub relative_antenna_location: EntityCoordinateVector,
            pub system_id: SystemId,
            pub system_designator: u8,
            pub system_specific_data: u8,
            pub fundamental_operational_data: FundamentalOperationalData,
            pub layer_header: LayerHeader,
            pub beam_data: BeamData,
            pub secondary_operational_data: SecondaryOperationalData,
            pub iff_parameters: Vec<IFFFundamentalParameterData>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.6.6
    pub struct SupplementalEmissionPdu {
        header: PduHeader,
        pdu_type: PduType::SupplementalEmission,
        protocol_family: ProtocolFamily::DistributedEmissionRegeneration,
        fields: {
            pub originating_entity_id: EntityId,
            pub infrared_signature_representation_index: u16,
            pub acoustic_signature_representation_index: u16,
            pub radar_cross_section_signature_representation_index: u16,
            pub number_of_propulsion_systems: u16,
            pub number_of_vectoring_nozzle_systems: u16,
            pub propulsion_system_data: Vec<PropulsionSystemData>,
            pub vectoring_nozzle_system_data: Vec<VectoringNozzleSystemData>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{common::pdu::Pdu, pdu_tests};

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
        default_bits: 704,
        wrong_type: SupplementalEmissionPdu,
    );
    pdu_tests!(
        supplemental_emission_pdu_tests, SupplementalEmissionPdu,
        pdu_type: PduType::SupplementalEmission,
        default_bits: 224,
        wrong_type: ElectromagneticEmissionsPdu,
    );
}
