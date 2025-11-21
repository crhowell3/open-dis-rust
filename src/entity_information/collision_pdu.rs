//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    EntityCoordinateVector, LinearVelocity, SerializedLength,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily},
    event_id::EventId,
    pdu::Pdu,
    pdu_header::PduHeader,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.2.3
pub struct CollisionPdu {
    pdu_header: PduHeader,
    pub issuing_entity_id: EntityId,
    pub colliding_entity_id: EntityId,
    pub event_id: EventId,
    pub collision_type: u8,
    pub padding: u8,
    pub velocity: LinearVelocity,
    pub mass: f32,
    pub location_wrt_entity: EntityCoordinateVector,
}

impl Default for CollisionPdu {
    fn default() -> Self {
        CollisionPdu {
            pdu_header: PduHeader::default(),
            issuing_entity_id: EntityId::default(1),
            colliding_entity_id: EntityId::default(2),
            event_id: EventId::default(1),
            collision_type: 0,
            padding: 0,
            velocity: LinearVelocity::default(),
            mass: 0.0,
            location_wrt_entity: EntityCoordinateVector::default(),
        }
    }
}

impl Pdu for CollisionPdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH + EntityId::LENGTH * 2;

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let size = std::mem::size_of_val(self);
        self.pdu_header.length = u16::try_from(size).map_err(|_| DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })?;
        self.pdu_header.serialize(buf);
        self.issuing_entity_id.serialize(buf);
        self.colliding_entity_id.serialize(buf);
        self.event_id.serialize(buf);
        buf.put_u8(self.collision_type);
        buf.put_u8(self.padding);
        self.velocity.serialize(buf);
        buf.put_f32(self.mass);
        self.location_wrt_entity.serialize(buf);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::Collision {
            return Err(DISError::invalid_header(
                format!("Expected PDU type Collision, got {:?}", header.pdu_type),
                None,
            ));
        }
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }

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

impl CollisionPdu {
    /// Creates a new `CollisionPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `CollisionPdu`:
    /// ```
    /// use open_dis_rust::entity_information::CollisionPdu;
    /// let pdu = CollisionPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::Collision;
        pdu.pdu_header.protocol_family = ProtocolFamily::EntityInformation;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let issuing_entity_id = EntityId::deserialize(buf);
        let colliding_entity_id = EntityId::deserialize(buf);
        let event_id = EventId::deserialize(buf);
        let collision_type = buf.get_u8();
        let padding = buf.get_u8();
        let velocity = LinearVelocity::deserialize(buf);
        let mass = buf.get_f32();
        let location_wrt_entity = EntityCoordinateVector::deserialize(buf);
        CollisionPdu {
            pdu_header: PduHeader::default(),
            issuing_entity_id,
            colliding_entity_id,
            event_id,
            collision_type,
            padding,
            velocity,
            mass,
            location_wrt_entity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CollisionPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

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
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = CollisionPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / 8;
        let pdu = CollisionPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
