//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::data_types::fixed_binary_8::FixedBinary8;

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §6.2.7
/// For all fields, assume right-hand rule for directionality
pub struct LEEntityAngularVelocity {
    /// The angular velocity in radians/second about the entity's X-axis
    pub rate_about_x_axis: FixedBinary8,
    /// The angular velocity in radians/second about the entity's Y-axis
    pub rate_about_y_axis: FixedBinary8,
    /// The angular velocity in radians/second about the entity's Z-axis
    pub rate_about_z_axis: FixedBinary8,
}

impl LEEntityAngularVelocity {
    /// Create a new `AngularVelocity` struct with existing values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
