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
    pub const fn new(
        entity_kind: u8,
        domain: u8,
        country: u16,
        category: u8,
        nomenclature_version: u8,
        nomenclature: u16,
    ) -> Self {
        Self {
            entity_kind,
            domain,
            country,
            category,
            nomenclature_version,
            nomenclature,
        }
    }
}
