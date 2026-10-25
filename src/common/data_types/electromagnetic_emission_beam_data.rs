//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use super::{
    fundamental_parameter_data::FundamentalParameterData, track_jam_target::TrackJamTarget,
};

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ElectromagneticEmissionBeamData {
    pub beam_data_length: u8,
    pub beam_id_number: u8,
    pub beam_parameter_index: u16,
    pub fundamental_parameter_data: FundamentalParameterData,
    pub beam_function: u8,
    #[deku(update = "self.track_jam_targets.len()")]
    pub number_of_track_jam_targets: u8,
    pub high_density_track_jam: u8,
    pub pad4: u8,
    pub jamming_mode_sequence: u32,
    #[deku(count = "number_of_track_jam_targets")]
    pub track_jam_targets: Vec<TrackJamTarget>,
}
