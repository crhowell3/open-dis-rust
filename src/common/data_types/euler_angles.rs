//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §6.2.32
pub struct EulerAngles {
    /// Angle of rotation about the Z-axis
    pub psi: f32,
    /// Angle of rotation about the Y-axis
    pub theta: f32,
    /// Angle of rotation about the X-axis
    pub phi: f32,
}

impl EulerAngles {
    #[must_use]
    #[allow(clippy::similar_names)]
    pub const fn new(psi: f32, theta: f32, phi: f32) -> Self {
        Self { psi, theta, phi }
    }
}
