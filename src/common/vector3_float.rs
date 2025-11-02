//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
/// Custom vector type containing 3 single precision fields
pub struct Vector3Float {
    /// The first value within the vector
    pub x: f32,
    /// The second value within the vector
    pub y: f32,
    /// The third value within the vector
    pub z: f32,
}

impl Vector3Float {
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3Float { x, y, z }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.x);
        buf.put_f32(self.y);
        buf.put_f32(self.z);
    }

    pub fn deserialize(buf: &mut BytesMut) -> Vector3Float {
        Vector3Float {
            x: buf.get_f32(),
            y: buf.get_f32(),
            z: buf.get_f32(),
        }
    }
}
