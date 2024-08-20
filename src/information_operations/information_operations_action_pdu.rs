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

#[derive(Copy, Clone, Debug)]
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
    pub number_of_io_records: u16,
    pub io_records: Vec<StandardVariableSpecification>,
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
        }
    }
}

impl Pdu for InformationOperationsActionPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::InformationOperationsAction {
            Ok(InformationOperationsActionPdu { pdu_header })
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
        Ok(InformationOperationsActionPdu { pdu_header })
    }
}
