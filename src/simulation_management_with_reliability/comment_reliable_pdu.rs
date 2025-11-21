//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    SerializedLength,
    constants::MAX_PDU_SIZE_OCTETS,
    datum_records::{FixedDatumRecord, VariableDatumRecord},
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.11.13
pub struct CommentReliablePdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub number_of_fixed_datum_records: u32,
    pub number_of_variable_datum_records: u32,
    pub fixed_datum_records: Vec<FixedDatumRecord>,
    pub variable_datum_records: Vec<VariableDatumRecord>,
}

impl Default for CommentReliablePdu {
    fn default() -> Self {
        CommentReliablePdu {
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

impl Pdu for CommentReliablePdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH + EntityId::LENGTH * 2 + 4 + 4;

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
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::CommentReliable {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type CommentReliable, got {:?}",
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

impl CommentReliablePdu {
    /// Creates a new `CommentReliablePdu`
    ///
    /// # Examples
    ///
    /// Initializing an `CommentReliablePdu`:
    /// ```
    /// use open_dis_rust::simulation_management_with_reliability::CommentReliablePdu;
    /// let pdu = CommentReliablePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::CommentReliable;
        pdu.pdu_header.protocol_family = ProtocolFamily::SimulationManagementWithReliability;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_entity_id = EntityId::deserialize(buf);
        let receiving_entity_id = EntityId::deserialize(buf);
        let number_of_fixed_datum_records = buf.get_u32();
        let number_of_variable_datum_records = buf.get_u32();
        let mut fixed_datum_records: Vec<FixedDatumRecord> = vec![];
        fixed_datum_records.reserve(number_of_fixed_datum_records.try_into().unwrap_or_default());
        for _record in 0..number_of_fixed_datum_records as usize {
            fixed_datum_records.push(FixedDatumRecord::deserialize(buf));
        }
        let mut variable_datum_records: Vec<VariableDatumRecord> = vec![];
        variable_datum_records.reserve(
            number_of_variable_datum_records
                .try_into()
                .unwrap_or_default(),
        );
        for _record in 0..number_of_variable_datum_records as usize {
            variable_datum_records.push(VariableDatumRecord::deserialize(buf));
        }

        CommentReliablePdu {
            pdu_header: PduHeader::default(),
            originating_entity_id,
            receiving_entity_id,
            number_of_fixed_datum_records,
            number_of_variable_datum_records,
            fixed_datum_records,
            variable_datum_records,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CommentReliablePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let pdu = CommentReliablePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<CommentReliablePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = CommentReliablePdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = CommentReliablePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
        let pdu = CommentReliablePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
