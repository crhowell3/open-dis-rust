//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.6.4 Table 163
pub struct ApaData {
    pub parameter_index: u16,
    pub parameter_value: i16,
}

impl ApaData {
    #[must_use]
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

    pub fn deserialize(buf: &mut BytesMut) -> ApaData {
        ApaData {
            parameter_index: buf.get_u16(),
            parameter_value: buf.get_i16(),
        }
    }
}
