use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};
use bytes::{Buf, BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Copy, Clone, Debug)]
pub struct ActionResponseReliablePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub request_status: RequestStatus,
    pub number_of_fixed_datum_records: u32,
    pub number_of_variable_datum_records: u32,
    pub fixed_datum_records: u64,
    pub variable_datum_records: u64,
}

impl Default for ActionResponseReliablePdu {
    fn default() -> Self {
        ActionResponseReliablePdu {
            pdu_header: PduHeader::default(
                PduType::ActionResponseReliable,
                ProtocolFamily::SimulationManagementWithReliability,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            request_id: 0,
            request_status: RequestStatus::Other,
            number_of_fixed_datum_records: 0,
            number_of_variable_datum_records: 0,
            fixed_datum_records: 0,
            variable_datum_records: 0,
        }
    }
}

impl Pdu for ActionResponseReliablePdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u32(self.request_id);
        buf.put_u32(self.request_status as u32);
        buf.put_u32(self.number_of_fixed_datum_records);
        buf.put_u32(self.number_of_variable_datum_records);
        buf.put_u64(self.fixed_datum_records);
        buf.put_u64(self.variable_datum_records);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::ActionResponseReliable {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let request_id = buffer.get_u32();
            let request_status = RequestStatus::decode(&mut buffer);
            let number_of_fixed_datum_records = buffer.get_u32();
            let number_of_variable_datum_records = buffer.get_u32();
            let mut fixed_datum_records: u64 = 0;
            for _record in 0..number_of_fixed_datum_records as usize {
                fixed_datum_records += buffer.get_u64();
            }
            let mut variable_datum_records: u64 = 0;
            for _record in 0..number_of_variable_datum_records as usize {
                variable_datum_records += buffer.get_u64();
            }

            Ok(ActionResponseReliablePdu {
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
        let originating_entity_id = EntityId::decode(&mut buffer);
        let receiving_entity_id = EntityId::decode(&mut buffer);
        let request_id = buffer.get_u32();
        let request_status = RequestStatus::decode(&mut buffer);
        let number_of_fixed_datum_records = buffer.get_u32();
        let number_of_variable_datum_records = buffer.get_u32();
        let mut fixed_datum_records: u64 = 0;
        for _record in 0..number_of_fixed_datum_records as usize {
            fixed_datum_records += buffer.get_u64();
        }
        let mut variable_datum_records: u64 = 0;
        for _record in 0..number_of_variable_datum_records as usize {
            variable_datum_records += buffer.get_u64();
        }

        Ok(ActionResponseReliablePdu {
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

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RequestStatus {
    Other = 0,
    Pending = 1,
    Executing = 2,
    PartiallyComplete = 3,
    Complete = 4,
    RequestRejected = 5,
    RetransmitRequestNow = 6,
    RetransmitRequestLater = 7,
    InvalidTimeParameters = 8,
    SimulationTimeExceeded = 9,
    RequestDone = 10,
    TACCSFLOSReplyTypeOne = 100,
    TACCSFLOSReplyTypeTwo = 101,
    JoinExerciseRequestRejected = 201,
}

impl RequestStatus {
    pub fn decode(buf: &mut BytesMut) -> RequestStatus {
        match buf.get_u32() {
            0 => RequestStatus::Other,
            1 => RequestStatus::Pending,
            2 => RequestStatus::Executing,
            3 => RequestStatus::PartiallyComplete,
            4 => RequestStatus::Complete,
            5 => RequestStatus::RequestRejected,
            6 => RequestStatus::RetransmitRequestNow,
            7 => RequestStatus::RetransmitRequestLater,
            8 => RequestStatus::InvalidTimeParameters,
            9 => RequestStatus::SimulationTimeExceeded,
            10 => RequestStatus::RequestDone,
            100 => RequestStatus::TACCSFLOSReplyTypeOne,
            101 => RequestStatus::TACCSFLOSReplyTypeTwo,
            201 => RequestStatus::JoinExerciseRequestRejected,
            _ => RequestStatus::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ActionResponseReliablePdu;
    use crate::common::pdu_header::{PduHeader, PduType, ProtocolFamily};

    #[test]
    fn create_header() {
        let action_response_reliable_pdu = ActionResponseReliablePdu::default();
        let pdu_header = PduHeader::default(
            PduType::ActionResponseReliable,
            ProtocolFamily::SimulationManagementWithReliability,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            action_response_reliable_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            action_response_reliable_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            action_response_reliable_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            action_response_reliable_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            action_response_reliable_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.padding,
            action_response_reliable_pdu.pdu_header.padding
        );
    }
}
