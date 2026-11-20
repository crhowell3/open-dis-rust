//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Entity Information / Interaction protocol family

#![allow(deprecated)]

use crate::records::{
    AngularVelocityVector, Attribute, EntityAppearance, EntityCapabilities, EntityIdentifier,
    EntityMarking, EntityType, EulerAngles, EventIdentifier, PduHeader, SimulationAddress,
    VariableParameter, Vector, WorldCoordinates,
};
use crate::siso::enums::{
    DISAttributeActionCode, DeadReckoningAlgorithm, ForceId, PduType, ProtocolFamily,
};

/// How receivers should extrapolate an entity's position and orientation between
/// Entity State PDUs.
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct DeadReckoningParameters {
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    pub dead_reckoning_other_parameters: [u8; 15],
    pub entity_linear_acceleration: Vector,
    pub entity_angular_velocity: AngularVelocityVector,
}

impl DeadReckoningParameters {
    #[must_use]
    pub const fn new(
        dead_reckoning_algorithm: DeadReckoningAlgorithm,
        entity_linear_acceleration: Vector,
        entity_angular_velocity: AngularVelocityVector,
    ) -> Self {
        Self {
            dead_reckoning_algorithm,
            dead_reckoning_other_parameters: [0u8; 15],
            entity_linear_acceleration,
            entity_angular_velocity,
        }
    }
}

/// The attribute records an Attribute PDU carries for one entity or object.
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AttributeRecordSet {
    pub entity_id: EntityIdentifier,
    #[deku(
        update = "crate::wire::update_each(&mut self.attribute_records)?",
        assert = "crate::wire::fits_in_pdu(*number_of_attribute_records)"
    )]
    pub number_of_attribute_records: u16,
    #[deku(count = "number_of_attribute_records")]
    pub attribute_records: Vec<Attribute>,
}

impl AttributeRecordSet {
    #[must_use]
    pub fn new() -> Self {
        Self {
            entity_id: EntityIdentifier::default(),
            number_of_attribute_records: 0u16,
            attribute_records: vec![],
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §7.2.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct EntityStatePdu {
    #[deku(skip)]
    header: PduHeader,
    pub entity_id: EntityIdentifier,
    pub force_id: ForceId,
    #[deku(update = "self.variable_parameter_records.len()")]
    pub number_of_variable_parameter_records: u8,
    pub entity_type: EntityType,
    pub alternative_entity_type: EntityType,
    pub entity_linear_velocity: Vector,
    pub entity_location: WorldCoordinates,
    pub entity_orientation: EulerAngles,
    pub entity_appearance: EntityAppearance,
    pub dead_reckoning_parameters: DeadReckoningParameters,
    pub entity_marking: EntityMarking,
    pub entity_capabilities: EntityCapabilities,
    #[deku(count = "number_of_variable_parameter_records")]
    pub variable_parameter_records: Vec<VariableParameter>,
}
crate::pdu::impl_pdu!(
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
    pub issuing_entity_id: EntityIdentifier,
    pub colliding_entity_id: EntityIdentifier,
    pub event_id: EventIdentifier,
    pub collision_type: u8,
    padding: u8,
    pub velocity: Vector,
    pub mass: f32,
    pub location_wrt_entity: Vector,
}
crate::pdu::impl_pdu!(
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
    pub issuing_entity_id: EntityIdentifier,
    pub colliding_entity_id: EntityIdentifier,
    pub event_id: EventIdentifier,
    padding: u16,
    pub contact_velocity: Vector,
    pub mass: f32,
    pub location_of_impact: Vector,
    pub collision_intermediate_result_xx: f32,
    pub collision_intermediate_result_xy: f32,
    pub collision_intermediate_result_xz: f32,
    pub collision_intermediate_result_yy: f32,
    pub collision_intermediate_result_yz: f32,
    pub collision_intermediate_result_zz: f32,
    pub unit_surface_normal: Vector,
    pub coefficient_of_restitution: f32,
}
crate::pdu::impl_pdu!(
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
    pub entity_id: EntityIdentifier,
    padding: u8,
    #[deku(update = "self.variable_parameter_records.len()")]
    pub number_of_variable_parameters: u8,
    pub entity_linear_velocity: Vector,
    pub entity_location: WorldCoordinates,
    pub entity_orientation: EulerAngles,
    pub entity_appearance: EntityAppearance,
    #[deku(count = "number_of_variable_parameters")]
    pub variable_parameter_records: Vec<VariableParameter>,
}
crate::pdu::impl_pdu!(
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
        update = "crate::wire::update_each(&mut self.attribute_record_sets)?",
        assert = "crate::wire::fits_in_pdu(*number_of_attribute_record_sets)"
    )]
    pub number_of_attribute_record_sets: u16,
    #[deku(count = "number_of_attribute_record_sets")]
    pub attribute_record_sets: Vec<AttributeRecordSet>,
}
crate::pdu::impl_pdu!(
    AttributePdu,
    PduHeader,
    PduType::Attribute,
    ProtocolFamily::EntityInformation
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
