//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §6.2.98
pub struct WorldCoordinate {
    /// The coordinate value along the X-axis which passes through the prime meridian at the equator
    pub x_coordinate: f64,
    /// The coordinate value along the Y-axis which passes through 90°E longitude at the equator
    pub y_coordinate: f64,
    /// The coordinate value along the Z-axis which passes through the North Pole
    pub z_coordinate: f64,
}

impl WorldCoordinate {
    /// Create a new `WorldCoordinate`
    ///
    /// # Examples
    ///
    /// Instantiating a new `WorldCoordinate`:
    /// ```
    /// use open_dis_rust::common::world_coordinate::WorldCoordinate;
    /// let mut world_coordinate = WorldCoordinate::default();
    /// ```
    ///
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        WorldCoordinate {
            x_coordinate: x,
            y_coordinate: y,
            z_coordinate: z,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f64(self.x_coordinate);
        buf.put_f64(self.y_coordinate);
        buf.put_f64(self.z_coordinate);
    }

    pub fn deserialize(buf: &mut BytesMut) -> WorldCoordinate {
        WorldCoordinate {
            x_coordinate: buf.get_f64(),
            y_coordinate: buf.get_f64(),
            z_coordinate: buf.get_f64(),
        }
    }
}
