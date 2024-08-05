use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Copy, Clone, Debug)]
pub struct ActionRequestReliablePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub required_reliability_service: u8,
    pub pad1: u16,
    pub pad2: u8,
    pub request_id: u32,
    pub action_id: u32,
    pub number_of_fixed_datum_records: u32,
    pub number_of_variable_datum_records: u32,
    pub fixed_datum_records: u64,
    pub variable_datum_records: u64,
}

impl Default for ActionRequestReliablePdu {
    fn default() -> Self {
        ActionRequestReliablePdu {
            pdu_header: PduHeader::default(
                PduType::ActionRequestReliable,
                ProtocolFamily::SimulationManagementWithReliability,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            required_reliability_service: 0,
            pad1: 0,
            pad2: 0,
            request_id: 0,
            action_id: 0,
            number_of_fixed_datum_records: 0,
            number_of_variable_datum_records: 0,
            fixed_datum_records: 0,
            variable_datum_records: 0,
        }
    }
}

impl Pdu for ActionRequestReliablePdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u8(self.required_reliability_service);
        buf.put_u16(self.pad1);
        buf.put_u8(self.pad2);
        buf.put_u32(self.request_id);
        buf.put_u32(self.action_id);
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
        if pdu_header.pdu_type == PduType::ActionRequestReliable {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let required_reliability_service = buffer.get_u8();
            let pad1 = buffer.get_u16();
            let pad2 = buffer.get_u8();
            let request_id = buffer.get_u32();
            let action_id = buffer.get_u32();
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

            Ok(ActionRequestReliablePdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                required_reliability_service,
                pad1,
                pad2,
                request_id,
                action_id,
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
        let required_reliability_service = buffer.get_u8();
        let pad1 = buffer.get_u16();
        let pad2 = buffer.get_u8();
        let request_id = buffer.get_u32();
        let action_id = buffer.get_u32();
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

        Ok(ActionRequestReliablePdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            required_reliability_service,
            pad1,
            pad2,
            request_id,
            action_id,
            number_of_fixed_datum_records,
            number_of_variable_datum_records,
            fixed_datum_records,
            variable_datum_records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ActionRequestReliablePdu;
    use crate::common::pdu_header::{PduHeader, PduType, ProtocolFamily};

    #[test]
    fn create_header() {
        let action_request_reliable_pdu = ActionRequestReliablePdu::default();
        let pdu_header = PduHeader::default(
            PduType::ActionRequestReliable,
            ProtocolFamily::SimulationManagementWithReliability,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            action_request_reliable_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            action_request_reliable_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            action_request_reliable_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            action_request_reliable_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            action_request_reliable_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.padding,
            action_request_reliable_pdu.pdu_header.padding
        );
    }
}
