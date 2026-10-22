//     open-dis-rust - Rust implementation of the IEEE-1278.1-2012 Distributed Interactive Simulation
//                     (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Entity Information / Interaction protocol family

#![allow(deprecated)]

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            AttributeRecordSet, DeadReckoningParameters, EntityCoordinateVector, EntityId,
            EntityMarking, EntityType, EulerAngles, EventId, LinearVelocity, SimulationAddress,
            VariableParameter, WorldCoordinate,
        },
        enums::{DISAttributeActionCode, EntityCapabilities, ForceId, PduType, ProtocolFamily},
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
    use crate::{
        common::{constants::BITS_PER_BYTE, pdu::Pdu},
        pdu_tests,
    };
    use bytes::{Buf, Bytes, BytesMut};

    /// Serialize a PDU, failing the test if serialization returns an error.
    fn to_bytes<P: Pdu>(pdu: &mut P) -> Bytes {
        let mut buf = BytesMut::new();
        pdu.serialize(&mut buf).expect("serialize failed");
        buf.freeze()
    }

    pdu_tests!(
        entity_state_pdu_tests, EntityStatePdu,
        pdu_type: PduType::EntityState,
        default_bits: 1152,
        wrong_type: CollisionPdu,
    );
    pdu_tests!(
        collision_pdu_tests, CollisionPdu,
        pdu_type: PduType::Collision,
        default_bits: 480,
        wrong_type: CollisionElasticPdu,
    );
    pdu_tests!(
        collision_elastic_pdu_tests, CollisionElasticPdu,
        pdu_type: PduType::CollisionElastic,
        default_bits: 800,
        wrong_type: EntityStateUpdatePdu,
    );
    pdu_tests!(
        entity_state_update_pdu_tests, EntityStateUpdatePdu,
        pdu_type: PduType::EntityStateUpdate,
        default_bits: 576,
        wrong_type: AttributePdu,
    );
    pdu_tests!(
        attribute_pdu_tests, AttributePdu,
        pdu_type: PduType::Attribute,
        default_bits: 256,
        wrong_type: CollisionPdu,
    );
}
