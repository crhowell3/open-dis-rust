use super::{
    electromagnetic_emission_beam_data::ElectromagneticEmissionBeamData,
    emitter_system::EmitterSystem,
};
use bytes::{Buf, BufMut, BytesMut};

use crate::common::vector3_float::Vector3Float;

#[derive(Clone, Debug, Default)]
pub struct ElectromagneticEmissionSystemData {
    pub system_data_length: u8,
    pub number_of_beams: u8,
    pub emissions_padding2: u16,
    pub emitter_system: EmitterSystem,
    pub location: Vector3Float,
    pub beam_data_records: Vec<ElectromagneticEmissionBeamData>,
}

impl ElectromagneticEmissionSystemData {
    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.system_data_length);
        buf.put_u8(self.number_of_beams);
        buf.put_u16(self.emissions_padding2);
        self.emitter_system.serialize(buf);
        self.location.serialize(buf);
        for beams in &self.beam_data_records {
            beams.serialize(buf);
        }
    }

    pub fn decode(buf: &mut BytesMut) -> ElectromagneticEmissionSystemData {
        let system_data_length = buf.get_u8();
        let number_of_beams = buf.get_u8();
        let emissions_padding2 = buf.get_u16();
        let emitter_system = EmitterSystem::decode(buf);
        let location = Vector3Float::decode(buf);
        let mut beam_data_records: Vec<ElectromagneticEmissionBeamData> = vec![];
        for _i in 0..number_of_beams {
            beam_data_records.push(ElectromagneticEmissionBeamData::decode(buf));
        }

        ElectromagneticEmissionSystemData {
            system_data_length,
            number_of_beams,
            emissions_padding2,
            emitter_system,
            location,
            beam_data_records,
        }
    }
}
