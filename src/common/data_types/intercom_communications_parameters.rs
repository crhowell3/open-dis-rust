//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};
use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug, Default)]
pub struct IntercomCommunicationsParameters {
    pub record_type: u16,
    pub record_length: u16,
    pub record_specific_field: u32,
}

impl FieldSerialize for IntercomCommunicationsParameters {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for IntercomCommunicationsParameters {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for IntercomCommunicationsParameters {
    fn field_len(&self) -> usize {
        2 + 2 + 4
    }
}

impl IntercomCommunicationsParameters {
    #[must_use]
    pub const fn new(record_type: u16, record_length: u16, record_specific_field: u32) -> Self {
        Self {
            record_type,
            record_length,
            record_specific_field,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.record_type);
        buf.put_u16(self.record_length);
        buf.put_u32(self.record_specific_field);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            record_type: buf.get_u16(),
            record_length: buf.get_u16(),
            record_specific_field: buf.get_u32(),
        }
    }
}
