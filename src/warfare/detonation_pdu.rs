//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    EntityCoordinateVector, LinearVelocity, WorldCoordinate,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{DetonationResult, PduType, ProtocolFamily},
    event_id::EventId,
    pdu::Pdu,
    pdu_header::PduHeader,
};

use super::data_types::{
    munition_descriptor::MunitionDescriptor, variable_parameter::VariableParameter,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.3.3
pub struct DetonationPdu {
    pdu_header: PduHeader,
    pub firing_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub exploding_entity_id: EntityId,
    pub event_id: EventId,
    pub velocity: LinearVelocity,
    pub location_in_world_coordinates: WorldCoordinate,
    pub descriptor: MunitionDescriptor,
    pub location_in_entitys_coordinates: EntityCoordinateVector,
    pub detonation_result: DetonationResult,
    pub number_of_variable_parameters: u8,
    padding: u16,
    pub variable_parameters: Vec<VariableParameter>,
}

impl Default for DetonationPdu {
    fn default() -> Self {
        DetonationPdu {
            pdu_header: PduHeader::default(),
            firing_entity_id: EntityId::default(),
            target_entity_id: EntityId::default(),
            exploding_entity_id: EntityId::default(),
            event_id: EventId::default(1),
            velocity: LinearVelocity::default(),
            location_in_world_coordinates: WorldCoordinate::default(),
            descriptor: MunitionDescriptor::default(),
            location_in_entitys_coordinates: EntityCoordinateVector::default(),
            detonation_result: DetonationResult::default(),
            number_of_variable_parameters: 0,
            padding: 0u16,
            variable_parameters: vec![],
        }
    }
}

impl Pdu for DetonationPdu {
    fn length(&self) -> Result<u16, DISError> {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>() * 3
            + std::mem::size_of::<EventId>()
            + std::mem::size_of::<LinearVelocity>()
            + std::mem::size_of::<WorldCoordinate>()
            + std::mem::size_of::<MunitionDescriptor>()
            + std::mem::size_of::<EntityCoordinateVector>()
            + std::mem::size_of::<DetonationResult>()
            + std::mem::size_of::<u8>()
            + std::mem::size_of::<u16>();

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
        self.firing_entity_id.serialize(buf);
        self.target_entity_id.serialize(buf);
        self.exploding_entity_id.serialize(buf);
        self.event_id.serialize(buf);
        self.velocity.serialize(buf);
        self.location_in_world_coordinates.serialize(buf);
        self.descriptor.serialize(buf);
        self.location_in_entitys_coordinates.serialize(buf);
        buf.put_u8(self.detonation_result as u8);
        buf.put_u8(self.number_of_variable_parameters);
        buf.put_u16(self.padding);
        for i in 0..self.variable_parameters.len() {
            self.variable_parameters[i].serialize(buf);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::Detonation {
            return Err(DISError::invalid_header(
                format!("Expected PDU type Detonation, got {:?}", header.pdu_type),
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

impl DetonationPdu {
    #[must_use]
    /// Creates a new Entity Damage Status PDU
    ///
    /// # Examples
    ///
    /// Initializing an Entity Damage Status PDU:
    /// ```
    /// use open_dis_rust::warfare::DetonationPdu;
    /// let pdu = DetonationPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::Detonation;
        pdu.pdu_header.protocol_family = ProtocolFamily::Warfare;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let firing_entity_id = EntityId::deserialize(buf);
        let target_entity_id = EntityId::deserialize(buf);
        let exploding_entity_id = EntityId::deserialize(buf);
        let event_id = EventId::deserialize(buf);
        let velocity = LinearVelocity::deserialize(buf);
        let location_in_world_coordinates = WorldCoordinate::deserialize(buf);
        let descriptor = MunitionDescriptor::deserialize(buf);
        let location_in_entitys_coordinates = EntityCoordinateVector::deserialize(buf);
        let detonation_result = DetonationResult::deserialize(buf);
        let number_of_variable_parameters = buf.get_u8();
        let padding = buf.get_u16();
        let mut variable_parameters: Vec<VariableParameter> = vec![];
        for _ in 0..number_of_variable_parameters {
            variable_parameters.push(VariableParameter::deserialize(buf));
        }

        DetonationPdu {
            pdu_header: PduHeader::default(),
            firing_entity_id,
            target_entity_id,
            exploding_entity_id,
            event_id,
            velocity,
            location_in_world_coordinates,
            descriptor,
            location_in_entitys_coordinates,
            detonation_result,
            number_of_variable_parameters,
            padding,
            variable_parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DetonationPdu;
    use crate::common::pdu::Pdu;
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let detonation_pdu = DetonationPdu::new();
        let any_pdu = detonation_pdu.as_any();

        assert!(any_pdu.is::<DetonationPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = DetonationPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = DetonationPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 832 / 8;
        let pdu = DetonationPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
