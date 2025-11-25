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
    enums::{IsGroupOfGroupedEntityCategory, PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.8.3
pub struct IsGroupOfPdu {
    pdu_header: PduHeader,
    pub group_entity_id: EntityId,
    pub grouped_entity_category: IsGroupOfGroupedEntityCategory,
    pub number_of_grouped_entities: u8,
    padding: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub grouped_entity_descriptions: Vec<u64>,
}

impl Default for IsGroupOfPdu {
    fn default() -> Self {
        IsGroupOfPdu {
            pdu_header: PduHeader::default(),
            group_entity_id: EntityId::default(),
            grouped_entity_category: IsGroupOfGroupedEntityCategory::default(),
            number_of_grouped_entities: 0,
            padding: 0,
            latitude: 0.0,
            longitude: 0.0,
            grouped_entity_descriptions: vec![],
        }
    }
}

impl Pdu for IsGroupOfPdu {
    fn length(&self) -> Result<u16, DISError> {
        let length = PduHeader::LENGTH + EntityId::LENGTH + 1 + 1 + 4 + 8 + 8;

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
        self.group_entity_id.serialize(buf);
        buf.put_u8(self.grouped_entity_category as u8);
        buf.put_u8(self.number_of_grouped_entities);
        buf.put_u32(self.padding);
        buf.put_f64(self.latitude);
        buf.put_f64(self.longitude);
        for i in 0..self.grouped_entity_descriptions.len() {
            buf.put_u64(self.grouped_entity_descriptions[i]);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::IsGroupOf {
            return Err(DISError::invalid_header(
                format!("Expected PDU type IsGroupOf, got {:?}", header.pdu_type),
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

impl IsGroupOfPdu {
    #[must_use]
    /// Creates a new `IsGroupOfPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `IsGroupOfPdu`:
    /// ```
    /// use open_dis_rust::entity_management::IsGroupOfPdu;
    /// let pdu = IsGroupOfPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::IsGroupOf;
        pdu.pdu_header.protocol_family = ProtocolFamily::EntityManagement;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let group_entity_id = EntityId::deserialize(buf);
        let grouped_entity_category = IsGroupOfGroupedEntityCategory::deserialize(buf);
        let number_of_grouped_entities = buf.get_u8();
        let padding = buf.get_u32();
        let latitude = buf.get_f64();
        let longitude = buf.get_f64();
        let mut grouped_entity_descriptions: Vec<u64> = vec![];
        for _i in 0..number_of_grouped_entities {
            grouped_entity_descriptions.push(buf.get_u64());
        }
        IsGroupOfPdu {
            pdu_header: PduHeader::default(),
            group_entity_id,
            grouped_entity_category,
            number_of_grouped_entities,
            padding,
            latitude,
            longitude,
            grouped_entity_descriptions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IsGroupOfPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = IsGroupOfPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<IsGroupOfPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = IsGroupOfPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = IsGroupOfPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
        let pdu = IsGroupOfPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
