//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::simulation_address::SimulationAddress;

#[derive(Clone, Debug, Default)]
pub struct MinefieldIdentifier {
    pub simulation_address: SimulationAddress,
    pub minefield_number: u16,
}

impl MinefieldIdentifier {
    #[must_use]
    pub fn new(simulation_address: SimulationAddress, minefield_number: u16) -> Self {
        MinefieldIdentifier {
            simulation_address,
            minefield_number,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.simulation_address.serialize(buf);
        buf.put_u16(self.minefield_number);
    }

    pub fn deserialize(buf: &mut BytesMut) -> MinefieldIdentifier {
        MinefieldIdentifier {
            simulation_address: SimulationAddress::deserialize(buf),
            minefield_number: buf.get_u16(),
        }
    }
}
