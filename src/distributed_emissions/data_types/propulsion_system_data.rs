//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct PropulsionSystemData {
    pub power_setting: f32,
    pub engine_rpm: f32,
}

impl PropulsionSystemData {
    #[must_use]
    pub fn new(power_setting: f32, engine_rpm: f32) -> Self {
        PropulsionSystemData {
            power_setting,
            engine_rpm,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.power_setting);
        buf.put_f32(self.engine_rpm);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> PropulsionSystemData {
        PropulsionSystemData {
            power_setting: buf.get_f32(),
            engine_rpm: buf.get_f32(),
        }
    }
}
