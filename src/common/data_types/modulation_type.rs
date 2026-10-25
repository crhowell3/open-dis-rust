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
pub struct ModulationType {
    pub spread_spectrum: u16,
    pub major: u16,
    pub detail: u16,
    pub system: u16,
}

impl ModulationType {
    #[must_use]
    pub const fn new(spread_spectrum: u16, major: u16, detail: u16, system: u16) -> Self {
        Self {
            spread_spectrum,
            major,
            detail,
            system,
        }
    }
}

