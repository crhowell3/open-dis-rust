//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct VectoringNozzleSystemData {
    pub horizontal_deflection_angle: f32,
    pub vertical_deflection_angle: f32,
}

impl VectoringNozzleSystemData {
    #[must_use]
    pub const fn new(horizontal_deflection_angle: f32, vertical_deflection_angle: f32) -> Self {
        Self {
            horizontal_deflection_angle,
            vertical_deflection_angle,
        }
    }
}

