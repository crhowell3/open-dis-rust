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
pub struct PropulsionSystemData {
    pub power_setting: f32,
    pub engine_rpm: f32,
}

impl PropulsionSystemData {
    #[must_use]
    pub const fn new(power_setting: f32, engine_rpm: f32) -> Self {
        Self {
            power_setting,
            engine_rpm,
        }
    }
}

