//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{EntityCoordinateVector, EntityId, EventId, LinearVelocity},
        enums::{PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.2.3
    pub struct CollisionPdu {
        header: PduHeader,
        pdu_type: PduType::Collision,
        protocol_family: ProtocolFamily::EntityInformation,
        fields: {
            pub issuing_entity_id: EntityId,
            pub colliding_entity_id: EntityId,
            pub event_id: EventId,
            pub collision_type: u8,
            padding: u8,
            pub velocity: LinearVelocity,
            pub mass: f32,
            pub location_wrt_entity: EntityCoordinateVector,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CollisionPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = CollisionPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<CollisionPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = CollisionPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = CollisionPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
        assert_eq!(new_pdu.header, pdu.header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 480 / BITS_PER_BYTE;
        let pdu = CollisionPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
