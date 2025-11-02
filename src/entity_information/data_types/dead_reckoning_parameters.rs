//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{BufMut, BytesMut};

use crate::common::{
    angular_velocity_vector::AngularVelocity, enums::DeadReckoningAlgorithm,
    linear_acceleration::LinearAcceleration,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct DeadReckoningParameters {
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    pub dead_reckoning_other_parameters: u8,
    pub entity_linear_acceleration: LinearAcceleration,
    pub entity_angular_velocity: AngularVelocity,
}

impl DeadReckoningParameters {
    #[must_use]
    pub fn new(
        dead_reckoning_algorithm: DeadReckoningAlgorithm,
        entity_linear_acceleration: LinearAcceleration,
        entity_angular_velocity: AngularVelocity,
    ) -> Self {
        DeadReckoningParameters {
            dead_reckoning_algorithm,
            dead_reckoning_other_parameters: 0,
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

    pub fn deserialize(buf: &mut BytesMut) -> DeadReckoningParameters {
        DeadReckoningParameters {
            dead_reckoning_algorithm: DeadReckoningAlgorithm::deserialize(buf),
            dead_reckoning_other_parameters: 0,
            entity_linear_acceleration: LinearAcceleration::deserialize(buf),
            entity_angular_velocity: AngularVelocity::deserialize(buf),
        }
    }
}
