//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use super::acoustic_beam_fundamental_parameter::AcousticBeamFundamentalParameter;

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AcousticBeamData {
    pub beam_data_length: u16,
    pub beam_id_number: u8,
    pub pad2: u16,
    pub fundamental_data_parameters: AcousticBeamFundamentalParameter,
}

impl AcousticBeamData {
    #[must_use]
    pub const fn new(
        beam_data_length: u16,
        beam_id_number: u8,
        pad2: u16,
        fundamental_data_parameters: AcousticBeamFundamentalParameter,
    ) -> Self {
        Self {
            beam_data_length,
            beam_id_number,
            pad2,
            fundamental_data_parameters,
        }
    }
}
