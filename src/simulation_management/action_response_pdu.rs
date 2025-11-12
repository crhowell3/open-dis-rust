//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    datum_records::{FixedDatumRecord, VariableDatumRecord},
    dis_error::DISError,
    entity_id::EntityId,
    enums::ActionResponseRequestStatus,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
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
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
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
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::ActionResponse {
            let originating_entity_id = EntityId::deserialize(&mut buffer);
            let receiving_entity_id = EntityId::deserialize(&mut buffer);
            let request_id = buffer.get_u32();
            let request_status = ActionResponseRequestStatus::deserialize(&mut buffer);
            let number_of_fixed_datum_records = buffer.get_u32();
            let number_of_variable_datum_records = buffer.get_u32();
            let mut fixed_datum_records: Vec<FixedDatumRecord> = vec![];
            fixed_datum_records.reserve(number_of_fixed_datum_records.try_into().unwrap());
            for _record in 0..number_of_fixed_datum_records as usize {
                fixed_datum_records.push(FixedDatumRecord::deserialize(&mut buffer));
            }
            let mut variable_datum_records: Vec<VariableDatumRecord> = vec![];
            variable_datum_records.reserve(number_of_variable_datum_records.try_into().unwrap());
            for _record in 0..number_of_variable_datum_records as usize {
                variable_datum_records.push(VariableDatumRecord::deserialize(&mut buffer));
            }

            Ok(ActionResponsePdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                request_id,
                request_status,
                number_of_fixed_datum_records,
                number_of_variable_datum_records,
                fixed_datum_records,
                variable_datum_records,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type ActionResponse, got {:?}",
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
        let originating_entity_id = EntityId::deserialize(&mut buffer);
        let receiving_entity_id = EntityId::deserialize(&mut buffer);
        let request_id = buffer.get_u32();
        let request_status = ActionResponseRequestStatus::deserialize(&mut buffer);
        let number_of_fixed_datum_records = buffer.get_u32();
        let number_of_variable_datum_records = buffer.get_u32();
        let mut fixed_datum_records: Vec<FixedDatumRecord> = vec![];
        fixed_datum_records.reserve(number_of_fixed_datum_records.try_into().unwrap());
        for _record in 0..number_of_fixed_datum_records as usize {
            fixed_datum_records.push(FixedDatumRecord::deserialize(&mut buffer));
        }
        let mut variable_datum_records: Vec<VariableDatumRecord> = vec![];
        variable_datum_records.reserve(number_of_variable_datum_records.try_into().unwrap());
        for _record in 0..number_of_variable_datum_records as usize {
            variable_datum_records.push(VariableDatumRecord::deserialize(&mut buffer));
        }

        Ok(ActionResponsePdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            request_id,
            request_status,
            number_of_fixed_datum_records,
            number_of_variable_datum_records,
            fixed_datum_records,
            variable_datum_records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ActionResponsePdu;
    use crate::common::pdu_header::PduHeader;

    #[test]
    fn create_header() {
        let action_request_pdu = ActionResponsePdu::default();
        let pdu_header = PduHeader::default();

        assert_eq!(
            pdu_header.protocol_version,
            action_request_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            action_request_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, action_request_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            action_request_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, action_request_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.status_record,
            action_request_pdu.pdu_header.status_record
        );
    }
}
