//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::data_types::fixed_binary_8::FixedBinary8,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
pub struct LEEntityLinearAcceleration {
    pub first_vector_component: FixedBinary8,
    pub second_vector_component: FixedBinary8,
    pub third_vector_component: FixedBinary8,
}

impl LEEntityLinearAcceleration {
    #[must_use]
    pub const fn new(x: FixedBinary8, y: FixedBinary8, z: FixedBinary8) -> Self {
        Self {
            first_vector_component: x,
            second_vector_component: y,
            third_vector_component: z,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i8(self.first_vector_component.to_i8());
        buf.put_i8(self.second_vector_component.to_i8());
        buf.put_i8(self.third_vector_component.to_i8());
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            first_vector_component: FixedBinary8::from_i8(buf.get_i8()),
            second_vector_component: FixedBinary8::from_i8(buf.get_i8()),
            third_vector_component: FixedBinary8::from_i8(buf.get_i8()),
        }
    }
}

impl FieldSerialize for LEEntityLinearAcceleration {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for LEEntityLinearAcceleration {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for LEEntityLinearAcceleration {
    fn field_len(&self) -> usize {
        3
    }
}
