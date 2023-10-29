use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Copy, Clone, Debug)]
pub struct RemoveEntityPdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
}

impl RemoveEntityPdu {
    pub fn default() -> Self {
        RemoveEntityPdu {
            pdu_header: PduHeader::default(
                PduType::RemoveEntity,
                ProtocolFamily::SimulationManagement,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            request_id: 0,
        }
    }
}

impl Pdu for RemoveEntityPdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u32(self.request_id as u32);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::RemoveEntity {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let request_id = buffer.get_u32();

            return Ok(RemoveEntityPdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                request_id,
            });
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

        return Ok(RemoveEntityPdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            request_id,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::RemoveEntityPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let remove_entity_pdu = RemoveEntityPdu::default();
        let pdu_header = PduHeader::default(
            PduType::RemoveEntity,
            ProtocolFamily::SimulationManagement,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            remove_entity_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            remove_entity_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, remove_entity_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            remove_entity_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, remove_entity_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, remove_entity_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let remove_entity_pdu = RemoveEntityPdu::default();
        let mut buffer = BytesMut::new();
        remove_entity_pdu.serialize(&mut buffer);

        let new_remove_entity_pdu = RemoveEntityPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_remove_entity_pdu.pdu_header,
            remove_entity_pdu.pdu_header
        );
    }
}
