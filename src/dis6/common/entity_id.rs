use super::simulation_address::SimulationAddress;
use bytes::{Buf, BufMut, BytesMut};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EntityId {
    pub simulation_address: SimulationAddress,
    pub entity_id: u16,
}

impl EntityId {
    pub fn new(site_id: u16, application_id: u16, entity_id: u16) -> Self {
        EntityId {
            simulation_address: SimulationAddress::new(site_id, application_id),
            entity_id,
        }
    }

    pub fn default(entity_id: u16) -> Self {
        EntityId {
            simulation_address: SimulationAddress::default(),
            entity_id,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.simulation_address.serialize(buf);
        buf.put_u16(self.entity_id);
    }

    pub fn decode(buf: &mut BytesMut) -> EntityId {
        EntityId {
            simulation_address: EntityId::decode_simulation_address(buf),
            entity_id: buf.get_u16(),
        }
    }

    fn decode_simulation_address(buf: &mut BytesMut) -> SimulationAddress {
        SimulationAddress {
            site_id: buf.get_u16(),
            application_id: buf.get_u16(),
        }
    }
}
