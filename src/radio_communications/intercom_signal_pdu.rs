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
/// Implemented according to IEEE 1278.1-2012 §7.7.5
pub struct IntercomSignalPdu {
    pub pdu_header: PduHeader,
    pub entity_id: EntityId,
    pub radio_id: u16,
    pub communications_device_id: u16,
    pub encoding_scheme: u16,
    pub tdl_type: u16,
    pub sample_rate: u32,
    pub data_length: u16,
    pub samples: u16,
    pub data: Vec<u8>,
}

impl Default for IntercomSignalPdu {
    /// Creates a default Intercom Signal PDU with arbitrary originating and receiving
    /// entity IDs
    ///
    /// # Examples
    ///
    /// Initializing an Intercom Signal PDU:
    /// ```
    /// use open_dis_rust::radio_communications::intercom_signal_pdu::IntercomSignalPdu;
    /// let intercom_signal_pdu = IntercomSignalPdu::default();
    /// ```
    ///
    fn default() -> Self {
        IntercomSignalPdu {
            pdu_header: PduHeader::default(
                PduType::IntercomSignal,
                ProtocolFamily::RadioCommunications,
                56,
            ),
            entity_id: EntityId::default(1),
            radio_id: 0,
            communications_device_id: 0,
            encoding_scheme: 0,
            tdl_type: 0,
            sample_rate: 0,
            data_length: 0,
            samples: 0,
            data: vec![],
        }
    }
}

impl Pdu for IntercomSignalPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.entity_id.serialize(buf);
        buf.put_u16(self.radio_id);
        buf.put_u16(self.communications_device_id);
        buf.put_u16(self.encoding_scheme);
        buf.put_u16(self.tdl_type);
        buf.put_u32(self.sample_rate);
        buf.put_u16(self.data_length);
        buf.put_u16(self.samples);
        for i in 0..self.data.len() {
            buf.put_u8(self.data[i]);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::IntercomSignal {
            let entity_id = EntityId::decode(&mut buffer);
            let radio_id = buffer.get_u16();
            let communications_device_id = buffer.get_u16();
            let encoding_scheme = buffer.get_u16();
            let tdl_type = buffer.get_u16();
            let sample_rate = buffer.get_u32();
            let data_length = buffer.get_u16();
            let samples = buffer.get_u16();
            let mut data: Vec<u8> = vec![];
            for _i in 0..data_length {
                data.push(buffer.get_u8());
            }
            Ok(IntercomSignalPdu {
                pdu_header,
                entity_id,
                radio_id,
                communications_device_id,
                encoding_scheme,
                tdl_type,
                sample_rate,
                data_length,
                samples,
                data,
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
        let communications_device_id = buffer.get_u16();
        let encoding_scheme = buffer.get_u16();
        let tdl_type = buffer.get_u16();
        let sample_rate = buffer.get_u32();
        let data_length = buffer.get_u16();
        let samples = buffer.get_u16();
        let mut data: Vec<u8> = vec![];
        for _i in 0..data_length {
            data.push(buffer.get_u8());
        }
        Ok(IntercomSignalPdu {
            pdu_header,
            entity_id,
            radio_id,
            communications_device_id,
            encoding_scheme,
            tdl_type,
            sample_rate,
            data_length,
            samples,
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IntercomSignalPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let intercom_signal_pdu = IntercomSignalPdu::default();
        let pdu_header = PduHeader::default(
            PduType::IntercomSignal,
            ProtocolFamily::RadioCommunications,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            intercom_signal_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            intercom_signal_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, intercom_signal_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            intercom_signal_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, intercom_signal_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, intercom_signal_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut intercom_signal_pdu = IntercomSignalPdu::default();
        let mut buffer = BytesMut::new();
        intercom_signal_pdu.serialize(&mut buffer);

        let new_intercom_signal_pdu = IntercomSignalPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_intercom_signal_pdu.pdu_header,
            intercom_signal_pdu.pdu_header
        );
    }
}
