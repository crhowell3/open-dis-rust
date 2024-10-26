//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use super::data_types::electromagnetic_emission_system_data::ElectromagneticEmissionSystemData;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    event_id::EventId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.6.2
pub struct ElectromagneticEmissionsPdu {
    pub pdu_header: PduHeader,
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub state_update_indicator: u8,
    pub number_of_systems: u8,
    pub padding_for_emissions_pdu: u8,
    pub systems: Vec<ElectromagneticEmissionSystemData>,
}

impl Default for ElectromagneticEmissionsPdu {
    /// Creates a default-initialized Electromagnetic Emissions PDU
    ///
    /// # Examples
    ///
    /// Initializing an Electromagnetic Emissions PDU:
    /// ```
    /// use open_dis_rust::distributed_emissions::electromagnetic_emissions_pdu::ElectromagneticEmissionsPdu;
    /// let mut electromagnetic_emissions_pdu = ElectromagneticEmissionsPdu::default();
    /// ```
    ///
    fn default() -> Self {
        ElectromagneticEmissionsPdu {
            pdu_header: PduHeader::default(
                PduType::ElectromagneticEmission,
                ProtocolFamily::DistributedEmissionRegeneration,
                28,
            ),
            emitting_entity_id: EntityId::default(1),
            event_id: EventId::default(1),
            state_update_indicator: 0,
            number_of_systems: 1,
            padding_for_emissions_pdu: 0,
            systems: vec![ElectromagneticEmissionSystemData::default()],
        }
    }
}

impl Pdu for ElectromagneticEmissionsPdu {
    /// Serialize contents of `ElectromagneticEmissionsPdu` into `BytesMut` buffer
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
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

    /// Deserialize bytes from `BytesMut` buffer and interpret as `ElectromagneticEmissionsPdu`
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

    /// Treat `ElectromagneticEmissionsPdu` as Any type
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Deserialize bytes from `BytesMut` buffer, but assume PDU header exists already
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
#[cfg(test)]
mod tests {
    use super::ElectromagneticEmissionsPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let electromagnetic_emissions_pdu = ElectromagneticEmissionsPdu::default();
        let pdu_header = PduHeader::default(
            PduType::ElectromagneticEmission,
            ProtocolFamily::DistributedEmissionRegeneration,
            28,
        );

        assert_eq!(
            pdu_header.protocol_version,
            electromagnetic_emissions_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            electromagnetic_emissions_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            electromagnetic_emissions_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            electromagnetic_emissions_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            electromagnetic_emissions_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.padding,
            electromagnetic_emissions_pdu.pdu_header.padding
        );
    }

    #[test]
    fn cast_to_any() {
        let electromagnetic_emissions_pdu = ElectromagneticEmissionsPdu::default();
        let any_pdu = electromagnetic_emissions_pdu.as_any();

        assert!(any_pdu.is::<ElectromagneticEmissionsPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut electromagnetic_emissions_pdu = ElectromagneticEmissionsPdu::default();
        let mut buffer = BytesMut::new();
        electromagnetic_emissions_pdu.serialize(&mut buffer);

        let new_electromagnetic_emissions_pdu =
            ElectromagneticEmissionsPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_electromagnetic_emissions_pdu.pdu_header,
            electromagnetic_emissions_pdu.pdu_header
        );
    }
}
