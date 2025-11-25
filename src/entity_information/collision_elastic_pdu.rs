//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    SerializedLength,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_coordinate_vector::EntityCoordinateVector,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily},
    event_id::EventId,
    linear_velocity::LinearVelocity,
    pdu::Pdu,
    pdu_header::PduHeader,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.2.4
pub struct CollisionElasticPdu {
    pdu_header: PduHeader,
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

impl Default for CollisionElasticPdu {
    fn default() -> Self {
        CollisionElasticPdu {
            pdu_header: PduHeader::default(),
            issuing_entity_id: EntityId::default(),
            colliding_entity_id: EntityId::default(),
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
    fn length(&self) -> Result<u16, DISError> {
        let length = PduHeader::LENGTH
            + EntityId::LENGTH * 2
            + EventId::LENGTH
            + LinearVelocity::LENGTH
            + EntityCoordinateVector::LENGTH * 2
            + std::mem::size_of::<u16>()
            + std::mem::size_of::<f32>() * 8;

        u16::try_from(length).map_err(|_| DISError::PduSizeExceeded {
            size: length,
            max_size: MAX_PDU_SIZE_OCTETS,
        })
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
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::CollisionElastic {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type CollisionElastic, got {:?}",
                    header.pdu_type
                ),
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

impl CollisionElasticPdu {
    #[must_use]
    /// Creates a new `CollisionElasticPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `CollisionElasticPdu`:
    /// ```
    /// use open_dis_rust::entity_information::CollisionElasticPdu;
    /// let pdu = CollisionElasticPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::CollisionElastic;
        pdu.pdu_header.protocol_family = ProtocolFamily::EntityInformation;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let issuing_entity_id = EntityId::deserialize(buf);
        let colliding_entity_id = EntityId::deserialize(buf);
        let event_id = EventId::deserialize(buf);
        let padding = buf.get_u16();
        let contact_velocity = LinearVelocity::deserialize(buf);
        let mass = buf.get_f32();
        let location_of_impact = EntityCoordinateVector::deserialize(buf);
        let collision_intermediate_result_xx = buf.get_f32();
        let collision_intermediate_result_xy = buf.get_f32();
        let collision_intermediate_result_xz = buf.get_f32();
        let collision_intermediate_result_yy = buf.get_f32();
        let collision_intermediate_result_yz = buf.get_f32();
        let collision_intermediate_result_zz = buf.get_f32();
        let unit_surface_normal = EntityCoordinateVector::deserialize(buf);
        let coefficient_of_restitution = buf.get_f32();
        CollisionElasticPdu {
            pdu_header: PduHeader::default(),
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
        let new_pdu = CollisionElasticPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 800 / BITS_PER_BYTE;
        let pdu = CollisionElasticPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
