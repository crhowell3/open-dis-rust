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
pub struct VectoringNozzleSystemData {
    pub horizontal_deflection_angle: f32,
    pub vertical_deflection_angle: f32,
}

impl VectoringNozzleSystemData {
    #[must_use]
    pub const fn new(horizontal_deflection_angle: f32, vertical_deflection_angle: f32) -> Self {
        Self {
            horizontal_deflection_angle,
            vertical_deflection_angle,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.horizontal_deflection_angle);
        buf.put_f32(self.vertical_deflection_angle);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            horizontal_deflection_angle: buf.get_f32(),
            vertical_deflection_angle: buf.get_f32(),
        }
    }
}

impl FieldSerialize for VectoringNozzleSystemData {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for VectoringNozzleSystemData {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for VectoringNozzleSystemData {
    fn field_len(&self) -> usize {
        Self::LENGTH
    }
}

impl SerializedLength for VectoringNozzleSystemData {
    const LENGTH: usize = 8;
}
