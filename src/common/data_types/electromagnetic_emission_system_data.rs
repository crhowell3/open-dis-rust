//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use super::{
    electromagnetic_emission_beam_data::ElectromagneticEmissionBeamData,
    emitter_system::EmitterSystem,
};

use crate::common::data_types::vector3_float::Vector3Float;

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ElectromagneticEmissionSystemData {
    pub system_data_length: u8,
    #[deku(update = "crate::common::wire::update_each(&mut self.beam_data_records)?")]
    pub number_of_beams: u8,
    pub emissionspadding2: u16,
    pub emitter_system: EmitterSystem,
    pub location: Vector3Float,
    #[deku(count = "number_of_beams")]
    pub beam_data_records: Vec<ElectromagneticEmissionBeamData>,
}

