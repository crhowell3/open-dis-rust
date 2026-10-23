//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LiveEventIdentifier {
    pub site_number: u8,
    pub application_number: u8,
    pub event_number: u16,
}

impl LiveEventIdentifier {
    #[must_use]
    pub const fn new(site_number: u8, application_number: u8, event_number: u16) -> Self {
        Self {
            site_number,
            application_number,
            event_number,
        }
    }
}
