use bytes::{Buf, BufMut, BytesMut};

use super::{
    fundamental_parameter_data::FundamentalParameterData, track_jam_target::TrackJamTarget,
};

#[derive(Clone, Debug)]
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

impl ElectromagneticEmissionBeamData {
    pub fn default() -> Self {
        ElectromagneticEmissionBeamData {
            beam_data_length: 0,
            beam_id_number: 0,
            beam_parameter_index: 0,
            fundamental_parameter_data: FundamentalParameterData::default(),
            beam_function: 0,
            number_of_track_jam_targets: 0,
            high_density_track_jam: 0,
            pad4: 0,
            jamming_mode_sequence: 0,
            track_jam_targets: vec![],
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.beam_data_length as u8);
        buf.put_u8(self.beam_id_number as u8);
        buf.put_u8(self.beam_parameter_index as u8);
        self.fundamental_parameter_data.serialize(buf);
        buf.put_u8(self.beam_function as u8);
        buf.put_u8(self.number_of_track_jam_targets as u8);
        buf.put_u8(self.high_density_track_jam as u8);
        buf.put_u8(self.pad4 as u8);
        buf.put_u32(self.jamming_mode_sequence as u32);
        for target in &self.track_jam_targets {
            target.serialize(buf);
        }
    }

    pub fn decode(buf: &mut BytesMut) -> ElectromagneticEmissionBeamData {
        let beam_data_length = buf.get_u8();
        let beam_id_number = buf.get_u8();
        let beam_parameter_index = buf.get_u8();
        let fundamental_parameter_data = FundamentalParameterData::decode(buf);
        let beam_function = buf.get_u8();
        let number_of_track_jam_targets = buf.get_u8();
        let high_density_track_jam = buf.get_u8();
        let pad4 = buf.get_u8();
        let jamming_mode_sequence = buf.get_u32();
        let mut track_jam_targets: Vec<TrackJamTarget> = vec![];
        for _i in 0..number_of_track_jam_targets {
            track_jam_targets.push(TrackJamTarget::decode(buf));
        }

        ElectromagneticEmissionBeamData {
            beam_data_length,
            beam_id_number,
            beam_parameter_index,
            fundamental_parameter_data,
            beam_function,
            number_of_track_jam_targets,
            high_density_track_jam,
            pad4,
            jamming_mode_sequence,
            track_jam_targets,
        }
    }
}
