//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
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
}

