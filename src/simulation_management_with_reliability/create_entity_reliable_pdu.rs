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
/// Implemented according to IEEE 1278.1-2012 §7.11.2
pub struct CreateEntityReliablePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub required_reliability_service: u8,
    pub pad1: u16,
    pub pad2: u8,
    pub request_id: u32,
}

impl Default for CreateEntityReliablePdu {
    fn default() -> Self {
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
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::CreateEntityReliable {
            let originating_entity_id = EntityId::deserialize(&mut buffer);
            let receiving_entity_id = EntityId::deserialize(&mut buffer);
            let required_reliability_service = buffer.get_u8();
            let pad1 = buffer.get_u16();
            let pad2 = buffer.get_u8();
            let request_id = buffer.get_u32();

            Ok(CreateEntityReliablePdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                required_reliability_service,
                pad1,
                pad2,
                request_id,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type CreateEntityReliable, got {:?}",
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
        let required_reliability_service = buffer.get_u8();
        let pad1 = buffer.get_u16();
        let pad2 = buffer.get_u8();
        let request_id = buffer.get_u32();

        Ok(CreateEntityReliablePdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            required_reliability_service,
            pad1,
            pad2,
            request_id,
        })
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
            pdu_header.status_record,
            create_entity_reliable_pdu.pdu_header.status_record
        );
    }
}
