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
pub struct StopFreezeReliablePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub real_world_time: ClockTime,
    pub reason: Reason,
    pub frozen_behavior: FrozenBehavior,
    pub required_reliability_service: u8,
    pub pad1: u8,
    pub request_id: u32,
}

impl Default for StopFreezeReliablePdu {
    fn default() -> Self {
        StopFreezeReliablePdu {
            pdu_header: PduHeader::default(
                PduType::StopFreezeReliable,
                ProtocolFamily::SimulationManagementWithReliability,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            real_world_time: ClockTime::default(),
            reason: Reason::Other,
            frozen_behavior: FrozenBehavior::Frozen,
            required_reliability_service: 0,
            pad1: 0,
            request_id: 0,
        }
    }
}

impl Pdu for StopFreezeReliablePdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        self.real_world_time.serialize(buf);
        buf.put_u8(self.reason as u8);
        buf.put_u8(self.frozen_behavior as u8);
        buf.put_u8(self.required_reliability_service);
        buf.put_u8(self.pad1);
        buf.put_u32(self.request_id);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::StopFreezeReliable {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let real_world_time = ClockTime::decode(&mut buffer);
            let reason = Reason::from_u8(buffer.get_u8());
            let frozen_behavior = FrozenBehavior::from_u8(buffer.get_u8());
            let required_reliability_service = buffer.get_u8();
            let pad1 = buffer.get_u8();
            let request_id = buffer.get_u32();

            Ok(StopFreezeReliablePdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                real_world_time,
                reason,
                frozen_behavior,
                required_reliability_service,
                pad1,
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
        let reason = Reason::from_u8(buffer.get_u8());
        let frozen_behavior = FrozenBehavior::from_u8(buffer.get_u8());
        let required_reliability_service = buffer.get_u8();
        let pad1 = buffer.get_u8();
        let request_id = buffer.get_u32();

        Ok(StopFreezeReliablePdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            real_world_time,
            reason,
            frozen_behavior,
            required_reliability_service,
            pad1,
            request_id,
        })
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub enum Reason {
    #[default]
    Other = 0,
    Recess = 1,
    Termination = 2,
    SystemFailure = 3,
    SecurityViolation = 4,
    EntityReconstitution = 5,
    StopForReset = 6,
    StopForRestart = 7,
    AbortTrainingReturnToTacticalOperations = 8,
}

impl Reason {
    #[must_use]
    pub fn from_u8(bit: u8) -> Reason {
        match bit {
            1 => Reason::Recess,
            2 => Reason::Termination,
            3 => Reason::SystemFailure,
            4 => Reason::SecurityViolation,
            5 => Reason::EntityReconstitution,
            6 => Reason::StopForReset,
            7 => Reason::StopForRestart,
            8 => Reason::AbortTrainingReturnToTacticalOperations,
            _ => Reason::Other,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub enum FrozenBehavior {
    #[default]
    Frozen = 0,
    RunSimClock = 1,
    TransmitUpdates = 2,
    TransmitAndRun = 3,
    ProcessUpdates = 4,
    ProcessAndRun = 5,
    ProcessAndTransmit = 6,
    ProcessTransmitAndRun = 7,
}

impl FrozenBehavior {
    #[must_use]
    pub fn from_u8(bit: u8) -> FrozenBehavior {
        match bit {
            1 => FrozenBehavior::RunSimClock,
            2 => FrozenBehavior::TransmitUpdates,
            3 => FrozenBehavior::TransmitAndRun,
            4 => FrozenBehavior::ProcessUpdates,
            5 => FrozenBehavior::ProcessAndRun,
            6 => FrozenBehavior::ProcessAndTransmit,
            7 => FrozenBehavior::ProcessTransmitAndRun,
            _ => FrozenBehavior::Frozen,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StopFreezeReliablePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let stop_freeze_reliable_pdu = StopFreezeReliablePdu::default();
        let pdu_header = PduHeader::default(
            PduType::StopFreezeReliable,
            ProtocolFamily::SimulationManagementWithReliability,
            56,
        );

        assert_eq!(
            pdu_header.protocol_version,
            stop_freeze_reliable_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            stop_freeze_reliable_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            stop_freeze_reliable_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            stop_freeze_reliable_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            stop_freeze_reliable_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.padding,
            stop_freeze_reliable_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let stop_freeze_reliable_pdu = StopFreezeReliablePdu::default();
        let mut buffer = BytesMut::new();
        stop_freeze_reliable_pdu.serialize(&mut buffer);

        let new_stop_freeze_reliable_pdu = StopFreezeReliablePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_stop_freeze_reliable_pdu.pdu_header,
            stop_freeze_reliable_pdu.pdu_header
        );
    }
}
