use bytes::{Buf, BufMut, BytesMut};

use crate::dis6::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

use super::acknowledge_pdu::{AcknowledgeFlag, ResponseFlag};

#[derive(Copy, Clone, Debug)]
pub struct AcknowledgeReliablePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub acknowledge_flag: AcknowledgeFlag,
    pub response_flag: ResponseFlag,
    pub request_id: u32,
}

impl AcknowledgeReliablePdu {
    pub fn default() -> Self {
        AcknowledgeReliablePdu {
            pdu_header: PduHeader::default(
                PduType::AcknowledgeReliable,
                ProtocolFamily::SimulationManagement,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            acknowledge_flag: AcknowledgeFlag::CreateEntity,
            response_flag: ResponseFlag::Other,
            request_id: 0,
        }
    }
}

impl Pdu for AcknowledgeReliablePdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u8(self.acknowledge_flag as u8);
        buf.put_u8(self.response_flag as u8);
        buf.put_u32(self.request_id as u32);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, crate::dis6::common::dis_error::DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::AcknowledgeReliable {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let acknowledge_flag = buffer.get_u8();
            let response_flag = buffer.get_u8();
            let request_id = buffer.get_u32();

            return Ok(AcknowledgeReliablePdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                acknowledge_flag,
                response_flag,
                request_id,
            });
        } else {
            Err(DISError::InvalidDISHeader)
        }
    }
}
