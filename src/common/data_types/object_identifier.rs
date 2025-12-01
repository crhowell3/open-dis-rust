//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::data_types::SimulationAddress,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Clone, Debug, Default)]
pub struct ObjectIdentifier {
    pub simulation_address: SimulationAddress,
    pub object_number: u16,
}

impl ObjectIdentifier {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.simulation_address.serialize(buf);
        buf.put_u16(self.object_number);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            simulation_address: SimulationAddress::deserialize(buf),
            object_number: buf.get_u16(),
        }
    }
}

impl FieldSerialize for ObjectIdentifier {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for ObjectIdentifier {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for ObjectIdentifier {
    fn field_len(&self) -> usize {
        self.simulation_address.field_len() + self.object_number.field_len()
    }
}
