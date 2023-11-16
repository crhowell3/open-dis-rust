//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §5.6.5.6
pub struct IntercomControlPdu {
    pub pdu_header: PduHeader,
    pub entity_id: EntityId,
    pub radio_id: u16,
    pub control_type: u8,
    pub communications_channel_type: u8,
    pub source_entity_id: EntityId,
    pub source_communications_device_id: u8,
    pub source_line_id: u8,
    pub transmit_priority: u8,
    pub transmit_line_state: u8,
    pub command: u8,
    pub master_entity_id: EntityId,
    pub master_communications_device_id: u16,
    pub intercom_parameters_length: u32,
    pub intercom_parameters: Vec<IntercomCommunicationsParameters>,
}

impl Default for IntercomControlPdu {
    /// Creates a default IntercomControl PDU with arbitrary originating and receiving
    /// entity IDs
    ///
    /// # Examples
    ///
    /// Initializing an IntercomControl PDU:
    /// ```
    /// use open_dis_rust::simulation_management::intercom_control_pdu::IntercomControlPdu;
    /// let intercom_control_pdu = IntercomControlPdu::default();
    /// ```
    ///
    fn default() -> Self {
        IntercomControlPdu {
            pdu_header: PduHeader::default(
                PduType::IntercomControl,
                ProtocolFamily::RadioCommunications,
                56,
            ),
            entity_id: EntityId::default(1),
            radio_id: 0,
            control_type: 0,
            communications_channel_type: 0,
            source_entity_id: EntityId::default(2),
            source_communications_device_id: 0,
            source_line_id: 0,
            transmit_priority: 0,
            transmit_line_state: 0,
            command: 0,
            master_entity_id: EntityId::default(3),
            master_communications_device_id: 0,
            intercom_parameters_length: 0,
            intercom_parameters: vec![],
        }
    }
}

impl Pdu for IntercomControlPdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.entity_id.serialize(buf);
        buf.put_u16(self.radio_id);
        buf.put_u8(self.control_type);
        buf.put_u8(self.communications_channel_type);
        self.source_entity_id.serialize(buf);
        buf.put_u8(self.source_communications_device_id);
        buf.put_u8(self.source_line_id);
        buf.put_u8(self.transmit_priority);
        buf.put_u8(self.transmit_line_state);
        buf.put_u8(self.command);
        self.master_entity_id.serialize(buf);
        buf.put_u16(self.master_communications_device_id);
        buf.put_u32(self.intercom_parameters_length);
        for i in 0..self.intercom_parameters_length {
            buf.put_u64(self.intercom_parameters[i]);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::IntercomControl {
            let entity_id = EntityId::decode(&mut buffer);
            let radio_id = buffer.get_u16();
            let control_type = buffer.get_u8();
            let communications_channel_type = buffer.get_u8();
            let source_entity_id = EntityId::decode(&mut buffer);
            let source_communications_device_id = buffer.get_u8();
            let source_line_id = buffer.get_u8();
            let transmit_priority = buffer.get_u8();
            let transmit_line_state = buffer.get_u8();
            let command = buffer.get_u8();
            let master_entity_id = EntityId::decode(&mut buffer);
            let master_communications_device_id = buffer.get_u16();
            let intercom_parameters_length = buffer.get_u32();
            let mut intercom_parameters: Vec<u64>;
            for i in 0..intercom_parameters_length {
                intercom_parameters.push(buffer.get_u64());
            }
            Ok(IntercomControlPdu {
                pdu_header,
                entity_id,
                radio_id,
                control_type,
                communications_channel_type,
                source_entity_id,
                source_communications_device_id,
                source_line_id,
                transmit_priority,
                transmit_line_state,
                command,
                master_entity_id,
                master_communications_device_id,
                intercom_parameters_length,
                intercom_parameters,
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
        let entity_id = EntityId::decode(&mut buffer);
        let radio_id = buffer.get_u16();
        let control_type = buffer.get_u8();
        let communications_channel_type = buffer.get_u8();
        let source_entity_id = EntityId::decode(&mut buffer);
        let source_communications_device_id = buffer.get_u8();
        let source_line_id = buffer.get_u8();
        let transmit_priority = buffer.get_u8();
        let transmit_line_state = buffer.get_u8();
        let command = buffer.get_u8();
        let master_entity_id = EntityId::decode(&mut buffer);
        let master_communications_device_id = buffer.get_u16();
        let intercom_parameters_length = buffer.get_u32();
        let mut intercom_parameters: Vec<u64>;
        for i in 0..intercom_parameters_length {
            intercom_parameters.push(buffer.get_u64());
        }
        Ok(IntercomControlPdu {
            pdu_header,
            entity_id,
            radio_id,
            control_type,
            communications_channel_type,
            source_entity_id,
            source_communications_device_id,
            source_line_id,
            transmit_priority,
            transmit_line_state,
            command,
            master_entity_id,
            master_communications_device_id,
            intercom_parameters_length,
            intercom_parameters,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IntercomControlPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let intercom_control_pdu = IntercomControlPdu::default();
        let pdu_header = PduHeader::default(
            PduType::IntercomControl,
            ProtocolFamily::RadioCommunications,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            intercom_control_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            intercom_control_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            intercom_control_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            intercom_control_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, intercom_control_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, intercom_control_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let intercom_control_pdu = IntercomControlPdu::default();
        let mut buffer = BytesMut::new();
        intercom_control_pdu.serialize(&mut buffer);

        let new_intercom_control_pdu = IntercomControlPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_intercom_control_pdu.pdu_header,
            intercom_control_pdu.pdu_header
        );
    }
}
