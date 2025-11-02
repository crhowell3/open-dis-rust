//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use super::simulation_address::SimulationAddress;
use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
/// Implemented according to IEEE 1278.1-2012 §6.2.28
pub struct EntityId {
    /// The simulation's designation associated with all object identifiers
    pub simulation_address: SimulationAddress,
    /// The unique identification number of the entity
    pub entity_id: u16,
}

impl EntityId {
    #[must_use]
    pub fn new(site_id: u16, application_id: u16, entity_id: u16) -> Self {
        EntityId {
            simulation_address: SimulationAddress::new(site_id, application_id),
            entity_id,
        }
    }

    #[must_use]
    pub fn default(entity_id: u16) -> Self {
        EntityId {
            simulation_address: SimulationAddress::default(),
            entity_id,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.simulation_address.serialize(buf);
        buf.put_u16(self.entity_id);
    }

    pub fn deserialize(buf: &mut BytesMut) -> EntityId {
        EntityId {
            simulation_address: EntityId::deserialize_simulation_address(buf),
            entity_id: buf.get_u16(),
        }
    }

    fn deserialize_simulation_address(buf: &mut BytesMut) -> SimulationAddress {
        SimulationAddress {
            site_id: buf.get_u16(),
            application_id: buf.get_u16(),
        }
    }
}
