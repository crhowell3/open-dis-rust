//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::data_types::fixed_binary_16::FixedBinary16,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct OrientationError {
    pub azimuth_error: FixedBinary16,
    pub elevation_error: FixedBinary16,
    pub rotation_error: FixedBinary16,
}

impl OrientationError {
    #[must_use]
    pub const fn new(
        azimuth_error: FixedBinary16,
        elevation_error: FixedBinary16,
        rotation_error: FixedBinary16,
    ) -> Self {
        Self {
            azimuth_error,
            elevation_error,
            rotation_error,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(self.azimuth_error.to_i16());
        buf.put_i16(self.elevation_error.to_i16());
        buf.put_i16(self.rotation_error.to_i16());
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            azimuth_error: FixedBinary16::from_i16(buf.get_i16()),
            elevation_error: FixedBinary16::from_i16(buf.get_i16()),
            rotation_error: FixedBinary16::from_i16(buf.get_i16()),
        }
    }
}

impl FieldSerialize for OrientationError {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for OrientationError {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for OrientationError {
    fn field_len(&self) -> usize {
        6
    }
}
