//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::enums::{EmitterName, EmitterSystemFunction};

#[derive(Copy, Clone, Debug, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EmitterSystem {
    pub emitter_name: EmitterName,
    pub function: EmitterSystemFunction,
    pub emitter_id_number: u8,
}

impl Default for EmitterSystem {
    fn default() -> Self {
        Self {
            emitter_name: EmitterName::default(),
            function: EmitterSystemFunction::Other,
            emitter_id_number: 0,
        }
    }
}

impl EmitterSystem {
    #[must_use]
    pub const fn new(name: EmitterName, function: EmitterSystemFunction, id: u8) -> Self {
        Self {
            emitter_name: name,
            function,
            emitter_id_number: id,
        }
    }
}
