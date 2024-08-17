//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012
pub struct AngularVelocity {
    pub rate_about_x_axis: f32,
    pub rate_about_y_axis: f32,
    pub rate_about_z_axis: f32,
}

impl AngularVelocity {
    /// Create a new AngularVelocity struct with existing values
    #[must_use]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        AngularVelocity {
            rate_about_x_axis: x,
            rate_about_y_axis: y,
            rate_about_z_axis: z,
        }
    }

    /// Serialize an instance of an AngularVelocity into a mutable byte stream
    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.rate_about_x_axis);
        buf.put_f32(self.rate_about_y_axis);
        buf.put_f32(self.rate_about_z_axis);
    }

    /// Decode an AngularVelocity from a mutable byte stream
    pub fn decode(buf: &mut BytesMut) -> AngularVelocity {
        AngularVelocity {
            rate_about_x_axis: buf.get_f32(),
            rate_about_y_axis: buf.get_f32(),
            rate_about_z_axis: buf.get_f32(),
        }
    }
}
