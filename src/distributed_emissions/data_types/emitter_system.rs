//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::enums::{EmitterName, EmitterSystemFunction};
use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
pub struct EmitterSystem {
    pub emitter_name: EmitterName,
    pub function: EmitterSystemFunction,
    pub emitter_id_number: u8,
}

impl Default for EmitterSystem {
    fn default() -> Self {
        EmitterSystem {
            emitter_name: EmitterName::default(),
            function: EmitterSystemFunction::Other,
            emitter_id_number: 0,
        }
    }
}

impl EmitterSystem {
    #[must_use]
    pub fn new(name: EmitterName, function: EmitterSystemFunction, id: u8) -> Self {
        EmitterSystem {
            emitter_name: name,
            function,
            emitter_id_number: id,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.emitter_name as u16);
        buf.put_u8(self.function as u8);
        buf.put_u8(self.emitter_id_number);
    }

    #[must_use]
    pub fn deserialize<B: Buf>(buf: &mut B) -> EmitterSystem {
        EmitterSystem {
            emitter_name: EmitterName::deserialize(buf),
            function: EmitterSystemFunction::deserialize(buf),
            emitter_id_number: buf.get_u8(),
        }
    }
}
