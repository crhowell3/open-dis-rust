//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::{
    common::{
        SimulationIdentifier,
        constants::MAX_PDU_SIZE_OCTETS,
        dis_error::DISError,
        entity_id::EntityId,
        enums::{
            IOActionIOActionPhase, IOActionIOActionType, IOActionIOSimulationSource,
            IOActionIOWarfareType, PduType, ProtocolFamily,
        },
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    warfare::data_types::standard_variable_specification::StandardVariableSpecification,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.12.2
pub struct InformationOperationsActionPdu {
    pdu_header: PduHeader,
    pub originating_simulation_id: SimulationIdentifier,
    pub receiving_simulation_id: SimulationIdentifier,
    pub request_id: u32,
    pub io_warfare_type: IOActionIOWarfareType,
    pub io_simulation_source: IOActionIOSimulationSource,
    pub io_action_type: IOActionIOActionType,
    pub io_action_phase: IOActionIOActionPhase,
    _padding: u32,
    pub io_attacker_entity_id: EntityId,
    pub io_primary_target_entity_id: EntityId,
    _padding2: u16,
    pub io_records: StandardVariableSpecification,
}

impl Default for InformationOperationsActionPdu {
    fn default() -> Self {
        Self {
            pdu_header: PduHeader::default(),
            originating_simulation_id: SimulationIdentifier::default(),
            receiving_simulation_id: SimulationIdentifier::default(),
            request_id: 0u32,
            io_warfare_type: IOActionIOWarfareType::default(),
            io_simulation_source: IOActionIOSimulationSource::default(),
            io_action_type: IOActionIOActionType::default(),
            io_action_phase: IOActionIOActionPhase::default(),
            _padding: 0u32,
            io_attacker_entity_id: EntityId::default(3),
            io_primary_target_entity_id: EntityId::default(4),
            _padding2: 0u16,
            io_records: StandardVariableSpecification::default(),
        }
    }
}

impl Pdu for InformationOperationsActionPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<SimulationIdentifier>() * 2
            + std::mem::size_of::<EntityId>() * 2
            + std::mem::size_of::<u32>() * 2
            + std::mem::size_of::<u16>()
            + std::mem::size_of::<IOActionIOWarfareType>()
            + std::mem::size_of::<IOActionIOSimulationSource>()
            + std::mem::size_of::<IOActionIOActionType>()
            + std::mem::size_of::<IOActionIOActionPhase>()
            + std::mem::size_of::<StandardVariableSpecification>();

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let size = std::mem::size_of_val(self);
        self.pdu_header.length = u16::try_from(size).map_err(|_| DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })?;
        self.pdu_header.serialize(buf);
        self.originating_simulation_id.serialize(buf);
        self.receiving_simulation_id.serialize(buf);
        buf.put_u32(self.request_id);
        buf.put_u8(self.io_warfare_type as u8);
        buf.put_u8(self.io_simulation_source as u8);
        buf.put_u8(self.io_action_type as u8);
        buf.put_u8(self.io_action_phase as u8);
        buf.put_u32(self._padding);
        self.io_attacker_entity_id.serialize(buf);
        self.io_primary_target_entity_id.serialize(buf);
        buf.put_u16(self._padding2);
        self.io_records.serialize(buf);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::InformationOperationsAction {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type InformationOperationsAction, got {:?}",
                    header.pdu_type
                ),
                None,
            ));
        }
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn deserialize_without_header<B: Buf>(buf: &mut B, header: PduHeader) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }
}

impl InformationOperationsActionPdu {
    /// Creates a new `InformationOperationsActionPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `InformationOperationsActionPdu`:
    /// ```
    /// use open_dis_rust::information_operations::InformationOperationsActionPdu;
    /// let pdu = InformationOperationsActionPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::InformationOperationsAction;
        pdu.pdu_header.protocol_family = ProtocolFamily::InformationOperations;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_simulation_id = SimulationIdentifier::deserialize(buf);
        let receiving_simulation_id = SimulationIdentifier::deserialize(buf);
        let request_id = buf.get_u32();
        let io_warfare_type = IOActionIOWarfareType::deserialize(buf);
        let io_simulation_source = IOActionIOSimulationSource::deserialize(buf);
        let io_action_type = IOActionIOActionType::deserialize(buf);
        let io_action_phase = IOActionIOActionPhase::deserialize(buf);
        let _padding = buf.get_u32();
        let io_attacker_entity_id = EntityId::deserialize(buf);
        let io_primary_target_entity_id = EntityId::deserialize(buf);
        let _padding2 = buf.get_u16();
        let io_records = StandardVariableSpecification::deserialize(buf);

        InformationOperationsActionPdu {
            pdu_header: PduHeader::default(),
            originating_simulation_id,
            receiving_simulation_id,
            request_id,
            io_warfare_type,
            io_simulation_source,
            io_action_type,
            io_action_phase,
            _padding,
            io_attacker_entity_id,
            io_primary_target_entity_id,
            _padding2,
            io_records,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InformationOperationsActionPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let pdu = InformationOperationsActionPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<InformationOperationsActionPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = InformationOperationsActionPdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = InformationOperationsActionPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 448 / 8;
        let pdu = InformationOperationsActionPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
