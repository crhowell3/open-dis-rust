//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug)]
pub struct Relationship {
    pub nature: u16,
    pub position: u16,
}

impl Default for Relationship {
    fn default() -> Self {
        Relationship {
            nature: 0,
            position: 0,
        }
    }
}

impl Relationship {
    pub fn new(nature: u16, position: u16) -> Self {
        Relationship { nature, position }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.nature);
        buf.put_u16(self.position);
    }

    pub fn decode(buf: &mut BytesMut) -> Relationship {
        Relationship {
            nature: buf.get_u16(),
            position: buf.get_u16(),
        }
    }
}
