//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SimulationAddress {
    pub site_identifier: u16,
    pub application_identifier: u16,
}

impl SimulationAddress {
    pub fn new(site_identifier: u16, application_identifier: u16) -> Self {
        SimulationAddress {
            site_identifier,
            application_identifier,
        }
    }

    pub fn default() -> Self {
        SimulationAddress {
            site_identifier: 1,
            application_identifier: 1,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.site_identifier);
        buf.put_u16(self.application_identifier);
    }

    pub fn decode(buf: &mut BytesMut) -> SimulationAddress {
        SimulationAddress {
            site_identifier: buf.get_u16(),
            application_identifier: buf.get_u16(),
        }
    }
}
