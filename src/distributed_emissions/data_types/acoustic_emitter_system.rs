//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::enums::{UAAcousticEmitterSystemFunction, UAAcousticSystemName};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §6.2.2
pub struct AcousticEmitterSystem {
    pub acoustic_name: UAAcousticSystemName,
    pub acoustic_function: UAAcousticEmitterSystemFunction,
    pub acoustic_id: u8,
}

impl AcousticEmitterSystem {
    #[must_use]
    pub fn new(
        acoustic_name: UAAcousticSystemName,
        acoustic_function: UAAcousticEmitterSystemFunction,
        acoustic_id: u8,
    ) -> Self {
        AcousticEmitterSystem {
            acoustic_name,
            acoustic_function,
            acoustic_id,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.acoustic_name as u16);
        buf.put_u8(self.acoustic_function as u8);
        buf.put_u8(self.acoustic_id);
    }

    pub fn deserialize(buf: &mut BytesMut) -> AcousticEmitterSystem {
        AcousticEmitterSystem {
            acoustic_name: UAAcousticSystemName::deserialize(buf),
            acoustic_function: UAAcousticEmitterSystemFunction::deserialize(buf),
            acoustic_id: buf.get_u8(),
        }
    }
}
