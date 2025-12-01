//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §6.2.14
pub struct ClockTime {
    /// The hours since 0000h 1 January 1970 UTC (The Epoch)
    pub hour: u32,
    /// Time past the hour indicated in the hour field
    pub time_past_hour: u32,
}

impl ClockTime {
    #[must_use]
    pub const fn new(h: u32, p: u32) -> Self {
        Self {
            hour: h,
            time_past_hour: p,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(self.hour);
        buf.put_u32(self.time_past_hour);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            hour: buf.get_u32(),
            time_past_hour: buf.get_u32(),
        }
    }
}

impl FieldSerialize for ClockTime {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for ClockTime {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for ClockTime {
    fn field_len(&self) -> usize {
        8
    }
}
