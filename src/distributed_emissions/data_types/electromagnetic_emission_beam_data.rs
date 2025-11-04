use bytes::{Buf, BufMut, BytesMut};

use super::{
    fundamental_parameter_data::FundamentalParameterData, track_jam_target::TrackJamTarget,
};

#[derive(Clone, Debug, Default)]
pub struct ElectromagneticEmissionBeamData {
    pub beam_data_length: u8,
    pub beam_id_number: u8,
    pub beam_parameter_index: u16,
    pub fundamental_parameter_data: FundamentalParameterData,
    pub beam_function: u8,
    pub number_of_track_jam_targets: u8,
    pub high_density_track_jam: u8,
    pub pad4: u8,
    pub jamming_mode_sequence: u32,
    pub track_jam_targets: Vec<TrackJamTarget>,
}

impl ElectromagneticEmissionBeamData {
    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.beam_data_length);
        buf.put_u8(self.beam_id_number);
        buf.put_u16(self.beam_parameter_index);
        self.fundamental_parameter_data.serialize(buf);
        buf.put_u8(self.beam_function);
        buf.put_u8(self.number_of_track_jam_targets);
        buf.put_u8(self.high_density_track_jam);
        buf.put_u8(self.pad4);
        buf.put_u32(self.jamming_mode_sequence);
        for target in &self.track_jam_targets {
            target.serialize(buf);
        }
    }

    pub fn deserialize(buf: &mut BytesMut) -> ElectromagneticEmissionBeamData {
        let beam_data_length = buf.get_u8();
        let beam_id_number = buf.get_u8();
        let beam_parameter_index = buf.get_u16();
        let fundamental_parameter_data = FundamentalParameterData::deserialize(buf);
        let beam_function = buf.get_u8();
        let number_of_track_jam_targets = buf.get_u8();
        let high_density_track_jam = buf.get_u8();
        let pad4 = buf.get_u8();
        let jamming_mode_sequence = buf.get_u32();
        let mut track_jam_targets: Vec<TrackJamTarget> = vec![];
        for _i in 0..number_of_track_jam_targets {
            track_jam_targets.push(TrackJamTarget::deserialize(buf));
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
