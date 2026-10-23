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
/// Implemented according to IEEE 1278.1-2012 §6.2.32
pub struct LEEulerAngles {
    /// Angle of rotation about the Z-axis
    pub psi: FixedBinary8,
    /// Angle of rotation about the Y-axis
    pub theta: FixedBinary8,
    /// Angle of rotation about the X-axis
    pub phi: FixedBinary8,
}

impl LEEulerAngles {
    #[must_use]
    #[allow(clippy::similar_names)]
    pub const fn new(psi: FixedBinary8, theta: FixedBinary8, phi: FixedBinary8) -> Self {
        Self { psi, theta, phi }
    }
}

