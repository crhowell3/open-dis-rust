//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    ProtocolFamily,
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType},
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

    /// Serialize contents of `DirectedEnergyFirePdu` into `BytesMut` buffer
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

    /// Deserialize bytes from `BytesMut` buffer and interpret as `DirectedEnergyFirePdu`
    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::DirectedEnergyFire {
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

            Ok(DirectedEnergyFirePdu {
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
                    "Expected PDU type DirectedEnergyFire, got {:?}",
                    pdu_header.pdu_type
                ),
                None,
            ))
        }
    }

    /// Treat `DirectedEnergyFirePdu` as Any type
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Deserialize
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

        Ok(DirectedEnergyFirePdu {
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
}

#[cfg(test)]
mod tests {
    use super::DirectedEnergyFirePdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let directed_energy_fire_pdu = DirectedEnergyFirePdu::default();
        let pdu_header = PduHeader::default();

        assert_eq!(
            pdu_header.protocol_version,
            directed_energy_fire_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            directed_energy_fire_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            directed_energy_fire_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            directed_energy_fire_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            directed_energy_fire_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.status_record,
            directed_energy_fire_pdu.pdu_header.status_record
        );
    }

    #[test]
    fn cast_to_any() {
        let directed_energy_fire_pdu = DirectedEnergyFirePdu::default();
        let any_pdu = directed_energy_fire_pdu.as_any();

        assert!(any_pdu.is::<DirectedEnergyFirePdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut directed_energy_fire_pdu = DirectedEnergyFirePdu::default();
        let mut buffer = BytesMut::new();
        directed_energy_fire_pdu.serialize(&mut buffer);

        let new_directed_energy_fire_pdu = DirectedEnergyFirePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_directed_energy_fire_pdu.pdu_header,
            directed_energy_fire_pdu.pdu_header
        );
    }
}
