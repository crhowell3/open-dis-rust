//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[must_use]
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.x);
        buf.put_f32(self.y);
    }

    pub fn decode(buf: &mut BytesMut) -> Point {
        Point {
            x: buf.get_f32(),
            y: buf.get_f32(),
        }
    }
}
