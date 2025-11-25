//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use super::data_types::electromagnetic_emission_system_data::ElectromagneticEmissionSystemData;

use crate::common::{
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{EEAttributeStateIndicator, PduType, ProtocolFamily},
    event_id::EventId,
    pdu::Pdu,
    pdu_header::PduHeader,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.6.2
pub struct ElectromagneticEmissionsPdu {
    pdu_header: PduHeader,
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub state_update_indicator: EEAttributeStateIndicator,
    pub number_of_systems: u8,
    pub padding: u16,
    pub systems: Vec<ElectromagneticEmissionSystemData>,
}

impl Default for ElectromagneticEmissionsPdu {
    fn default() -> Self {
        ElectromagneticEmissionsPdu {
            pdu_header: PduHeader::default(),
            emitting_entity_id: EntityId::default(),
            event_id: EventId::default(1),
            state_update_indicator: EEAttributeStateIndicator::default(),
            number_of_systems: 0u8,
            padding: 0u16,
            systems: vec![],
        }
    }
}

impl Pdu for ElectromagneticEmissionsPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>()
            + std::mem::size_of::<EventId>()
            + std::mem::size_of::<EEAttributeStateIndicator>()
            + std::mem::size_of::<u8>()
            + std::mem::size_of::<u16>();

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    /// Serialize contents of `ElectromagneticEmissionsPdu` into `BytesMut` buf
    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let size = std::mem::size_of_val(self);
        self.pdu_header.length = u16::try_from(size).map_err(|_| DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })?;
        self.pdu_header.serialize(buf);
        self.emitting_entity_id.serialize(buf);
        self.event_id.serialize(buf);
        buf.put_u8(self.state_update_indicator as u8);
        buf.put_u8(self.number_of_systems);
        buf.put_u16(self.padding);
        for i in 0..self.number_of_systems {
            self.systems[i as usize].serialize(buf);
        }
        Ok(())
    }

    /// Deserialize bytes from `BytesMut` buf and interpret as `ElectromagneticEmissionsPdu`
    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::ElectromagneticEmission {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type ElectromagneticEmission, got {:?}",
                    header.pdu_type
                ),
                None,
            ));
        }
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn deserialize_without_header<B: Buf>(buf: &mut B, header: PduHeader) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }
}

impl ElectromagneticEmissionsPdu {
    /// Creates a new `ElectromagneticEmissionsPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `ElectromagneticEmissionsPdu`:
    /// ```
    /// use open_dis_rust::distributed_emissions::ElectromagneticEmissionsPdu;
    /// let pdu = ElectromagneticEmissionsPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::ElectromagneticEmission;
        pdu.pdu_header.protocol_family = ProtocolFamily::DistributedEmissionRegeneration;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let emitting_entity_id = EntityId::deserialize(buf);
        let event_id = EventId::deserialize(buf);
        let state_update_indicator = EEAttributeStateIndicator::deserialize(buf);
        let number_of_systems = buf.get_u8();
        let padding = buf.get_u16();
        let mut systems: Vec<ElectromagneticEmissionSystemData> = vec![];
        for _i in 0..number_of_systems {
            systems.push(ElectromagneticEmissionSystemData::deserialize(buf));
        }

        ElectromagneticEmissionsPdu {
            pdu_header: PduHeader::default(),
            emitting_entity_id,
            event_id,
            state_update_indicator,
            number_of_systems,
            padding,
            systems,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ElectromagneticEmissionsPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = ElectromagneticEmissionsPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<ElectromagneticEmissionsPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = ElectromagneticEmissionsPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = ElectromagneticEmissionsPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
        let pdu = ElectromagneticEmissionsPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
