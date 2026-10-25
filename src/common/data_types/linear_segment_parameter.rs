//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::data_types::{WorldCoordinate, euler_angles::EulerAngles};

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LinearSegmentParameter {
    pub segment_number: u8,
    pub segment_modification: u8,
    pub general_segment_appearance: u32,
    pub specific_segment_appearance: u32, // TODO(@anyone) Implement Specific Object Appearance
    pub segment_location: WorldCoordinate,
    pub segment_orientation: EulerAngles,
    pub segment_length: f32,
    pub segment_width: f32,
    pub segment_height: f32,
    pub segment_depth: f32,
    padding: u32,
}

impl LinearSegmentParameter {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        segment_number: u8,
        segment_modification: u8,
        general_segment_appearance: u32,
        specific_segment_appearance: u32,
        segment_location: WorldCoordinate,
        segment_orientation: EulerAngles,
        segment_length: f32,
        segment_width: f32,
        segment_height: f32,
        segment_depth: f32,
    ) -> Self {
        Self {
            segment_number,
            segment_modification,
            general_segment_appearance,
            specific_segment_appearance,
            segment_location,
            segment_orientation,
            segment_length,
            segment_width,
            segment_height,
            segment_depth,
            padding: 0_u32,
        }
    }
}
