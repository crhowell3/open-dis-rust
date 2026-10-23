//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::data_types::fixed_binary_16::FixedBinary16;

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct RelativeWorldCoordinates {
    pub reference_point: u16,
    pub delta_x: FixedBinary16,
    pub delta_y: FixedBinary16,
    pub delta_z: FixedBinary16,
}

impl RelativeWorldCoordinates {
    #[must_use]
    pub const fn new(
        reference_point: u16,
        delta_x: FixedBinary16,
        delta_y: FixedBinary16,
        delta_z: FixedBinary16,
    ) -> Self {
        Self {
            reference_point,
            delta_x,
            delta_y,
            delta_z,
        }
    }
}
