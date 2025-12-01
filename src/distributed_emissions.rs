//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Distributed Emission Regeneration protocol family

#![allow(deprecated)]

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            EntityCoordinateVector, EntityId, EventId, LinearAcceleration, WorldCoordinate,
            acoustic_emitter_system::AcousticEmitterSystem, apa_data::ApaData, beam_data::BeamData,
            electromagnetic_emission_system_data::ElectromagneticEmissionSystemData,
            fundamental_operational_data::FundamentalOperationalData,
            iff_fundamental_parameter_data::IFFFundamentalParameterData, layer_header::LayerHeader,
            propulsion_system_data::PropulsionSystemData,
            secondary_operational_data::SecondaryOperationalData, shaft_rpms::ShaftRPMs,
            system_id::SystemId, vectoring_nozzle_system_data::VectoringNozzleSystemData,
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
        pdu_type: PduType::ElectromagneticEmission,
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
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    mod designator_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = DesignatorPdu::new();
            let any_pdu = pdu.as_any();
            assert!(any_pdu.is::<DesignatorPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = DesignatorPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = DesignatorPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header(), pdu.header());
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 704 / BITS_PER_BYTE;
            let pdu = DesignatorPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod electromagnetic_emissions_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = ElectromagneticEmissionsPdu::new();
            let any_pdu = pdu.as_any();
            assert!(any_pdu.is::<ElectromagneticEmissionsPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = ElectromagneticEmissionsPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                ElectromagneticEmissionsPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header(), pdu.header());
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
            let pdu = ElectromagneticEmissionsPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod underwater_acoustic_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = UnderwaterAcousticPdu::new();
            let any_pdu = pdu.as_any();
            assert!(any_pdu.is::<UnderwaterAcousticPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = UnderwaterAcousticPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                UnderwaterAcousticPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header(), pdu.header());
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
            let pdu = UnderwaterAcousticPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod iff_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = IFFPdu::new();
            let any_pdu = pdu.as_any();
            assert!(any_pdu.is::<IFFPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = IFFPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = IFFPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header(), pdu.header());
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 704 / BITS_PER_BYTE;
            let pdu = IFFPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod supplemental_emission_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = SupplementalEmissionPdu::new();
            let any_pdu = pdu.as_any();
            assert!(any_pdu.is::<SupplementalEmissionPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = SupplementalEmissionPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                SupplementalEmissionPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header(), pdu.header());
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
            let pdu = SupplementalEmissionPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }
}
