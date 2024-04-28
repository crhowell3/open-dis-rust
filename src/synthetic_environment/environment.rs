//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug)]
pub struct Environment {
    pub environment_type: u32,
    pub length: u16,
    pub index: u8,
    pub padding: u8,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            environment_type: 0,
            length: 0,
            index: 0,
            padding: 0,
        }
    }
}

impl Environment {
    pub fn new(environment_type: u32, length: u16, index: u8, padding: u8) -> Self {
        Environment {
            environment_type,
            length,
            index,
            padding,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(self.environment_type);
        buf.put_u16(self.length);
        buf.put_u8(self.index);
        buf.put_u8(self.padding);
    }

    pub fn decode(buf: &mut BytesMut) -> Environment {
        Environment {
            environment_type: buf.get_u32(),
            length: buf.get_u16(),
            index: buf.get_u8(),
            padding: buf.get_u8(),
        }
    }
}
