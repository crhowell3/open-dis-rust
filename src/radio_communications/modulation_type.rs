//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug)]
pub struct ModulationType {
    pub spread_spectrum: u16,
    pub major: u16,
    pub detail: u16,
    pub system: u16,
}

impl Default for ModulationType {
    fn default() -> Self {
        ModulationType {
            spread_spectrum: 0,
            major: 0,
            detail: 0,
            system: 0,
        }
    }
}

impl ModulationType {
    pub fn new(spread_spectrum: u16, major: u16, detail: u16, system: u16) -> Self {
        ModulationType {
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

    pub fn decode(buf: &mut BytesMut) -> ModulationType {
        ModulationType {
            spread_spectrum: buf.get_u16(),
            major: buf.get_u16(),
            detail: buf.get_u16(),
            system: buf.get_u16(),
        }
    }
}
