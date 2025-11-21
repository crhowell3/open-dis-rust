//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::{
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily, ReceiverReceiverState},
    pdu::Pdu,
    pdu_header::PduHeader,
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.7.4
pub struct ReceiverPdu {
    pdu_header: PduHeader,
    pub entity_id: EntityId,
    pub radio_id: u16,
    pub receiver_state: ReceiverReceiverState,
    _padding: u16,
    pub received_power: f32,
    pub transmitter_radio_reference_id: EntityId,
    pub transmitter_radio_id: u16,
}

impl Default for ReceiverPdu {
    fn default() -> Self {
        ReceiverPdu {
            pdu_header: PduHeader::default(),
            entity_id: EntityId::default(1),
            radio_id: 0,
            receiver_state: ReceiverReceiverState::default(),
            _padding: 0u16,
            received_power: 0.0,
            transmitter_radio_reference_id: EntityId::default(2),
            transmitter_radio_id: 0,
        }
    }
}

impl Pdu for ReceiverPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>() * 2
            + std::mem::size_of::<u16>() * 3
            + std::mem::size_of::<ReceiverReceiverState>()
            + std::mem::size_of::<f32>();

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
        self.entity_id.serialize(buf);
        buf.put_u16(self.radio_id);
        buf.put_u16(self.receiver_state as u16);
        buf.put_u16(self._padding);
        buf.put_f32(self.received_power);
        self.transmitter_radio_reference_id.serialize(buf);
        buf.put_u16(self.transmitter_radio_id);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::Receiver {
            return Err(DISError::invalid_header(
                format!("Expected PDU type Receiver, got {:?}", header.pdu_type),
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

impl ReceiverPdu {
    /// Creates a new `ReceiverPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `ReceiverPdu`:
    /// ```
    /// use open_dis_rust::radio_communications::ReceiverPdu;
    /// let pdu = ReceiverPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::Receiver;
        pdu.pdu_header.protocol_family = ProtocolFamily::RadioCommunications;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let entity_id = EntityId::deserialize(buf);
        let radio_id = buf.get_u16();
        let receiver_state = ReceiverReceiverState::deserialize(buf);
        let _padding = buf.get_u16();
        let received_power = buf.get_f32();
        let transmitter_radio_reference_id = EntityId::deserialize(buf);
        let transmitter_radio_id = buf.get_u16();
        ReceiverPdu {
            pdu_header: PduHeader::default(),
            entity_id,
            radio_id,
            receiver_state,
            _padding,
            received_power,
            transmitter_radio_reference_id,
            transmitter_radio_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ReceiverPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let pdu = ReceiverPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<ReceiverPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = ReceiverPdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = ReceiverPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / 8;
        let pdu = ReceiverPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
