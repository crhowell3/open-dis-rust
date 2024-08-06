//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct AcousticBeamFundamentalParameter {
    pub active_emission_parameter_index: u16,
    pub scan_pattern: u16,
    pub beam_center_azimuth: f32,
    pub azimuthal_beamwidth: f32,
    pub beam_center_de: f32,
    pub de_beamwidth: f32,
}

impl AcousticBeamFundamentalParameter {
    #[must_use]
    pub fn new(
        active_emission_parameter_index: u16,
        scan_pattern: u16,
        beam_center_azimuth: f32,
        azimuthal_beamwidth: f32,
        beam_center_de: f32,
        de_beamwidth: f32,
    ) -> Self {
        AcousticBeamFundamentalParameter {
            active_emission_parameter_index,
            scan_pattern,
            beam_center_azimuth,
            azimuthal_beamwidth,
            beam_center_de,
            de_beamwidth,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.active_emission_parameter_index);
        buf.put_u16(self.scan_pattern);
        buf.put_f32(self.beam_center_azimuth);
        buf.put_f32(self.azimuthal_beamwidth);
        buf.put_f32(self.beam_center_de);
        buf.put_f32(self.de_beamwidth);
    }

    pub fn decode(buf: &mut BytesMut) -> AcousticBeamFundamentalParameter {
        AcousticBeamFundamentalParameter {
            active_emission_parameter_index: buf.get_u16(),
            scan_pattern: buf.get_u16(),
            beam_center_azimuth: buf.get_f32(),
            azimuthal_beamwidth: buf.get_f32(),
            beam_center_de: buf.get_f32(),
            de_beamwidth: buf.get_f32(),
        }
    }
}
