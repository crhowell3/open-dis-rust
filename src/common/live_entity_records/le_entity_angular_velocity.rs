//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::data_types::fixed_binary_8::FixedBinary8,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §6.2.7
/// For all fields, assume right-hand rule for directionality
pub struct LEEntityAngularVelocity {
    /// The angular velocity in radians/second about the entity's X-axis
    pub rate_about_x_axis: FixedBinary8,
    /// The angular velocity in radians/second about the entity's Y-axis
    pub rate_about_y_axis: FixedBinary8,
    /// The angular velocity in radians/second about the entity's Z-axis
    pub rate_about_z_axis: FixedBinary8,
}

impl LEEntityAngularVelocity {
    /// Create a new `AngularVelocity` struct with existing values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Serialize an instance of an `AngularVelocity` into a mutable byte stream
    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i8(self.rate_about_x_axis.to_i8());
        buf.put_i8(self.rate_about_y_axis.to_i8());
        buf.put_i8(self.rate_about_z_axis.to_i8());
    }

    /// Decode an `AngularVelocity` from a mutable byte stream
    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            rate_about_x_axis: FixedBinary8::from_i8(buf.get_i8()),
            rate_about_y_axis: FixedBinary8::from_i8(buf.get_i8()),
            rate_about_z_axis: FixedBinary8::from_i8(buf.get_i8()),
        }
    }
}

impl FieldSerialize for LEEntityAngularVelocity {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for LEEntityAngularVelocity {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for LEEntityAngularVelocity {
    fn field_len(&self) -> usize {
        3
    }
}
