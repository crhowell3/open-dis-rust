//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::{SerializedLength, SimulationAddress};

#[derive(Copy, Clone, Debug, Default)]
pub struct SimulationIdentifier {
    pub simulation_address: SimulationAddress,
    pub reference_number: u16,
}

impl SimulationIdentifier {
    #[must_use]
    pub fn new() -> Self {
        Self {
            simulation_address: SimulationAddress::default(),
            reference_number: 0u16,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.simulation_address.serialize(buf);
        buf.put_u16(0u16);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            simulation_address: SimulationAddress::deserialize(buf),
            reference_number: buf.get_u16(),
        }
    }
}

impl SerializedLength for SimulationIdentifier {
    const LENGTH: usize = SimulationAddress::LENGTH + 2;
}
