//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::data_types::fixed_binary::FixedBinary8;

pub type LEEulerAnglesComponent = FixedBinary8<0>;

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §6.2.32
pub struct LEEulerAngles {
    /// Angle of rotation about the Z-axis
    pub psi: LEEulerAnglesComponent,
    /// Angle of rotation about the Y-axis
    pub theta: LEEulerAnglesComponent,
    /// Angle of rotation about the X-axis
    pub phi: LEEulerAnglesComponent,
}

impl LEEulerAngles {
    #[must_use]
    #[allow(clippy::similar_names)]
    pub const fn new(
        psi: LEEulerAnglesComponent,
        theta: LEEulerAnglesComponent,
        phi: LEEulerAnglesComponent,
    ) -> Self {
        Self { psi, theta, phi }
    }
}
