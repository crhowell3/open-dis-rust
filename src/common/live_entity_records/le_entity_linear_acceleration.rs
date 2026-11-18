//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::data_types::fixed_binary::FixedBinary8;

/// One component of [`LEEntityLinearAcceleration`].
pub type LELinearAccelerationComponent = FixedBinary8<0>;

/// An entity's linear acceleration in the compact Live Entity form.
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LEEntityLinearAcceleration {
    pub first_vector_component: LELinearAccelerationComponent,
    pub second_vector_component: LELinearAccelerationComponent,
    pub third_vector_component: LELinearAccelerationComponent,
}

impl LEEntityLinearAcceleration {
    #[must_use]
    pub const fn new(
        x: LELinearAccelerationComponent,
        y: LELinearAccelerationComponent,
        z: LELinearAccelerationComponent,
    ) -> Self {
        Self {
            first_vector_component: x,
            second_vector_component: y,
            third_vector_component: z,
        }
    }
}
