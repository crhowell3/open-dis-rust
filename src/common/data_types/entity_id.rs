//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::SerializedLength;

use super::simulation_address::SimulationAddress;
use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};
use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
/// Implemented according to IEEE 1278.1-2012 §6.2.28
pub struct EntityId {
    /// The simulation's designation associated with all object identifiers
    pub simulation_address: SimulationAddress,
    /// The unique identification number of the entity
    pub entity_id: u16,
}

impl EntityId {
    #[must_use]
    pub const fn new(site_id: u16, application_id: u16, entity_id: u16) -> Self {
        Self {
            simulation_address: SimulationAddress::new(site_id, application_id),
            entity_id,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.simulation_address.serialize(buf);
        buf.put_u16(self.entity_id);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            simulation_address: Self::deserialize_simulation_address(buf),
            entity_id: buf.get_u16(),
        }
    }

    fn deserialize_simulation_address<B: Buf>(buf: &mut B) -> SimulationAddress {
        SimulationAddress {
            site_id: buf.get_u16(),
            application_id: buf.get_u16(),
        }
    }
}

impl FieldSerialize for EntityId {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for EntityId {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for EntityId {
    fn field_len(&self) -> usize {
        Self::LENGTH
    }
}

impl SerializedLength for EntityId {
    const LENGTH: usize = 6;
}
