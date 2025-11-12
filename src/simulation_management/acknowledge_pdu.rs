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
/// Implemented according to IEEE 1278.1-2012 §7.5.6
pub struct AcknowledgePdu {
    pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub acknowledge_flag: AcknowledgeFlag,
    pub response_flag: AcknowledgeResponseFlag,
    pub request_id: u32,
}

impl Default for AcknowledgePdu {
    fn default() -> Self {
        AcknowledgePdu {
            pdu_header: PduHeader::default(),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            acknowledge_flag: AcknowledgeFlag::default(),
            response_flag: AcknowledgeResponseFlag::default(),
            request_id: 0,
        }
    }
}

impl Pdu for AcknowledgePdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>() * 2
            + std::mem::size_of::<AcknowledgeFlag>()
            + std::mem::size_of::<AcknowledgeResponseFlag>()
            + std::mem::size_of::<u32>();

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

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
        if pdu_header.pdu_type == PduType::Acknowledge {
            let originating_entity_id = EntityId::deserialize(&mut buffer);
            let receiving_entity_id = EntityId::deserialize(&mut buffer);
            let acknowledge_flag = AcknowledgeFlag::deserialize(&mut buffer);
            let response_flag = AcknowledgeResponseFlag::deserialize(&mut buffer);
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
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type Acknowledge, got {:?}",
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

impl AcknowledgePdu {
    /// Creates an Acknowledge PDU
    ///
    /// # Examples
    ///
    /// Initializing an Acknowledge PDU:
    /// ```
    /// use open_dis_rust::simulation_management::AcknowledgePdu;
    /// let mut acknowledge_pdu = AcknowledgePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::Acknowledge;
        pdu.pdu_header.protocol_family = ProtocolFamily::SimulationManagement;
        pdu.finalize();
        pdu
    }
}

#[cfg(test)]
mod tests {
    use std::any::type_name_of_val;

    use super::AcknowledgePdu;
    use crate::common::{PduType, pdu::Pdu, pdu_header::PduHeader};
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let acknowledge_pdu = AcknowledgePdu::new();
        let pdu_header = PduHeader::default();

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
        assert_eq!(
            pdu_header.status_record,
            acknowledge_pdu.pdu_header.status_record
        );
    }

    #[test]
    fn cast_to_any() {
        let acknowledge_pdu = AcknowledgePdu::default();
        let any_pdu = acknowledge_pdu.as_any();

        assert!(any_pdu.is::<AcknowledgePdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut acknowledge_pdu = AcknowledgePdu::default();
        let mut buffer = BytesMut::new();
        acknowledge_pdu.serialize(&mut buffer);

        let new_acknowledge_pdu = AcknowledgePdu::deserialize(buffer).unwrap();
        assert_eq!(new_acknowledge_pdu.pdu_header, acknowledge_pdu.pdu_header);
    }

    #[test]
    fn create_new_pdu() {
        let comment_pdu = AcknowledgePdu::new();
        assert!(type_name_of_val(&comment_pdu).contains("AcknowledgePdu"));
        assert_eq!(comment_pdu.header().pdu_type, PduType::Acknowledge);
    }
}
