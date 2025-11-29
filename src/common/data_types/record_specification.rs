//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

use super::record_specification_element::RecordSpecificationElement;

#[derive(Clone, Debug, Default)]
pub struct RecordSpecification {
    pub number_of_record_sets: u32,
    pub record_sets: Vec<RecordSpecificationElement>,
}

impl RecordSpecification {
    #[must_use]
    pub const fn new(
        number_of_record_sets: u32,
        record_sets: Vec<RecordSpecificationElement>,
    ) -> Self {
        Self {
            number_of_record_sets,
            record_sets,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(self.number_of_record_sets);
        for i in 0..self.record_sets.len() {
            self.record_sets[i].serialize(buf);
        }
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let number_of_record_sets = buf.get_u32();
        let mut record_sets: Vec<RecordSpecificationElement> = vec![];
        for _i in 0..number_of_record_sets {
            record_sets.push(RecordSpecificationElement::deserialize(buf));
        }
        Self {
            number_of_record_sets,
            record_sets,
        }
    }
}

impl FieldSerialize for RecordSpecification {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for RecordSpecification {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for RecordSpecification {
    fn field_len(&self) -> usize {
        4 + self.record_sets.field_len()
    }
}
