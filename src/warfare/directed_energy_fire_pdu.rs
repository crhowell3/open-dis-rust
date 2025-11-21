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

use super::data_types::directed_energy_damage::DirectedEnergyDamage;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.3.4
pub struct DirectedEnergyFirePdu {
    pdu_header: PduHeader,
    pub firing_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub damaged_entity_id: EntityId,
    _padding: u16,
    _padding2: u16,
    pub number_of_damage_descriptions: u16,
    pub damage_descriptions: Vec<DirectedEnergyDamage>,
}

impl Default for DirectedEnergyFirePdu {
    fn default() -> Self {
        DirectedEnergyFirePdu {
            pdu_header: PduHeader::default(),
            firing_entity_id: EntityId::default(1),
            target_entity_id: EntityId::default(2),
            damaged_entity_id: EntityId::default(3),
            _padding: 0u16,
            _padding2: 0u16,
            number_of_damage_descriptions: 0,
            damage_descriptions: vec![],
        }
    }
}

impl Pdu for DirectedEnergyFirePdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH + EntityId::LENGTH + 2 + 2 + 2;

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    /// Serialize contents of `DirectedEnergyFirePdu` into `BytesMut` buffer
    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let size = std::mem::size_of_val(self);
        self.pdu_header.length = u16::try_from(size).map_err(|_| DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })?;
        self.pdu_header.serialize(buf);
        self.firing_entity_id.serialize(buf);
        self.target_entity_id.serialize(buf);
        self.damaged_entity_id.serialize(buf);
        buf.put_u16(self._padding);
        buf.put_u16(self._padding2);
        buf.put_u16(self.number_of_damage_descriptions);
        for i in 0..self.damage_descriptions.len() {
            self.damage_descriptions[i].serialize(buf);
        }
        Ok(())
    }

    /// Deserialize bytes from `BytesMut` buffer and interpret as `DirectedEnergyFirePdu`
    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::DirectedEnergyFire {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type DirectedEnergyFire, got {:?}",
                    header.pdu_type
                ),
                None,
            ));
        }
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }

    /// Treat `DirectedEnergyFirePdu` as Any type
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

impl DirectedEnergyFirePdu {
    /// Creates a new Entity Damage Status PDU
    ///
    /// # Examples
    ///
    /// Initializing an Entity Damage Status PDU:
    /// ```
    /// use open_dis_rust::warfare::DirectedEnergyFirePdu;
    /// let pdu = DirectedEnergyFirePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::DirectedEnergyFire;
        pdu.pdu_header.protocol_family = ProtocolFamily::Warfare;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let firing_entity_id = EntityId::deserialize(buf);
        let target_entity_id = EntityId::deserialize(buf);
        let damaged_entity_id = EntityId::deserialize(buf);
        let _padding = buf.get_u16();
        let _padding2 = buf.get_u16();
        let number_of_damage_descriptions = buf.get_u16();
        let mut damage_descriptions: Vec<DirectedEnergyDamage> = vec![];
        for _ in 0..number_of_damage_descriptions {
            damage_descriptions.push(DirectedEnergyDamage::deserialize(buf));
        }

        DirectedEnergyFirePdu {
            pdu_header: PduHeader::default(),
            firing_entity_id,
            target_entity_id,
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
    use super::DirectedEnergyFirePdu;
    use crate::common::pdu::Pdu;
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let pdu = DirectedEnergyFirePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<DirectedEnergyFirePdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = DirectedEnergyFirePdu::new();
        let mut serialize_buffer = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buffer);

        let mut deserialize_buffer = Bytes::new();
        let new_pdu = DirectedEnergyFirePdu::deserialize(&mut deserialize_buffer).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 384 / 8;
        let pdu = DirectedEnergyFirePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
