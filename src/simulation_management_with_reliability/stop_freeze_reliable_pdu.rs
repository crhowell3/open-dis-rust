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
    enums::{FrozenBehavior, PduType, ProtocolFamily, Reason},
    pdu::Pdu,
    pdu_header::PduHeader,
};

#[derive(Copy, Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.11.5
pub struct StopFreezeReliablePdu {
    pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub real_world_time: ClockTime,
    pub reason: Reason,
    pub frozen_behavior: FrozenBehavior,
    pub required_reliability_service: u8,
    padding: u8,
    pub request_id: u32,
}

impl Pdu for StopFreezeReliablePdu {
    fn length(&self) -> u16 {
        let length =
            PduHeader::LENGTH + EntityId::LENGTH * 2 + ClockTime::LENGTH + 1 + 1 + 1 + 1 + 4;

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
        buf.put_u8(self.reason as u8);
        buf.put_u8(self.frozen_behavior.as_u8());
        buf.put_u8(self.required_reliability_service);
        buf.put_u8(self.padding);
        buf.put_u32(self.request_id);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::StopFreezeReliable {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type StopFreezeReliable, got {:?}",
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

impl StopFreezeReliablePdu {
    #[must_use]
    /// Creates an StopFreezeReliable PDU
    ///
    /// # Examples
    ///
    /// Initializing an StopFreezeReliable PDU:
    /// ```
    /// use open_dis_rust::simulation_management_with_reliability::StopFreezeReliablePdu;
    /// let mut pdu = StopFreezeReliablePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::StopFreezeReliable;
        pdu.pdu_header.protocol_family = ProtocolFamily::SimulationManagementWithReliability;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_entity_id = EntityId::deserialize(buf);
        let receiving_entity_id = EntityId::deserialize(buf);
        let real_world_time = ClockTime::deserialize(buf);
        let reason = Reason::deserialize(buf);
        let frozen_behavior = FrozenBehavior::from_u8(buf.get_u8()).unwrap();
        let required_reliability_service = buf.get_u8();
        let padding = buf.get_u8();
        let request_id = buf.get_u32();

        StopFreezeReliablePdu {
            pdu_header: PduHeader::default(),
            originating_entity_id,
            receiving_entity_id,
            real_world_time,
            reason,
            frozen_behavior,
            required_reliability_service,
            padding,
            request_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StopFreezeReliablePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = StopFreezeReliablePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<StopFreezeReliablePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = StopFreezeReliablePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = StopFreezeReliablePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
        let pdu = StopFreezeReliablePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
