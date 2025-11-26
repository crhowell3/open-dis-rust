//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use super::standard_variable_records::StandardVariableRecords;
use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};
use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug, Default)]
pub struct StandardVariableSpecification {
    pub number_of_standard_variable_records: u16,
    pub standard_variable_records: Vec<StandardVariableRecords>,
}

impl StandardVariableSpecification {
    #[must_use]
    pub const fn new(
        number_of_standard_variable_records: u16,
        standard_variable_records: Vec<StandardVariableRecords>,
    ) -> Self {
        Self {
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

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let number_of_standard_variable_records = buf.get_u16();
        let mut standard_variable_records: Vec<StandardVariableRecords> = vec![];
        for _i in 0..number_of_standard_variable_records {
            standard_variable_records.push(StandardVariableRecords::deserialize(buf));
        }
        Self {
            number_of_standard_variable_records,
            standard_variable_records,
        }
    }
}

impl FieldSerialize for StandardVariableSpecification {
    fn serialize_field(&self, buf: &mut BytesMut) {
        buf.put_u16(self.number_of_standard_variable_records);
        self.standard_variable_records.serialize_field(buf);
    }
}

impl FieldDeserialize for StandardVariableSpecification {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        let number_of_standard_variable_records = buf.get_u16();
        let mut records = Vec::with_capacity(number_of_standard_variable_records as usize);
        for _ in 0..number_of_standard_variable_records {
            records.push(StandardVariableRecords::deserialize_field(buf));
        }
        Self {
            number_of_standard_variable_records,
            standard_variable_records: records,
        }
    }
}

impl FieldLen for StandardVariableSpecification {
    fn field_len(&self) -> usize {
        2 + self.standard_variable_records.field_len()
    }
}
