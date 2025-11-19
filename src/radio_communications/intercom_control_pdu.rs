//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use super::data_types::intercom_communications_parameters::IntercomCommunicationsParameters;
use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    enums::{
        IntercomControlCommand, IntercomControlControlType, IntercomControlTransmitLineState,
        PduType, ProtocolFamily,
    },
    pdu::Pdu,
    pdu_header::PduHeader,
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.7.6
pub struct IntercomControlPdu {
    pdu_header: PduHeader,
    pub control_type: IntercomControlControlType,
    pub communications_channel_type: u8,
    pub source_entity_id: EntityId,
    pub source_communications_device_id: u8,
    pub source_line_id: u8,
    pub transmit_priority: u8,
    pub transmit_line_state: IntercomControlTransmitLineState,
    pub command: IntercomControlCommand,
    pub master_intercom_number: EntityId,
    pub master_communications_device_id: u16,
    pub intercom_parameters_length: u32,
    pub intercom_parameters: Vec<IntercomCommunicationsParameters>,
}

impl Default for IntercomControlPdu {
    fn default() -> Self {
        IntercomControlPdu {
            pdu_header: PduHeader::default(),
            control_type: IntercomControlControlType::default(),
            communications_channel_type: 0,
            source_entity_id: EntityId::default(2),
            source_communications_device_id: 0,
            source_line_id: 0,
            transmit_priority: 0,
            transmit_line_state: IntercomControlTransmitLineState::default(),
            command: IntercomControlCommand::default(),
            master_intercom_number: EntityId::default(3),
            master_communications_device_id: 0,
            intercom_parameters_length: 0,
            intercom_parameters: vec![],
        }
    }
}

impl Pdu for IntercomControlPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>() * 3
            + std::mem::size_of::<u16>() * 2
            + std::mem::size_of::<u8>() * 7
            + std::mem::size_of::<u32>();

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
        buf.put_u8(self.control_type as u8);
        buf.put_u8(self.communications_channel_type);
        self.source_entity_id.serialize(buf);
        buf.put_u8(self.source_communications_device_id);
        buf.put_u8(self.source_line_id);
        buf.put_u8(self.transmit_priority);
        buf.put_u8(self.transmit_line_state as u8);
        buf.put_u8(self.command as u8);
        self.master_intercom_number.serialize(buf);
        buf.put_u16(self.master_communications_device_id);
        buf.put_u32(self.intercom_parameters_length);
        for i in 0usize..self.intercom_parameters_length as usize {
            self.intercom_parameters[i].serialize(buf);
        }
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::IntercomControl {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type IntercomControl, got {:?}",
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

impl IntercomControlPdu {
    /// Creates a new `IntercomControlPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `IntercomControlPdu`:
    /// ```
    /// use open_dis_rust::radio_communications::IntercomControlPdu;
    /// let pdu = IntercomControlPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::IntercomControl;
        pdu.pdu_header.protocol_family = ProtocolFamily::RadioCommunications;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let control_type = IntercomControlControlType::deserialize(buf);
        let communications_channel_type = buf.get_u8();
        let source_entity_id = EntityId::deserialize(buf);
        let source_communications_device_id = buf.get_u8();
        let source_line_id = buf.get_u8();
        let transmit_priority = buf.get_u8();
        let transmit_line_state = IntercomControlTransmitLineState::deserialize(buf);
        let command = IntercomControlCommand::deserialize(buf);
        let master_intercom_number = EntityId::deserialize(buf);
        let master_communications_device_id = buf.get_u16();
        let intercom_parameters_length = buf.get_u32();
        let mut intercom_parameters: Vec<IntercomCommunicationsParameters> = vec![];
        for _i in 0..intercom_parameters_length {
            intercom_parameters.push(IntercomCommunicationsParameters::deserialize(buf));
        }
        IntercomControlPdu {
            pdu_header: PduHeader::default(),
            control_type,
            communications_channel_type,
            source_entity_id,
            source_communications_device_id,
            source_line_id,
            transmit_priority,
            transmit_line_state,
            command,
            master_intercom_number,
            master_communications_device_id,
            intercom_parameters_length,
            intercom_parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IntercomControlPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn create_header() {
        let pdu = IntercomControlPdu::new();
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
        let pdu = IntercomControlPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<IntercomControlPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = IntercomControlPdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = IntercomControlPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 320 / 8;
        let pdu = IntercomControlPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
