//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::data_types::fixed_binary::FixedBinary16;

pub type OrientationErrorComponent = FixedBinary16<8>;

#[derive(Debug, Clone, Copy, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct OrientationError {
    pub azimuth_error: OrientationErrorComponent,
    pub elevation_error: OrientationErrorComponent,
    pub rotation_error: OrientationErrorComponent,
}

impl OrientationError {
    #[must_use]
    pub const fn new(
        azimuth_error: OrientationErrorComponent,
        elevation_error: OrientationErrorComponent,
        rotation_error: OrientationErrorComponent,
    ) -> Self {
        Self {
            azimuth_error,
            elevation_error,
            rotation_error,
        }
    }
}
