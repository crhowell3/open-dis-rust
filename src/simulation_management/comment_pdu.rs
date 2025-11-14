//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    datum_records::{FixedDatumRecord, VariableDatumRecord},
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.5.13
pub struct CommentPdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub number_of_fixed_datum_records: u32,
    pub number_of_variable_datum_records: u32,
    pub fixed_datum_records: Vec<FixedDatumRecord>,
    pub variable_datum_records: Vec<VariableDatumRecord>,
}

impl Default for CommentPdu {
    fn default() -> Self {
        CommentPdu {
            pdu_header: PduHeader::default(),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            number_of_fixed_datum_records: 0,
            number_of_variable_datum_records: 0,
            fixed_datum_records: vec![],
            variable_datum_records: vec![],
        }
    }
}

impl Pdu for CommentPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>() * 2
            + std::mem::size_of::<u32>() * 2;

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
        buf.put_u32(self.number_of_fixed_datum_records);
        buf.put_u32(self.number_of_variable_datum_records);
        for i in 0..self.fixed_datum_records.len() {
            self.fixed_datum_records[i].serialize(buf);
        }
        for i in 0..self.variable_datum_records.len() {
            self.variable_datum_records[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::Comment {
            let originating_entity_id = EntityId::deserialize(&mut buffer);
            let receiving_entity_id = EntityId::deserialize(&mut buffer);
            let number_of_fixed_datum_records = buffer.get_u32();
            let number_of_variable_datum_records = buffer.get_u32();
            let mut fixed_datum_records: Vec<FixedDatumRecord> = vec![];
            fixed_datum_records.reserve(number_of_fixed_datum_records.try_into().unwrap());
            for _record in 0..number_of_fixed_datum_records as usize {
                fixed_datum_records.push(FixedDatumRecord::deserialize(&mut buffer));
            }
            let mut variable_datum_records: Vec<VariableDatumRecord> = vec![];
            variable_datum_records.reserve(number_of_variable_datum_records.try_into().unwrap());
            for _record in 0..number_of_variable_datum_records as usize {
                variable_datum_records.push(VariableDatumRecord::deserialize(&mut buffer));
            }

            Ok(CommentPdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                number_of_fixed_datum_records,
                number_of_variable_datum_records,
                fixed_datum_records,
                variable_datum_records,
            })
        } else {
            Err(DISError::invalid_header(
                format!("Expected PDU type Comment, got {:?}", pdu_header.pdu_type),
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
        let originating_entity_id = EntityId::deserialize(&mut buffer);
        let receiving_entity_id = EntityId::deserialize(&mut buffer);
        let number_of_fixed_datum_records = buffer.get_u32();
        let number_of_variable_datum_records = buffer.get_u32();
        let mut fixed_datum_records: Vec<FixedDatumRecord> = vec![];
        fixed_datum_records.reserve(number_of_fixed_datum_records.try_into().unwrap());
        for _record in 0..number_of_fixed_datum_records as usize {
            fixed_datum_records.push(FixedDatumRecord::deserialize(&mut buffer));
        }
        let mut variable_datum_records: Vec<VariableDatumRecord> = vec![];
        variable_datum_records.reserve(number_of_variable_datum_records.try_into().unwrap());
        for _record in 0..number_of_variable_datum_records as usize {
            variable_datum_records.push(VariableDatumRecord::deserialize(&mut buffer));
        }

        Ok(CommentPdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            number_of_fixed_datum_records,
            number_of_variable_datum_records,
            fixed_datum_records,
            variable_datum_records,
        })
    }
}

impl CommentPdu {
    /// Creates a Comment PDU
    ///
    /// # Examples
    ///
    /// Initializing a Comment PDU:
    /// ```
    /// use open_dis_rust::simulation_management::CommentPdu;
    /// let mut comment_pdu = CommentPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::Comment;
        pdu.pdu_header.protocol_family = ProtocolFamily::SimulationManagement;
        pdu.finalize();
        pdu
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::CommentPdu;
    use crate::common::{Pdu, PduType, pdu_header::PduHeader};

    #[test]
    fn create_header() {
        let comment_pdu = CommentPdu::default();
        let pdu_header = PduHeader::default();

        assert_eq!(
            pdu_header.protocol_version,
            comment_pdu.pdu_header.protocol_version
        );
        assert_eq!(pdu_header.exercise_id, comment_pdu.pdu_header.exercise_id);
        assert_eq!(pdu_header.pdu_type, comment_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            comment_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, comment_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.status_record,
            comment_pdu.pdu_header.status_record
        );
    }

    #[test]
    fn cast_to_any() {
        let comment_pdu = CommentPdu::default();
        let any_pdu = comment_pdu.as_any();

        assert!(any_pdu.is::<CommentPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut comment_pdu = CommentPdu::default();
        let mut buffer = BytesMut::new();
        comment_pdu.serialize(&mut buffer);

        let new_comment_pdu = CommentPdu::deserialize(buffer).unwrap();
        assert_eq!(new_comment_pdu.pdu_header, comment_pdu.pdu_header);
    }

    #[test]
    fn create_new_pdu() {
        let comment_pdu = CommentPdu::new();

        assert_eq!(comment_pdu.header().pdu_type, PduType::Comment);
    }
}
