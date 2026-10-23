//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    enums::DeadReckoningAlgorithm,
    live_entity_records::{
        le_entity_angular_velocity::LEEntityAngularVelocity,
        le_entity_linear_acceleration::LEEntityLinearAcceleration,
    },
};

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
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
}

