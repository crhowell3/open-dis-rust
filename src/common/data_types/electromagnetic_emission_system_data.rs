use super::{
    electromagnetic_emission_beam_data::ElectromagneticEmissionBeamData,
    emitter_system::EmitterSystem,
};
use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::{SerializedLength, data_types::vector3_float::Vector3Float},
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Clone, Debug, Default)]
pub struct ElectromagneticEmissionSystemData {
    pub system_data_length: u8,
    pub number_of_beams: u8,
    pub emissionspadding2: u16,
    pub emitter_system: EmitterSystem,
    pub location: Vector3Float,
    pub beam_data_records: Vec<ElectromagneticEmissionBeamData>,
}

impl ElectromagneticEmissionSystemData {
    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.system_data_length);
        buf.put_u8(self.number_of_beams);
        buf.put_u16(self.emissionspadding2);
        self.emitter_system.serialize(buf);
        self.location.serialize(buf);
        for beams in &self.beam_data_records {
            beams.serialize(buf);
        }
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let system_data_length = buf.get_u8();
        let number_of_beams = buf.get_u8();
        let emissionspadding2 = buf.get_u16();
        let emitter_system = EmitterSystem::deserialize(buf);
        let location = Vector3Float::deserialize(buf);
        let mut beam_data_records: Vec<ElectromagneticEmissionBeamData> = vec![];
        for _i in 0..number_of_beams {
            beam_data_records.push(ElectromagneticEmissionBeamData::deserialize(buf));
        }

        Self {
            system_data_length,
            number_of_beams,
            emissionspadding2,
            emitter_system,
            location,
            beam_data_records,
        }
    }
}

impl FieldSerialize for ElectromagneticEmissionSystemData {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for ElectromagneticEmissionSystemData {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for ElectromagneticEmissionSystemData {
    fn field_len(&self) -> usize {
        1 + 1
            + 2
            + EmitterSystem::LENGTH
            + self.location.field_len()
            + self.beam_data_records.field_len()
    }
}
