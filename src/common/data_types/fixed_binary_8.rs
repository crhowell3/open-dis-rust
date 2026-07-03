//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Debug, Clone, Copy, Default)]

pub struct FixedBinary8(i8);

impl FixedBinary8 {
    #[must_use]
    /// Creates a fixed-point value from meters with 3 fractional bits
    pub fn from_meters(value: f32) -> Self {
        Self((value * 8.0).round() as i8)
    }

    #[must_use]
    /// Converts the fixed-point value into meters
    pub fn to_meters(self) -> f32 {
        f32::from(self.0) / 8.0
    }

    #[must_use]
    /// Encodes to raw network bytes
    pub const fn to_be_bytes(self) -> [u8; 1] {
        self.0.to_be_bytes()
    }

    #[must_use]
    /// Decodes from raw network bytes
    pub const fn from_be_bytes(bytes: [u8; 1]) -> Self {
        Self(i8::from_be_bytes(bytes))
    }

    #[must_use]
    pub const fn from_i8(raw: i8) -> Self {
        Self(raw)
    }

    #[must_use]
    pub const fn to_i8(self) -> i8 {
        self.0
    }
}

impl FieldSerialize for FixedBinary8 {
    fn serialize_field(&self, buf: &mut BytesMut) {
        buf.put_i8(self.to_i8());
    }
}

impl FieldDeserialize for FixedBinary8 {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        let bytes = buf.get_i8();
        Self::from_i8(bytes)
    }
}

impl FieldLen for FixedBinary8 {
    fn field_len(&self) -> usize {
        1
    }
}
