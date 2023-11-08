use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    clock_time::ClockTime,
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Copy, Clone, Debug)]
pub struct StartResumeReliablePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub real_world_time: ClockTime,
    pub simulation_time: ClockTime,
    pub required_reliability_service: u8,
    pub pad1: u16,
    pub pad2: u8,
    pub request_id: u32,
}

impl StartResumeReliablePdu {
    pub fn default() -> Self {
        StartResumeReliablePdu {
            pdu_header: PduHeader::default(
                PduType::StartResumeReliable,
                ProtocolFamily::SimulationManagementWithReliability,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            real_world_time: ClockTime::default(),
            simulation_time: ClockTime::default(),
            required_reliability_service: 0,
            pad1: 0,
            pad2: 0,
            request_id: 0,
        }
    }
}

impl Pdu for StartResumeReliablePdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        self.real_world_time.serialize(buf);
        self.simulation_time.serialize(buf);
        buf.put_u8(self.required_reliability_service);
        buf.put_u16(self.pad1);
        buf.put_u8(self.pad2);
        buf.put_u32(self.request_id as u32);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::StartResumeReliable {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let real_world_time = ClockTime::decode(&mut buffer);
            let simulation_time = ClockTime::decode(&mut buffer);
            let required_reliability_service = buffer.get_u8();
            let pad1 = buffer.get_u16();
            let pad2 = buffer.get_u8();
            let request_id = buffer.get_u32();

            Ok(StartResumeReliablePdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                real_world_time,
                simulation_time,
                required_reliability_service,
                pad1,
                pad2,
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
        let real_world_time = ClockTime::decode(&mut buffer);
        let simulation_time = ClockTime::decode(&mut buffer);
        let required_reliability_service = buffer.get_u8();
        let pad1 = buffer.get_u16();
        let pad2 = buffer.get_u8();
        let request_id = buffer.get_u32();

        Ok(StartResumeReliablePdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            real_world_time,
            simulation_time,
            required_reliability_service,
            pad1,
            pad2,
            request_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::StartResumeReliablePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let start_resume_reliable_pdu = StartResumeReliablePdu::default();
        let pdu_header = PduHeader::default(
            PduType::StartResumeReliable,
            ProtocolFamily::SimulationManagementWithReliability,
            56,
        );

        assert_eq!(
            pdu_header.protocol_version,
            start_resume_reliable_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            start_resume_reliable_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            start_resume_reliable_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            start_resume_reliable_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            start_resume_reliable_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.padding,
            start_resume_reliable_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let start_resume_reliable_pdu = StartResumeReliablePdu::default();
        let mut buffer = BytesMut::new();
        start_resume_reliable_pdu.serialize(&mut buffer);

        let new_start_resume_reliable_pdu = StartResumeReliablePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_start_resume_reliable_pdu.pdu_header,
            start_resume_reliable_pdu.pdu_header
        );
    }
}
