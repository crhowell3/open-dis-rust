//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct WorldCoordinate {
    pub x_coordinate: f64,
    pub y_coordinate: f64,
    pub z_coordinate: f64,
}

impl WorldCoordinate {
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

    pub fn decode(buf: &mut BytesMut) -> WorldCoordinate {
        WorldCoordinate {
            x_coordinate: buf.get_f64(),
            y_coordinate: buf.get_f64(),
            z_coordinate: buf.get_f64(),
        }
    }
}
