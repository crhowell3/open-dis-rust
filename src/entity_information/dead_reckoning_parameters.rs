//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use num_derive::FromPrimitive;

use crate::common::{
    angular_velocity_vector::AngularVelocity, linear_acceleration::LinearAcceleration,
};

#[derive(Copy, Clone, Debug)]
pub struct DeadReckoningParameters {
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    pub dead_reckoning_other_parameters: u8,
    pub entity_linear_acceleration: LinearAcceleration,
    pub entity_angular_velocity: AngularVelocity,
}

impl DeadReckoningParameters {
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

    pub fn default() -> Self {
        DeadReckoningParameters {
            dead_reckoning_algorithm: DeadReckoningAlgorithm::Static,
            dead_reckoning_other_parameters: 0,
            entity_linear_acceleration: LinearAcceleration::new(0.0, 0.0, 0.0),
            entity_angular_velocity: AngularVelocity::new(0.0, 0.0, 0.0),
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.dead_reckoning_algorithm as u8);
        buf.put_bytes(0u8, 15);
        self.entity_linear_acceleration.serialize(buf);
        self.entity_angular_velocity.serialize(buf);
    }

    pub fn decode(buf: &mut BytesMut) -> DeadReckoningParameters {
        DeadReckoningParameters {
            dead_reckoning_algorithm: DeadReckoningAlgorithm::from_u8(buf.get_u8()),
            dead_reckoning_other_parameters: 0,
            entity_linear_acceleration: LinearAcceleration::decode(buf),
            entity_angular_velocity: AngularVelocity::decode(buf),
        }
    }
}

#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
pub enum DeadReckoningAlgorithm {
    Other = 0,
    Static = 1,
    DRMFPW = 2,
    DRMRPW = 3,
    DRMRVW = 4,
    DRMFVW = 5,
    DRMFPB = 6,
    DRMRPB = 7,
    DRMRVB = 8,
    DRMFVB = 9,
}

impl DeadReckoningAlgorithm {
    pub fn from_u8(bit: u8) -> DeadReckoningAlgorithm {
        match bit {
            0 => DeadReckoningAlgorithm::Other,
            1 => DeadReckoningAlgorithm::Static,
            2 => DeadReckoningAlgorithm::DRMFPW,
            3 => DeadReckoningAlgorithm::DRMRPW,
            4 => DeadReckoningAlgorithm::DRMRVW,
            5 => DeadReckoningAlgorithm::DRMFVW,
            6 => DeadReckoningAlgorithm::DRMFPB,
            7 => DeadReckoningAlgorithm::DRMRPB,
            8 => DeadReckoningAlgorithm::DRMRVB,
            9 => DeadReckoningAlgorithm::DRMFVB,
            10_u8..=u8::MAX => DeadReckoningAlgorithm::Other,
        }
    }
}
