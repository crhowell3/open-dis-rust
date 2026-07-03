//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::data_types::fixed_binary_16::FixedBinary16,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
pub struct LELinearVelocity {
    pub x_component: FixedBinary16,
    pub y_component: FixedBinary16,
    pub z_component: FixedBinary16,
}

impl LELinearVelocity {
    #[must_use]
    pub const fn new(
        x_component: FixedBinary16,
        y_component: FixedBinary16,
        z_component: FixedBinary16,
    ) -> Self {
        Self {
            x_component,
            y_component,
            z_component,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(self.x_component.to_i16());
        buf.put_i16(self.y_component.to_i16());
        buf.put_i16(self.z_component.to_i16());
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            x_component: FixedBinary16::from_i16(buf.get_i16()),
            y_component: FixedBinary16::from_i16(buf.get_i16()),
            z_component: FixedBinary16::from_i16(buf.get_i16()),
        }
    }
}

impl FieldSerialize for LELinearVelocity {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for LELinearVelocity {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for LELinearVelocity {
    fn field_len(&self) -> usize {
        6
    }
}
