//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::{
    common::SerializedLength,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

use super::simulation_address::SimulationAddress;

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct EventId {
    pub simulation_address: SimulationAddress,
    pub event_identifier: u16,
}

impl EventId {
    #[must_use]
    pub const fn new(
        site_identifier: u16,
        application_identifier: u16,
        event_identifier: u16,
    ) -> Self {
        Self {
            simulation_address: SimulationAddress::new(site_identifier, application_identifier),
            event_identifier,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.simulation_address.serialize(buf);
        buf.put_u16(self.event_identifier);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            simulation_address: SimulationAddress::deserialize(buf),
            event_identifier: buf.get_u16(),
        }
    }
}

impl FieldSerialize for EventId {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for EventId {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for EventId {
    fn field_len(&self) -> usize {
        Self::LENGTH
    }
}

impl SerializedLength for EventId {
    const LENGTH: usize = SimulationAddress::LENGTH + 2;
}
