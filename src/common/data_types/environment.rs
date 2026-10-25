//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct Environment {
    pub environment_type: u32,
    pub length: u16,
    pub index: u8,
    pub padding: u8,
}

impl Environment {
    #[must_use]
    pub const fn new(environment_type: u32, length: u16, index: u8, padding: u8) -> Self {
        Self {
            environment_type,
            length,
            index,
            padding,
        }
    }
}
