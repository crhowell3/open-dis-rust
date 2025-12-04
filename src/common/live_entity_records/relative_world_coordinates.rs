//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::data_types::fixed_binary_16::FixedBinary16,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Clone, Debug, Default)]
pub struct RelativeWorldCoordinates {
    pub reference_point: u16,
    pub delta_x: FixedBinary16,
    pub delta_y: FixedBinary16,
    pub delta_z: FixedBinary16,
}

impl RelativeWorldCoordinates {
    #[must_use]
    pub const fn new(
        reference_point: u16,
        delta_x: FixedBinary16,
        delta_y: FixedBinary16,
        delta_z: FixedBinary16,
    ) -> Self {
        Self {
            reference_point,
            delta_x,
            delta_y,
            delta_z,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.reference_point);
        buf.put_i16(self.delta_x.to_i16());
        buf.put_i16(self.delta_y.to_i16());
        buf.put_i16(self.delta_z.to_i16());
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            reference_point: buf.get_u16(),
            delta_x: FixedBinary16::from_i16(buf.get_i16()),
            delta_y: FixedBinary16::from_i16(buf.get_i16()),
            delta_z: FixedBinary16::from_i16(buf.get_i16()),
        }
    }
}

impl FieldSerialize for RelativeWorldCoordinates {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for RelativeWorldCoordinates {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for RelativeWorldCoordinates {
    fn field_len(&self) -> usize {
        8
    }
}
