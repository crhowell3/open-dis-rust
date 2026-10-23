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
pub struct SystemId {
    pub system_type: u16,
    pub system_name: u16,
    pub system_mode: u8,
    pub change_options: u8,
}

impl SystemId {
    #[must_use]
    pub const fn new(
        system_type: u16,
        system_name: u16,
        system_mode: u8,
        change_options: u8,
    ) -> Self {
        Self {
            system_type,
            system_name,
            system_mode,
            change_options,
        }
    }
}

