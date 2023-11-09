use super::electromagnetic_emission_beam_data::ElectromagneticEmissionBeamData;
use bytes::{Buf, BufMut, BytesMut};

use crate::common::vector3_float::Vector3Float;
use crate::distributed_emissions::emitter_system::EmitterSystem;

#[derive(Clone, Debug)]
pub struct ElectromagneticEmissionSystemData {
    pub system_data_length: u8,
    pub number_of_beams: u8,
    pub emissions_padding2: u8,
    pub emitter_system: EmitterSystem,
    pub location: Vector3Float,
    pub beam_data_records: Vec<ElectromagneticEmissionBeamData>,
}

impl ElectromagneticEmissionSystemData {
    pub fn default() -> Self {
        ElectromagneticEmissionSystemData {
            system_data_length: 0,
            number_of_beams: 0,
            emissions_padding2: 0,
            emitter_system: EmitterSystem::default(),
            location: Vector3Float::new(0.0, 0.0, 0.0),
            beam_data_records: vec![],
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.system_data_length as u8);
        buf.put_u8(self.number_of_beams as u8);
        buf.put_u8(self.emissions_padding2 as u8);
        self.emitter_system.serialize(buf);
        self.location.serialize(buf);
        for beams in &self.beam_data_records {
            beams.serialize(buf);
        }
    }

    pub fn decode(buf: &mut BytesMut) -> ElectromagneticEmissionSystemData {
        let system_data_length = buf.get_u8();
        let number_of_beams = buf.get_u8();
        let emissions_padding2 = buf.get_u8();
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
