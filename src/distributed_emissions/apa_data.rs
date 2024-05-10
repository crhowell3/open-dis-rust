//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
pub struct ApaData {
    pub parameter_index: u16,
    pub parameter_value: i16,
}

impl Default for ApaData {
    fn default() -> Self {
        ApaData {
            parameter_index: 0,
            parameter_value: 0,
        }
    }
}

impl ApaData {
    pub fn new(parameter_index: u16, parameter_value: i16) -> Self {
        ApaData {
            parameter_index,
            parameter_value,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.parameter_index);
        buf.put_i16(self.parameter_value);
    }

    pub fn decode(buf: &mut BytesMut) -> ApaData {
        ApaData {
            parameter_index: buf.get_u16(),
            parameter_value: buf.get_i16(),
        }
    }
}
