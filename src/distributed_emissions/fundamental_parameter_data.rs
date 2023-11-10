//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct FundamentalParameterData {
    pub frequency: f32,
    pub frequency_range: f32,
    pub effective_radiated_power: f32,
    pub pulse_repetition_frequency: f32,
    pub pulse_width: f32,
    pub beam_azimuth_center: f32,
    pub beam_azimuth_sweep: f32,
    pub beam_elevation_center: f32,
    pub beam_elevation_sweep: f32,
    pub beam_sweep_sync: f32,
}

impl FundamentalParameterData {
    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.frequency);
        buf.put_f32(self.frequency_range);
        buf.put_f32(self.effective_radiated_power);
        buf.put_f32(self.pulse_repetition_frequency);
        buf.put_f32(self.pulse_width);
        buf.put_f32(self.beam_azimuth_center);
        buf.put_f32(self.beam_azimuth_sweep);
        buf.put_f32(self.beam_elevation_center);
        buf.put_f32(self.beam_elevation_sweep);
        buf.put_f32(self.beam_sweep_sync);
    }

    pub fn decode(buf: &mut BytesMut) -> FundamentalParameterData {
        FundamentalParameterData {
            frequency: buf.get_f32(),
            frequency_range: buf.get_f32(),
            effective_radiated_power: buf.get_f32(),
            pulse_repetition_frequency: buf.get_f32(),
            pulse_width: buf.get_f32(),
            beam_azimuth_center: buf.get_f32(),
            beam_azimuth_sweep: buf.get_f32(),
            beam_elevation_center: buf.get_f32(),
            beam_elevation_sweep: buf.get_f32(),
            beam_sweep_sync: buf.get_f32(),
        }
    }
}
