//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use super::simulation_address::SimulationAddress;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §6.2.28
pub struct EntityId {
    /// The simulation's designation associated with all object identifiers
    pub simulation_address: SimulationAddress,
    /// The unique identification number of the entity
    pub entity_id: u16,
}

impl EntityId {
    #[must_use]
    pub const fn new(site_id: u16, application_id: u16, entity_id: u16) -> Self {
        Self {
            simulation_address: SimulationAddress::new(site_id, application_id),
            entity_id,
        }
    }
}

