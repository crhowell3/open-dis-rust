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

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.11.15
pub struct SetRecordReliablePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub required_reliability_service: u8,
    pub pad1: u16,
    pub pad2: u8,
    pub number_of_records: u32,
    pub record_ids: Vec<u32>,
}

impl Default for SetRecordReliablePdu {
    fn default() -> Self {
        SetRecordReliablePdu {
            pdu_header: PduHeader::default(
                PduType::SetRecordReliable,
                ProtocolFamily::SimulationManagementWithReliability,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            request_id: 0,
            required_reliability_service: 0,
            pad1: 0,
            pad2: 0,
            number_of_records: 0,
            record_ids: vec![],
        }
    }
}

impl Pdu for SetRecordReliablePdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u32(self.request_id);
        buf.put_u8(self.required_reliability_service);
        buf.put_u16(self.pad1);
        buf.put_u8(self.pad2);
        buf.put_u32(self.number_of_records);
        for i in &self.record_ids {
            buf.put_u32(*i);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::SetRecordReliable {
            let originating_entity_id = EntityId::deserialize(&mut buffer);
            let receiving_entity_id = EntityId::deserialize(&mut buffer);
            let request_id = buffer.get_u32();
            let required_reliability_service = buffer.get_u8();
            let pad1 = buffer.get_u16();
            let pad2 = buffer.get_u8();
            let number_of_records = buffer.get_u32();
            let mut record_ids: Vec<u32> = vec![];
            for _i in 0..number_of_records {
                record_ids.push(buffer.get_u32());
            }

            Ok(SetRecordReliablePdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                request_id,
                required_reliability_service,
                pad1,
                pad2,
                number_of_records,
                record_ids,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type SetRecordReliable, got {:?}",
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
        let required_reliability_service = buffer.get_u8();
        let pad1 = buffer.get_u16();
        let pad2 = buffer.get_u8();
        let number_of_records = buffer.get_u32();
        let mut record_ids: Vec<u32> = vec![];
        for _i in 0..number_of_records {
            record_ids.push(buffer.get_u32());
        }

        Ok(SetRecordReliablePdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            request_id,
            required_reliability_service,
            pad1,
            pad2,
            number_of_records,
            record_ids,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SetRecordReliablePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let set_record_reliable_pdu = SetRecordReliablePdu::default();
        let pdu_header = PduHeader::default(
            PduType::SetRecordReliable,
            ProtocolFamily::SimulationManagementWithReliability,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            set_record_reliable_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            set_record_reliable_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            set_record_reliable_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            set_record_reliable_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, set_record_reliable_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.padding,
            set_record_reliable_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let mut set_record_reliable_pdu = SetRecordReliablePdu::default();
        let mut buffer = BytesMut::new();
        set_record_reliable_pdu.serialize(&mut buffer);

        let new_set_record_reliable_pdu = SetRecordReliablePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_set_record_reliable_pdu.pdu_header,
            set_record_reliable_pdu.pdu_header
        );
    }
}
