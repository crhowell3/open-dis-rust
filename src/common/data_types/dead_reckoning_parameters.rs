//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::{
        SerializedLength,
        data_types::{
            angular_velocity_vector::AngularVelocity, linear_acceleration::LinearAcceleration,
        },
        enums::DeadReckoningAlgorithm,
    },
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
pub struct DeadReckoningParameters {
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    pub dead_reckoning_other_parameters: [u8; 15],
    pub entity_linear_acceleration: LinearAcceleration,
    pub entity_angular_velocity: AngularVelocity,
}

impl DeadReckoningParameters {
    #[must_use]
    pub const fn new(
        dead_reckoning_algorithm: DeadReckoningAlgorithm,
        entity_linear_acceleration: LinearAcceleration,
        entity_angular_velocity: AngularVelocity,
    ) -> Self {
        Self {
            dead_reckoning_algorithm,
            dead_reckoning_other_parameters: [0u8; 15],
            entity_linear_acceleration,
            entity_angular_velocity,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.dead_reckoning_algorithm as u8);
        buf.put_bytes(0u8, 15);
        self.entity_linear_acceleration.serialize(buf);
        self.entity_angular_velocity.serialize(buf);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let dead_reckoning_algorithm = DeadReckoningAlgorithm::deserialize(buf);
        let mut dead_reckoning_other_parameters: [u8; 15] = [0; 15];
        for param in &mut dead_reckoning_other_parameters {
            *param = buf.get_u8();
        }
        let entity_linear_acceleration = LinearAcceleration::deserialize(buf);
        let entity_angular_velocity = AngularVelocity::deserialize(buf);
        Self {
            dead_reckoning_algorithm,
            dead_reckoning_other_parameters,
            entity_linear_acceleration,
            entity_angular_velocity,
        }
    }
}

impl FieldSerialize for DeadReckoningParameters {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for DeadReckoningParameters {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for DeadReckoningParameters {
    fn field_len(&self) -> usize {
        Self::LENGTH
    }
}

impl SerializedLength for DeadReckoningParameters {
    const LENGTH: usize = 40;
}
