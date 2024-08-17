//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

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
/// Implemented according to IEEE 1278.1-2012 §5.4.4
pub struct DirectedEnergyFirePdu {
    pub pdu_header: PduHeader,
    pub firing_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub damaged_entity_id: EntityId,
    pub padding1: u16,
    pub padding2: u16,
    pub number_of_damage_descriptions: u16,
    pub damage_descriptions: Vec<DirectedEnergyDamage>,
}

impl Default for DirectedEnergyFirePdu {
    /// Creates a default-initialized Directed Energy Fire PDU
    ///
    /// # Examples
    ///
    /// Initializing a Directed Energy Fire PDU:
    /// ```
    /// use open_dis_rust::warfare::directed_energy_fire_pdu::DirectedEnergyFirePdu;
    /// let directed_energy_fire_pdu = DirectedEnergyFirePdu::default();
    /// ```
    ///
    fn default() -> Self {
        DirectedEnergyFirePdu {
            pdu_header: PduHeader::default(
                PduType::DirectedEnergyFire,
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

impl Pdu for DirectedEnergyFirePdu {
    /// Serialize contents of DirectedEnergyFirePdu into BytesMut buffer
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

    /// Deserialize bytes from BytesMut buffer and interpret as DirectedEnergyFirePdu
    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::DirectedEnergyFire {
            let firing_entity_id = EntityId::decode(&mut buffer);
            let target_entity_id = EntityId::decode(&mut buffer);
            let damaged_entity_id = EntityId::decode(&mut buffer);
            let padding1 = buffer.get_u16();
            let padding2 = buffer.get_u16();
            let number_of_damage_descriptions = buffer.get_u16();
            let damage_descriptions: Vec<DirectedEnergyDamage> = vec![];
            Ok(DirectedEnergyFirePdu {
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

    /// Treat DirectedEnergyFirePdu as Any type
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
        let firing_entity_id = EntityId::decode(&mut buffer);
        let target_entity_id = EntityId::decode(&mut buffer);
        let damaged_entity_id = EntityId::decode(&mut buffer);
        let padding1 = buffer.get_u16();
        let padding2 = buffer.get_u16();
        let number_of_damage_descriptions = buffer.get_u16();
        let damage_descriptions: Vec<DirectedEnergyDamage> = vec![];
        Ok(DirectedEnergyFirePdu {
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
    use super::DirectedEnergyFirePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let directed_energy_fire_pdu = DirectedEnergyFirePdu::default();
        let pdu_header = PduHeader::default(
            PduType::DirectedEnergyFire,
            ProtocolFamily::Warfare,
            448 / 8,
        );

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
            pdu_header.padding,
            directed_energy_fire_pdu.pdu_header.padding
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
