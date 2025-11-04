//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug, Default)]
pub struct NamedLocation {
    pub station_name: u16,
    pub station_number: u16,
}

impl NamedLocation {
    #[must_use]
    pub fn new(station_name: u16, station_number: u16) -> Self {
        NamedLocation {
            station_name,
            station_number,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.station_name);
        buf.put_u16(self.station_number);
    }

    pub fn deserialize(buf: &mut BytesMut) -> NamedLocation {
        NamedLocation {
            station_name: buf.get_u16(),
            station_number: buf.get_u16(),
        }
    }
}
