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
    enums::{AcknowledgeFlag, AcknowledgeResponseFlag},
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Copy, Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.11.6
pub struct AcknowledgeReliablePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub acknowledge_flag: AcknowledgeFlag,
    pub response_flag: AcknowledgeResponseFlag,
    pub request_id: u32,
}

impl Default for AcknowledgeReliablePdu {
    /// Creates a default Acknowledge Reliable PDU with arbitrary originating and receiving
    /// entity IDs
    ///
    /// # Examples
    ///
    /// Initializing an Acknowledge Reliable PDU:
    /// ```
    /// use open_dis_rust::simulation_management_with_reliability::acknowledge_reliable_pdu::AcknowledgeReliablePdu;
    /// let acknowledge_reliable_pdu = AcknowledgeReliablePdu::default();
    /// ```
    ///
    fn default() -> Self {
        AcknowledgeReliablePdu {
            pdu_header: PduHeader::default(
                PduType::AcknowledgeReliable,
                ProtocolFamily::SimulationManagementWithReliability,
                32,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            acknowledge_flag: AcknowledgeFlag::default(),
            response_flag: AcknowledgeResponseFlag::default(),
            request_id: 0,
        }
    }
}

impl Pdu for AcknowledgeReliablePdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u16(self.acknowledge_flag as u16);
        buf.put_u16(self.response_flag as u16);
        buf.put_u32(self.request_id);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::AcknowledgeReliable {
            let originating_entity_id = EntityId::deserialize(&mut buffer);
            let receiving_entity_id = EntityId::deserialize(&mut buffer);
            let acknowledge_flag = AcknowledgeFlag::deserialize(&mut buffer);
            let response_flag = AcknowledgeResponseFlag::deserialize(&mut buffer);
            let request_id = buffer.get_u32();

            Ok(AcknowledgeReliablePdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                acknowledge_flag,
                response_flag,
                request_id,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type AcknowledgeReliable, got {:?}",
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
        let acknowledge_flag = AcknowledgeFlag::deserialize(&mut buffer);
        let response_flag = AcknowledgeResponseFlag::deserialize(&mut buffer);
        let request_id = buffer.get_u32();

        Ok(AcknowledgeReliablePdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            acknowledge_flag,
            response_flag,
            request_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::AcknowledgeReliablePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let acknowledge_reliable_pdu = AcknowledgeReliablePdu::default();
        let pdu_header = PduHeader::default(
            PduType::AcknowledgeReliable,
            ProtocolFamily::SimulationManagementWithReliability,
            32,
        );

        assert_eq!(
            pdu_header.protocol_version,
            acknowledge_reliable_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            acknowledge_reliable_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            acknowledge_reliable_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            acknowledge_reliable_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            acknowledge_reliable_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.status_record,
            acknowledge_reliable_pdu.pdu_header.status_record
        );
    }

    #[test]
    fn deserialize_header() {
        let mut acknowledge_reliable_pdu = AcknowledgeReliablePdu::default();
        let mut buffer = BytesMut::new();
        acknowledge_reliable_pdu.serialize(&mut buffer);

        let new_acknowledge_reliable_pdu = AcknowledgeReliablePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_acknowledge_reliable_pdu.pdu_header,
            acknowledge_reliable_pdu.pdu_header
        );
    }
}
