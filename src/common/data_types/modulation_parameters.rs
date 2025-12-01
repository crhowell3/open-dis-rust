//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldDeserializeWithLen, FieldLen, FieldSerialize};

#[derive(Clone, Debug, Default)]
pub struct ModulationParameters {
    record_specific_fields: Vec<u8>,
}

impl ModulationParameters {
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

    pub fn deserialize<B: Buf>(buf: &mut B, length: u8) -> Option<Self> {
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

impl FieldSerialize for ModulationParameters {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for ModulationParameters {
    fn deserialize_field<B: Buf>(_buf: &mut B) -> Self {
        // Default behavior for non-length-aware deserialization: return default.
        Self::default()
    }
}

impl FieldDeserializeWithLen for ModulationParameters {
    fn deserialize_with_len<B: Buf>(buf: &mut B, len: usize) -> Self {
        // Underlying API expects u8; clamp/cast from usize safely.
        let len_u8 = u8::try_from(len).expect("Should not be larger than u8");
        Self::deserialize(buf, len_u8).unwrap_or_default()
    }
}

impl FieldLen for ModulationParameters {
    fn field_len(&self) -> usize {
        self.record_specific_fields.field_len()
    }
}
