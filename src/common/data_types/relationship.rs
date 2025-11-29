//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Clone, Debug, Default)]
pub struct Relationship {
    pub nature: u16,
    pub position: u16,
}

impl Relationship {
    #[must_use]
    pub const fn new(nature: u16, position: u16) -> Self {
        Self { nature, position }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.nature);
        buf.put_u16(self.position);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            nature: buf.get_u16(),
            position: buf.get_u16(),
        }
    }
}

impl FieldSerialize for Relationship {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for Relationship {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for Relationship {
    fn field_len(&self) -> usize {
        4
    }
}
