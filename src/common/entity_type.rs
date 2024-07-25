//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::enums::{Country, EntityKind};
use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct EntityType {
    pub kind: EntityKind,
    pub domain: u8,
    pub country: Country,
    pub category: u8,
    pub subcategory: u8,
    pub specific: u8,
    pub extra: u8,
}

impl EntityType {
    #[must_use]
    pub fn new(
        kind: EntityKind,
        domain: u8,
        country: Country,
        category: u8,
        subcategory: u8,
        specific: u8,
        extra: u8,
    ) -> Self {
        EntityType {
            kind,
            domain,
            country,
            category,
            subcategory,
            specific,
            extra,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.kind as u8);
        buf.put_u8(self.domain);
        buf.put_u16(self.country as u16);
        buf.put_u8(self.category);
        buf.put_u8(self.subcategory);
        buf.put_u8(self.specific);
        buf.put_u8(self.extra);
    }

    pub fn decode(buf: &mut BytesMut) -> EntityType {
        EntityType {
            kind: EntityKind::decode(buf),
            domain: buf.get_u8(),
            country: Country::decode(buf),
            category: buf.get_u8(),
            subcategory: buf.get_u8(),
            specific: buf.get_u8(),
            extra: buf.get_u8(),
        }
    }
}
