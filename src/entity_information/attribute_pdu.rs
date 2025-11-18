//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#![allow(deprecated)]

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::{
    common::{
        SimulationAddress,
        dis_error::DISError,
        entity_id::EntityId,
        enums::DISAttributeActionCode,
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    },
    entity_information::data_types::attribute_record_set::AttributeRecordSet,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.6.3
pub struct AttributePdu {
    pdu_header: PduHeader,
    pub originating_simulation_address: SimulationAddress,
    _padding: u32,
    _padding2: u16,
    pub attribute_record_pdu_type: u8,
    pub attribute_record_protocol_version: u8,
    pub master_attribute_record_type: u32,
    pub action_code: DISAttributeActionCode,
    _padding3: u8,
    pub number_of_attribute_record_sets: u16,
    pub attribute_record_sets: Vec<AttributeRecordSet>,
}

impl Default for AttributePdu {
    fn default() -> Self {
        AttributePdu {
            pdu_header: PduHeader::default(),
            originating_simulation_address: SimulationAddress::default(),
            _padding: 0u32,
            _padding2: 0u16,
            attribute_record_pdu_type: 0,
            attribute_record_protocol_version: 0,
            master_attribute_record_type: 0,
            action_code: DISAttributeActionCode::default(),
            _padding3: 0u8,
            number_of_attribute_record_sets: 0,
            attribute_record_sets: vec![],
        }
    }
}

impl Pdu for AttributePdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>() + std::mem::size_of::<EntityId>();

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    /// Serialize contents of `AttributePdu` into `BytesMut` buf
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_simulation_address.serialize(buf);
        buf.put_u32(self._padding);
        buf.put_u16(self._padding2);
        buf.put_u8(self.attribute_record_pdu_type);
        buf.put_u8(self.attribute_record_protocol_version);
        buf.put_u32(self.master_attribute_record_type);
        buf.put_u8(self.action_code as u8);
        buf.put_u8(self._padding3);
        buf.put_u16(self.number_of_attribute_record_sets);
        for i in 0..self.attribute_record_sets.len() {
            self.attribute_record_sets[i].serialize(buf);
        }
    }

    /// Deserialize bytes from `BytesMut` buf and interpret as `AttributePdu`
    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::Attribute {
            return Err(DISError::invalid_header(
                format!("Expected PDU type Attribute, got {:?}", header.pdu_type),
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

impl AttributePdu {
    /// Creates a new `AttributePdu`
    ///
    /// # Examples
    ///
    /// Initializing an `AttributePdu`:
    /// ```
    /// use open_dis_rust::entity_information::AttributePdu;
    /// let pdu = AttributePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::Attribute;
        pdu.pdu_header.protocol_family = ProtocolFamily::EntityInformation;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_simulation_address = SimulationAddress::deserialize(buf);
        let _padding = buf.get_u32();
        let _padding2 = buf.get_u16();
        let attribute_record_pdu_type = buf.get_u8();
        let attribute_record_protocol_version = buf.get_u8();
        let master_attribute_record_type = buf.get_u32();
        let action_code = DISAttributeActionCode::deserialize(buf);
        let _padding3 = buf.get_u8();
        let number_of_attribute_record_sets = buf.get_u16();
        let mut attribute_record_sets: Vec<AttributeRecordSet> = vec![];
        for _ in 0..number_of_attribute_record_sets {
            attribute_record_sets.push(AttributeRecordSet::deserialize(buf));
        }
        AttributePdu {
            pdu_header: PduHeader::default(),
            originating_simulation_address,
            _padding,
            _padding2,
            attribute_record_pdu_type,
            attribute_record_protocol_version,
            master_attribute_record_type,
            action_code,
            _padding3,
            number_of_attribute_record_sets,
            attribute_record_sets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AttributePdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn create_header() {
        let pdu = AttributePdu::new();
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
        let pdu = AttributePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<AttributePdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = AttributePdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = AttributePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / 8;
        let pdu = AttributePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
