//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Clone, Debug)]
pub struct RecordSpecificationElement {
    pub record_id: u32,
    pub record_set_serial_number: u32,
    pub record_length: u16,
    pub record_count: u16,
    pub record_values: Vec<u8>,
}

impl RecordSpecificationElement {
    #[must_use]
    pub const fn new(
        record_id: u32,
        record_set_serial_number: u32,
        record_length: u16,
        record_count: u16,
        record_values: Vec<u8>,
    ) -> Self {
        Self {
            record_id,
            record_set_serial_number,
            record_length,
            record_count,
            record_values,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(self.record_id);
        buf.put_u32(self.record_set_serial_number);
        buf.put_u16(self.record_length);
        buf.put_u16(self.record_count);
        let num_record_values: usize = (self.record_length * self.record_count).into();
        for i in 0..num_record_values {
            buf.put_u8(self.record_values[i]);
        }
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let record_id = buf.get_u32();
        let record_set_serial_number = buf.get_u32();
        let record_length = buf.get_u16();
        let record_count = buf.get_u16();
        let num_record_values = record_length * record_count;
        let mut record_values: Vec<u8> = vec![];
        for _ in 0..num_record_values {
            record_values.push(buf.get_u8());
        }

        Self {
            record_id,
            record_set_serial_number,
            record_length,
            record_count,
            record_values,
        }
    }
}

impl FieldSerialize for RecordSpecificationElement {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for RecordSpecificationElement {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for RecordSpecificationElement {
    fn field_len(&self) -> usize {
        self.record_id.field_len()
            + self.record_set_serial_number.field_len()
            + self.record_length.field_len()
            + self.record_count.field_len()
            + self.record_values.field_len()
    }
}
