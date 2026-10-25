//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::data_types::fixed_binary_16::FixedBinary16;

#[derive(Debug, Clone, Copy, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct OrientationError {
    pub azimuth_error: FixedBinary16,
    pub elevation_error: FixedBinary16,
    pub rotation_error: FixedBinary16,
}

impl OrientationError {
    #[must_use]
    pub const fn new(
        azimuth_error: FixedBinary16,
        elevation_error: FixedBinary16,
        rotation_error: FixedBinary16,
    ) -> Self {
        Self {
            azimuth_error,
            elevation_error,
            rotation_error,
        }
    }
}
