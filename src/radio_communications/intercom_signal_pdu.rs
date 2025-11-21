//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::{
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily, SignalTDLType},
    pdu::Pdu,
    pdu_header::PduHeader,
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.7.5
pub struct IntercomSignalPdu {
    pdu_header: PduHeader,
    pub entity_id: EntityId,
    pub radio_id: u16,
    pub communications_device_id: u16,
    pub encoding_scheme: u16,
    pub tdl_type: SignalTDLType,
    pub sample_rate: u32,
    pub data_length: u16,
    pub samples: u16,
    pub data: Vec<u8>,
}

impl Default for IntercomSignalPdu {
    fn default() -> Self {
        IntercomSignalPdu {
            pdu_header: PduHeader::default(),
            entity_id: EntityId::default(1),
            radio_id: 0,
            communications_device_id: 0,
            encoding_scheme: 0,
            tdl_type: SignalTDLType::default(),
            sample_rate: 0,
            data_length: 0,
            samples: 0,
            data: vec![],
        }
    }
}

impl Pdu for IntercomSignalPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>()
            + std::mem::size_of::<u16>() * 5
            + std::mem::size_of::<SignalTDLType>()
            + std::mem::size_of::<u32>();

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
        buf.put_u16(self.communications_device_id);
        buf.put_u16(self.encoding_scheme);
        buf.put_u16(self.tdl_type as u16);
        buf.put_u32(self.sample_rate);
        buf.put_u16(self.data_length);
        buf.put_u16(self.samples);
        for i in 0..self.data.len() {
            buf.put_u8(self.data[i]);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::IntercomSignal {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type IntercomSignal, got {:?}",
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

impl IntercomSignalPdu {
    /// Creates a new `IntercomSignalPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `IntercomSignalPdu`:
    /// ```
    /// use open_dis_rust::radio_communications::IntercomSignalPdu;
    /// let pdu = IntercomSignalPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::IntercomSignal;
        pdu.pdu_header.protocol_family = ProtocolFamily::RadioCommunications;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let entity_id = EntityId::deserialize(buf);
        let radio_id = buf.get_u16();
        let communications_device_id = buf.get_u16();
        let encoding_scheme = buf.get_u16();
        let tdl_type = SignalTDLType::deserialize(buf);
        let sample_rate = buf.get_u32();
        let data_length = buf.get_u16();
        let samples = buf.get_u16();
        let mut data: Vec<u8> = vec![];
        for _i in 0..data_length {
            data.push(buf.get_u8());
        }
        IntercomSignalPdu {
            pdu_header: PduHeader::default(),
            entity_id,
            radio_id,
            communications_device_id,
            encoding_scheme,
            tdl_type,
            sample_rate,
            data_length,
            samples,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IntercomSignalPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let pdu = IntercomSignalPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<IntercomSignalPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = IntercomSignalPdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = IntercomSignalPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / 8;
        let pdu = IntercomSignalPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
