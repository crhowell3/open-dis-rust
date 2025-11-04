//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Copy, Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.5.2
pub struct CreateEntityPdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
}

impl Default for CreateEntityPdu {
    fn default() -> Self {
        CreateEntityPdu {
            pdu_header: PduHeader::default(
                PduType::CreateEntity,
                ProtocolFamily::SimulationManagement,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            request_id: 0,
        }
    }
}

impl Pdu for CreateEntityPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u32(self.request_id);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::CreateEntity {
            let originating_entity_id = EntityId::deserialize(&mut buffer);
            let receiving_entity_id = EntityId::deserialize(&mut buffer);
            let request_id = buffer.get_u32();

            Ok(CreateEntityPdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                request_id,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type CreateEntity, got {:?}",
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

        Ok(CreateEntityPdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            request_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::CreateEntityPdu;
    use crate::common::pdu_header::{PduHeader, PduType, ProtocolFamily};

    #[test]
    fn create_header() {
        let action_request_pdu = CreateEntityPdu::default();
        let pdu_header = PduHeader::default(
            PduType::CreateEntity,
            ProtocolFamily::SimulationManagement,
            448 / 8,
        );

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
