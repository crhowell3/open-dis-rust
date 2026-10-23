//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

#[derive(Copy, Clone, Debug, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct IFFFundamentalParameterData {
    pub erp: f32,
    pub frequency: f32,
    pub pgrf: f32,
    pub pulse_width: f32,
    pub burst_length: u32,
    pub applicable_modes: u8,
    pub system_specific_data: [u8; 3],
}

impl Default for IFFFundamentalParameterData {
    fn default() -> Self {
        Self {
            erp: 0.0,
            frequency: 0.0,
            pgrf: 0.0,
            pulse_width: 0.0,
            burst_length: 0,
            applicable_modes: 0,
            system_specific_data: [0; 3],
        }
    }
}

impl IFFFundamentalParameterData {
    #[must_use]
    pub const fn new(
        erp: f32,
        frequency: f32,
        pgrf: f32,
        pulse_width: f32,
        burst_length: u32,
        applicable_modes: u8,
        system_specific_data: [u8; 3],
    ) -> Self {
        Self {
            erp,
            frequency,
            pgrf,
            pulse_width,
            burst_length,
            applicable_modes,
            system_specific_data,
        }
    }
}
