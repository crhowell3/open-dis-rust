//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
/// Custom vector type containing 3 double precision fields
pub struct Vector3Double {
    /// The first value within the vector
    pub x: f64,
    /// The second value within the vector
    pub y: f64,
    /// The third value within the vector
    pub z: f64,
}

impl Vector3Double {
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3Double { x, y, z }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f64(self.x);
        buf.put_f64(self.y);
        buf.put_f64(self.z);
    }

    pub fn decode(buf: &mut BytesMut) -> Vector3Double {
        Vector3Double {
            x: buf.get_f64(),
            y: buf.get_f64(),
            z: buf.get_f64(),
        }
    }
}
