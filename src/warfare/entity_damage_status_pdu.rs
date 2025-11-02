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
    pub padding1: u16,
    pub padding2: u16,
    pub number_of_damage_descriptions: u16,
    pub damage_descriptions: Vec<DirectedEnergyDamage>,
}

impl Default for EntityDamageStatusPdu {
    /// Creates a default Entity Damage Status PDU with arbitrary firing entity ID and target entity ID
    ///
    /// # Examples
    ///
    /// Initializing an Entity Damage Status PDU:
    /// ```
    /// use open_dis_rust::warfare::entity_damage_status_pdu::EntityDamageStatusPdu;
    /// let entity_damage_status_pdu = EntityDamageStatusPdu::default();
    /// ```
    ///
    fn default() -> Self {
        EntityDamageStatusPdu {
            pdu_header: PduHeader::default(
                PduType::EntityDamageStatus,
                ProtocolFamily::Warfare,
                56,
            ),
            firing_entity_id: EntityId::default(1),
            target_entity_id: EntityId::default(2),
            damaged_entity_id: EntityId::default(3),
            padding1: 0,
            padding2: 0,
            number_of_damage_descriptions: 0,
            damage_descriptions: vec![],
        }
    }
}

impl Pdu for EntityDamageStatusPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.firing_entity_id.serialize(buf);
        self.target_entity_id.serialize(buf);
        self.damaged_entity_id.serialize(buf);
        buf.put_u16(self.padding1);
        buf.put_u16(self.padding2);
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
            let padding1 = buffer.get_u16();
            let padding2 = buffer.get_u16();
            let number_of_damage_descriptions = buffer.get_u16();
            let damage_descriptions: Vec<DirectedEnergyDamage> = vec![];
            Ok(EntityDamageStatusPdu {
                pdu_header,
                firing_entity_id,
                target_entity_id,
                damaged_entity_id,
                padding1,
                padding2,
                number_of_damage_descriptions,
                damage_descriptions,
            })
        } else {
            Err(DISError::InvalidDISHeader)
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
        let padding1 = buffer.get_u16();
        let padding2 = buffer.get_u16();
        let number_of_damage_descriptions = buffer.get_u16();
        let damage_descriptions: Vec<DirectedEnergyDamage> = vec![];
        Ok(EntityDamageStatusPdu {
            pdu_header,
            firing_entity_id,
            target_entity_id,
            damaged_entity_id,
            padding1,
            padding2,
            number_of_damage_descriptions,
            damage_descriptions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::EntityDamageStatusPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let entity_damage_status_pdu = EntityDamageStatusPdu::default();
        let pdu_header = PduHeader::default(
            PduType::EntityDamageStatus,
            ProtocolFamily::Warfare,
            448 / 8,
        );

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
            pdu_header.padding,
            entity_damage_status_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let mut entity_damage_status_pdu = EntityDamageStatusPdu::default();
        let mut buffer = BytesMut::new();
        entity_damage_status_pdu.serialize(&mut buffer);

        let new_entity_damage_status_pdu = EntityDamageStatusPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_entity_damage_status_pdu.pdu_header,
            entity_damage_status_pdu.pdu_header
        );
    }
}
