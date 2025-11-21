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
        SerializedLength, SimulationAddress,
        constants::MAX_PDU_SIZE_OCTETS,
        dis_error::DISError,
        enums::{DISAttributeActionCode, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    entity_information::data_types::attribute_record_set::AttributeRecordSet,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.6.3
#[derive(Default)]
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

impl Pdu for AttributePdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH + 4; // TODO(@anyone): Get length

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    /// Serialize contents of `AttributePdu` into `BytesMut` buf
    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let size = std::mem::size_of_val(self);
        self.pdu_header.length = u16::try_from(size).map_err(|_| DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })?;
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
        Ok(())
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
    fn cast_to_any() {
        let pdu = AttributePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<AttributePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = AttributePdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
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
