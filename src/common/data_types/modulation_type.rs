//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Clone, Debug, Default)]
pub struct ModulationType {
    pub spread_spectrum: u16,
    pub major: u16,
    pub detail: u16,
    pub system: u16,
}

impl ModulationType {
    #[must_use]
    pub const fn new(spread_spectrum: u16, major: u16, detail: u16, system: u16) -> Self {
        Self {
            spread_spectrum,
            major,
            detail,
            system,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.spread_spectrum);
        buf.put_u16(self.major);
        buf.put_u16(self.detail);
        buf.put_u16(self.system);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            spread_spectrum: buf.get_u16(),
            major: buf.get_u16(),
            detail: buf.get_u16(),
            system: buf.get_u16(),
        }
    }
}

impl FieldSerialize for ModulationType {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for ModulationType {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for ModulationType {
    fn field_len(&self) -> usize {
        self.spread_spectrum.field_len()
            + self.major.field_len()
            + self.detail.field_len()
            + self.system.field_len()
    }
}
