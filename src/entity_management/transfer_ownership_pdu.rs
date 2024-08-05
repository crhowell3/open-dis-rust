//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    simulation_address::SimulationAddress,
};

use super::record_specification::RecordSpecification;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §5.9.2.2
pub struct TransferOwnershipPdu {
    pub pdu_header: PduHeader,
    pub originating_id: SimulationAddress,
    pub receiving_id: SimulationAddress,
    pub request_id: u32,
    pub required_reliability_service: u8,
    pub transfer_type: u8,
    pub transfer_entity_id: EntityId,
    pub record_information: RecordSpecification,
}

impl Default for TransferOwnershipPdu {
    /// Creates a default Transfer Ownership PDU with arbitrary originating and receiving IDs
    ///
    /// # Examples
    ///
    /// Initializing a Transfer Ownership PDU:
    /// ```
    /// use open_dis_rust::entity_management::transfer_ownership_pdu::TransferOwnershipPdu;
    /// let transfer_ownership_pdu = TransferOwnershipPdu::default();
    /// ```
    ///
    fn default() -> Self {
        TransferOwnershipPdu {
            pdu_header: PduHeader::default(
                PduType::TransferOwnership,
                ProtocolFamily::EntityManagement,
                56,
            ),
            originating_id: SimulationAddress::default(),
            receiving_id: SimulationAddress::default(),
            request_id: 0,
            required_reliability_service: 0,
            transfer_type: 0,
            transfer_entity_id: EntityId::default(1),
            record_information: RecordSpecification::default(),
        }
    }
}

impl Pdu for TransferOwnershipPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_id.serialize(buf);
        self.receiving_id.serialize(buf);
        buf.put_u32(self.request_id);
        buf.put_u8(self.required_reliability_service);
        buf.put_u8(self.transfer_type);
        self.transfer_entity_id.serialize(buf);
        self.record_information.serialize(buf);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::TransferOwnership {
            let originating_id = SimulationAddress::decode(&mut buffer);
            let receiving_id = SimulationAddress::decode(&mut buffer);
            let request_id = buffer.get_u32();
            let required_reliability_service = buffer.get_u8();
            let transfer_type = buffer.get_u8();
            let transfer_entity_id = EntityId::decode(&mut buffer);
            let record_information = RecordSpecification::decode(&mut buffer);
            Ok(TransferOwnershipPdu {
                pdu_header,
                originating_id,
                receiving_id,
                request_id,
                required_reliability_service,
                transfer_type,
                transfer_entity_id,
                record_information,
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
        let originating_id = SimulationAddress::decode(&mut buffer);
        let receiving_id = SimulationAddress::decode(&mut buffer);
        let request_id = buffer.get_u32();
        let required_reliability_service = buffer.get_u8();
        let transfer_type = buffer.get_u8();
        let transfer_entity_id = EntityId::decode(&mut buffer);
        let record_information = RecordSpecification::decode(&mut buffer);
        Ok(TransferOwnershipPdu {
            pdu_header,
            originating_id,
            receiving_id,
            request_id,
            required_reliability_service,
            transfer_type,
            transfer_entity_id,
            record_information,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::TransferOwnershipPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let transfer_ownership_pdu = TransferOwnershipPdu::default();
        let pdu_header = PduHeader::default(
            PduType::TransferOwnership,
            ProtocolFamily::EntityManagement,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            transfer_ownership_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            transfer_ownership_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            transfer_ownership_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            transfer_ownership_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, transfer_ownership_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.padding,
            transfer_ownership_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let transfer_ownership_pdu = TransferOwnershipPdu::default();
        let mut buffer = BytesMut::new();
        transfer_ownership_pdu.serialize(&mut buffer);

        let new_transfer_ownership_pdu = TransferOwnershipPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_transfer_ownership_pdu.pdu_header,
            transfer_ownership_pdu.pdu_header
        );
    }
}
