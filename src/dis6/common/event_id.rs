//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use super::simulation_address::SimulationAddress;

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct EventId {
    pub simulation_address: SimulationAddress,
    pub event_identifier: u16,
}

impl EventId {
    pub fn new(site_identifier: u16, application_identifier: u16, event_identifier: u16) -> Self {
        EventId {
            simulation_address: SimulationAddress::new(site_identifier, application_identifier),
            event_identifier,
        }
    }

    pub fn default(event_identifier: u16) -> Self {
        EventId {
            simulation_address: SimulationAddress::default(),
            event_identifier,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.simulation_address.serialize(buf);
        buf.put_u16(self.event_identifier);
    }

    pub fn decode(buf: &mut BytesMut) -> EventId {
        EventId {
            simulation_address: SimulationAddress::decode(buf),
            event_identifier: buf.get_u16(),
        }
    }
}
