//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

//! The Warfare protocol family

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            ClockTime, EntityCoordinateVector, EntityId, EntityType, EventId, LinearVelocity,
            VariableParameter, WorldCoordinate, directed_energy_damage::DirectedEnergyDamage,
            munition_descriptor::MunitionDescriptor,
        },
        enums::{DEFirePulseShape, DetonationResult, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.3.2
    pub struct FirePdu {
        header: PduHeader,
        pdu_type: PduType::Fire,
        protocol_family: ProtocolFamily::Warfare,
        fields: {
            pub firing_entity_id: EntityId,
            pub target_entity_id: EntityId,
            pub munition_expendable_id: EntityId,
            pub event_id: EventId,
            pub fire_mission_index: u32,
            pub location_in_world_coordinates: WorldCoordinate,
            pub descriptor: MunitionDescriptor,
            pub velocity: LinearVelocity,
            pub range: f32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.3.3
    pub struct DetonationPdu {
        header: PduHeader,
        pdu_type: PduType::Detonation,
        protocol_family: ProtocolFamily::Warfare,
        fields: {
            pub firing_entity_id: EntityId,
            pub target_entity_id: EntityId,
            pub exploding_entity_id: EntityId,
            pub event_id: EventId,
            pub velocity: LinearVelocity,
            pub location_in_world_coordinates: WorldCoordinate,
            pub descriptor: MunitionDescriptor,
            pub location_in_entitys_coordinates: EntityCoordinateVector,
            pub detonation_result: DetonationResult,
            pub number_of_variable_parameters: u8,
            padding: u16,
            pub variable_parameters: Vec<VariableParameter>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.3.4
    pub struct DirectedEnergyFirePdu {
        header: PduHeader,
        pdu_type: PduType::DirectedEnergyFire,
        protocol_family: ProtocolFamily::Warfare,
        fields: {
            pub firing_entity_id: EntityId,
            pub event_id: EventId,
            pub munition_type: EntityType,
            pub shot_start_time: ClockTime,
            pub cumulative_shot_time: f32,
            pub aperture_emitter_location: EntityCoordinateVector,
            pub aperture_diameter: f32,
            pub wavelength: f32,
            padding: u32,
            pub pulse_repetition_frequency: f32,
            pub pulse_width: f32,
            pub flags: u16,
            pub pulse_shape: DEFirePulseShape,
            padding2: u8,
            padding3: u32,
            padding4: u16,
            pub number_of_de_records: u16,
            pub damage_descriptions: Vec<DirectedEnergyDamage>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.3.5
    pub struct EntityDamageStatusPdu {
        header: PduHeader,
        pdu_type: PduType::EntityDamageStatus,
        protocol_family: ProtocolFamily::Warfare,
        fields: {
            pub damaged_entity_id: EntityId,
            padding: u16,
            padding2: u16,
            pub number_of_damage_descriptions: u16,
            pub damage_descriptions: Vec<DirectedEnergyDamage>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    mod fire_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let fire_pdu = FirePdu::new();
            let any_pdu = fire_pdu.as_any();

            assert!(any_pdu.is::<FirePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = FirePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = FirePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 768 / 8;
            let pdu = FirePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod detonation_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let detonation_pdu = DetonationPdu::new();
            let any_pdu = detonation_pdu.as_any();

            assert!(any_pdu.is::<DetonationPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = DetonationPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = DetonationPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 832 / BITS_PER_BYTE;
            let pdu = DetonationPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod directed_energy_fire_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = DirectedEnergyFirePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<DirectedEnergyFirePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = DirectedEnergyFirePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                DirectedEnergyFirePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 704 / BITS_PER_BYTE;
            let pdu = DirectedEnergyFirePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod entity_damage_status_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let entity_damage_status_pdu = EntityDamageStatusPdu::new();
            let any_pdu = entity_damage_status_pdu.as_any();

            assert!(any_pdu.is::<EntityDamageStatusPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = EntityDamageStatusPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                EntityDamageStatusPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 192 / 8;
            let pdu = EntityDamageStatusPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }
}
