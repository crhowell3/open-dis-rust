//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Copy, Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §5.12.4.6
pub struct AcknowledgeReliablePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub acknowledge_flag: AcknowledgeFlag,
    pub response_flag: ResponseFlag,
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
        buf.put_u32(self.request_id);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::AcknowledgeReliable {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let acknowledge_flag = AcknowledgeFlag::from_u8(buffer.get_u8());
            let response_flag = ResponseFlag::from_u8(buffer.get_u8());
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
        let acknowledge_flag = AcknowledgeFlag::from_u8(buffer.get_u8());
        let response_flag = ResponseFlag::from_u8(buffer.get_u8());
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

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to SISO-REF-010-2020 UID 69
pub enum AcknowledgeFlag {
    #[default]
    CreateEntity = 1,
    RemoveEntity = 2,
    StartResume = 3,
    StopFreeze = 4,
    TransferOwnership = 5,
}

impl AcknowledgeFlag {
    #[must_use]
    pub fn from_u8(bit: u8) -> AcknowledgeFlag {
        match bit {
            2 => AcknowledgeFlag::RemoveEntity,
            3 => AcknowledgeFlag::StartResume,
            4 => AcknowledgeFlag::StopFreeze,
            5 => AcknowledgeFlag::TransferOwnership,
            _ => AcknowledgeFlag::CreateEntity,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to SISO-REF-010-2020 UID 70
pub enum ResponseFlag {
    #[default]
    Other = 0,
    AbleToComply = 1,
    UnableToComply = 2,
    PendingOperatorAction = 3,
}

impl ResponseFlag {
    #[must_use]
    pub fn from_u8(byte: u8) -> ResponseFlag {
        match byte {
            1 => ResponseFlag::AbleToComply,
            2 => ResponseFlag::UnableToComply,
            3 => ResponseFlag::PendingOperatorAction,
            _ => ResponseFlag::Other,
        }
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
            448 / 8,
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
            pdu_header.padding,
            acknowledge_reliable_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let acknowledge_reliable_pdu = AcknowledgeReliablePdu::default();
        let mut buffer = BytesMut::new();
        acknowledge_reliable_pdu.serialize(&mut buffer);

        let new_acknowledge_reliable_pdu = AcknowledgeReliablePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_acknowledge_reliable_pdu.pdu_header,
            acknowledge_reliable_pdu.pdu_header
        );
    }
}
