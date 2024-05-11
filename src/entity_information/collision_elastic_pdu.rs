//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_coordinate_vector::EntityCoordinateVector,
    entity_id::EntityId,
    event_id::EventId,
    linear_velocity::LinearVelocity,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Clone, Debug)]
pub struct CollisionElasticPdu {
    pub pdu_header: PduHeader,
    pub issuing_entity_id: EntityId,
    pub colliding_entity_id: EntityId,
    pub event_id: EventId,
    pub padding: u16,
    pub contact_velocity: LinearVelocity,
    pub mass: f32,
    pub location_of_impact: EntityCoordinateVector,
    pub collision_intermediate_result_xx: f32,
    pub collision_intermediate_result_xy: f32,
    pub collision_intermediate_result_xz: f32,
    pub collision_intermediate_result_yy: f32,
    pub collision_intermediate_result_yz: f32,
    pub collision_intermediate_result_zz: f32,
    pub unit_surface_normal: EntityCoordinateVector,
    pub coefficient_of_restitution: f32,
}

impl Default for CollisionElasticPdu {
    fn default() -> Self {
        CollisionElasticPdu {
            pdu_header: PduHeader::default(
                PduType::CollisionElastic,
                ProtocolFamily::EntityInformation,
                100,
            ),
            issuing_entity_id: EntityId::default(1),
            colliding_entity_id: EntityId::default(2),
            event_id: EventId::default(1),
            padding: 0,
            contact_velocity: LinearVelocity::default(),
            mass: 0.0,
            location_of_impact: EntityCoordinateVector::default(),
            collision_intermediate_result_xx: 0.0,
            collision_intermediate_result_xy: 0.0,
            collision_intermediate_result_xz: 0.0,
            collision_intermediate_result_yy: 0.0,
            collision_intermediate_result_yz: 0.0,
            collision_intermediate_result_zz: 0.0,
            unit_surface_normal: EntityCoordinateVector::default(),
            coefficient_of_restitution: 0.0,
        }
    }
}

impl Pdu for CollisionElasticPdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.issuing_entity_id.serialize(buf);
        self.colliding_entity_id.serialize(buf);
        self.event_id.serialize(buf);
        buf.put_u16(self.padding);
        self.contact_velocity.serialize(buf);
        buf.put_f32(self.mass);
        self.location_of_impact.serialize(buf);
        buf.put_f32(self.collision_intermediate_result_xx);
        buf.put_f32(self.collision_intermediate_result_xy);
        buf.put_f32(self.collision_intermediate_result_xz);
        buf.put_f32(self.collision_intermediate_result_yy);
        buf.put_f32(self.collision_intermediate_result_yz);
        buf.put_f32(self.collision_intermediate_result_zz);
        self.unit_surface_normal.serialize(buf);
        buf.put_f32(self.coefficient_of_restitution);
    }

    #[allow(clippy::similar_names)]
    fn deserialize(mut buffer: BytesMut) -> Result<CollisionElasticPdu, DISError> {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::CollisionElastic {
            let issuing_entity_id = EntityId::decode(&mut buffer);
            let colliding_entity_id = EntityId::decode(&mut buffer);
            let event_id = EventId::decode(&mut buffer);
            let padding = buffer.get_u16();
            let contact_velocity = LinearVelocity::decode(&mut buffer);
            let mass = buffer.get_f32();
            let location_of_impact = EntityCoordinateVector::decode(&mut buffer);
            let collision_intermediate_result_xx = buffer.get_f32();
            let collision_intermediate_result_xy = buffer.get_f32();
            let collision_intermediate_result_xz = buffer.get_f32();
            let collision_intermediate_result_yy = buffer.get_f32();
            let collision_intermediate_result_yz = buffer.get_f32();
            let collision_intermediate_result_zz = buffer.get_f32();
            let unit_surface_normal = EntityCoordinateVector::decode(&mut buffer);
            let coefficient_of_restitution = buffer.get_f32();
            Ok(CollisionElasticPdu {
                pdu_header,
                issuing_entity_id,
                colliding_entity_id,
                event_id,
                padding,
                contact_velocity,
                mass,
                location_of_impact,
                collision_intermediate_result_xx,
                collision_intermediate_result_xy,
                collision_intermediate_result_xz,
                collision_intermediate_result_yy,
                collision_intermediate_result_yz,
                collision_intermediate_result_zz,
                unit_surface_normal,
                coefficient_of_restitution,
            })
        } else {
            Err(DISError::InvalidDISHeader)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    #[allow(clippy::similar_names)]
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
        let padding = buffer.get_u16();
        let contact_velocity = LinearVelocity::decode(&mut buffer);
        let mass = buffer.get_f32();
        let location_of_impact = EntityCoordinateVector::decode(&mut buffer);
        let collision_intermediate_result_xx = buffer.get_f32();
        let collision_intermediate_result_xy = buffer.get_f32();
        let collision_intermediate_result_xz = buffer.get_f32();
        let collision_intermediate_result_yy = buffer.get_f32();
        let collision_intermediate_result_yz = buffer.get_f32();
        let collision_intermediate_result_zz = buffer.get_f32();
        let unit_surface_normal = EntityCoordinateVector::decode(&mut buffer);
        let coefficient_of_restitution = buffer.get_f32();
        Ok(CollisionElasticPdu {
            pdu_header,
            issuing_entity_id,
            colliding_entity_id,
            event_id,
            padding,
            contact_velocity,
            mass,
            location_of_impact,
            collision_intermediate_result_xx,
            collision_intermediate_result_xy,
            collision_intermediate_result_xz,
            collision_intermediate_result_yy,
            collision_intermediate_result_yz,
            collision_intermediate_result_zz,
            unit_surface_normal,
            coefficient_of_restitution,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::CollisionElasticPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let collision_elastic_pdu = CollisionElasticPdu::default();
        let header = PduHeader::default(
            PduType::CollisionElastic,
            ProtocolFamily::EntityInformation,
            100,
        );
        assert_eq!(
            header.protocol_version,
            collision_elastic_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            header.exercise_id,
            collision_elastic_pdu.pdu_header.exercise_id
        );
        assert_eq!(header.pdu_type, collision_elastic_pdu.pdu_header.pdu_type);
        assert_eq!(
            header.protocol_family,
            collision_elastic_pdu.pdu_header.protocol_family
        );
        assert_eq!(header.length, collision_elastic_pdu.pdu_header.length);
        assert_eq!(header.padding, collision_elastic_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let collision_elastic_pdu = CollisionElasticPdu::default();
        let mut buffer = BytesMut::new();
        collision_elastic_pdu.serialize(&mut buffer);

        let new_collision_elastic_pdu = CollisionElasticPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_collision_elastic_pdu.pdu_header,
            collision_elastic_pdu.pdu_header
        );
    }
}
