//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::data_types::vector3_float::Vector3Float;

use super::{acoustic_beam_data::AcousticBeamData, acoustic_emitter_system::AcousticEmitterSystem};

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AcousticEmitterSystemData {
    pub emitter_system_data_length: u8,
    #[deku(update = "self.beam_records.len()")]
    pub number_of_beams: u8,
    pub pad2: u16,
    pub acoustic_emitter_system: AcousticEmitterSystem,
    pub emitter_location: Vector3Float,
    #[deku(count = "number_of_beams")]
    pub beam_records: Vec<AcousticBeamData>,
}

impl AcousticEmitterSystemData {
    #[must_use]
    pub const fn new(
        emitter_system_data_length: u8,
        number_of_beams: u8,
        pad2: u16,
        acoustic_emitter_system: AcousticEmitterSystem,
        emitter_location: Vector3Float,
        beam_records: Vec<AcousticBeamData>,
    ) -> Self {
        Self {
            emitter_system_data_length,
            number_of_beams,
            pad2,
            acoustic_emitter_system,
            emitter_location,
            beam_records,
        }
    }
}

