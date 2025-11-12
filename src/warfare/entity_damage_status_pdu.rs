//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

use super::data_types::directed_energy_damage::DirectedEnergyDamage;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.3.5
pub struct EntityDamageStatusPdu {
    pub pdu_header: PduHeader,
    pub firing_entity_id: EntityId,
    pub target_entity_id: EntityId,
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

impl Pdu for EntityDamageStatusPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>() * 3
            + std::mem::size_of::<u16>() * 3;

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
        self.firing_entity_id.serialize(buf);
        self.target_entity_id.serialize(buf);
        self.damaged_entity_id.serialize(buf);
        buf.put_u16(self._padding);
        buf.put_u16(self._padding2);
        buf.put_u16(self.number_of_damage_descriptions);
        for i in 0..self.damage_descriptions.len() {
            self.damage_descriptions[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::EntityDamageStatus {
            let firing_entity_id = EntityId::deserialize(&mut buffer);
            let target_entity_id = EntityId::deserialize(&mut buffer);
            let damaged_entity_id = EntityId::deserialize(&mut buffer);
            let _padding = buffer.get_u16();
            let _padding2 = buffer.get_u16();
            let number_of_damage_descriptions = buffer.get_u16();
            let mut damage_descriptions: Vec<DirectedEnergyDamage> = vec![];
            for _ in 0..number_of_damage_descriptions {
                damage_descriptions.push(DirectedEnergyDamage::deserialize(&mut buffer));
            }

            Ok(EntityDamageStatusPdu {
                pdu_header,
                firing_entity_id,
                target_entity_id,
                damaged_entity_id,
                _padding,
                _padding2,
                number_of_damage_descriptions,
                damage_descriptions,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type EntityDamageStatus, got {:?}",
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
        let firing_entity_id = EntityId::deserialize(&mut buffer);
        let target_entity_id = EntityId::deserialize(&mut buffer);
        let damaged_entity_id = EntityId::deserialize(&mut buffer);
        let _padding = buffer.get_u16();
        let _padding2 = buffer.get_u16();
        let number_of_damage_descriptions = buffer.get_u16();
        let mut damage_descriptions: Vec<DirectedEnergyDamage> = vec![];
        for _ in 0..number_of_damage_descriptions {
            damage_descriptions.push(DirectedEnergyDamage::deserialize(&mut buffer));
        }
        Ok(EntityDamageStatusPdu {
            pdu_header,
            firing_entity_id,
            target_entity_id,
            damaged_entity_id,
            _padding,
            _padding2,
            number_of_damage_descriptions,
            damage_descriptions,
        })
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
    /// let entity_damage_status_pdu = EntityDamageStatusPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::EntityDamageStatus;
        pdu.pdu_header.protocol_family = ProtocolFamily::Warfare;
        pdu.finalize();
        pdu
    }
}

#[cfg(test)]
mod tests {
    use std::any::type_name_of_val;

    use super::EntityDamageStatusPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let entity_damage_status_pdu = EntityDamageStatusPdu::new();
        let pdu_header = PduHeader::default();

        assert_eq!(
            pdu_header.protocol_version,
            entity_damage_status_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            entity_damage_status_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            entity_damage_status_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            entity_damage_status_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            entity_damage_status_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.status_record,
            entity_damage_status_pdu.pdu_header.status_record
        );
    }

    #[test]
    fn cast_to_any() {
        let entity_damage_status_pdu = EntityDamageStatusPdu::new();
        let any_pdu = entity_damage_status_pdu.as_any();

        assert!(any_pdu.is::<EntityDamageStatusPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut entity_damage_status_pdu = EntityDamageStatusPdu::new();
        let mut buffer = BytesMut::new();
        entity_damage_status_pdu.serialize(&mut buffer);

        let new_entity_damage_status_pdu = EntityDamageStatusPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_entity_damage_status_pdu.pdu_header,
            entity_damage_status_pdu.pdu_header
        );
    }

    #[test]
    fn create_new_pdu() {
        let entity_damage_status_pdu = EntityDamageStatusPdu::new();
        assert!(type_name_of_val(&entity_damage_status_pdu).contains("EntityDamageStatusPdu"));
        assert_eq!(
            entity_damage_status_pdu.header().pdu_type,
            PduType::EntityDamageStatus
        );
    }
}
