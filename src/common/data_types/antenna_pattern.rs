//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License
use crate::pdu_macro::{FieldDeserialize, FieldDeserializeWithLen, FieldLen, FieldSerialize};
use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug, Default)]
pub struct AntennaPattern {
    record_specific_fields: Vec<u8>,
}

impl FieldSerialize for AntennaPattern {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for AntennaPattern {
    fn deserialize_field<B: Buf>(_buf: &mut B) -> Self {
        Self::default()
    }
}

impl FieldDeserializeWithLen for AntennaPattern {
    fn deserialize_with_len<B: Buf>(buf: &mut B, len: usize) -> Self {
        let len_u16 = u16::try_from(len).expect("Should not be larger than u16");
        Self::deserialize(buf, len_u16).unwrap_or_default()
    }
}

impl FieldLen for AntennaPattern {
    fn field_len(&self) -> usize {
        self.record_specific_fields.len()
    }
}

impl AntennaPattern {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            record_specific_fields: vec![],
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        for i in 0..self.record_specific_fields.len() {
            buf.put_u8(self.record_specific_fields[i]);
        }
    }

    pub fn deserialize<B: Buf>(buf: &mut B, length: u16) -> Option<Self> {
        if length == 0 {
            return None;
        }

        let mut record_specific_fields: Vec<u8> = vec![];
        for _ in 0..length {
            record_specific_fields.push(buf.get_u8());
        }

        Some(Self {
            record_specific_fields,
        })
    }
}
