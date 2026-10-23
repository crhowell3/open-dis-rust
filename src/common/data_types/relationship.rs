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
pub struct Relationship {
    pub nature: u16,
    pub position: u16,
}

impl Relationship {
    #[must_use]
    pub const fn new(nature: u16, position: u16) -> Self {
        Self { nature, position }
    }
}
