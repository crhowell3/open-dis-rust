//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License
use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug, Default)]
pub struct AntennaPattern {
    record_specific_fields: Vec<u8>,
}

impl AntennaPattern {
    #[must_use]
    pub fn new() -> Self {
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
