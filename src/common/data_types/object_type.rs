//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Clone, Debug, Default)]
pub struct ObjectType {
    pub domain: u8,
    pub object_kind: u8,
    pub category: u8,
    pub subcategory: u8,
}

impl ObjectType {
    #[must_use]
    pub const fn new(domain: u8, object_kind: u8, category: u8, subcategory: u8) -> Self {
        Self {
            domain,
            object_kind,
            category,
            subcategory,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.domain);
        buf.put_u8(self.object_kind);
        buf.put_u8(self.category);
        buf.put_u8(self.subcategory);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            domain: buf.get_u8(),
            object_kind: buf.get_u8(),
            category: buf.get_u8(),
            subcategory: buf.get_u8(),
        }
    }
}

impl FieldSerialize for ObjectType {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for ObjectType {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for ObjectType {
    fn field_len(&self) -> usize {
        4
    }
}
