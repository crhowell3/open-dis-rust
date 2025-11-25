//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BytesMut};
use std::any::Any;

use crate::common::{
    SerializedLength,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    entity_type::EntityType,
    enums::{PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
    vector3_float::Vector3Float,
};

use super::data_types::{named_location::NamedLocation, relationship::Relationship};

#[derive(Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.8.5
pub struct IsPartOfPdu {
    pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub relationship: Relationship,
    pub part_location: Vector3Float,
    pub named_location_id: NamedLocation,
    pub part_entity_type: EntityType,
}

impl Pdu for IsPartOfPdu {
    fn length(&self) -> Result<u16, DISError> {
        let length = PduHeader::LENGTH
            + EntityId::LENGTH * 2
            + Relationship::LENGTH
            + Vector3Float::LENGTH
            + NamedLocation::LENGTH
            + EntityType::LENGTH;

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
        self.relationship.serialize(buf);
        self.part_location.serialize(buf);
        self.named_location_id.serialize(buf);
        self.part_entity_type.serialize(buf);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::IsPartOf {
            return Err(DISError::invalid_header(
                format!("Expected PDU type IsPartOf, got {:?}", header.pdu_type),
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

impl IsPartOfPdu {
    #[must_use]
    /// Creates a new `IsPartOfPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `IsPartOfPdu`:
    /// ```
    /// use open_dis_rust::entity_management::IsPartOfPdu;
    /// let pdu = IsPartOfPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::IsPartOf;
        pdu.pdu_header.protocol_family = ProtocolFamily::EntityManagement;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_entity_id = EntityId::deserialize(buf);
        let receiving_entity_id = EntityId::deserialize(buf);
        let relationship = Relationship::deserialize(buf);
        let part_location = Vector3Float::deserialize(buf);
        let named_location_id = NamedLocation::deserialize(buf);
        let part_entity_type = EntityType::deserialize(buf);
        IsPartOfPdu {
            pdu_header: PduHeader::default(),
            originating_entity_id,
            receiving_entity_id,
            relationship,
            part_location,
            named_location_id,
            part_entity_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IsPartOfPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = IsPartOfPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<IsPartOfPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = IsPartOfPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = IsPartOfPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 416 / BITS_PER_BYTE;
        let pdu = IsPartOfPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
