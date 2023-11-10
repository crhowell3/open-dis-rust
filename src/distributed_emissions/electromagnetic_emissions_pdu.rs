//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use super::electromagnetic_emission_system_data::ElectromagneticEmissionSystemData;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    event_id::EventId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §5.7.3
pub struct ElectromagneticEmissionsPdu {
    pub pdu_header: PduHeader,
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub state_update_indicator: u8,
    pub number_of_systems: u8,
    pub padding_for_emissions_pdu: u8,
    pub systems: Vec<ElectromagneticEmissionSystemData>,
}

impl ElectromagneticEmissionsPdu {
    /// Creates a default Electromagnetic Emissions PDU with arbitrary data
    ///
    /// # Examples
    ///
    /// Initializing an Electromagnetic Emissions PDU:
    /// ```
    /// use open_dis_rust::distributed_emissions::electromagnetic_emissions_pdu::ElectromagneticEmissionsPdu;
    /// let electromagnetic_emissions_pdu = ElectromagneticEmissionsPdu::default();
    /// ```
    ///
    pub fn default() -> Self {
        ElectromagneticEmissionsPdu {
            pdu_header: PduHeader::default(
                PduType::ElectromagneticEmission,
                ProtocolFamily::DistributedEmissionRegeneration,
                56,
            ),
            emitting_entity_id: EntityId::default(1),
            event_id: EventId::default(1),
            state_update_indicator: 0,
            number_of_systems: 0,
            padding_for_emissions_pdu: 0,
            systems: vec![],
        }
    }
}

impl Pdu for ElectromagneticEmissionsPdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.emitting_entity_id.serialize(buf);
        self.event_id.serialize(buf);
        buf.put_u8(self.state_update_indicator);
        buf.put_u8(self.number_of_systems);
        buf.put_u8(self.padding_for_emissions_pdu);
        for i in 0..self.number_of_systems {
            self.systems[i as usize].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::ElectromagneticEmission {
            let emitting_entity_id = EntityId::decode(&mut buffer);
            let event_id = EventId::decode(&mut buffer);
            let state_update_indicator = buffer.get_u8();
            let number_of_systems = buffer.get_u8();
            let padding_for_emissions_pdu = buffer.get_u8();
            let mut systems: Vec<ElectromagneticEmissionSystemData> = vec![];
            for _i in 0..number_of_systems {
                systems.push(ElectromagneticEmissionSystemData::decode(&mut buffer));
            }

            Ok(ElectromagneticEmissionsPdu {
                pdu_header,
                emitting_entity_id,
                event_id,
                state_update_indicator,
                number_of_systems,
                padding_for_emissions_pdu,
                systems,
            })
        } else {
            Err(DISError::InvalidDISHeader)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn deserialize_without_header(
        mut buffer: BytesMut,
        pdu_header: PduHeader,
    ) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let emitting_entity_id = EntityId::decode(&mut buffer);
        let event_id = EventId::decode(&mut buffer);
        let state_update_indicator = buffer.get_u8();
        let number_of_systems = buffer.get_u8();
        let padding_for_emissions_pdu = buffer.get_u8();
        let mut systems: Vec<ElectromagneticEmissionSystemData> = vec![];
        for _i in 0..number_of_systems {
            systems.push(ElectromagneticEmissionSystemData::decode(&mut buffer));
        }

        Ok(ElectromagneticEmissionsPdu {
            pdu_header,
            emitting_entity_id,
            event_id,
            state_update_indicator,
            number_of_systems,
            padding_for_emissions_pdu,
            systems,
        })
    }
}
