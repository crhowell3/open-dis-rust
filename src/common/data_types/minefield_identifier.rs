//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::data_types::simulation_address::SimulationAddress,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Clone, Debug, Default)]
pub struct MinefieldIdentifier {
    pub simulation_address: SimulationAddress,
    pub minefield_number: u16,
}

impl MinefieldIdentifier {
    #[must_use]
    pub fn new(simulation_address: SimulationAddress, minefield_number: u16) -> Self {
        Self {
            simulation_address,
            minefield_number,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.simulation_address.serialize(buf);
        buf.put_u16(self.minefield_number);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            simulation_address: SimulationAddress::deserialize(buf),
            minefield_number: buf.get_u16(),
        }
    }
}

impl FieldSerialize for MinefieldIdentifier {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for MinefieldIdentifier {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for MinefieldIdentifier {
    fn field_len(&self) -> usize {
        self.simulation_address.field_len() + self.minefield_number.field_len()
    }
}
