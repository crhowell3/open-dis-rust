//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::SerializedLength,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.6.4(j-l)
pub struct ShaftRPMs {
    pub current_shaft_rpms: i16,
    pub ordered_shaft_rpms: i16,
    pub shaft_rpm_rate_of_change: i32,
}

impl ShaftRPMs {
    #[must_use]
    pub const fn new(
        current_shaft_rpms: i16,
        ordered_shaft_rpms: i16,
        shaft_rpm_rate_of_change: i32,
    ) -> Self {
        Self {
            current_shaft_rpms,
            ordered_shaft_rpms,
            shaft_rpm_rate_of_change,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(self.current_shaft_rpms);
        buf.put_i16(self.ordered_shaft_rpms);
        buf.put_i32(self.shaft_rpm_rate_of_change);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            current_shaft_rpms: buf.get_i16(),
            ordered_shaft_rpms: buf.get_i16(),
            shaft_rpm_rate_of_change: buf.get_i32(),
        }
    }
}

impl FieldSerialize for ShaftRPMs {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for ShaftRPMs {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for ShaftRPMs {
    fn field_len(&self) -> usize {
        Self::LENGTH
    }
}

impl SerializedLength for ShaftRPMs {
    const LENGTH: usize = 8;
}
