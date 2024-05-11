//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector3Double {
    pub x: f64,
    pub y: f64,
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
