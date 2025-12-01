//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Clone, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.x);
        buf.put_f32(self.y);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            x: buf.get_f32(),
            y: buf.get_f32(),
        }
    }
}

impl FieldSerialize for Point {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for Point {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for Point {
    fn field_len(&self) -> usize {
        self.x.field_len() + self.y.field_len()
    }
}
