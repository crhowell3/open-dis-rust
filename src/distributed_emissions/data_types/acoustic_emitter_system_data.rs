//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::vector3_float::Vector3Float;

use super::{acoustic_beam_data::AcousticBeamData, acoustic_emitter_system::AcousticEmitterSystem};

#[derive(Clone, Debug, Default)]
pub struct AcousticEmitterSystemData {
    pub emitter_system_data_length: u8,
    pub number_of_beams: u8,
    pub pad2: u16,
    pub acoustic_emitter_system: AcousticEmitterSystem,
    pub emitter_location: Vector3Float,
    pub beam_records: Vec<AcousticBeamData>,
}

impl AcousticEmitterSystemData {
    #[must_use]
    pub fn new(
        emitter_system_data_length: u8,
        number_of_beams: u8,
        pad2: u16,
        acoustic_emitter_system: AcousticEmitterSystem,
        emitter_location: Vector3Float,
        beam_records: Vec<AcousticBeamData>,
    ) -> Self {
        AcousticEmitterSystemData {
            emitter_system_data_length,
            number_of_beams,
            pad2,
            acoustic_emitter_system,
            emitter_location,
            beam_records,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.emitter_system_data_length);
        buf.put_u8(self.number_of_beams);
        buf.put_u16(self.pad2);
        self.acoustic_emitter_system.serialize(buf);
        self.emitter_location.serialize(buf);
        for i in 0..self.beam_records.len() {
            self.beam_records[i].serialize(buf);
        }
    }

    pub fn deserialize(buf: &mut BytesMut) -> AcousticEmitterSystemData {
        let emitter_system_data_length = buf.get_u8();
        let number_of_beams = buf.get_u8();
        let pad2 = buf.get_u16();
        let acoustic_emitter_system = AcousticEmitterSystem::deserialize(buf);
        let emitter_location = Vector3Float::deserialize(buf);
        let mut beam_records: Vec<AcousticBeamData> = vec![];
        for _i in 0..number_of_beams {
            beam_records.push(AcousticBeamData::deserialize(buf));
        }
        AcousticEmitterSystemData {
            emitter_system_data_length,
            number_of_beams,
            pad2,
            acoustic_emitter_system,
            emitter_location,
            beam_records,
        }
    }
}
