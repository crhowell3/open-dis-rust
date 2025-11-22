//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    LinearVelocity, SerializedLength, WorldCoordinate,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily},
    event_id::EventId,
    pdu::Pdu,
    pdu_header::PduHeader,
};

use super::data_types::munition_descriptor::MunitionDescriptor;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.3.2
pub struct FirePdu {
    pdu_header: PduHeader,
    pub firing_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub munition_expendable_id: EntityId,
    pub event_id: EventId,
    pub fire_mission_index: u32,
    pub location_in_world_coordinates: WorldCoordinate,
    pub descriptor: MunitionDescriptor,
    pub velocity: LinearVelocity,
    pub range: f32,
}

impl Default for FirePdu {
    fn default() -> Self {
        FirePdu {
            pdu_header: PduHeader::default(),
            firing_entity_id: EntityId::default(),
            target_entity_id: EntityId::default(),
            munition_expendable_id: EntityId::default(),
            event_id: EventId::default(1),
            fire_mission_index: 0,
            location_in_world_coordinates: WorldCoordinate::default(),
            descriptor: MunitionDescriptor::default(),
            velocity: LinearVelocity::default(),
            range: 0.0,
        }
    }
}

impl Pdu for FirePdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH
            + EntityId::LENGTH * 3
            + std::mem::size_of::<EventId>()
            + std::mem::size_of::<u32>()
            + WorldCoordinate::LENGTH
            + std::mem::size_of::<MunitionDescriptor>()
            + std::mem::size_of::<LinearVelocity>()
            + std::mem::size_of::<f32>();

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
        self.firing_entity_id.serialize(buf);
        self.target_entity_id.serialize(buf);
        self.munition_expendable_id.serialize(buf);
        self.event_id.serialize(buf);
        buf.put_u32(self.fire_mission_index);
        self.location_in_world_coordinates.serialize(buf);
        self.descriptor.serialize(buf);
        self.velocity.serialize(buf);
        buf.put_f32(self.range);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::Fire {
            return Err(DISError::invalid_header(
                format!("Expected PDU type Fire, got {:?}", header.pdu_type),
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

impl FirePdu {
    /// Creates a new Fire PDU
    ///
    /// # Examples
    ///
    /// Initializing a Fire PDU:
    /// ```
    /// use open_dis_rust::warfare::FirePdu;
    /// let fire_pdu = FirePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::Fire;
        pdu.pdu_header.protocol_family = ProtocolFamily::Warfare;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let firing_entity_id = EntityId::deserialize(buf);
        let target_entity_id = EntityId::deserialize(buf);
        let munition_expendable_id = EntityId::deserialize(buf);
        let event_id = EventId::deserialize(buf);
        let fire_mission_index = buf.get_u32();
        let location_in_world_coordinates = WorldCoordinate::deserialize(buf);
        let descriptor = MunitionDescriptor::deserialize(buf);
        let velocity = LinearVelocity::deserialize(buf);
        let range = buf.get_f32();

        FirePdu {
            pdu_header: PduHeader::default(),
            firing_entity_id,
            target_entity_id,
            munition_expendable_id,
            event_id,
            fire_mission_index,
            location_in_world_coordinates,
            descriptor,
            velocity,
            range,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FirePdu;
    use crate::common::pdu::Pdu;
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let fire_pdu = FirePdu::new();
        let any_pdu = fire_pdu.as_any();

        assert!(any_pdu.is::<FirePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = FirePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = FirePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 768 / 8;
        let pdu = FirePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
