//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulationAddress {
    pub site_id: u16,
    pub application_id: u16,
}

impl Default for SimulationAddress {
    fn default() -> Self {
        SimulationAddress {
            site_id: 1,
            application_id: 1,
        }
    }
}

impl SimulationAddress {
    #[must_use]
    pub fn new(site_id: u16, application_id: u16) -> Self {
        SimulationAddress {
            site_id,
            application_id,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.site_id);
        buf.put_u16(self.application_id);
    }

    pub fn decode(buf: &mut BytesMut) -> SimulationAddress {
        SimulationAddress {
            site_id: buf.get_u16(),
            application_id: buf.get_u16(),
        }
    }
}
