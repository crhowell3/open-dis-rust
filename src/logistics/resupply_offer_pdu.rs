//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    SerializedLength,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
};

use super::data_types::supply_quantity::SupplyQuantity;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.4.3
pub struct ResupplyOfferPdu {
    pdu_header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub supplying_entity_id: EntityId,
    pub number_of_supply_types: u8,
    _padding: u8,
    _padding2: u16,
    pub supplies: Vec<SupplyQuantity>,
}

impl Default for ResupplyOfferPdu {
    fn default() -> Self {
        ResupplyOfferPdu {
            pdu_header: PduHeader::default(),
            receiving_entity_id: EntityId::default(1),
            supplying_entity_id: EntityId::default(2),
            number_of_supply_types: 0,
            _padding: 0,
            _padding2: 0,
            supplies: vec![],
        }
    }
}

impl Pdu for ResupplyOfferPdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH + EntityId::LENGTH * 2 + 1 + 1 + 2;

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
        self.receiving_entity_id.serialize(buf);
        self.supplying_entity_id.serialize(buf);
        buf.put_u8(self.number_of_supply_types);
        buf.put_u8(self._padding);
        buf.put_u16(self._padding2);
        for i in 0..self.supplies.len() {
            self.supplies[i].serialize(buf);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::ResupplyOffer {
            return Err(DISError::invalid_header(
                format!("Expected PDU type ResupplyOffer, got {:?}", header.pdu_type),
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

impl ResupplyOfferPdu {
    /// Creates a new `ResupplyOfferPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `ResupplyOfferPdu`:
    /// ```
    /// use open_dis_rust::logistics::ResupplyOfferPdu;
    /// let pdu = ResupplyOfferPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::ResupplyOffer;
        pdu.pdu_header.protocol_family = ProtocolFamily::Logistics;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let receiving_entity_id = EntityId::deserialize(buf);
        let supplying_entity_id = EntityId::deserialize(buf);
        let number_of_supply_types = buf.get_u8();
        let _padding = buf.get_u8();
        let _padding2 = buf.get_u16();
        let mut supplies: Vec<SupplyQuantity> = vec![];
        for _i in 0..number_of_supply_types {
            supplies.push(SupplyQuantity::deserialize(buf));
        }

        ResupplyOfferPdu {
            pdu_header: PduHeader::default(),
            receiving_entity_id,
            supplying_entity_id,
            number_of_supply_types,
            _padding,
            _padding2,
            supplies,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ResupplyOfferPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let pdu = ResupplyOfferPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<ResupplyOfferPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = ResupplyOfferPdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = ResupplyOfferPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
        let pdu = ResupplyOfferPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
