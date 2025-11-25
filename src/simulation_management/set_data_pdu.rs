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

#[derive(Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.5.10
pub struct SetDataPdu {
    pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    padding: u32,
    pub number_of_fixed_datum_records: u32,
    pub number_of_variable_datum_records: u32,
    pub fixed_datum_records: Vec<FixedDatumRecord>,
    pub variable_datum_records: Vec<VariableDatumRecord>,
}

impl Pdu for SetDataPdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH + EntityId::LENGTH * 2 + 4 + 4 + 4 + 4;

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
        buf.put_u32(self.request_id);
        buf.put_u32(self.padding);
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
        if header.pdu_type != PduType::SetData {
            return Err(DISError::invalid_header(
                format!("Expected PDU type SetData, got {:?}", header.pdu_type),
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

impl SetDataPdu {
    #[must_use]
    /// Creates a new `SetDataPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `SetDataPdu`:
    /// ```
    /// use open_dis_rust::simulation_management::SetDataPdu;
    /// let pdu = SetDataPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::SetData;
        pdu.pdu_header.protocol_family = ProtocolFamily::SimulationManagement;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_entity_id = EntityId::deserialize(buf);
        let receiving_entity_id = EntityId::deserialize(buf);
        let request_id = buf.get_u32();
        let padding = buf.get_u32();
        let number_of_fixed_datum_records = buf.get_u32();
        let number_of_variable_datum_records = buf.get_u32();
        let mut fixed_datum_records: Vec<FixedDatumRecord> =
            Vec::with_capacity(number_of_fixed_datum_records.try_into().unwrap_or_default());
        for _record in 0..number_of_fixed_datum_records as usize {
            fixed_datum_records.push(FixedDatumRecord::deserialize(buf));
        }
        let mut variable_datum_records: Vec<VariableDatumRecord> = Vec::with_capacity(
            number_of_variable_datum_records
                .try_into()
                .unwrap_or_default(),
        );
        for _record in 0..number_of_variable_datum_records as usize {
            variable_datum_records.push(VariableDatumRecord::deserialize(buf));
        }

        SetDataPdu {
            pdu_header: PduHeader::default(),
            originating_entity_id,
            receiving_entity_id,
            request_id,
            padding,
            number_of_fixed_datum_records,
            number_of_variable_datum_records,
            fixed_datum_records,
            variable_datum_records,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SetDataPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = SetDataPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<SetDataPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = SetDataPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = SetDataPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
        let pdu = SetDataPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
