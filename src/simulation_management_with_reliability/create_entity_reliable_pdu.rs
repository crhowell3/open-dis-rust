//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    SerializedLength,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily, RequiredReliabilityService},
    pdu::Pdu,
    pdu_header::PduHeader,
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.11.2
pub struct CreateEntityReliablePdu {
    pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    padding: u8,
    padding2: u16,
    pub request_id: u32,
}

impl Pdu for CreateEntityReliablePdu {
    fn calculate_length(&self) -> Result<u16, DISError> {
        let length = PduHeader::LENGTH + EntityId::LENGTH * 2 + 1 + 1 + 2 + 4;

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
        let length = self.calculate_length()?;
        self.pdu_header.set_length(length);
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u8(self.required_reliability_service as u8);
        buf.put_u8(self.padding);
        buf.put_u16(self.padding2);
        buf.put_u32(self.request_id);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::CreateEntityReliable {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type CreateEntityReliable, got {:?}",
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

impl CreateEntityReliablePdu {
    #[must_use]
    /// Creates a new `CreateEntityReliablePdu`
    ///
    /// # Examples
    ///
    /// Initializing an `CreateEntityReliablePdu`:
    /// ```
    /// use open_dis_rust::simulation_management_with_reliability::CreateEntityReliablePdu;
    /// let pdu = CreateEntityReliablePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::CreateEntityReliable;
        pdu.pdu_header.protocol_family = ProtocolFamily::SimulationManagementWithReliability;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_entity_id = EntityId::deserialize(buf);
        let receiving_entity_id = EntityId::deserialize(buf);
        let required_reliability_service = RequiredReliabilityService::deserialize(buf);
        let padding = buf.get_u8();
        let padding2 = buf.get_u16();
        let request_id = buf.get_u32();

        CreateEntityReliablePdu {
            pdu_header: PduHeader::default(),
            originating_entity_id,
            receiving_entity_id,
            required_reliability_service,
            padding,
            padding2,
            request_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CreateEntityReliablePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = CreateEntityReliablePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<CreateEntityReliablePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = CreateEntityReliablePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = CreateEntityReliablePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
        let pdu = CreateEntityReliablePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
