//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct LinearAcceleration {
    pub first_vector_component: f32,
    pub second_vector_component: f32,
    pub third_vector_component: f32,
}

impl LinearAcceleration {
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        LinearAcceleration {
            first_vector_component: x,
            second_vector_component: y,
            third_vector_component: z,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.first_vector_component);
        buf.put_f32(self.second_vector_component);
        buf.put_f32(self.third_vector_component);
    }

    pub fn decode(buf: &mut BytesMut) -> LinearAcceleration {
        LinearAcceleration {
            first_vector_component: buf.get_f32(),
            second_vector_component: buf.get_f32(),
            third_vector_component: buf.get_f32(),
        }
    }
}
