
use bytes::{BytesMut, BufMut, Buf};

use crate::dis6::common::{pdu_header::{PduHeader, PduType, ProtocolFamily}, entity_id::EntityId, pdu::Pdu, dis_error::DISError};

#[derive(Copy, Clone, Debug)]
pub struct AcknowledgePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub acknowledge_flag: AcknowledgeFlag,
    pub response_flag: ResponseFlag,
    pub request_id: u32,
}

impl AcknowledgePdu {
    pub fn default() -> Self {
        AcknowledgePdu {
            pdu_header: PduHeader::default(PduType::Acknowledge, ProtocolFamily::SimulationManagement, 56),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            acknowledge_flag: ,
            response_flag: ,
            request_id: 0,
        }
    }
}

impl Pdu for AcknowledgePdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u8(self.acknowledge_flag as u8);
        buf.put_u8(self.response_flag as u8);
        buf.put_u32(self.request_id as u32);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, crate::dis6::common::dis_error::DISError> where Self: Sized {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::Acknowledge {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let acknowledge_flag = buffer.get_u8();
            let response_flag = buffer.get_u8();
            let request_id = buffer.get_u32();

            return Ok(AcknowledgePdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                acknowledge_flag,
                response_flag,
                request_id,
            })
        } else {
            Err(DISError::InvalidDISHeader)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AcknowledgeFlag {
    
}

impl AcknowledgeFlag {
    pub fn from_u8(bit: u8) -> AcknowledgeFlag {
        
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ResponseFlag {
    
}

impl ResponseFlag {
    pub fn from_u8(bit: u8) -> ResponseFlag {
        
    }
}
