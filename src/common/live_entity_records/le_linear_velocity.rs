//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::data_types::fixed_binary::FixedBinary16;

pub type LELinearVelocityComponent = FixedBinary16<0>;

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LELinearVelocity {
    pub x_component: LELinearVelocityComponent,
    pub y_component: LELinearVelocityComponent,
    pub z_component: LELinearVelocityComponent,
}

impl LELinearVelocity {
    #[must_use]
    pub const fn new(
        x_component: LELinearVelocityComponent,
        y_component: LELinearVelocityComponent,
        z_component: LELinearVelocityComponent,
    ) -> Self {
        Self {
            x_component,
            y_component,
            z_component,
        }
    }
}
