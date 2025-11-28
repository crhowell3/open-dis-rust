//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::{
        SerializedLength,
        data_types::{EntityId, attribute_record::AttributeRecord},
    },
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Clone, Debug, Default)]
pub struct AttributeRecordSet {
    pub entity_id: EntityId,
    pub number_of_attribute_records: u16,
    pub attribute_records: Vec<AttributeRecord>,
}

impl AttributeRecordSet {
    #[must_use]
    pub fn new() -> Self {
        Self {
            entity_id: EntityId::default(),
            number_of_attribute_records: 0u16,
            attribute_records: vec![],
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.entity_id.serialize(buf);
        buf.put_u16(self.number_of_attribute_records);
        for i in 0..self.attribute_records.len() {
            self.attribute_records[i].serialize(buf);
        }
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let entity_id = EntityId::deserialize(buf);
        let number_of_attribute_records = buf.get_u16();
        let mut attribute_records: Vec<AttributeRecord> = vec![];
        for _ in 0..number_of_attribute_records {
            attribute_records.push(AttributeRecord::deserialize(buf));
        }
        Self {
            entity_id,
            number_of_attribute_records,
            attribute_records,
        }
    }
}

impl FieldSerialize for AttributeRecordSet {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for AttributeRecordSet {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for AttributeRecordSet {
    fn field_len(&self) -> usize {
        EntityId::LENGTH + 2 + self.attribute_records.field_len()
    }
}
