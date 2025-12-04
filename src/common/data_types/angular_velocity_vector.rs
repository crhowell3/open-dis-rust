//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §6.2.7
/// For all fields, assume right-hand rule for directionality
pub struct AngularVelocity {
    /// The angular velocity in radians/second about the entity's X-axis
    pub rate_about_x_axis: f32,
    /// The angular velocity in radians/second about the entity's Y-axis
    pub rate_about_y_axis: f32,
    /// The angular velocity in radians/second about the entity's Z-axis
    pub rate_about_z_axis: f32,
}

impl AngularVelocity {
    /// Create a new `AngularVelocity` struct with existing values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Serialize an instance of an `AngularVelocity` into a mutable byte stream
    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.rate_about_x_axis);
        buf.put_f32(self.rate_about_y_axis);
        buf.put_f32(self.rate_about_z_axis);
    }

    /// Decode an `AngularVelocity` from a mutable byte stream
    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            rate_about_x_axis: buf.get_f32(),
            rate_about_y_axis: buf.get_f32(),
            rate_about_z_axis: buf.get_f32(),
        }
    }
}

impl FieldSerialize for AngularVelocity {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for AngularVelocity {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for AngularVelocity {
    fn field_len(&self) -> usize {
        12
    }
}
