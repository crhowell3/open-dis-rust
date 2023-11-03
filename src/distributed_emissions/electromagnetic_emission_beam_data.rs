use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
pub struct ElectromagneticEmissionBeamData {
    pub beam_data_length: u8,
    pub beam_id_number: u8,
    pub beam_parameter_index: u8,
    pub fundamental_parameter_data: FundamentalParameterData,
    pub beam_function: u8,
    pub number_of_track_jam_targets: u8,
    pub high_density_track_jam: u8,
    pub pad4: u8,
    pub jamming_mode_sequence: u32,
    pub track_jam_targets: Vec<TrackJamTarget>,
}

impl ElectromagneticEmissionBeamData {}
