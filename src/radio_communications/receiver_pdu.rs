//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.7.4
pub struct ReceiverPdu {
    pub pdu_header: PduHeader,
    pub entity_id: EntityId,
    pub radio_id: u16,
    pub receiver_state: u16,
    pub padding1: u16,
    pub received_power: f32,
    pub transmitter_entity_id: EntityId,
    pub transmitter_radio_id: u16,
}

impl Default for ReceiverPdu {
    /// Creates a default Receiver PDU with arbitrary originating and receiving
    /// entity IDs
    ///
    /// # Examples
    ///
    /// Initializing a Receiver PDU:
    /// ```
    /// use open_dis_rust::radio_communications::receiver_pdu::ReceiverPdu;
    /// let receiver_pdu = ReceiverPdu::default();
    /// ```
    ///
    fn default() -> Self {
        ReceiverPdu {
            pdu_header: PduHeader::default(
                PduType::Receiver,
                ProtocolFamily::RadioCommunications,
                56,
            ),
            entity_id: EntityId::default(1),
            radio_id: 0,
            receiver_state: 0,
            padding1: 0,
            received_power: 0.0,
            transmitter_entity_id: EntityId::default(2),
            transmitter_radio_id: 0,
        }
    }
}

impl Pdu for ReceiverPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.entity_id.serialize(buf);
        buf.put_u16(self.radio_id);
        buf.put_u16(self.receiver_state);
        buf.put_u16(self.padding1);
        buf.put_f32(self.received_power);
        self.transmitter_entity_id.serialize(buf);
        buf.put_u16(self.transmitter_radio_id);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::Receiver {
            let entity_id = EntityId::deserialize(&mut buffer);
            let radio_id = buffer.get_u16();
            let receiver_state = buffer.get_u16();
            let padding1 = buffer.get_u16();
            let received_power = buffer.get_f32();
            let transmitter_entity_id = EntityId::deserialize(&mut buffer);
            let transmitter_radio_id = buffer.get_u16();
            Ok(ReceiverPdu {
                pdu_header,
                entity_id,
                radio_id,
                receiver_state,
                padding1,
                received_power,
                transmitter_entity_id,
                transmitter_radio_id,
            })
        } else {
            Err(DISError::invalid_header(
                format!("Expected PDU type Receiver, got {:?}", pdu_header.pdu_type),
                None,
            ))
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
        let entity_id = EntityId::deserialize(&mut buffer);
        let radio_id = buffer.get_u16();
        let receiver_state = buffer.get_u16();
        let padding1 = buffer.get_u16();
        let received_power = buffer.get_f32();
        let transmitter_entity_id = EntityId::deserialize(&mut buffer);
        let transmitter_radio_id = buffer.get_u16();
        Ok(ReceiverPdu {
            pdu_header,
            entity_id,
            radio_id,
            receiver_state,
            padding1,
            received_power,
            transmitter_entity_id,
            transmitter_radio_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ReceiverPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let receiver_pdu = ReceiverPdu::default();
        let pdu_header = PduHeader::default(
            PduType::Receiver,
            ProtocolFamily::RadioCommunications,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            receiver_pdu.pdu_header.protocol_version
        );
        assert_eq!(pdu_header.exercise_id, receiver_pdu.pdu_header.exercise_id);
        assert_eq!(pdu_header.pdu_type, receiver_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            receiver_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, receiver_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.status_record,
            receiver_pdu.pdu_header.status_record
        );
    }

    #[test]
    fn deserialize_header() {
        let mut receiver_pdu = ReceiverPdu::default();
        let mut buffer = BytesMut::new();
        receiver_pdu.serialize(&mut buffer);

        let new_receiver_pdu = ReceiverPdu::deserialize(buffer).unwrap();
        assert_eq!(new_receiver_pdu.pdu_header, receiver_pdu.pdu_header);
    }
}
