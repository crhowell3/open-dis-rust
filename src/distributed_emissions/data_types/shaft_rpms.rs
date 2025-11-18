//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.6.4(j-l)
pub struct ShaftRPMs {
    pub current_shaft_rpms: i16,
    pub ordered_shaft_rpms: i16,
    pub shaft_rpm_rate_of_change: i32,
}

impl ShaftRPMs {
    #[must_use]
    pub fn new(
        current_shaft_rpms: i16,
        ordered_shaft_rpms: i16,
        shaft_rpm_rate_of_change: i32,
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
        buf.put_i32(self.shaft_rpm_rate_of_change);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> ShaftRPMs {
        ShaftRPMs {
            current_shaft_rpms: buf.get_i16(),
            ordered_shaft_rpms: buf.get_i16(),
            shaft_rpm_rate_of_change: buf.get_i32(),
        }
    }
}
