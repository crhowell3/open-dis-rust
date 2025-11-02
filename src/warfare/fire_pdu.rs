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
    event_id::EventId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    vector3_double::Vector3Double,
    vector3_float::Vector3Float,
};

use super::data_types::munition_descriptor::MunitionDescriptor;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.3.2
pub struct FirePdu {
    pub pdu_header: PduHeader,
    pub firing_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub munition_expendable_id: EntityId,
    pub event_id: EventId,
    pub fire_mission_index: u32,
    pub location_in_world_coordinates: Vector3Double,
    pub descriptor: MunitionDescriptor,
    pub velocity: Vector3Float,
    pub range: f32,
}

impl Default for FirePdu {
    /// Creates a default Fire PDU with arbitrary firing entity ID and target entity ID
    ///
    /// # Examples
    ///
    /// Initializing a Fire PDU:
    /// ```
    /// use open_dis_rust::warfare::fire_pdu::FirePdu;
    /// let fire_pdu = FirePdu::default();
    /// ```
    ///
    fn default() -> Self {
        FirePdu {
            pdu_header: PduHeader::default(PduType::Fire, ProtocolFamily::Warfare, 56),
            firing_entity_id: EntityId::default(1),
            target_entity_id: EntityId::default(2),
            munition_expendable_id: EntityId::default(3),
            event_id: EventId::default(1),
            fire_mission_index: 0,
            location_in_world_coordinates: Vector3Double::default(),
            descriptor: MunitionDescriptor::default(),
            velocity: Vector3Float::default(),
            range: 0.0,
        }
    }
}

impl Pdu for FirePdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.firing_entity_id.serialize(buf);
        self.target_entity_id.serialize(buf);
        self.munition_expendable_id.serialize(buf);
        self.event_id.serialize(buf);
        buf.put_u32(self.fire_mission_index);
        self.location_in_world_coordinates.serialize(buf);
        self.descriptor.serialize(buf);
        self.velocity.serialize(buf);
        buf.put_f32(self.range);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::Fire {
            let firing_entity_id = EntityId::deserialize(&mut buffer);
            let target_entity_id = EntityId::deserialize(&mut buffer);
            let munition_expendable_id = EntityId::deserialize(&mut buffer);
            let event_id = EventId::deserialize(&mut buffer);
            let fire_mission_index = buffer.get_u32();
            let location_in_world_coordinates = Vector3Double::deserialize(&mut buffer);
            let descriptor = MunitionDescriptor::deserialize(&mut buffer);
            let velocity = Vector3Float::deserialize(&mut buffer);
            let range = buffer.get_f32();
            Ok(FirePdu {
                pdu_header,
                firing_entity_id,
                target_entity_id,
                munition_expendable_id,
                event_id,
                fire_mission_index,
                location_in_world_coordinates,
                descriptor,
                velocity,
                range,
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
        let munition_expendable_id = EntityId::deserialize(&mut buffer);
        let event_id = EventId::deserialize(&mut buffer);
        let fire_mission_index = buffer.get_u32();
        let location_in_world_coordinates = Vector3Double::deserialize(&mut buffer);
        let descriptor = MunitionDescriptor::deserialize(&mut buffer);
        let velocity = Vector3Float::deserialize(&mut buffer);
        let range = buffer.get_f32();
        Ok(FirePdu {
            pdu_header,
            firing_entity_id,
            target_entity_id,
            munition_expendable_id,
            event_id,
            fire_mission_index,
            location_in_world_coordinates,
            descriptor,
            velocity,
            range,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::FirePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let fire_pdu = FirePdu::default();
        let pdu_header = PduHeader::default(PduType::Fire, ProtocolFamily::Warfare, 448 / 8);

        assert_eq!(
            pdu_header.protocol_version,
            fire_pdu.pdu_header.protocol_version
        );
        assert_eq!(pdu_header.exercise_id, fire_pdu.pdu_header.exercise_id);
        assert_eq!(pdu_header.pdu_type, fire_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            fire_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, fire_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, fire_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut fire_pdu = FirePdu::default();
        let mut buffer = BytesMut::new();
        fire_pdu.serialize(&mut buffer);

        let new_fire_pdu = FirePdu::deserialize(buffer).unwrap();
        assert_eq!(new_fire_pdu.pdu_header, fire_pdu.pdu_header);
    }
}
