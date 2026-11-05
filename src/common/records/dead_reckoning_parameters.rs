//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    enums::DeadReckoningAlgorithm,
    records::{angular_velocity_vector::AngularVelocityVector, vector::Vector},
};

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
