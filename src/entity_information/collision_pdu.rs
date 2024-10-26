//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
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
    vector3_float::Vector3Float,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.2.3
pub struct CollisionPdu {
    pub pdu_header: PduHeader,
    pub issuing_entity_id: EntityId,
    pub colliding_entity_id: EntityId,
    pub event_id: EventId,
    pub collision_type: u8,
    pub padding: u8,
    pub velocity: Vector3Float,
    pub mass: f32,
    pub location_wrt_entity: Vector3Float,
}

impl Default for CollisionPdu {
    fn default() -> Self {
        CollisionPdu {
            pdu_header: PduHeader::default(
                PduType::Collision,
                ProtocolFamily::EntityInformation,
                60,
            ),
            issuing_entity_id: EntityId::default(1),
            colliding_entity_id: EntityId::default(2),
            event_id: EventId::default(1),
            collision_type: 0,
            padding: 0,
            velocity: Vector3Float::default(),
            mass: 0.0,
            location_wrt_entity: Vector3Float::default(),
        }
    }
}

impl Pdu for CollisionPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.issuing_entity_id.serialize(buf);
        self.colliding_entity_id.serialize(buf);
        self.event_id.serialize(buf);
        buf.put_u8(self.collision_type);
        buf.put_u8(self.padding);
        self.velocity.serialize(buf);
        buf.put_f32(self.mass);
        self.location_wrt_entity.serialize(buf);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<CollisionPdu, DISError> {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::Collision {
            let issuing_entity_id = EntityId::decode(&mut buffer);
            let colliding_entity_id = EntityId::decode(&mut buffer);
            let event_id = EventId::decode(&mut buffer);
            let collision_type = buffer.get_u8();
            let padding = buffer.get_u8();
            let velocity = Vector3Float::decode(&mut buffer);
            let mass = buffer.get_f32();
            let location_wrt_entity = Vector3Float::decode(&mut buffer);
            Ok(CollisionPdu {
                pdu_header,
                issuing_entity_id,
                colliding_entity_id,
                event_id,
                collision_type,
                padding,
                velocity,
                mass,
                location_wrt_entity,
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
        let issuing_entity_id = EntityId::decode(&mut buffer);
        let colliding_entity_id = EntityId::decode(&mut buffer);
        let event_id = EventId::decode(&mut buffer);
        let collision_type = buffer.get_u8();
        let padding = buffer.get_u8();
        let velocity = Vector3Float::decode(&mut buffer);
        let mass = buffer.get_f32();
        let location_wrt_entity = Vector3Float::decode(&mut buffer);
        Ok(CollisionPdu {
            pdu_header,
            issuing_entity_id,
            colliding_entity_id,
            event_id,
            collision_type,
            padding,
            velocity,
            mass,
            location_wrt_entity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::CollisionPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let collision_pdu = CollisionPdu::default();
        let header = PduHeader::default(PduType::Collision, ProtocolFamily::EntityInformation, 60);
        assert_eq!(
            header.protocol_version,
            collision_pdu.pdu_header.protocol_version
        );
        assert_eq!(header.exercise_id, collision_pdu.pdu_header.exercise_id);
        assert_eq!(header.pdu_type, collision_pdu.pdu_header.pdu_type);
        assert_eq!(
            header.protocol_family,
            collision_pdu.pdu_header.protocol_family
        );
        assert_eq!(header.length, collision_pdu.pdu_header.length);
        assert_eq!(header.padding, collision_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut collision_pdu = CollisionPdu::default();
        let mut buffer = BytesMut::new();
        collision_pdu.serialize(&mut buffer);

        let new_collision_pdu = CollisionPdu::deserialize(buffer).unwrap();
        assert_eq!(new_collision_pdu.pdu_header, collision_pdu.pdu_header);
    }
}
