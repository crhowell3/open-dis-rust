//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::SerializedLength,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §6.2.96
pub struct EntityCoordinateVector {
    /// Location along the X-axis relative to the entity's origin
    pub x_coordinate: f32,
    /// Location along the Y-axis relative to the entity's origin
    pub y_coordinate: f32,
    /// Location along the Z-axis relative to the entity's origin
    pub z_coordinate: f32,
}

impl EntityCoordinateVector {
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x_coordinate: x,
            y_coordinate: y,
            z_coordinate: z,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.x_coordinate);
        buf.put_f32(self.y_coordinate);
        buf.put_f32(self.z_coordinate);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            x_coordinate: buf.get_f32(),
            y_coordinate: buf.get_f32(),
            z_coordinate: buf.get_f32(),
        }
    }
}

impl FieldSerialize for EntityCoordinateVector {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for EntityCoordinateVector {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for EntityCoordinateVector {
    fn field_len(&self) -> usize {
        Self::LENGTH
    }
}

impl SerializedLength for EntityCoordinateVector {
    const LENGTH: usize = 12;
}
