//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License
use bytes::{Buf, BufMut, BytesMut};

use crate::common::enums::VariableParameterRecordType;

#[derive(Clone, Debug, Default)]
pub struct VariableTransmitterParameters {
    pub record_type: VariableParameterRecordType,
    pub record_length: u16,
    pub record_specific_fields: Vec<u8>,
}

impl VariableTransmitterParameters {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(self.record_type as u32);
        buf.put_u16(self.record_length);
        for i in 0..self.record_specific_fields.len() {
            buf.put_u8(self.record_specific_fields[i]);
        }
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let record_type = VariableParameterRecordType::deserialize(buf);
        let record_length = buf.get_u16();
        let mut record_specific_fields: Vec<u8> = vec![];
        for _ in 0..record_length {
            record_specific_fields.push(buf.get_u8());
        }

        Self {
            record_type,
            record_length,
            record_specific_fields,
        }
    }
}
