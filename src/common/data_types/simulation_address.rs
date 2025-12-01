//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::SerializedLength,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Implemented according to IEEE 1278.1-2012 §6.2.80
pub struct SimulationAddress {
    /// Identification number representing the site, which may be a facility,
    /// installation, organizational unit, or a geographical location. Valid
    /// site ID values range from 1 to 65,534
    pub site_id: u16,
    /// Identification number representing the software program that is used to
    /// generate and process distributed simulation data. Valid application ID
    /// values range from 1 to 65,534
    pub application_id: u16,
}

impl Default for SimulationAddress {
    fn default() -> Self {
        Self {
            site_id: 1,
            application_id: 1,
        }
    }
}

impl SimulationAddress {
    #[must_use]
    pub const fn new(site_id: u16, application_id: u16) -> Self {
        Self {
            site_id,
            application_id,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.site_id);
        buf.put_u16(self.application_id);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            site_id: buf.get_u16(),
            application_id: buf.get_u16(),
        }
    }
}

impl FieldSerialize for SimulationAddress {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for SimulationAddress {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for SimulationAddress {
    fn field_len(&self) -> usize {
        Self::LENGTH
    }
}

impl SerializedLength for SimulationAddress {
    const LENGTH: usize = 4;
}
