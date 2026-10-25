//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::data_types::fixed_binary_16::FixedBinary16;

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LELinearVelocity {
    pub x_component: FixedBinary16,
    pub y_component: FixedBinary16,
    pub z_component: FixedBinary16,
}

impl LELinearVelocity {
    #[must_use]
    pub const fn new(
        x_component: FixedBinary16,
        y_component: FixedBinary16,
        z_component: FixedBinary16,
    ) -> Self {
        Self {
            x_component,
            y_component,
            z_component,
        }
    }
}

