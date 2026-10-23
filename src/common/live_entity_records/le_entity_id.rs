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
/// Implemented according to IEEE 1278.1-2012 §6.2.28
pub struct LEEntityId {
    /// The unique identification number of the site
    pub site_number: u8,
    /// The unique identification number of the application
    pub application_number: u8,
    /// The unique identification number of the entity
    pub entity_number: u16,
}

impl LEEntityId {
    #[must_use]
    pub const fn new(site_number: u8, application_number: u8, entity_number: u16) -> Self {
        Self {
            site_number,
            application_number,
            entity_number,
        }
    }
}

