//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Clone, Debug, Default)]
pub struct Environment {
    pub environment_type: u32,
    pub length: u16,
    pub index: u8,
    pub padding: u8,
}

impl Environment {
    #[must_use]
    pub const fn new(environment_type: u32, length: u16, index: u8, padding: u8) -> Self {
        Self {
            environment_type,
            length,
            index,
            padding,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(self.environment_type);
        buf.put_u16(self.length);
        buf.put_u8(self.index);
        buf.put_u8(self.padding);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            environment_type: buf.get_u32(),
            length: buf.get_u16(),
            index: buf.get_u8(),
            padding: buf.get_u8(),
        }
    }
}

impl FieldSerialize for Environment {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for Environment {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for Environment {
    fn field_len(&self) -> usize {
        self.environment_type.field_len()
            + self.length.field_len()
            + self.index.field_len()
            + self.padding.field_len()
    }
}
