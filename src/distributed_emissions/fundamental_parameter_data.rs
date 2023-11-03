use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
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

impl FundamentalParameterData {}
