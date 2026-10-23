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
/// Implemented according to IEEE 1278.1-2012 §6.2.98
pub struct WorldCoordinate {
    /// The coordinate value along the X-axis which passes through the prime meridian at the equator
    pub x: f64,
    /// The coordinate value along the Y-axis which passes through 90°E longitude at the equator
    pub y: f64,
    /// The coordinate value along the Z-axis which passes through the North Pole
    pub z: f64,
}

impl WorldCoordinate {
    /// Create a new `WorldCoordinate`
    ///
    /// # Examples
    ///
    /// Instantiating a new `WorldCoordinate`:
    /// ```
    /// use open_dis_rust::common::data_types::WorldCoordinate;
    /// let mut world_coordinate = WorldCoordinate::new(0.0, 0.0, 0.0);
    /// ```
    ///
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
