//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
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
    pub const fn new(
        beam_azimuth_center: f32,
        beam_azimuth_sweep: f32,
        beam_elevation_center: f32,
        beam_elevation_sweep: f32,
        beam_sweep_sync: f32,
    ) -> Self {
        Self {
            beam_azimuth_center,
            beam_azimuth_sweep,
            beam_elevation_center,
            beam_elevation_sweep,
            beam_sweep_sync,
        }
    }
}
