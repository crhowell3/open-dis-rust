//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BytesMut};

use crate::{
    impl_record_fields,
    pdu_macro::{FieldDeserialize, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct LiveEventIdentifier {
    pub site_number: u8,
    pub application_number: u8,
    pub event_number: u16,
}
impl_record_fields!(LiveEventIdentifier {
    site_number: u8,
    application_number: u8,
    event_number: u16,
});

impl LiveEventIdentifier {
    #[must_use]
    pub const fn new(site_number: u8, application_number: u8, event_number: u16) -> Self {
        Self {
            site_number,
            application_number,
            event_number,
        }
    }

    /// Convenience wrapper around [`FieldSerialize::serialize_field`].
    pub fn serialize(&self, buf: &mut BytesMut) {
        self.serialize_field(buf);
    }

    /// Convenience wrapper around [`FieldDeserialize::deserialize_field`].
    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize_field(buf)
    }
}
