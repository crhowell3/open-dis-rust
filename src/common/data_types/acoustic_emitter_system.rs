//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::enums::{UAAcousticEmitterSystemFunction, UAAcousticSystemName};

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §6.2.2
pub struct AcousticEmitterSystem {
    pub acoustic_name: UAAcousticSystemName,
    pub acoustic_function: UAAcousticEmitterSystemFunction,
    pub acoustic_id: u8,
}

impl AcousticEmitterSystem {
    #[must_use]
    pub const fn new(
        acoustic_name: UAAcousticSystemName,
        acoustic_function: UAAcousticEmitterSystemFunction,
        acoustic_id: u8,
    ) -> Self {
        Self {
            acoustic_name,
            acoustic_function,
            acoustic_id,
        }
    }
}

