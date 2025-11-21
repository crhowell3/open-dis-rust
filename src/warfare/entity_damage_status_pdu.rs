//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
};

use super::data_types::directed_energy_damage::DirectedEnergyDamage;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.3.5
pub struct EntityDamageStatusPdu {
    pdu_header: PduHeader,
    pub damaged_entity_id: EntityId,
    _padding: u16,
    _padding2: u16,
    pub number_of_damage_descriptions: u16,
    pub damage_descriptions: Vec<DirectedEnergyDamage>,
}

impl Default for EntityDamageStatusPdu {
    fn default() -> Self {
        EntityDamageStatusPdu {
            pdu_header: PduHeader::default(),
            damaged_entity_id: EntityId::default(3),
            _padding: 0u16,
            _padding2: 0u16,
            number_of_damage_descriptions: 0,
            damage_descriptions: vec![],
        }
    }
}

impl Pdu for EntityDamageStatusPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>()
            + std::mem::size_of::<u16>() * 3;

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
        self.damaged_entity_id.serialize(buf);
        buf.put_u16(self._padding);
        buf.put_u16(self._padding2);
        buf.put_u16(self.number_of_damage_descriptions);
        for i in 0..self.damage_descriptions.len() {
            self.damage_descriptions[i].serialize(buf);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::EntityDamageStatus {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type EntityDamageStatus, got {:?}",
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

impl EntityDamageStatusPdu {
    /// Creates a new Entity Damage Status PDU
    ///
    /// # Examples
    ///
    /// Initializing an Entity Damage Status PDU:
    /// ```
    /// use open_dis_rust::warfare::EntityDamageStatusPdu;
    /// let pdu = EntityDamageStatusPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::EntityDamageStatus;
        pdu.pdu_header.protocol_family = ProtocolFamily::Warfare;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let damaged_entity_id = EntityId::deserialize(buf);
        let _padding = buf.get_u16();
        let _padding2 = buf.get_u16();
        let number_of_damage_descriptions = buf.get_u16();
        let mut damage_descriptions: Vec<DirectedEnergyDamage> = vec![];
        for _ in 0..number_of_damage_descriptions {
            damage_descriptions.push(DirectedEnergyDamage::deserialize(buf));
        }

        EntityDamageStatusPdu {
            pdu_header: PduHeader::default(),
            damaged_entity_id,
            _padding,
            _padding2,
            number_of_damage_descriptions,
            damage_descriptions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EntityDamageStatusPdu;
    use crate::common::pdu::Pdu;
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let entity_damage_status_pdu = EntityDamageStatusPdu::new();
        let any_pdu = entity_damage_status_pdu.as_any();

        assert!(any_pdu.is::<EntityDamageStatusPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = EntityDamageStatusPdu::default();
        let mut serialize_buffer = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buffer);

        let mut deserialize_buffer = Bytes::new();
        let new_pdu = EntityDamageStatusPdu::deserialize(&mut deserialize_buffer).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 192 / 8;
        let pdu = EntityDamageStatusPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
