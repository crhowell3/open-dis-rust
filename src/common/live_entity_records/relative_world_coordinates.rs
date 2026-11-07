//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::data_types::fixed_binary::FixedBinary16;

pub type RelativeWorldCoordinatesComponent = FixedBinary16<3>;

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct RelativeWorldCoordinates {
    pub reference_point: u16,
    pub delta_x: RelativeWorldCoordinatesComponent,
    pub delta_y: RelativeWorldCoordinatesComponent,
    pub delta_z: RelativeWorldCoordinatesComponent,
}

impl RelativeWorldCoordinates {
    #[must_use]
    pub const fn new(
        reference_point: u16,
        delta_x: RelativeWorldCoordinatesComponent,
        delta_y: RelativeWorldCoordinatesComponent,
        delta_z: RelativeWorldCoordinatesComponent,
    ) -> Self {
        Self {
            reference_point,
            delta_x,
            delta_y,
            delta_z,
        }
    }
}
