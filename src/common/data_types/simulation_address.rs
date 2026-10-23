//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#[derive(Copy, Clone, Debug, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §6.2.80
pub struct SimulationAddress {
    /// Identification number representing the site, which may be a facility,
    /// installation, organizational unit, or a geographical location. Valid
    /// site ID values range from 1 to 65,534
    pub site_id: u16,
    /// Identification number representing the software program that is used to
    /// generate and process distributed simulation data. Valid application ID
    /// values range from 1 to 65,534
    pub application_id: u16,
}

impl Default for SimulationAddress {
    fn default() -> Self {
        Self {
            site_id: 1,
            application_id: 1,
        }
    }
}

impl SimulationAddress {
    #[must_use]
    pub const fn new(site_id: u16, application_id: u16) -> Self {
        Self {
            site_id,
            application_id,
        }
    }
}

