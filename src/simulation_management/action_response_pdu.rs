//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    constants::MAX_PDU_SIZE_OCTETS,
    datum_records::{FixedDatumRecord, VariableDatumRecord},
    dis_error::DISError,
    entity_id::EntityId,
    enums::{ActionResponseRequestStatus, PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
};

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.5.8
pub struct ActionResponsePdu {
    pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub request_status: ActionResponseRequestStatus,
    pub number_of_fixed_datum_records: u32,
    pub number_of_variable_datum_records: u32,
    pub fixed_datum_records: Vec<FixedDatumRecord>,
    pub variable_datum_records: Vec<VariableDatumRecord>,
}

impl Default for ActionResponsePdu {
    fn default() -> Self {
        ActionResponsePdu {
            pdu_header: PduHeader::default(),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            request_id: 0,
            request_status: ActionResponseRequestStatus::default(),
            number_of_fixed_datum_records: 0,
            number_of_variable_datum_records: 0,
            fixed_datum_records: vec![],
            variable_datum_records: vec![],
        }
    }
}

impl Pdu for ActionResponsePdu {
    fn length(&self) -> u16 {
        std::mem::size_of::<PduHeader>() as u16
            + std::mem::size_of::<EntityId>() as u16 * 2
            + std::mem::size_of::<u32>() as u16
            + std::mem::size_of::<ActionResponseRequestStatus>() as u16
            + std::mem::size_of::<u32>() as u16 * 2
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
        buf.put_u32(self.request_status as u32);
        buf.put_u32(self.number_of_fixed_datum_records);
        buf.put_u32(self.number_of_variable_datum_records);
        for i in 0..self.fixed_datum_records.len() {
            self.fixed_datum_records[i].serialize(buf);
        }
        for i in 0..self.variable_datum_records.len() {
            self.variable_datum_records[i].serialize(buf);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::ActionResponse {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type ActionResponse, got {:?}",
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

impl ActionResponsePdu {
    /// Creates a new `ActionResponsePdu`
    ///
    /// # Examples
    ///
    /// Initializing an `ActionResponsePdu`:
    /// ```
    /// use open_dis_rust::simulation_management::ActionResponsePdu;
    /// let pdu = ActionResponsePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::ActionResponse;
        pdu.pdu_header.protocol_family = ProtocolFamily::SimulationManagement;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_entity_id = EntityId::deserialize(buf);
        let receiving_entity_id = EntityId::deserialize(buf);
        let request_id = buf.get_u32();
        let request_status = ActionResponseRequestStatus::deserialize(buf);
        let number_of_fixed_datum_records = buf.get_u32();
        let number_of_variable_datum_records = buf.get_u32();
        let mut fixed_datum_records: Vec<FixedDatumRecord> = vec![];
        fixed_datum_records.reserve(number_of_fixed_datum_records.try_into().unwrap());
        for _record in 0..number_of_fixed_datum_records as usize {
            fixed_datum_records.push(FixedDatumRecord::deserialize(buf));
        }
        let mut variable_datum_records: Vec<VariableDatumRecord> = vec![];
        variable_datum_records.reserve(number_of_variable_datum_records.try_into().unwrap());
        for _record in 0..number_of_variable_datum_records as usize {
            variable_datum_records.push(VariableDatumRecord::deserialize(buf));
        }

        ActionResponsePdu {
            pdu_header: PduHeader::default(),
            originating_entity_id,
            receiving_entity_id,
            request_id,
            request_status,
            number_of_fixed_datum_records,
            number_of_variable_datum_records,
            fixed_datum_records,
            variable_datum_records,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ActionResponsePdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let pdu = ActionResponsePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<ActionResponsePdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = ActionResponsePdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = ActionResponsePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / 8;
        let pdu = ActionResponsePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
