//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::SerializedLength;

#[derive(Copy, Clone, Debug, Default)]
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
    /// use open_dis_rust::common::WorldCoordinate;
    /// let mut world_coordinate = WorldCoordinate::new(0.0, 0.0, 0.0);
    /// ```
    ///
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        WorldCoordinate { x, y, z }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f64(self.x);
        buf.put_f64(self.y);
        buf.put_f64(self.z);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> WorldCoordinate {
        WorldCoordinate {
            x: buf.get_f64(),
            y: buf.get_f64(),
            z: buf.get_f64(),
        }
    }
}

impl SerializedLength for WorldCoordinate {
    const LENGTH: usize = 24;
}
