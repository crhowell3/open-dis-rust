//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
pub struct StandardVariableRecords {
    pub record_type: u32,
    pub record_length: u16,
    pub re
}

impl Default for StandardVariableRecords {
    fn default() -> Self {
        StandardVariableRecords {
            number_of_standard_variable_records: 0,
            standard_variable_records: vec![],
        }
    }
}

impl StandardVariableRecords {
    pub fn new(
        number_of_standard_variable_records: u16,
        standard_variable_records: Vec<StandardVariableRecords>,
    ) -> Self {
        StandardVariableRecords {
            number_of_standard_variable_records,
            standard_variable_records,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.number_of_standard_variable_records);
        for i in 0..self.standard_variable_records.len() {
            self.standard_variable_records[i].serialize(buf);
        }
    }

    pub fn decode(buf: &mut BytesMut) -> StandardVariableRecords {
        let number_of_standard_variable_records = buf.get_u16();
        let mut standard_variable_records: Vec<StandardVariableRecords> = vec![];
        for _i in 0..number_of_standard_variable_records {
            standard_variable_records.push(StandardVariableRecords::decode(buf));
        }
        StandardVariableRecords {
            number_of_standard_variable_records,
            standard_variable_records,
        }
    }
}
