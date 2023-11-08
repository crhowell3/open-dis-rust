use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Copy, Clone, Debug)]
pub struct CreateEntityReliablePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub required_reliability_service: u8,
    pub pad1: u16,
    pub pad2: u8,
    pub request_id: u32,
}

impl CreateEntityReliablePdu {
    pub fn default() -> Self {
        CreateEntityReliablePdu {
            pdu_header: PduHeader::default(
                PduType::CreateEntityReliable,
                ProtocolFamily::SimulationManagementWithReliability,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            required_reliability_service: 0,
            pad1: 0,
            pad2: 0,
            request_id: 0,
        }
    }
}

impl Pdu for CreateEntityReliablePdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u8(self.required_reliability_service as u8);
        buf.put_u16(self.pad1 as u16);
        buf.put_u8(self.pad2 as u8);
        buf.put_u32(self.request_id as u32);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::CreateEntityReliable {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let required_reliability_service = buffer.get_u8();
            let pad1 = buffer.get_u16();
            let pad2 = buffer.get_u8();
            let request_id = buffer.get_u32();

            return Ok(CreateEntityReliablePdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                required_reliability_service,
                pad1,
                pad2,
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
        let required_reliability_service = buffer.get_u8();
        let pad1 = buffer.get_u16();
        let pad2 = buffer.get_u8();
        let request_id = buffer.get_u32();

        return Ok(CreateEntityReliablePdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            required_reliability_service,
            pad1,
            pad2,
            request_id,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::CreateEntityReliablePdu;
    use crate::common::pdu_header::{PduHeader, PduType, ProtocolFamily};

    #[test]
    fn create_header() {
        let create_entity_reliable_pdu = CreateEntityReliablePdu::default();
        let pdu_header = PduHeader::default(
            PduType::CreateEntityReliable,
            ProtocolFamily::SimulationManagementWithReliability,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            create_entity_reliable_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            create_entity_reliable_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            create_entity_reliable_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            create_entity_reliable_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            create_entity_reliable_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.padding,
            create_entity_reliable_pdu.pdu_header.padding
        );
    }
}
