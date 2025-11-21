//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.9.5
pub struct MinefieldResponseNackPdu {
    pdu_header: PduHeader,
    pub minefield_id: EntityId,
    pub requesting_entity_id: EntityId,
    pub request_id: u8,
    pub number_of_missing_pdus: u8,
    pub missing_pdu_sequence_numbers: Vec<u64>,
}

impl Default for MinefieldResponseNackPdu {
    fn default() -> Self {
        MinefieldResponseNackPdu {
            pdu_header: PduHeader::default(),
            minefield_id: EntityId::default(1),
            requesting_entity_id: EntityId::default(2),
            request_id: 0,
            number_of_missing_pdus: 0,
            missing_pdu_sequence_numbers: vec![],
        }
    }
}

impl Pdu for MinefieldResponseNackPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>() * 2
            + std::mem::size_of::<u8>() * 2;

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
        self.minefield_id.serialize(buf);
        self.requesting_entity_id.serialize(buf);
        buf.put_u8(self.request_id);
        buf.put_u8(self.number_of_missing_pdus);
        for i in 0..self.missing_pdu_sequence_numbers.len() {
            buf.put_u64(self.missing_pdu_sequence_numbers[i]);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::MinefieldResponseNack {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type MinefieldResponseNack, got {:?}",
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

impl MinefieldResponseNackPdu {
    /// Creates a new `MinefieldResponseNackPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `MinefieldResponseNackPdu`:
    /// ```
    /// use open_dis_rust::minefield::MinefieldResponseNackPdu;
    /// let pdu = MinefieldResponseNackPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::MinefieldResponseNack;
        pdu.pdu_header.protocol_family = ProtocolFamily::Minefield;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let minefield_id = EntityId::deserialize(buf);
        let requesting_entity_id = EntityId::deserialize(buf);
        let request_id = buf.get_u8();
        let number_of_missing_pdus = buf.get_u8();
        let mut missing_pdu_sequence_numbers: Vec<u64> = vec![];
        for _i in 0..number_of_missing_pdus {
            missing_pdu_sequence_numbers.push(buf.get_u64());
        }

        MinefieldResponseNackPdu {
            pdu_header: PduHeader::default(),
            minefield_id,
            requesting_entity_id,
            request_id,
            number_of_missing_pdus,
            missing_pdu_sequence_numbers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MinefieldResponseNackPdu;
    use crate::common::{Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn create_header() {
        let pdu = MinefieldResponseNackPdu::new();
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
        let pdu = MinefieldResponseNackPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<MinefieldResponseNackPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = MinefieldResponseNackPdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = MinefieldResponseNackPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 576 / 8;
        let pdu = MinefieldResponseNackPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
