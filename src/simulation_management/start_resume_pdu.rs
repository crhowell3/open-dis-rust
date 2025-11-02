//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

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
/// Implemented according to IEEE 1278.1-2012 §7.5.4
pub struct StartResumePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub real_world_time: ClockTime,
    pub simulation_time: ClockTime,
    pub request_id: u32,
}

impl Default for StartResumePdu {
    fn default() -> Self {
        StartResumePdu {
            pdu_header: PduHeader::default(
                PduType::StartResume,
                ProtocolFamily::SimulationManagement,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            real_world_time: ClockTime::default(),
            simulation_time: ClockTime::default(),
            request_id: 0,
        }
    }
}

impl Pdu for StartResumePdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        self.real_world_time.serialize(buf);
        self.simulation_time.serialize(buf);
        buf.put_u32(self.request_id);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::StartResume {
            let originating_entity_id = EntityId::deserialize(&mut buffer);
            let receiving_entity_id = EntityId::deserialize(&mut buffer);
            let real_world_time = ClockTime::deserialize(&mut buffer);
            let simulation_time = ClockTime::deserialize(&mut buffer);
            let request_id = buffer.get_u32();

            Ok(StartResumePdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                real_world_time,
                simulation_time,
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
        let originating_entity_id = EntityId::deserialize(&mut buffer);
        let receiving_entity_id = EntityId::deserialize(&mut buffer);
        let real_world_time = ClockTime::deserialize(&mut buffer);
        let simulation_time = ClockTime::deserialize(&mut buffer);
        let request_id = buffer.get_u32();

        Ok(StartResumePdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            real_world_time,
            simulation_time,
            request_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::StartResumePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let start_resume_pdu = StartResumePdu::default();
        let pdu_header = PduHeader::default(
            PduType::StartResume,
            ProtocolFamily::SimulationManagement,
            56,
        );

        assert_eq!(
            pdu_header.protocol_version,
            start_resume_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            start_resume_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, start_resume_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            start_resume_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, start_resume_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, start_resume_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut start_resume_pdu = StartResumePdu::default();
        let mut buffer = BytesMut::new();
        start_resume_pdu.serialize(&mut buffer);

        let new_start_resume_pdu = StartResumePdu::deserialize(buffer).unwrap();
        assert_eq!(new_start_resume_pdu.pdu_header, start_resume_pdu.pdu_header);
    }
}
