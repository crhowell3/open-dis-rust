//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::enums::{Country, EntityKind};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
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
    pub const fn new(
        kind: EntityKind,
        domain: u8,
        country: Country,
        category: u8,
        subcategory: u8,
        specific: u8,
        extra: u8,
    ) -> Self {
        Self {
            kind,
            domain,
            country,
            category,
            subcategory,
            specific,
            extra,
        }
    }
}
