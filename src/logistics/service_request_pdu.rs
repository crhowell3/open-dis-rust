//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    SerializedLength,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily, ServiceRequestServiceTypeRequested},
    pdu::Pdu,
    pdu_header::PduHeader,
};

use super::data_types::supply_quantity::SupplyQuantity;

#[derive(Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.4.2
pub struct ServiceRequestPdu {
    pdu_header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub servicing_entity_id: EntityId,
    pub service_type_requested: ServiceRequestServiceTypeRequested,
    pub number_of_supply_types: u8,
    _padding: u16,
    pub supplies: Vec<SupplyQuantity>,
}

impl Pdu for ServiceRequestPdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH + EntityId::LENGTH * 2 + 1 + 1 + 2;

        length as u16
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
        self.receiving_entity_id.serialize(buf);
        self.servicing_entity_id.serialize(buf);
        buf.put_u8(self.service_type_requested as u8);
        buf.put_u8(self.number_of_supply_types);
        buf.put_u16(self._padding);
        for i in 0..self.supplies.len() {
            self.supplies[i].serialize(buf);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::ServiceRequest {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type ServiceRequest, got {:?}",
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

impl ServiceRequestPdu {
    /// Creates a new `ServiceRequestPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `ServiceRequestPdu`:
    /// ```
    /// use open_dis_rust::logistics::ServiceRequestPdu;
    /// let pdu = ServiceRequestPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::ServiceRequest;
        pdu.pdu_header.protocol_family = ProtocolFamily::Logistics;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let receiving_entity_id = EntityId::deserialize(buf);
        let servicing_entity_id = EntityId::deserialize(buf);
        let service_type_requested = ServiceRequestServiceTypeRequested::deserialize(buf);
        let number_of_supply_types = buf.get_u8();
        let _padding = buf.get_u16();
        let mut supplies: Vec<SupplyQuantity> = vec![];
        for _i in 0..number_of_supply_types {
            supplies.push(SupplyQuantity::deserialize(buf));
        }

        ServiceRequestPdu {
            pdu_header: PduHeader::default(),
            receiving_entity_id,
            servicing_entity_id,
            service_type_requested,
            number_of_supply_types,
            _padding,
            supplies,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceRequestPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = ServiceRequestPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<ServiceRequestPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = ServiceRequestPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = ServiceRequestPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
        let pdu = ServiceRequestPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
