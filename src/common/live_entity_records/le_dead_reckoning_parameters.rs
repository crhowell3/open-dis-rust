//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::{
        enums::DeadReckoningAlgorithm,
        live_entity_records::{
            le_entity_angular_velocity::LEEntityAngularVelocity,
            le_entity_linear_acceleration::LEEntityLinearAcceleration,
        },
    },
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
pub struct LEDeadReckoningParameters {
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    pub entity_linear_acceleration: LEEntityLinearAcceleration,
    pub entity_angular_velocity: LEEntityAngularVelocity,
}

impl LEDeadReckoningParameters {
    #[must_use]
    pub const fn new(
        dead_reckoning_algorithm: DeadReckoningAlgorithm,
        entity_linear_acceleration: LEEntityLinearAcceleration,
        entity_angular_velocity: LEEntityAngularVelocity,
    ) -> Self {
        Self {
            dead_reckoning_algorithm,
            entity_linear_acceleration,
            entity_angular_velocity,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.dead_reckoning_algorithm as u8);
        self.entity_linear_acceleration.serialize(buf);
        self.entity_angular_velocity.serialize(buf);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let dead_reckoning_algorithm = DeadReckoningAlgorithm::deserialize(buf);
        let entity_linear_acceleration = LEEntityLinearAcceleration::deserialize(buf);
        let entity_angular_velocity = LEEntityAngularVelocity::deserialize(buf);
        Self {
            dead_reckoning_algorithm,
            entity_linear_acceleration,
            entity_angular_velocity,
        }
    }
}

impl FieldSerialize for LEDeadReckoningParameters {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for LEDeadReckoningParameters {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for LEDeadReckoningParameters {
    fn field_len(&self) -> usize {
        self.dead_reckoning_algorithm.field_len()
            + self.entity_linear_acceleration.field_len()
            + self.entity_angular_velocity.field_len()
    }
}
