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
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
};

#[derive(Copy, Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.5.4
pub struct StartResumePdu {
    pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub real_world_time: ClockTime,
    pub simulation_time: ClockTime,
    pub request_id: u32,
}

impl Default for StartResumePdu {
    fn default() -> Self {
        StartResumePdu {
            pdu_header: PduHeader::default(),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            real_world_time: ClockTime::default(),
            simulation_time: ClockTime::default(),
            request_id: 0,
        }
    }
}

impl Pdu for StartResumePdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH + EntityId::LENGTH * 2 + ClockTime::LENGTH * 2 + 4;

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
        self.real_world_time.serialize(buf);
        self.simulation_time.serialize(buf);
        buf.put_u32(self.request_id);
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::StartResume {
            return Err(DISError::invalid_header(
                format!("Expected PDU type StartResume, got {:?}", header.pdu_type),
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

impl StartResumePdu {
    /// Creates a new `StartResumePdu`
    ///
    /// # Examples
    ///
    /// Initializing an `StartResumePdu`:
    /// ```
    /// use open_dis_rust::simulation_management::StartResumePdu;
    /// let pdu = StartResumePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::StartResume;
        pdu.pdu_header.protocol_family = ProtocolFamily::SimulationManagement;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_entity_id = EntityId::deserialize(buf);
        let receiving_entity_id = EntityId::deserialize(buf);
        let real_world_time = ClockTime::deserialize(buf);
        let simulation_time = ClockTime::deserialize(buf);
        let request_id = buf.get_u32();

        StartResumePdu {
            pdu_header: PduHeader::default(),
            originating_entity_id,
            receiving_entity_id,
            real_world_time,
            simulation_time,
            request_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StartResumePdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn create_header() {
        let pdu = StartResumePdu::new();
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
        let pdu = StartResumePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<StartResumePdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = StartResumePdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = StartResumePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / 8;
        let pdu = StartResumePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
