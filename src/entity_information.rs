//     open-dis-rust - Rust implementation of the IEEE-1278.1-2012 Distributed Interactive Simulation
//                     (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Entity Information / Interaction protocol family

#![allow(deprecated)]

use crate::common::data_types::attribute_record_set::AttributeRecordSet;
use crate::common::data_types::dead_reckoning_parameters::DeadReckoningParameters;
use crate::common::data_types::entity_marking::EntityMarking;
use crate::common::data_types::{
    EntityCoordinateVector, EntityId, EntityType, EulerAngles, EventId, LinearVelocity,
    SimulationAddress, VariableParameter, WorldCoordinate,
};
use crate::common::enums::{EntityCapabilities, ForceId};
use crate::{
    common::{
        GenericHeader, SerializedLength,
        enums::{DISAttributeActionCode, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.2.2
    pub struct EntityStatePdu {
        header: PduHeader,
        pdu_type: PduType::EntityState,
        protocol_family: ProtocolFamily::EntityInformation,
        fields: {
            pub entity_id: EntityId,
            pub force_id: ForceId,
            pub number_of_articulation_parameters: u8,
            pub entity_type: EntityType,
            pub alternative_entity_type: EntityType,
            pub entity_linear_velocity: LinearVelocity,
            pub entity_location: WorldCoordinate,
            pub entity_orientation: EulerAngles,
            pub entity_appearance: u32,
            pub dead_reckoning_parameters: DeadReckoningParameters,
            pub entity_marking: EntityMarking,
            pub entity_capabilities: EntityCapabilities,
            pub articulation_parameter: Vec<VariableParameter>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.2.3
    pub struct CollisionPdu {
        header: PduHeader,
        pdu_type: PduType::Collision,
        protocol_family: ProtocolFamily::EntityInformation,
        fields: {
            pub issuing_entity_id: EntityId,
            pub colliding_entity_id: EntityId,
            pub event_id: EventId,
            pub collision_type: u8,
            padding: u8,
            pub velocity: LinearVelocity,
            pub mass: f32,
            pub location_wrt_entity: EntityCoordinateVector,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.2.4
    pub struct CollisionElasticPdu {
        header: PduHeader,
        pdu_type: PduType::CollisionElastic,
        protocol_family: ProtocolFamily::EntityInformation,
        fields: {
            pub issuing_entity_id: EntityId,
            pub colliding_entity_id: EntityId,
            pub event_id: EventId,
            padding: u16,
            pub contact_velocity: LinearVelocity,
            pub mass: f32,
            pub location_of_impact: EntityCoordinateVector,
            pub collision_intermediate_result_xx: f32,
            pub collision_intermediate_result_xy: f32,
            pub collision_intermediate_result_xz: f32,
            pub collision_intermediate_result_yy: f32,
            pub collision_intermediate_result_yz: f32,
            pub collision_intermediate_result_zz: f32,
            pub unit_surface_normal: EntityCoordinateVector,
            pub coefficient_of_restitution: f32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.2.5
    pub struct EntityStateUpdatePdu {
        header: PduHeader,
        pdu_type: PduType::EntityStateUpdate,
        protocol_family: ProtocolFamily::EntityInformation,
        fields: {
            pub entity_id: EntityId,
            padding: u8,
            pub number_of_variable_parameters: u8,
            pub entity_linear_velocity: LinearVelocity,
            pub entity_location: WorldCoordinate,
            pub entity_orientation: EulerAngles,
            pub entity_appearance: u32,
            pub variable_parameter_records: Vec<VariableParameter>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.2.6
    pub struct AttributePdu {
        header: PduHeader,
        pdu_type: PduType::Attribute,
        protocol_family: ProtocolFamily::EntityInformation,
        fields: {
            pub originating_simulation_address: SimulationAddress,
            padding: u32,
            padding2: u16,
            pub attribute_record_pdu_type: u8,
            pub attribute_record_protocol_version: u8,
            pub master_attribute_record_type: u32,
            pub action_code: DISAttributeActionCode,
            padding3: u8,
            pub number_of_attribute_record_sets: u16,
            pub attribute_record_sets: Vec<AttributeRecordSet>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    mod entity_state_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = EntityStatePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<EntityStatePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = EntityStatePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = EntityStatePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 1152 / BITS_PER_BYTE;
            let pdu = EntityStatePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod collision_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = CollisionPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<CollisionPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = CollisionPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = CollisionPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 480 / BITS_PER_BYTE;
            let pdu = CollisionPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod collision_elastic_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = CollisionElasticPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<CollisionElasticPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = CollisionElasticPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                CollisionElasticPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 800 / BITS_PER_BYTE;
            let pdu = CollisionElasticPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod entity_state_update_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = EntityStateUpdatePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<EntityStateUpdatePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = EntityStateUpdatePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                EntityStateUpdatePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 576 / BITS_PER_BYTE;
            let pdu = EntityStateUpdatePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod attribute_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = AttributePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<AttributePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = AttributePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = AttributePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
            let pdu = AttributePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }
}
