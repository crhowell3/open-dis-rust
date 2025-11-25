//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::SerializedLength;

#[derive(Clone, Debug, Default)]
pub struct ObjectType {
    pub domain: u8,
    pub object_kind: u8,
    pub category: u8,
    pub subcategory: u8,
}

impl ObjectType {
    #[must_use]
    pub fn new(domain: u8, object_kind: u8, category: u8, subcategory: u8) -> Self {
        ObjectType {
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

    pub fn deserialize<B: Buf>(buf: &mut B) -> ObjectType {
        ObjectType {
            domain: buf.get_u8(),
            object_kind: buf.get_u8(),
            category: buf.get_u8(),
            subcategory: buf.get_u8(),
        }
    }
}

impl SerializedLength for ObjectType {
    const LENGTH: usize = 4;
}
