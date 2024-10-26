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
    enums::{FrozenBehavior, Reason},
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Copy, Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.11.5
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
    #[must_use]
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
            reason: Reason::default(),
            frozen_behavior: FrozenBehavior::default(),
            required_reliability_service: 0,
            pad1: 0,
            request_id: 0,
        }
    }
}

impl Pdu for StopFreezeReliablePdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        self.real_world_time.serialize(buf);
        buf.put_u8(self.reason as u8);
        buf.put_u8(self.frozen_behavior.as_u8());
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
            let reason = Reason::decode(&mut buffer);
            let frozen_behavior = FrozenBehavior::from_u8(buffer.get_u8()).unwrap();
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
        let reason = Reason::decode(&mut buffer);
        let frozen_behavior = FrozenBehavior::from_u8(buffer.get_u8()).unwrap();
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
        let mut stop_freeze_reliable_pdu = StopFreezeReliablePdu::default();
        let mut buffer = BytesMut::new();
        stop_freeze_reliable_pdu.serialize(&mut buffer);

        let new_stop_freeze_reliable_pdu = StopFreezeReliablePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_stop_freeze_reliable_pdu.pdu_header,
            stop_freeze_reliable_pdu.pdu_header
        );
    }
}
