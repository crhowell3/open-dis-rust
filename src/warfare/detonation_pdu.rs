//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    EntityCoordinateVector, LinearVelocity, WorldCoordinate,
    dis_error::DISError,
    entity_id::EntityId,
    enums::DetonationResult,
    event_id::EventId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    vector3_double::Vector3Double,
    vector3_float::Vector3Float,
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
    _padding: u16,
    pub variable_parameters: Vec<VariableParameter>,
}

impl Default for DetonationPdu {
    /// Creates a default Detonation PDU with arbitrary firing entity ID and target entity ID
    ///
    /// # Examples
    ///
    /// Initializing a Detonation PDU:
    /// ```
    /// use open_dis_rust::warfare::detonation_pdu::DetonationPdu;
    /// let detonation_pdu = DetonationPdu::default();
    /// ```
    ///
    fn default() -> Self {
        DetonationPdu {
            pdu_header: PduHeader::default(),
            firing_entity_id: EntityId::default(1),
            target_entity_id: EntityId::default(2),
            exploding_entity_id: EntityId::default(3),
            event_id: EventId::default(1),
            velocity: LinearVelocity::default(),
            location_in_world_coordinates: WorldCoordinate::default(),
            descriptor: MunitionDescriptor::default(),
            location_in_entitys_coordinates: EntityCoordinateVector::default(),
            detonation_result: DetonationResult::default(),
            number_of_variable_parameters: 0,
            _padding: 0u16,
            variable_parameters: vec![],
        }
    }
}

impl Pdu for DetonationPdu {
    fn length(&self) -> u16 {
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

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
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
        buf.put_u16(self._padding);
        for i in 0..self.variable_parameters.len() {
            self.variable_parameters[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::Detonation {
            let firing_entity_id = EntityId::deserialize(&mut buffer);
            let target_entity_id = EntityId::deserialize(&mut buffer);
            let exploding_entity_id = EntityId::deserialize(&mut buffer);
            let event_id = EventId::deserialize(&mut buffer);
            let velocity = LinearVelocity::deserialize(&mut buffer);
            let location_in_world_coordinates = WorldCoordinate::deserialize(&mut buffer);
            let descriptor = MunitionDescriptor::deserialize(&mut buffer);
            let location_in_entitys_coordinates = EntityCoordinateVector::deserialize(&mut buffer);
            let detonation_result = DetonationResult::deserialize(&mut buffer);
            let number_of_variable_parameters = buffer.get_u8();
            let _padding = buffer.get_u16();
            let mut variable_parameters: Vec<VariableParameter> = vec![];
            for _ in 0..number_of_variable_parameters {
                variable_parameters.push(VariableParameter::deserialize(&mut buffer));
            }
            Ok(DetonationPdu {
                pdu_header,
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
                _padding,
                variable_parameters,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type Detonation, got {:?}",
                    pdu_header.pdu_type
                ),
                None,
            ))
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
        let exploding_entity_id = EntityId::deserialize(&mut buffer);
        let event_id = EventId::deserialize(&mut buffer);
        let velocity = LinearVelocity::deserialize(&mut buffer);
        let location_in_world_coordinates = WorldCoordinate::deserialize(&mut buffer);
        let descriptor = MunitionDescriptor::deserialize(&mut buffer);
        let location_in_entitys_coordinates = EntityCoordinateVector::deserialize(&mut buffer);
        let detonation_result = DetonationResult::deserialize(&mut buffer);
        let number_of_variable_parameters = buffer.get_u8();
        let _padding = buffer.get_u16();
        let mut variable_parameters: Vec<VariableParameter> = vec![];
        for _ in 0..number_of_variable_parameters {
            variable_parameters.push(VariableParameter::deserialize(&mut buffer));
        }
        Ok(DetonationPdu {
            pdu_header,
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
            _padding,
            variable_parameters,
        })
    }
}

impl DetonationPdu {
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
}

#[cfg(test)]
mod tests {
    use super::DetonationPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let detonation_pdu = DetonationPdu::new();
        let pdu_header = PduHeader::default();

        assert_eq!(
            pdu_header.protocol_version,
            detonation_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            detonation_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, detonation_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            detonation_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, detonation_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.status_record,
            detonation_pdu.pdu_header.status_record
        );
    }

    #[test]
    fn cast_to_any() {
        let detonation_pdu = DetonationPdu::new();
        let any_pdu = detonation_pdu.as_any();

        assert!(any_pdu.is::<DetonationPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut detonation_pdu = DetonationPdu::new();
        let mut buffer = BytesMut::new();
        detonation_pdu.serialize(&mut buffer);

        let new_detonation_pdu = DetonationPdu::deserialize(buffer).unwrap();
        assert_eq!(new_detonation_pdu.pdu_header, detonation_pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 832 / 8;
        let pdu = DetonationPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
