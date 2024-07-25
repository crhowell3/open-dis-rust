//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

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
/// Implemented according to IEEE 1278.1-2012 §5.6.5.6
pub struct AcknowledgePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub acknowledge_flag: AcknowledgeFlag,
    pub response_flag: AcknowledgeResponseFlag,
    pub request_id: u32,
}

impl Default for AcknowledgePdu {
    /// Creates a default Acknowledge PDU with arbitrary originating and receiving
    /// entity IDs
    ///
    /// # Examples
    ///
    /// Initializing an Acknowledge PDU:
    /// ```
    /// use open_dis_rust::simulation_management::acknowledge_pdu::AcknowledgePdu;
    /// let acknowledge_pdu = AcknowledgePdu::default();
    /// ```
    ///
    fn default() -> Self {
        AcknowledgePdu {
            pdu_header: PduHeader::default(
                PduType::Acknowledge,
                ProtocolFamily::SimulationManagement,
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

impl Pdu for AcknowledgePdu {
    fn serialize(&self, buf: &mut BytesMut) {
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
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::Acknowledge {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let acknowledge_flag = AcknowledgeFlag::decode(&mut buffer);
            let response_flag = AcknowledgeResponseFlag::decode(&mut buffer);
            let request_id = buffer.get_u32();

            Ok(AcknowledgePdu {
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
        let acknowledge_flag = AcknowledgeFlag::decode(&mut buffer);
        let response_flag = AcknowledgeResponseFlag::decode(&mut buffer);
        let request_id = buffer.get_u32();

        Ok(AcknowledgePdu {
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
    use super::AcknowledgePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let acknowledge_pdu = AcknowledgePdu::default();
        let pdu_header = PduHeader::default(
            PduType::Acknowledge,
            ProtocolFamily::SimulationManagement,
            32,
        );

        assert_eq!(
            pdu_header.protocol_version,
            acknowledge_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            acknowledge_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, acknowledge_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            acknowledge_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, acknowledge_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, acknowledge_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let acknowledge_pdu = AcknowledgePdu::default();
        let mut buffer = BytesMut::new();
        acknowledge_pdu.serialize(&mut buffer);

        let new_acknowledge_pdu = AcknowledgePdu::deserialize(buffer).unwrap();
        assert_eq!(new_acknowledge_pdu.pdu_header, acknowledge_pdu.pdu_header);
    }
}
