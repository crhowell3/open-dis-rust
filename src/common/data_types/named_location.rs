//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Clone, Debug, Default)]
pub struct NamedLocation {
    pub station_name: u16,
    pub station_number: u16,
}

impl NamedLocation {
    #[must_use]
    pub const fn new(station_name: u16, station_number: u16) -> Self {
        Self {
            station_name,
            station_number,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.station_name);
        buf.put_u16(self.station_number);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            station_name: buf.get_u16(),
            station_number: buf.get_u16(),
        }
    }
}

impl FieldSerialize for NamedLocation {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for NamedLocation {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for NamedLocation {
    fn field_len(&self) -> usize {
        4
    }
}
