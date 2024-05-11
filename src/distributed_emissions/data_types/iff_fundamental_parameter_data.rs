//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
pub struct IFFFundamentalParameterData {
    pub erp: f32,
    pub frequency: f32,
    pub pgrf: f32,
    pub pulse_width: f32,
    pub burst_length: u32,
    pub applicable_modes: u8,
    pub system_specific_data: [u8; 3],
}

impl Default for IFFFundamentalParameterData {
    fn default() -> Self {
        IFFFundamentalParameterData {
            erp: 0.0,
            frequency: 0.0,
            pgrf: 0.0,
            pulse_width: 0.0,
            burst_length: 0,
            applicable_modes: 0,
            system_specific_data: [0; 3],
        }
    }
}

impl IFFFundamentalParameterData {
    #[must_use]
    pub fn new(
        erp: f32,
        frequency: f32,
        pgrf: f32,
        pulse_width: f32,
        burst_length: u32,
        applicable_modes: u8,
        system_specific_data: [u8; 3],
    ) -> Self {
        IFFFundamentalParameterData {
            erp,
            frequency,
            pgrf,
            pulse_width,
            burst_length,
            applicable_modes,
            system_specific_data,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.erp);
        buf.put_f32(self.frequency);
        buf.put_f32(self.pgrf);
        buf.put_f32(self.pulse_width);
        buf.put_u32(self.burst_length);
        buf.put_u8(self.applicable_modes);
        for i in 0..3 {
            buf.put_u8(self.system_specific_data[i]);
        }
    }

    pub fn decode(buf: &mut BytesMut) -> IFFFundamentalParameterData {
        let erp = buf.get_f32();
        let frequency = buf.get_f32();
        let pgrf = buf.get_f32();
        let pulse_width = buf.get_f32();
        let burst_length = buf.get_u32();
        let applicable_modes = buf.get_u8();
        let mut system_specific_data: [u8; 3] = [0; 3];
        for i in 0..3 {
            system_specific_data[i] = buf.get_u8();
        }
        IFFFundamentalParameterData {
            erp,
            frequency,
            pgrf,
            pulse_width,
            burst_length,
            applicable_modes,
            system_specific_data,
        }
    }
}
