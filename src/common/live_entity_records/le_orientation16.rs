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
pub struct LiveEntityOrientation16 {
    pub psi: i16,
    pub theta: i16,
    pub phi: i16,
}
impl_record_fields!(LiveEntityOrientation16 {
    psi: i16,
    theta: i16,
    phi: i16,
});

impl LiveEntityOrientation16 {
    #[must_use]
    pub const fn new(psi: i16, theta: i16, phi: i16) -> Self {
        Self { psi, theta, phi }
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
