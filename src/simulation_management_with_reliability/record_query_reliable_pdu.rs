//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    SerializedLength,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{EventType, PduType, ProtocolFamily, RequiredReliabilityService},
    pdu::Pdu,
    pdu_header::PduHeader,
};

#[derive(Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.11.14
pub struct RecordQueryReliablePdu {
    pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub required_reliability_service: RequiredReliabilityService,
    padding: u8,
    pub event_type: EventType,
    pub time: u32,
    pub number_of_records: u32,
    pub record_ids: Vec<u32>,
}

impl Pdu for RecordQueryReliablePdu {
    fn length(&self) -> Result<u16, DISError> {
        let length = PduHeader::LENGTH + EntityId::LENGTH * 2 + 4 + 1 + 1 + 2 + 4 + 4;

        u16::try_from(length).map_err(|_| DISError::PduSizeExceeded {
            size: length,
            max_size: MAX_PDU_SIZE_OCTETS,
        })
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let size = std::mem::size_of_val(self);
        self.pdu_header.length = u16::try_from(size).map_err(|_| DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })?;
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u32(self.request_id);
        buf.put_u8(self.required_reliability_service as u8);
        buf.put_u8(self.padding);
        buf.put_u32(self.event_type as u32);
        buf.put_u32(self.time);
        buf.put_u32(self.number_of_records);
        for i in &self.record_ids {
            buf.put_u32(*i);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::RecordQueryReliable {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type RecordQueryReliable, got {:?}",
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

impl RecordQueryReliablePdu {
    #[must_use]
    /// Creates a `RecordQueryReliablePdu`
    ///
    /// # Examples
    ///
    /// Initializing a `RecordQueryReliablePdu`:
    /// ```
    /// use open_dis_rust::simulation_management_with_reliability::RecordQueryReliablePdu;
    /// let mut pdu = RecordQueryReliablePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::RecordQueryReliable;
        pdu.pdu_header.protocol_family = ProtocolFamily::SimulationManagementWithReliability;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_entity_id = EntityId::deserialize(buf);
        let receiving_entity_id = EntityId::deserialize(buf);
        let request_id = buf.get_u32();
        let required_reliability_service = RequiredReliabilityService::deserialize(buf);
        let padding = buf.get_u8();
        let event_type = EventType::deserialize(buf);
        let time = buf.get_u32();
        let number_of_records = buf.get_u32();
        let mut record_ids: Vec<u32> = vec![];
        for _i in 0..number_of_records {
            record_ids.push(buf.get_u32());
        }

        RecordQueryReliablePdu {
            pdu_header: PduHeader::default(),
            originating_entity_id,
            receiving_entity_id,
            request_id,
            required_reliability_service,
            padding,
            event_type,
            time,
            number_of_records,
            record_ids,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RecordQueryReliablePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;
    #[test]
    fn cast_to_any() {
        let pdu = RecordQueryReliablePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<RecordQueryReliablePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = RecordQueryReliablePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = RecordQueryReliablePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
        let pdu = RecordQueryReliablePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
