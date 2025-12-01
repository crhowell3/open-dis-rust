//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::enums::{UAAcousticEmitterSystemFunction, UAAcousticSystemName},
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §6.2.2
pub struct AcousticEmitterSystem {
    pub acoustic_name: UAAcousticSystemName,
    pub acoustic_function: UAAcousticEmitterSystemFunction,
    pub acoustic_id: u8,
}

impl AcousticEmitterSystem {
    #[must_use]
    pub const fn new(
        acoustic_name: UAAcousticSystemName,
        acoustic_function: UAAcousticEmitterSystemFunction,
        acoustic_id: u8,
    ) -> Self {
        Self {
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

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            acoustic_name: UAAcousticSystemName::deserialize(buf),
            acoustic_function: UAAcousticEmitterSystemFunction::deserialize(buf),
            acoustic_id: buf.get_u8(),
        }
    }
}

impl FieldSerialize for AcousticEmitterSystem {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for AcousticEmitterSystem {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for AcousticEmitterSystem {
    fn field_len(&self) -> usize {
        self.acoustic_name.field_len() + self.acoustic_function.field_len() + 1
    }
}
