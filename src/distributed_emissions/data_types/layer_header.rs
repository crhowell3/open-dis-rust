//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::SerializedLength;

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §6.2.51
pub struct LayerHeader {
    pub layer_number: u8,
    pub layer_specific_information: u8,
    pub length: u16,
}

impl LayerHeader {
    #[must_use]
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

    pub fn deserialize<B: Buf>(buf: &mut B) -> LayerHeader {
        LayerHeader {
            layer_number: buf.get_u8(),
            layer_specific_information: buf.get_u8(),
            length: buf.get_u16(),
        }
    }
}

impl SerializedLength for LayerHeader {
    const LENGTH: usize = 4;
}
