//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use super::simulation_address::SimulationAddress;

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EventId {
    pub simulation_address: SimulationAddress,
    pub event_identifier: u16,
}

impl EventId {
    #[must_use]
    pub const fn new(
        site_identifier: u16,
        application_identifier: u16,
        event_identifier: u16,
    ) -> Self {
        Self {
            simulation_address: SimulationAddress::new(site_identifier, application_identifier),
            event_identifier,
        }
    }
}
