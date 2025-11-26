//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    GenericHeader, LiveEntityPduHeader, SerializedLength,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily, RepairGroups},
    pdu::Pdu,
};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §9.4.2.5
pub struct TimeSpacePositionInformationPdu {
    pdu_header: LiveEntityPduHeader,
    pub receiving_entity_id: EntityId,
    pub repairing_entity_id: EntityId,
    pub repair: RepairGroups,
    padding: u16,
}

impl Pdu for TimeSpacePositionInformationPdu {
    type Header = LiveEntityPduHeader;

    fn calculate_length(&self) -> Result<u16, DISError> {
        let length = LiveEntityPduHeader::LENGTH + EntityId::LENGTH * 2 + 2 + 2;

        u16::try_from(length).map_err(|_| DISError::PduSizeExceeded {
            size: length,
            max_size: MAX_PDU_SIZE_OCTETS,
        })
    }

    fn header(&self) -> &Self::Header {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut Self::Header {
        &mut self.pdu_header
    }

    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let length = self.calculate_length()?;
        self.pdu_header.set_length(length);
        self.pdu_header.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        self.repairing_entity_id.serialize(buf);
        buf.put_u16(self.repair as u16);
        buf.put_u16(self.padding);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: Self::Header = Self::Header::deserialize(buf);
        if header.pdu_type != PduType::TimeSpacePositionInformation {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type TimeSpacePositionInformation, got {:?}",
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

    fn deserialize_without_header<B: Buf>(
        buf: &mut B,
        header: Self::Header,
    ) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }
}

impl TimeSpacePositionInformationPdu {
    #[must_use]
    /// Creates a new `TimeSpacePositionInformationPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `TimeSpacePositionInformationPdu`:
    /// ```
    /// use open_dis_rust::logistics::TimeSpacePositionInformationPdu;
    /// let pdu = TimeSpacePositionInformationPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::TimeSpacePositionInformation;
        pdu.pdu_header.protocol_family = ProtocolFamily::Logistics;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let receiving_entity_id = EntityId::deserialize(buf);
        let repairing_entity_id = EntityId::deserialize(buf);
        let repair = RepairGroups::deserialize(buf);
        let padding = buf.get_u16();

        TimeSpacePositionInformationPdu {
            pdu_header: Self::Header::default(),
            receiving_entity_id,
            repairing_entity_id,
            repair,
            padding,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TimeSpacePositionInformationPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = TimeSpacePositionInformationPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<TimeSpacePositionInformationPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = TimeSpacePositionInformationPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = TimeSpacePositionInformationPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
        let pdu = TimeSpacePositionInformationPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
