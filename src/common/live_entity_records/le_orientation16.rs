//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LiveEntityOrientation16 {
    pub psi: i16,
    pub theta: i16,
    pub phi: i16,
}

impl LiveEntityOrientation16 {
    #[must_use]
    pub const fn new(psi: i16, theta: i16, phi: i16) -> Self {
        Self { psi, theta, phi }
    }
}

