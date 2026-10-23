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
/// Implemented according to IEEE 1278.1-2012 §6.2.96
pub struct EntityCoordinateVector {
    /// Location along the X-axis relative to the entity's origin
    pub x_coordinate: f32,
    /// Location along the Y-axis relative to the entity's origin
    pub y_coordinate: f32,
    /// Location along the Z-axis relative to the entity's origin
    pub z_coordinate: f32,
}

impl EntityCoordinateVector {
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x_coordinate: x,
            y_coordinate: y,
            z_coordinate: z,
        }
    }
}

