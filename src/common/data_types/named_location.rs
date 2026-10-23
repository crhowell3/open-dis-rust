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
pub struct NamedLocation {
    pub station_name: u16,
    pub station_number: u16,
}

impl NamedLocation {
    #[must_use]
    pub const fn new(station_name: u16, station_number: u16) -> Self {
        Self {
            station_name,
            station_number,
        }
    }
}
