//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::{
    common::{
        dis_error::DISError,
        entity_id::EntityId,
        enums::{
            IOActionIOActionPhase, IOActionIOActionType, IOActionIOSimulationSource,
            IOActionIOWarfareType,
        },
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    },
    warfare::data_types::standard_variable_specification::StandardVariableSpecification,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012
pub struct InformationOperationsActionPdu {
    pub pdu_header: PduHeader,
    pub originating_simulation_id: EntityId,
    pub receiving_simulation_id: EntityId,
    pub request_id: u32,
    pub io_warfare_type: IOActionIOWarfareType,
    pub io_simulation_source: IOActionIOSimulationSource,
    pub io_action_type: IOActionIOActionType,
    pub io_action_phase: IOActionIOActionPhase,
    pub padding1: u32,
    pub io_attacker_entity_id: EntityId,
    pub io_primary_target_entity_id: EntityId,
    pub padding2: u16,
    pub io_records: StandardVariableSpecification,
}

impl Default for InformationOperationsActionPdu {
    /// Creates default-initialized Information Operations Action PDU
    ///
    /// # Examples
    ///
    /// Initializing an Information Operations Action PDU:
    /// ```
    /// use open_dis_rust::information_operations::information_operations_action_pdu::InformationOperationsActionPdu;
    /// let mut io_action_pdu = InformationOperationsActionPdu::default();
    /// ```
    ///
    fn default() -> Self {
        InformationOperationsActionPdu {
            pdu_header: PduHeader::default(
                PduType::InformationOperationsAction,
                ProtocolFamily::InformationOperations,
                32,
            ),
            originating_simulation_id: EntityId::default(1),
            receiving_simulation_id: EntityId::default(2),
            request_id: 0,
            io_warfare_type: IOActionIOWarfareType::default(),
            io_simulation_source: IOActionIOSimulationSource::default(),
            io_action_type: IOActionIOActionType::default(),
            io_action_phase: IOActionIOActionPhase::default(),
            padding1: 0,
            io_attacker_entity_id: EntityId::default(3),
            io_primary_target_entity_id: EntityId::default(4),
            padding2: 0,
            io_records: StandardVariableSpecification::default(),
        }
    }
}

impl Pdu for InformationOperationsActionPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_simulation_id.serialize(buf);
        self.receiving_simulation_id.serialize(buf);
        buf.put_u32(self.request_id);
        buf.put_u8(self.io_warfare_type as u8);
        buf.put_u8(self.io_simulation_source as u8);
        buf.put_u8(self.io_action_type as u8);
        buf.put_u8(self.io_action_phase as u8);
        buf.put_u32(self.padding1);
        self.io_attacker_entity_id.serialize(buf);
        self.io_primary_target_entity_id.serialize(buf);
        buf.put_u16(self.padding2);
        self.io_records.serialize(buf);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::InformationOperationsAction {
            let originating_simulation_id = EntityId::decode(&mut buffer);
            let receiving_simulation_id = EntityId::decode(&mut buffer);
            let request_id = buffer.get_u32();
            let io_warfare_type = IOActionIOWarfareType::decode(&mut buffer);
            let io_simulation_source = IOActionIOSimulationSource::decode(&mut buffer);
            let io_action_type = IOActionIOActionType::decode(&mut buffer);
            let io_action_phase = IOActionIOActionPhase::decode(&mut buffer);
            let padding1 = buffer.get_u32();
            let io_attacker_entity_id = EntityId::decode(&mut buffer);
            let io_primary_target_entity_id = EntityId::decode(&mut buffer);
            let padding2 = buffer.get_u16();
            let io_records = StandardVariableSpecification::decode(&mut buffer);
            Ok(InformationOperationsActionPdu {
                pdu_header,
                originating_simulation_id,
                receiving_simulation_id,
                request_id,
                io_warfare_type,
                io_simulation_source,
                io_action_type,
                io_action_phase,
                padding1,
                io_attacker_entity_id,
                io_primary_target_entity_id,
                padding2,
                io_records,
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
        let originating_simulation_id = EntityId::decode(&mut buffer);
        let receiving_simulation_id = EntityId::decode(&mut buffer);
        let request_id = buffer.get_u32();
        let io_warfare_type = IOActionIOWarfareType::decode(&mut buffer);
        let io_simulation_source = IOActionIOSimulationSource::decode(&mut buffer);
        let io_action_type = IOActionIOActionType::decode(&mut buffer);
        let io_action_phase = IOActionIOActionPhase::decode(&mut buffer);
        let padding1 = buffer.get_u32();
        let io_attacker_entity_id = EntityId::decode(&mut buffer);
        let io_primary_target_entity_id = EntityId::decode(&mut buffer);
        let padding2 = buffer.get_u16();
        let io_records = StandardVariableSpecification::decode(&mut buffer);
        Ok(InformationOperationsActionPdu {
            pdu_header,
            originating_simulation_id,
            receiving_simulation_id,
            request_id,
            io_warfare_type,
            io_simulation_source,
            io_action_type,
            io_action_phase,
            padding1,
            io_attacker_entity_id,
            io_primary_target_entity_id,
            padding2,
            io_records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::InformationOperationsActionPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let information_operations_action_pdu = InformationOperationsActionPdu::default();
        let pdu_header = PduHeader::default(
            PduType::InformationOperationsAction,
            ProtocolFamily::InformationOperations,
            32,
        );

        assert_eq!(
            pdu_header.protocol_version,
            information_operations_action_pdu
                .pdu_header
                .protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            information_operations_action_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            information_operations_action_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            information_operations_action_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            information_operations_action_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.padding,
            information_operations_action_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let mut information_operations_action_pdu = InformationOperationsActionPdu::default();
        let mut buffer = BytesMut::new();
        information_operations_action_pdu.serialize(&mut buffer);

        let new_information_operations_action_pdu =
            InformationOperationsActionPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_information_operations_action_pdu.pdu_header,
            information_operations_action_pdu.pdu_header
        );
    }
}
