//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Debug, Clone, Copy, Default)]

pub struct FixedBinary16(i16);

impl FixedBinary16 {
    #[must_use]
    /// Creates a fixed-point value from meters with 3 fractional bits
    pub fn from_meters(value: f32) -> Self {
        Self((value * 8.0).round() as i16)
    }

    #[must_use]
    /// Converts the fixed-point value into meters
    pub fn to_meters(self) -> f32 {
        f32::from(self.0) / 8.0
    }

    #[must_use]
    /// Encodes to raw network bytes
    pub const fn to_be_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }

    #[must_use]
    /// Decodes from raw network bytes
    pub const fn from_be_bytes(bytes: [u8; 2]) -> Self {
        Self(i16::from_be_bytes(bytes))
    }

    #[must_use]
    pub const fn from_i16(raw: i16) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn to_i16(self) -> i16 {
        self.0
    }
}

impl FieldSerialize for FixedBinary16 {
    fn serialize_field(&self, buf: &mut BytesMut) {
        buf.put_i16(self.to_i16());
    }
}

impl FieldDeserialize for FixedBinary16 {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        let bytes = buf.get_i16();
        Self::from_i16(bytes)
    }
}

impl FieldLen for FixedBinary16 {
    fn field_len(&self) -> usize {
        2
    }
}
