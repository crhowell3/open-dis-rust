//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.9.5
pub struct MinefieldResponseNackPdu {
    pub pdu_header: PduHeader,
    pub minefield_id: EntityId,
    pub requesting_entity_id: EntityId,
    pub request_id: u8,
    pub number_of_missing_pdus: u8,
    pub missing_pdu_sequence_numbers: Vec<u64>,
}

impl Default for MinefieldResponseNackPdu {
    fn default() -> Self {
        MinefieldResponseNackPdu {
            pdu_header: PduHeader::default(
                PduType::MinefieldResponseNack,
                ProtocolFamily::Minefield,
                56,
            ),
            minefield_id: EntityId::default(1),
            requesting_entity_id: EntityId::default(2),
            request_id: 0,
            number_of_missing_pdus: 0,
            missing_pdu_sequence_numbers: vec![],
        }
    }
}

impl Pdu for MinefieldResponseNackPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.minefield_id.serialize(buf);
        self.requesting_entity_id.serialize(buf);
        buf.put_u8(self.request_id);
        buf.put_u8(self.number_of_missing_pdus);
        for i in 0..self.missing_pdu_sequence_numbers.len() {
            buf.put_u64(self.missing_pdu_sequence_numbers[i]);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::MinefieldResponseNack {
            let minefield_id = EntityId::deserialize(&mut buffer);
            let requesting_entity_id = EntityId::deserialize(&mut buffer);
            let request_id = buffer.get_u8();
            let number_of_missing_pdus = buffer.get_u8();
            let mut missing_pdu_sequence_numbers: Vec<u64> = vec![];
            for _i in 0..number_of_missing_pdus {
                missing_pdu_sequence_numbers.push(buffer.get_u64());
            }

            Ok(MinefieldResponseNackPdu {
                pdu_header,
                minefield_id,
                requesting_entity_id,
                request_id,
                number_of_missing_pdus,
                missing_pdu_sequence_numbers,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type MinefieldResponseNack, got {:?}",
                    pdu_header.pdu_type
                ),
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
        let minefield_id = EntityId::deserialize(&mut buffer);
        let requesting_entity_id = EntityId::deserialize(&mut buffer);
        let request_id = buffer.get_u8();
        let number_of_missing_pdus = buffer.get_u8();
        let mut missing_pdu_sequence_numbers: Vec<u64> = vec![];
        for _i in 0..number_of_missing_pdus {
            missing_pdu_sequence_numbers.push(buffer.get_u64());
        }

        Ok(MinefieldResponseNackPdu {
            pdu_header,
            minefield_id,
            requesting_entity_id,
            request_id,
            number_of_missing_pdus,
            missing_pdu_sequence_numbers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MinefieldResponseNackPdu;
    use crate::common::pdu_header::{PduHeader, PduType, ProtocolFamily};

    #[test]
    fn create_header() {
        let minefield_response_nack_pdu = MinefieldResponseNackPdu::default();
        let pdu_header = PduHeader::default(
            PduType::MinefieldResponseNack,
            ProtocolFamily::Minefield,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            minefield_response_nack_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            minefield_response_nack_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            minefield_response_nack_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            minefield_response_nack_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            minefield_response_nack_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.padding,
            minefield_response_nack_pdu.pdu_header.padding
        );
    }
}
