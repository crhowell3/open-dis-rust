//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct AcousticEmitterSystem {
    pub acoustic_name: u16,
    pub acoustic_function: u8,
    pub acoustic_id: u8,
}

impl AcousticEmitterSystem {
    #[must_use]
    pub fn new(acoustic_name: u16, acoustic_function: u8, acoustic_id: u8) -> Self {
        AcousticEmitterSystem {
            acoustic_name,
            acoustic_function,
            acoustic_id,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.acoustic_name);
        buf.put_u8(self.acoustic_function);
        buf.put_u8(self.acoustic_id);
    }

    pub fn decode(buf: &mut BytesMut) -> AcousticEmitterSystem {
        AcousticEmitterSystem {
            acoustic_name: buf.get_u16(),
            acoustic_function: buf.get_u8(),
            acoustic_id: buf.get_u8(),
        }
    }
}
