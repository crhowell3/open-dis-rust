//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug)]
pub struct RecordSpecificationElement {
    pub record_id: u32,
    pub record_set_serial_number: u32,
    pub record_length: u16,
    pub record_count: u16,
    pub record_values: u16,
    pub pad: u8,
}

impl RecordSpecificationElement {
    pub fn new(
        record_id: u32,
        record_set_serial_number: u32,
        record_length: u16,
        record_count: u16,
        record_values: u16,
        pad: u8,
    ) -> Self {
        RecordSpecificationElement {
            record_id,
            record_set_serial_number,
            record_length,
            record_count,
            record_values,
            pad,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(self.record_id);
        buf.put_u32(self.record_set_serial_number);
        buf.put_u16(self.record_length);
        buf.put_u16(self.record_count);
        buf.put_u16(self.record_values);
        buf.put_u8(self.pad);
    }

    pub fn decode(buf: &mut BytesMut) -> RecordSpecificationElement {
        RecordSpecificationElement {
            record_id: buf.get_u32(),
            record_set_serial_number: buf.get_u32(),
            record_length: buf.get_u16(),
            record_count: buf.get_u16(),
            record_values: buf.get_u16(),
            pad: buf.get_u8(),
        }
    }
}
