//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Clone, Debug, Default)]
pub struct StandardVariableRecords {
    pub record_type: u32,
    pub record_length: u16,
    pub record_specific_fields: Vec<u8>,
}

impl StandardVariableRecords {
    #[must_use]
    pub const fn new(
        record_type: u32,
        record_length: u16,
        record_specific_fields: Vec<u8>,
    ) -> Self {
        Self {
            record_type,
            record_length,
            record_specific_fields,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(self.record_type);
        buf.put_u16(self.record_length);
        for i in 0..self.record_specific_fields.len() {
            buf.put_u8(self.record_specific_fields[i]);
        }
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let record_type = buf.get_u32();
        let record_length = buf.get_u16();
        let mut record_specific_fields: Vec<u8> = vec![];
        for _i in 0..record_length {
            record_specific_fields.push(buf.get_u8());
        }
        Self {
            record_type,
            record_length,
            record_specific_fields,
        }
    }
}

impl FieldSerialize for StandardVariableRecords {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for StandardVariableRecords {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for StandardVariableRecords {
    fn field_len(&self) -> usize {
        4 + 2 + self.record_specific_fields.field_len()
    }
}
