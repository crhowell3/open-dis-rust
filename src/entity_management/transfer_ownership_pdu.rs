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
    enums::{PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
    simulation_address::SimulationAddress,
};

use super::data_types::record_specification::RecordSpecification;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.8.4
pub struct TransferOwnershipPdu {
    pdu_header: PduHeader,
    pub originating_id: SimulationAddress,
    pub receiving_id: SimulationAddress,
    pub request_id: u32,
    pub required_reliability_service: u8,
    pub transfer_type: u8,
    pub transfer_entity_id: EntityId,
    pub record_information: RecordSpecification,
}

impl Default for TransferOwnershipPdu {
    fn default() -> Self {
        TransferOwnershipPdu {
            pdu_header: PduHeader::default(),
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
    fn length(&self) -> u16 {
        let length =
            PduHeader::LENGTH + SimulationAddress::LENGTH * 2 + 4 + 1 + 1 + EntityId::LENGTH;

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
        self.originating_id.serialize(buf);
        self.receiving_id.serialize(buf);
        buf.put_u32(self.request_id);
        buf.put_u8(self.required_reliability_service);
        buf.put_u8(self.transfer_type);
        self.transfer_entity_id.serialize(buf);
        self.record_information.serialize(buf);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::TransferOwnership {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type TransferOwnership, got {:?}",
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

impl TransferOwnershipPdu {
    /// Creates a new `TransferOwnershipPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `TransferOwnershipPdu`:
    /// ```
    /// use open_dis_rust::entity_management::TransferOwnershipPdu;
    /// let pdu = TransferOwnershipPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::SetData;
        pdu.pdu_header.protocol_family = ProtocolFamily::EntityManagement;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_id = SimulationAddress::deserialize(buf);
        let receiving_id = SimulationAddress::deserialize(buf);
        let request_id = buf.get_u32();
        let required_reliability_service = buf.get_u8();
        let transfer_type = buf.get_u8();
        let transfer_entity_id = EntityId::deserialize(buf);
        let record_information = RecordSpecification::deserialize(buf);

        TransferOwnershipPdu {
            pdu_header: PduHeader::default(),
            originating_id,
            receiving_id,
            request_id,
            required_reliability_service,
            transfer_type,
            transfer_entity_id,
            record_information,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TransferOwnershipPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let pdu = TransferOwnershipPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<TransferOwnershipPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = TransferOwnershipPdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = TransferOwnershipPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / 8;
        let pdu = TransferOwnershipPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
