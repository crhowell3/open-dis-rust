//     open-dis-rust - Rust implementation of the IEEE-1278.1-2012 Distributed Interactive Simulation
//                     (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Entity Information / Interaction protocol family

#![allow(deprecated)]

use crate::common::{
    data_types::{
        AttributeRecordSet, DeadReckoningParameters, EntityCoordinateVector, EntityId,
        EntityMarking, EntityType, EulerAngles, EventId, LinearVelocity, SimulationAddress,
        VariableParameter, WorldCoordinate,
    },
    enums::{DISAttributeActionCode, ForceId, PduType, ProtocolFamily},
    pdu_header::PduHeader,
};

/// Implemented according to IEEE 1278.1-2012 §7.2.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct EntityStatePdu {
    #[deku(skip)]
    header: PduHeader,
    pub entity_id: EntityId,
    pub force_id: ForceId,
    #[deku(update = "self.articulation_parameter.len()")]
    pub number_of_articulation_parameters: u8,
    pub entity_type: EntityType,
    pub alternative_entity_type: EntityType,
    pub entity_linear_velocity: LinearVelocity,
    pub entity_location: WorldCoordinate,
    pub entity_orientation: EulerAngles,
    pub entity_appearance: u32,
    pub dead_reckoning_parameters: DeadReckoningParameters,
    pub entity_marking: EntityMarking,
    // SISO-REF-010 UID 55: a set of capability bits whose meaning
    // depends on the entity kind and domain.
    pub entity_capabilities: u32,
    #[deku(count = "number_of_articulation_parameters")]
    pub articulation_parameter: Vec<VariableParameter>,
}
crate::impl_pdu!(
    EntityStatePdu,
    PduHeader,
    PduType::EntityState,
    ProtocolFamily::EntityInformation
);

/// Implemented according to IEEE 1278.1-2012 §7.2.3
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct CollisionPdu {
    #[deku(skip)]
    header: PduHeader,
    pub issuing_entity_id: EntityId,
    pub colliding_entity_id: EntityId,
    pub event_id: EventId,
    pub collision_type: u8,
    padding: u8,
    pub velocity: LinearVelocity,
    pub mass: f32,
    pub location_wrt_entity: EntityCoordinateVector,
}
crate::impl_pdu!(
    CollisionPdu,
    PduHeader,
    PduType::Collision,
    ProtocolFamily::EntityInformation
);

/// Implemented according to IEEE 1278.1-2012 §7.2.4
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct CollisionElasticPdu {
    #[deku(skip)]
    header: PduHeader,
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
crate::impl_pdu!(
    CollisionElasticPdu,
    PduHeader,
    PduType::CollisionElastic,
    ProtocolFamily::EntityInformation
);

/// Implemented according to IEEE 1278.1-2012 §7.2.5
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct EntityStateUpdatePdu {
    #[deku(skip)]
    header: PduHeader,
    pub entity_id: EntityId,
    padding: u8,
    #[deku(update = "self.variable_parameter_records.len()")]
    pub number_of_variable_parameters: u8,
    pub entity_linear_velocity: LinearVelocity,
    pub entity_location: WorldCoordinate,
    pub entity_orientation: EulerAngles,
    pub entity_appearance: u32,
    #[deku(count = "number_of_variable_parameters")]
    pub variable_parameter_records: Vec<VariableParameter>,
}
crate::impl_pdu!(
    EntityStateUpdatePdu,
    PduHeader,
    PduType::EntityStateUpdate,
    ProtocolFamily::EntityInformation
);

/// Implemented according to IEEE 1278.1-2012 §7.2.6
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct AttributePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_simulation_address: SimulationAddress,
    padding: u32,
    padding2: u16,
    pub attribute_record_pdu_type: u8,
    pub attribute_record_protocol_version: u8,
    pub master_attribute_record_type: u32,
    pub action_code: DISAttributeActionCode,
    padding3: u8,
    #[deku(
        update = "crate::common::wire::update_each(&mut self.attribute_record_sets)?",
        assert = "crate::common::wire::fits_in_pdu(*number_of_attribute_record_sets)"
    )]
    pub number_of_attribute_record_sets: u16,
    #[deku(count = "number_of_attribute_record_sets")]
    pub attribute_record_sets: Vec<AttributeRecordSet>,
}
crate::impl_pdu!(
    AttributePdu,
    PduHeader,
    PduType::Attribute,
    ProtocolFamily::EntityInformation
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::{constants::BITS_PER_BYTE, pdu::Pdu},
        pdu_tests,
    };
    use bytes::{Buf, Bytes, BytesMut};

    /// Serialize a PDU, failing the test if serialization returns an error.
    pub fn to_bytes<P: Pdu>(pdu: &mut P) -> Bytes {
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
