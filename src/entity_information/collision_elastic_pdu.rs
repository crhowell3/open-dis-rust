//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#![allow(clippy::similar_names)]

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            entity_coordinate_vector::EntityCoordinateVector, entity_id::EntityId,
            event_id::EventId, linear_velocity::LinearVelocity,
        },
        enums::{PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.2.4
    pub struct CollisionElasticPdu {
        header: PduHeader,
        pdu_type: PduType::CollisionElastic,
        protocol_family: ProtocolFamily::EntityInformation,
        fields: {
            pub issuing_entity_id: EntityId,
            pub colliding_entity_id: EntityId,
            pub event_id: EventId,
            padding: u16,
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
    }
}
#[cfg(test)]
mod tests {
    use super::CollisionElasticPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = CollisionElasticPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<CollisionElasticPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = CollisionElasticPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = CollisionElasticPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
        assert_eq!(new_pdu.header, pdu.header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 800 / BITS_PER_BYTE;
        let pdu = CollisionElasticPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
