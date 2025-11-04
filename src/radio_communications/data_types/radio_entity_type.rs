//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug, Default)]
pub struct RadioEntityType {
    pub entity_kind: u8,
    pub domain: u8,
    pub country: u16,
    pub category: u8,
    pub nomenclature_version: u8,
    pub nomenclature: u16,
}

impl RadioEntityType {
    #[must_use]
    pub fn new(
        entity_kind: u8,
        domain: u8,
        country: u16,
        category: u8,
        nomenclature_version: u8,
        nomenclature: u16,
    ) -> Self {
        RadioEntityType {
            entity_kind,
            domain,
            country,
            category,
            nomenclature_version,
            nomenclature,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.entity_kind);
        buf.put_u8(self.domain);
        buf.put_u16(self.country);
        buf.put_u8(self.category);
        buf.put_u8(self.nomenclature_version);
        buf.put_u16(self.nomenclature);
    }

    pub fn deserialize(buf: &mut BytesMut) -> RadioEntityType {
        RadioEntityType {
            entity_kind: buf.get_u8(),
            domain: buf.get_u8(),
            country: buf.get_u16(),
            category: buf.get_u8(),
            nomenclature_version: buf.get_u8(),
            nomenclature: buf.get_u16(),
        }
    }
}
