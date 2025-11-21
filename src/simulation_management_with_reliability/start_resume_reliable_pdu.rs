//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    SerializedLength,
    clock_time::ClockTime,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily, RequiredReliabilityService},
    pdu::Pdu,
    pdu_header::PduHeader,
};

#[derive(Copy, Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.11.4
pub struct StartResumeReliablePdu {
    pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub real_world_time: ClockTime,
    pub simulation_time: ClockTime,
    pub required_reliability_service: RequiredReliabilityService,
    _padding: u8,
    _padding2: u16,
    pub request_id: u32,
}

impl Default for StartResumeReliablePdu {
    fn default() -> Self {
        StartResumeReliablePdu {
            pdu_header: PduHeader::default(),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            real_world_time: ClockTime::default(),
            simulation_time: ClockTime::default(),
            required_reliability_service: RequiredReliabilityService::default(),
            _padding: 0,
            _padding2: 0,
            request_id: 0,
        }
    }
}

impl Pdu for StartResumeReliablePdu {
    fn length(&self) -> u16 {
        let length =
            PduHeader::LENGTH + EntityId::LENGTH * 2 + ClockTime::LENGTH * 2 + 1 + 1 + 2 + 4;

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
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        self.real_world_time.serialize(buf);
        self.simulation_time.serialize(buf);
        buf.put_u8(self.required_reliability_service as u8);
        buf.put_u8(self._padding);
        buf.put_u16(self._padding2);
        buf.put_u32(self.request_id);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::StartResumeReliable {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type StartResumeReliable, got {:?}",
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

impl StartResumeReliablePdu {
    /// Creates an StartResumeReliable PDU
    ///
    /// # Examples
    ///
    /// Initializing an StartResumeReliable PDU:
    /// ```
    /// use open_dis_rust::simulation_management_with_reliability::StartResumeReliablePdu;
    /// let mut acknowledge_pdu = StartResumeReliablePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::StartResumeReliable;
        pdu.pdu_header.protocol_family = ProtocolFamily::SimulationManagementWithReliability;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_entity_id = EntityId::deserialize(buf);
        let receiving_entity_id = EntityId::deserialize(buf);
        let real_world_time = ClockTime::deserialize(buf);
        let simulation_time = ClockTime::deserialize(buf);
        let required_reliability_service = RequiredReliabilityService::deserialize(buf);
        let _padding = buf.get_u8();
        let _padding2 = buf.get_u16();
        let request_id = buf.get_u32();

        StartResumeReliablePdu {
            pdu_header: PduHeader::default(),
            originating_entity_id,
            receiving_entity_id,
            real_world_time,
            simulation_time,
            required_reliability_service,
            _padding,
            _padding2,
            request_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StartResumeReliablePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn create_header() {
        let pdu = StartResumeReliablePdu::new();
        let pdu_header = PduHeader::default();

        assert_eq!(pdu_header.protocol_version, pdu.pdu_header.protocol_version);
        assert_eq!(pdu_header.exercise_id, pdu.pdu_header.exercise_id);
        assert_eq!(pdu_header.pdu_type, pdu.pdu_header.pdu_type);
        assert_eq!(pdu_header.protocol_family, pdu.pdu_header.protocol_family);
        assert_eq!(pdu_header.length, pdu.pdu_header.length);
        assert_eq!(pdu_header.status_record, pdu.pdu_header.status_record);
    }

    #[test]
    fn cast_to_any() {
        let pdu = StartResumeReliablePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<StartResumeReliablePdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = StartResumeReliablePdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = StartResumeReliablePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 384 / BITS_PER_BYTE;
        let pdu = StartResumeReliablePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
