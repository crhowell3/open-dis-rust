//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct SystemId {
    pub system_type: u16,
    pub system_name: u16,
    pub system_mode: u8,
    pub change_options: u8,
}

impl SystemId {
    #[must_use]
    pub fn new(system_type: u16, system_name: u16, system_mode: u8, change_options: u8) -> Self {
        SystemId {
            system_type,
            system_name,
            system_mode,
            change_options,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.system_type);
        buf.put_u16(self.system_name);
        buf.put_u8(self.system_mode);
        buf.put_u8(self.change_options);
    }

    pub fn decode(buf: &mut BytesMut) -> SystemId {
        SystemId {
            system_type: buf.get_u16(),
            system_name: buf.get_u16(),
            system_mode: buf.get_u8(),
            change_options: buf.get_u8(),
        }
    }
}
