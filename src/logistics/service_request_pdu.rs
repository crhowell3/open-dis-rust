//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

use super::data_types::supply_quantity::SupplyQuantity;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.4.2
pub struct ServiceRequestPdu {
    pub pdu_header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub servicing_entity_id: EntityId,
    pub service_type_requested: u8,
    pub number_of_supply_types: u8,
    pub padding1: i16,
    pub supplies: Vec<SupplyQuantity>,
}

impl Default for ServiceRequestPdu {
    /// Creates a default Service Request PDU with arbitrary receiving and supplying entity IDs
    ///
    /// # Examples
    ///
    /// Initializing a Service Request PDU:
    /// ```
    /// use open_dis_rust::logistics::service_request_pdu::ServiceRequestPdu;
    /// let service_request_pdu = ServiceRequestPdu::default();
    /// ```
    ///
    fn default() -> Self {
        ServiceRequestPdu {
            pdu_header: PduHeader::default(PduType::ServiceRequest, ProtocolFamily::Logistics, 56),
            receiving_entity_id: EntityId::default(1),
            servicing_entity_id: EntityId::default(2),
            service_type_requested: 0,
            number_of_supply_types: 0,
            padding1: 0,
            supplies: vec![],
        }
    }
}

impl Pdu for ServiceRequestPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        self.servicing_entity_id.serialize(buf);
        buf.put_u8(self.service_type_requested);
        buf.put_u8(self.number_of_supply_types);
        buf.put_i16(self.padding1);
        for i in 0..self.supplies.len() {
            self.supplies[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::ServiceRequest {
            let receiving_entity_id = EntityId::deserialize(&mut buffer);
            let servicing_entity_id = EntityId::deserialize(&mut buffer);
            let service_type_requested = buffer.get_u8();
            let number_of_supply_types = buffer.get_u8();
            let padding1 = buffer.get_i16();
            let mut supplies: Vec<SupplyQuantity> = vec![];
            for _i in 0..number_of_supply_types {
                supplies.push(SupplyQuantity::deserialize(&mut buffer));
            }

            Ok(ServiceRequestPdu {
                pdu_header,
                receiving_entity_id,
                servicing_entity_id,
                service_type_requested,
                number_of_supply_types,
                padding1,
                supplies,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type ServiceRequest, got {:?}",
                    pdu_header.pdu_type
                ),
                None,
            ))
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
        let receiving_entity_id = EntityId::deserialize(&mut buffer);
        let servicing_entity_id = EntityId::deserialize(&mut buffer);
        let service_type_requested = buffer.get_u8();
        let number_of_supply_types = buffer.get_u8();
        let padding1 = buffer.get_i16();
        let mut supplies: Vec<SupplyQuantity> = vec![];
        for _i in 0..number_of_supply_types {
            supplies.push(SupplyQuantity::deserialize(&mut buffer));
        }

        Ok(ServiceRequestPdu {
            pdu_header,
            receiving_entity_id,
            servicing_entity_id,
            service_type_requested,
            number_of_supply_types,
            padding1,
            supplies,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ServiceRequestPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let service_request_pdu = ServiceRequestPdu::default();
        let pdu_header =
            PduHeader::default(PduType::ServiceRequest, ProtocolFamily::Logistics, 448 / 8);

        assert_eq!(
            pdu_header.protocol_version,
            service_request_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            service_request_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, service_request_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            service_request_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, service_request_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, service_request_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut service_request_pdu = ServiceRequestPdu::default();
        let mut buffer = BytesMut::new();
        service_request_pdu.serialize(&mut buffer);

        let new_service_request_pdu = ServiceRequestPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_service_request_pdu.pdu_header,
            service_request_pdu.pdu_header
        );
    }
}
