//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §6.2.11
pub struct BeamData {
    pub beam_azimuth_center: f32,
    pub beam_azimuth_sweep: f32,
    pub beam_elevation_center: f32,
    pub beam_elevation_sweep: f32,
    pub beam_sweep_sync: f32,
}

impl BeamData {
    #[must_use]
    pub fn new(
        beam_azimuth_center: f32,
        beam_azimuth_sweep: f32,
        beam_elevation_center: f32,
        beam_elevation_sweep: f32,
        beam_sweep_sync: f32,
    ) -> Self {
        BeamData {
            beam_azimuth_center,
            beam_azimuth_sweep,
            beam_elevation_center,
            beam_elevation_sweep,
            beam_sweep_sync,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f32(self.beam_azimuth_center);
        buf.put_f32(self.beam_azimuth_sweep);
        buf.put_f32(self.beam_elevation_center);
        buf.put_f32(self.beam_elevation_sweep);
        buf.put_f32(self.beam_sweep_sync);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> BeamData {
        BeamData {
            beam_azimuth_center: buf.get_f32(),
            beam_azimuth_sweep: buf.get_f32(),
            beam_elevation_center: buf.get_f32(),
            beam_elevation_sweep: buf.get_f32(),
            beam_sweep_sync: buf.get_f32(),
        }
    }
}
