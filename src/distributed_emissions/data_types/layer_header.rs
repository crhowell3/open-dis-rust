//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
pub struct LayerHeader {
    pub layer_number: u8,
    pub layer_specific_information: u8,
    pub length: u16,
}

impl Default for LayerHeader {
    fn default() -> Self {
        LayerHeader {
            layer_number: 0,
            layer_specific_information: 0,
            length: 0,
        }
    }
}

impl LayerHeader {
    pub fn new(layer_number: u8, layer_specific_information: u8, length: u16) -> Self {
        LayerHeader {
            layer_number,
            layer_specific_information,
            length,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.layer_number);
        buf.put_u8(self.layer_specific_information);
        buf.put_u16(self.length);
    }

    pub fn decode(buf: &mut BytesMut) -> LayerHeader {
        LayerHeader {
            layer_number: buf.get_u8(),
            layer_specific_information: buf.get_u8(),
            length: buf.get_u16(),
        }
    }
}
