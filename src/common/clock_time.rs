//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::SerializedLength;

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
    pub fn new(h: u32, p: u32) -> Self {
        ClockTime {
            hour: h,
            time_past_hour: p,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(self.hour);
        buf.put_u32(self.time_past_hour);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> ClockTime {
        ClockTime {
            hour: buf.get_u32(),
            time_past_hour: buf.get_u32(),
        }
    }
}

impl SerializedLength for ClockTime {
    const LENGTH: usize = 8;
}
