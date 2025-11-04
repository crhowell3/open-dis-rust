//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use super::acoustic_beam_fundamental_parameter::AcousticBeamFundamentalParameter;

#[derive(Copy, Clone, Debug, Default)]
pub struct AcousticBeamData {
    pub beam_data_length: u16,
    pub beam_id_number: u8,
    pub pad2: u16,
    pub fundamental_data_parameters: AcousticBeamFundamentalParameter,
}

impl AcousticBeamData {
    #[must_use]
    pub fn new(
        beam_data_length: u16,
        beam_id_number: u8,
        pad2: u16,
        fundamental_data_parameters: AcousticBeamFundamentalParameter,
    ) -> Self {
        AcousticBeamData {
            beam_data_length,
            beam_id_number,
            pad2,
            fundamental_data_parameters,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.beam_data_length);
        buf.put_u8(self.beam_id_number);
        buf.put_u16(self.pad2);
        self.fundamental_data_parameters.serialize(buf);
    }

    pub fn deserialize(buf: &mut BytesMut) -> AcousticBeamData {
        AcousticBeamData {
            beam_data_length: buf.get_u16(),
            beam_id_number: buf.get_u8(),
            pad2: buf.get_u16(),
            fundamental_data_parameters: AcousticBeamFundamentalParameter::deserialize(buf),
        }
    }
}
