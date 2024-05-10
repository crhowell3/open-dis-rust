//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
pub struct BeamData {
    pub beam_azimuth_center: f32,
    pub beam_azimuth_sweep: f32,
    pub beam_elevation_center: f32,
    pub beam_elevation_sweep: f32,
    pub beam_sweep_sync: f32,
}

impl Default for BeamData {
    fn default() -> Self {
        BeamData {
            beam_azimuth_center: 0.0,
            beam_azimuth_sweep: 0.0,
            beam_elevation_center: 0.0,
            beam_elevation_sweep: 0.0,
            beam_sweep_sync: 0.0,
        }
    }
}

impl BeamData {
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

    pub fn decode(buf: &mut BytesMut) -> BeamData {
        BeamData {
            beam_azimuth_center: buf.get_f32(),
            beam_azimuth_sweep: buf.get_f32(),
            beam_elevation_center: buf.get_f32(),
            beam_elevation_sweep: buf.get_f32(),
            beam_sweep_sync: buf.get_f32(),
        }
    }
}
