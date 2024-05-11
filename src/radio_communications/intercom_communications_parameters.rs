//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug, Default)]
pub struct IntercomCommunicationsParameters {
    pub record_type: u16,
    pub record_length: u16,
    pub record_specific_field: u32,
}

impl IntercomCommunicationsParameters {
    #[must_use]
    pub fn new(record_type: u16, record_length: u16, record_specific_field: u32) -> Self {
        IntercomCommunicationsParameters {
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

    pub fn decode(buf: &mut BytesMut) -> IntercomCommunicationsParameters {
        IntercomCommunicationsParameters {
            record_type: buf.get_u16(),
            record_length: buf.get_u16(),
            record_specific_field: buf.get_u32(),
        }
    }
}
