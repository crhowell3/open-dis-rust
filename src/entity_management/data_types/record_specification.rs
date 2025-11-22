//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use super::record_specification_element::RecordSpecificationElement;

#[derive(Clone, Debug, Default)]
pub struct RecordSpecification {
    pub number_of_record_sets: u32,
    pub record_sets: Vec<RecordSpecificationElement>,
}

impl RecordSpecification {
    #[must_use]
    pub fn new(number_of_record_sets: u32, record_sets: Vec<RecordSpecificationElement>) -> Self {
        RecordSpecification {
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

    pub fn deserialize<B: Buf>(buf: &mut B) -> RecordSpecification {
        let number_of_record_sets = buf.get_u32();
        let mut record_sets: Vec<RecordSpecificationElement> = vec![];
        for _i in 0..number_of_record_sets {
            record_sets.push(RecordSpecificationElement::deserialize(buf));
        }
        RecordSpecification {
            number_of_record_sets,
            record_sets,
        }
    }
}
