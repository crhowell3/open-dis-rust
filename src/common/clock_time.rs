//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct ClockTime {
    pub hour: u32,
    pub time_past_hour: u32,
}

impl ClockTime {
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

    pub fn decode(buf: &mut BytesMut) -> ClockTime {
        ClockTime {
            hour: buf.get_u32(),
            time_past_hour: buf.get_u32(),
        }
    }
}
