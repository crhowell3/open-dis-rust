//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::data_types::fixed_binary_8::FixedBinary8;

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LEEntityLinearAcceleration {
    pub first_vector_component: FixedBinary8,
    pub second_vector_component: FixedBinary8,
    pub third_vector_component: FixedBinary8,
}

impl LEEntityLinearAcceleration {
    #[must_use]
    pub const fn new(x: FixedBinary8, y: FixedBinary8, z: FixedBinary8) -> Self {
        Self {
            first_vector_component: x,
            second_vector_component: y,
            third_vector_component: z,
        }
    }
}

