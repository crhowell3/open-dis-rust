//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use std::simd::f64x32;

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct AngularVelocity {
    pub rate_about_x_axis: f32,
    pub rate_about_y_axis: f32,
    pub rate_about_z_axis: f32,
}

impl AngularVelocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        AngularVelocity {
            rate_about_x_axis: x,
            rate_about_y_axis: y,
            rate_about_z_axis: z,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.rate_about_x_axis);
        buf.put_f32(self.rate_about_y_axis);
        buf.put_f32(self.rate_about_z_axis);
    }

    pub fn decode(buf: &mut BytesMut) -> AngularVelocity {
        AngularVelocity {
            rate_about_x_axis: buf.get_f32(),
            rate_about_y_axis: buf.get_f32(),
            rate_about_z_axis: buf.get_f32(),
        }
    }
}
