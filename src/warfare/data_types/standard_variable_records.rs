//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug)]
pub struct StandardVariableRecords {
    pub record_type: u32,
    pub record_length: u16,
    pub record_specific_fields: Vec<u8>,
}

impl Default for StandardVariableRecords {
    fn default() -> Self {
        StandardVariableRecords {
            record_type: 0,
            record_length: 0,
            record_specific_fields: vec![],
        }
    }
}

impl StandardVariableRecords {
    pub fn new(record_type: u32, record_length: u16, record_specific_fields: Vec<u8>) -> Self {
        StandardVariableRecords {
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

    pub fn decode(buf: &mut BytesMut) -> StandardVariableRecords {
        let record_type = buf.get_u32();
        let record_length = buf.get_u16();
        let mut record_specific_fields: Vec<u8> = vec![];
        for _i in 0..record_length {
            record_specific_fields.push(buf.get_u8());
        }
        StandardVariableRecords {
            record_type,
            record_length,
            record_specific_fields,
        }
    }
}
