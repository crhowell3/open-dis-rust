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
pub struct PositionError {
    pub horizontal_error: FixedBinary16,
    pub vertical_error: FixedBinary16,
}

impl PositionError {
    #[must_use]
    pub const fn new(horizontal_error: FixedBinary16, vertical_error: FixedBinary16) -> Self {
        Self {
            horizontal_error,
            vertical_error,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(self.horizontal_error.to_i16());
        buf.put_i16(self.vertical_error.to_i16());
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            horizontal_error: FixedBinary16::from_i16(buf.get_i16()),
            vertical_error: FixedBinary16::from_i16(buf.get_i16()),
        }
    }
}

impl FieldSerialize for PositionError {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for PositionError {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for PositionError {
    fn field_len(&self) -> usize {
        4
    }
}
