//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
pub struct ShaftRPMs {
    pub current_shaft_rpms: i16,
    pub ordered_shaft_rpms: i16,
    pub shaft_rpm_rate_of_change: f32,
}

impl Default for ShaftRPMs {
    fn default() -> Self {
        ShaftRPMs {
            current_shaft_rpms: 0,
            ordered_shaft_rpms: 0,
            shaft_rpm_rate_of_change: 0.0,
        }
    }
}

impl ShaftRPMs {
    pub fn new(
        current_shaft_rpms: i16,
        ordered_shaft_rpms: i16,
        shaft_rpm_rate_of_change: f32,
    ) -> Self {
        ShaftRPMs {
            current_shaft_rpms,
            ordered_shaft_rpms,
            shaft_rpm_rate_of_change,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_i16(self.current_shaft_rpms);
        buf.put_i16(self.ordered_shaft_rpms);
        buf.put_f32(self.shaft_rpm_rate_of_change);
    }

    pub fn decode(buf: &mut BytesMut) -> ShaftRPMs {
        ShaftRPMs {
            current_shaft_rpms: buf.get_i16(),
            ordered_shaft_rpms: buf.get_i16(),
            shaft_rpm_rate_of_change: buf.get_f32(),
        }
    }
}
