//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::SerializedLength,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
pub struct PropulsionSystemData {
    pub power_setting: f32,
    pub engine_rpm: f32,
}

impl PropulsionSystemData {
    #[must_use]
    pub const fn new(power_setting: f32, engine_rpm: f32) -> Self {
        Self {
            power_setting,
            engine_rpm,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.power_setting);
        buf.put_f32(self.engine_rpm);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            power_setting: buf.get_f32(),
            engine_rpm: buf.get_f32(),
        }
    }
}

impl FieldSerialize for PropulsionSystemData {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for PropulsionSystemData {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for PropulsionSystemData {
    fn field_len(&self) -> usize {
        Self::LENGTH
    }
}

impl SerializedLength for PropulsionSystemData {
    const LENGTH: usize = 8;
}
