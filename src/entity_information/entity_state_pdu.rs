//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    SerializedLength,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    entity_type::EntityType,
    enums::{EntityCapabilities, ForceId, PduType, ProtocolFamily},
    euler_angles::EulerAngles,
    linear_velocity::LinearVelocity,
    pdu::Pdu,
    pdu_header::PduHeader,
    world_coordinate::WorldCoordinate,
};

use super::data_types::{
    dead_reckoning_parameters::DeadReckoningParameters, entity_marking::EntityMarking,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.2.2
pub struct EntityStatePdu {
    pdu_header: PduHeader,
    pub entity_id: EntityId,
    pub force_id: ForceId,
    pub number_of_articulation_parameters: u8,
    pub entity_type: EntityType,
    pub alternative_entity_type: EntityType,
    pub entity_linear_velocity: LinearVelocity,
    pub entity_location: WorldCoordinate,
    pub entity_orientation: EulerAngles,
    pub entity_appearance: u32,
    pub dead_reckoning_parameters: DeadReckoningParameters,
    pub entity_marking: EntityMarking,
    pub entity_capabilities: EntityCapabilities,
    pub articulation_parameter: f32,
}

impl Default for EntityStatePdu {
    fn default() -> Self {
        EntityStatePdu {
            pdu_header: PduHeader::default(),
            entity_id: EntityId::default(),
            force_id: ForceId::default(),
            number_of_articulation_parameters: 0,
            entity_type: EntityType::default(),
            alternative_entity_type: EntityType::default(),
            entity_linear_velocity: LinearVelocity::new(0.0, 0.0, 0.0),
            entity_location: WorldCoordinate::new(0.0, 0.0, 0.0),
            entity_orientation: EulerAngles::new(0.0, 0.0, 0.0),
            entity_appearance: 0,
            dead_reckoning_parameters: DeadReckoningParameters::default(),
            entity_marking: EntityMarking::default(String::new()),
            entity_capabilities: EntityCapabilities::default(),
            articulation_parameter: 0.0,
        }
    }
}

impl Pdu for EntityStatePdu {
    fn length(&self) -> Result<u16, DISError> {
        let length = PduHeader::LENGTH
            + EntityId::LENGTH
            + EntityType::LENGTH * 2
            + LinearVelocity::LENGTH
            + WorldCoordinate::LENGTH
            + EulerAngles::LENGTH
            + DeadReckoningParameters::LENGTH
            + EntityMarking::LENGTH
            + std::mem::size_of::<u8>() * 2
            + std::mem::size_of::<u32>() * 2;

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
        self.entity_id.serialize(buf);
        buf.put_u8(self.force_id as u8);
        buf.put_u8(self.number_of_articulation_parameters);
        self.entity_type.serialize(buf);
        self.alternative_entity_type.serialize(buf);
        self.entity_linear_velocity.serialize(buf);
        self.entity_location.serialize(buf);
        self.entity_orientation.serialize(buf);
        buf.put_u32(self.entity_appearance);
        self.dead_reckoning_parameters.serialize(buf);
        self.entity_marking.serialize(buf);
        buf.put_u8(self.entity_capabilities as u8);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::EntityState {
            return Err(DISError::invalid_header(
                format!("Expected PDU type EntityState, got {:?}", header.pdu_type),
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

impl EntityStatePdu {
    #[must_use]
    /// Creates a new `EntityStatePdu`
    ///
    /// # Examples
    ///
    /// Initializing an `EntityStatePdu`:
    /// ```
    /// use open_dis_rust::entity_information::EntityStatePdu;
    /// let pdu = EntityStatePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::EntityState;
        pdu.pdu_header.protocol_family = ProtocolFamily::EntityInformation;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let entity_id = EntityId::deserialize(buf);
        let force_id = ForceId::deserialize(buf);
        let articulation_params = &buf.get_u8();
        let entity_type = EntityType::deserialize(buf);
        let alt_entity_type = EntityType::deserialize(buf);
        let linear_velocity = LinearVelocity::deserialize(buf);
        let world_coordinate = WorldCoordinate::deserialize(buf);
        let orientation = EulerAngles::deserialize(buf);
        let appearance = buf.get_u32();
        let dead_reckoning_parameters = DeadReckoningParameters::deserialize(buf);
        let entity_marking = EntityMarking::deserialize(buf);
        let entity_capabilities = EntityCapabilities::deserialize(buf);

        EntityStatePdu {
            pdu_header: PduHeader::default(),
            entity_id,
            force_id,
            number_of_articulation_parameters: *articulation_params,
            entity_type,
            alternative_entity_type: alt_entity_type,
            entity_linear_velocity: linear_velocity,
            entity_location: world_coordinate,
            entity_orientation: orientation,
            entity_appearance: appearance,
            dead_reckoning_parameters,
            entity_marking,
            entity_capabilities,
            articulation_parameter: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EntityStatePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = EntityStatePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<EntityStatePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = EntityStatePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = EntityStatePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 1152 / BITS_PER_BYTE;
        let pdu = EntityStatePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
