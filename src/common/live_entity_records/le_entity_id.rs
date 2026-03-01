//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};
use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
/// Implemented according to IEEE 1278.1-2012 §6.2.28
pub struct LEEntityId {
    /// The unique identification number of the site
    pub site_number: u8,
    /// The unique identification number of the application
    pub application_number: u8,
    /// The unique identification number of the entity
    pub entity_number: u16,
}

impl LEEntityId {
    #[must_use]
    pub const fn new(site_number: u8, application_number: u8, entity_number: u16) -> Self {
        Self {
            site_number,
            application_number,
            entity_number,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.site_number);
        buf.put_u8(self.application_number);
        buf.put_u16(self.entity_number);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            site_number: buf.get_u8(),
            application_number: buf.get_u8(),
            entity_number: buf.get_u16(),
        }
    }
}

impl FieldSerialize for LEEntityId {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for LEEntityId {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for LEEntityId {
    fn field_len(&self) -> usize {
        4
    }
}
