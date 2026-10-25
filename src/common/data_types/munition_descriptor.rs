//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::data_types::entity_type::EntityType;

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct MunitionDescriptor {
    pub munition_type: EntityType,
    pub warhead: u16,
    pub fuse: u16,
    pub quantity: u16,
    pub rate: u16,
}

impl MunitionDescriptor {
    #[must_use]
    pub const fn new(
        munition_type: EntityType,
        warhead: u16,
        fuse: u16,
        quantity: u16,
        rate: u16,
    ) -> Self {
        Self {
            munition_type,
            warhead,
            fuse,
            quantity,
            rate,
        }
    }
}

