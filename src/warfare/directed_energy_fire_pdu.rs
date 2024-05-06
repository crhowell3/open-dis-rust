//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    clock_time::ClockTime,
    dis_error::DISError,
    entity_id::EntityId,
    entity_type::EntityType,
    event_id::EventId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    vector3_float::Vector3Float,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §5.4.4
pub struct DirectedEnergyFirePdu {
    pub pdu_header: PduHeader,
    pub firing_entity_id: EntityId,
    pub event_id: EventId,
    pub munition_type: EntityType,
    pub shot_start_time: ClockTime,
    pub cumulative_shot_time: f32,
    pub aperture_emitter_location: Vector3Float,
    pub aperture_diameter: f32,
    pub wavelength: f32,
    pub padding: u32,
    pub pulse_repetition_frequency: f32,
    pub pulse_width: f32,
    pub flags: u16,
    pub pulse_shape: u8,
    pub padding1: u8,
    pub padding2: u32,
    pub padding2: u16,
    pub number_of_de_records: u16,
    pub de_records: Vec<StandardVariableSpecification>,
}

impl Default for DirectedEnergyFirePdu {
    /// Creates a default Directed Energy Fire PDU with arbitrary firing entity ID and target entity
    /// ID
    ///
    /// # Examples
    ///
    /// Initializing a Directed Energy Fire PDU:
    /// ```
    /// use open_dis_rust::warfare::directed_enery_fire_pdu::DirectedEnergyFirePdu;
    /// let directed_enery_fire_pdu = DirectedEnergyFirePdu::default();
    /// ```
    ///
    fn default() -> Self {
        DirectedEnergyFirePdu {
            pdu_header: PduHeader::default(
                PduType::DirectedEnergyFire,
                ProtocolFamily::Warfare,
                56,
            ),
            firing_entity_id: EntityId::default(1),
            target_entity_id: EntityId::default(2),
            exploding_entity_id: EntityId::default(3),
            event_id: EventId::default(1),
            velocity: Vector3Float::default(),
            location_in_world_coordinates: Vector3Double::default(),
            descriptor: MunitionDescriptor::default(),
            location_in_entitys_coordinates: Vector3Float::default(),
            directed_enery_fire_result: 0,
            number_of_variable_parameters: 0,
            padding: 0,
            variable_parameters: vec![],
        }
    }
}

impl Pdu for DirectedEnergyFirePdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.firing_entity_id.serialize(buf);
        self.target_entity_id.serialize(buf);
        self.exploding_entity_id.serialize(buf);
        self.event_id.serialize(buf);
        self.velocity.serialize(buf);
        self.location_in_world_coordinates.serialize(buf);
        self.descriptor.serialize(buf);
        self.location_in_entitys_coordinates.serialize(buf);
        buf.put_u8(self.directed_enery_fire_result);
        buf.put_u8(self.number_of_variable_parameters);
        buf.put_u16(self.padding);
        for i in 0..self.variable_parameters.len() {
            self.variable_parameters[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::DirectedEnergyFire {
            let firing_entity_id = EntityId::decode(&mut buffer);
            let target_entity_id = EntityId::decode(&mut buffer);
            let exploding_entity_id = EntityId::decode(&mut buffer);
            let event_id = EventId::decode(&mut buffer);
            let velocity = Vector3Float::decode(&mut buffer);
            let location_in_world_coordinates = Vector3Double::decode(&mut buffer);
            let descriptor = MunitionDescriptor::decode(&mut buffer);
            let location_in_entitys_coordinates = Vector3Float::decode(&mut buffer);
            let directed_enery_fire_result = buffer.get_u8();
            let number_of_variable_parameters = buffer.get_u8();
            let padding = buffer.get_u16();
            let mut variable_parameters: Vec<VariableParameter> = vec![];
            for _i in 0..number_of_variable_parameters {
                variable_parameters.push(VariableParameter::decode(&mut buffer));
            }
            Ok(DirectedEnergyFirePdu {
                pdu_header,
                firing_entity_id,
                target_entity_id,
                exploding_entity_id,
                event_id,
                velocity,
                location_in_world_coordinates,
                descriptor,
                location_in_entitys_coordinates,
                directed_enery_fire_result,
                number_of_variable_parameters,
                padding,
                variable_parameters,
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
        let firing_entity_id = EntityId::decode(&mut buffer);
        let target_entity_id = EntityId::decode(&mut buffer);
        let exploding_entity_id = EntityId::decode(&mut buffer);
        let event_id = EventId::decode(&mut buffer);
        let velocity = Vector3Float::decode(&mut buffer);
        let location_in_world_coordinates = Vector3Double::decode(&mut buffer);
        let descriptor = MunitionDescriptor::decode(&mut buffer);
        let location_in_entitys_coordinates = Vector3Float::decode(&mut buffer);
        let directed_enery_fire_result = buffer.get_u8();
        let number_of_variable_parameters = buffer.get_u8();
        let padding = buffer.get_u16();
        let mut variable_parameters: Vec<VariableParameter> = vec![];
        for _i in 0..number_of_variable_parameters {
            variable_parameters.push(VariableParameter::decode(&mut buffer));
        }
        Ok(DirectedEnergyFirePdu {
            pdu_header,
            firing_entity_id,
            target_entity_id,
            exploding_entity_id,
            event_id,
            velocity,
            location_in_world_coordinates,
            descriptor,
            location_in_entitys_coordinates,
            directed_enery_fire_result,
            number_of_variable_parameters,
            padding,
            variable_parameters,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::DirectedEnergyFirePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let directed_enery_fire_pdu = DirectedEnergyFirePdu::default();
        let pdu_header = PduHeader::default(
            PduType::DirectedEnergyFire,
            ProtocolFamily::Warfare,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            directed_enery_fire_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            directed_enery_fire_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            directed_enery_fire_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            directed_enery_fire_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, directed_enery_fire_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.padding,
            directed_enery_fire_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let directed_enery_fire_pdu = DirectedEnergyFirePdu::default();
        let mut buffer = BytesMut::new();
        directed_enery_fire_pdu.serialize(&mut buffer);

        let new_directed_enery_fire_pdu = DirectedEnergyFirePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_directed_enery_fire_pdu.pdu_header,
            directed_enery_fire_pdu.pdu_header
        );
    }
}
