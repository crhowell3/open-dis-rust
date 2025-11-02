//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct VectoringNozzleSystemData {
    pub horizontal_deflection_angle: f32,
    pub vertical_deflection_angle: f32,
}

impl VectoringNozzleSystemData {
    #[must_use]
    pub fn new(horizontal_deflection_angle: f32, vertical_deflection_angle: f32) -> Self {
        VectoringNozzleSystemData {
            horizontal_deflection_angle,
            vertical_deflection_angle,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.horizontal_deflection_angle);
        buf.put_f32(self.vertical_deflection_angle);
    }

    pub fn deserialize(buf: &mut BytesMut) -> VectoringNozzleSystemData {
        VectoringNozzleSystemData {
            horizontal_deflection_angle: buf.get_f32(),
            vertical_deflection_angle: buf.get_f32(),
        }
    }
}
