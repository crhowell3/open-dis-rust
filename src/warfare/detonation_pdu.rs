//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
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
    pub pdu_header: PduHeader,
    pub firing_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub exploding_entity_id: EntityId,
    pub event_id: EventId,
    pub velocity: Vector3Float,
    pub location_in_world_coordinates: Vector3Double,
    pub descriptor: MunitionDescriptor,
    pub location_in_entitys_coordinates: Vector3Float,
    pub detonation_result: u8,
    pub number_of_variable_parameters: u8,
    pub padding: u16,
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
            pdu_header: PduHeader::default(PduType::Detonation, ProtocolFamily::Warfare, 56),
            firing_entity_id: EntityId::default(1),
            target_entity_id: EntityId::default(2),
            exploding_entity_id: EntityId::default(3),
            event_id: EventId::default(1),
            velocity: Vector3Float::default(),
            location_in_world_coordinates: Vector3Double::default(),
            descriptor: MunitionDescriptor::default(),
            location_in_entitys_coordinates: Vector3Float::default(),
            detonation_result: 0,
            number_of_variable_parameters: 0,
            padding: 0,
            variable_parameters: vec![],
        }
    }
}

impl Pdu for DetonationPdu {
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
        buf.put_u8(self.detonation_result);
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
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::Detonation {
            let firing_entity_id = EntityId::deserialize(&mut buffer);
            let target_entity_id = EntityId::deserialize(&mut buffer);
            let exploding_entity_id = EntityId::deserialize(&mut buffer);
            let event_id = EventId::deserialize(&mut buffer);
            let velocity = Vector3Float::deserialize(&mut buffer);
            let location_in_world_coordinates = Vector3Double::deserialize(&mut buffer);
            let descriptor = MunitionDescriptor::deserialize(&mut buffer);
            let location_in_entitys_coordinates = Vector3Float::deserialize(&mut buffer);
            let detonation_result = buffer.get_u8();
            let number_of_variable_parameters = buffer.get_u8();
            let padding = buffer.get_u16();
            let mut variable_parameters: Vec<VariableParameter> = vec![];
            for _i in 0..number_of_variable_parameters {
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
        let firing_entity_id = EntityId::deserialize(&mut buffer);
        let target_entity_id = EntityId::deserialize(&mut buffer);
        let exploding_entity_id = EntityId::deserialize(&mut buffer);
        let event_id = EventId::deserialize(&mut buffer);
        let velocity = Vector3Float::deserialize(&mut buffer);
        let location_in_world_coordinates = Vector3Double::deserialize(&mut buffer);
        let descriptor = MunitionDescriptor::deserialize(&mut buffer);
        let location_in_entitys_coordinates = Vector3Float::deserialize(&mut buffer);
        let detonation_result = buffer.get_u8();
        let number_of_variable_parameters = buffer.get_u8();
        let padding = buffer.get_u16();
        let mut variable_parameters: Vec<VariableParameter> = vec![];
        for _i in 0..number_of_variable_parameters {
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
            padding,
            variable_parameters,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::DetonationPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let detonation_pdu = DetonationPdu::default();
        let pdu_header = PduHeader::default(PduType::Detonation, ProtocolFamily::Warfare, 448 / 8);

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
        assert_eq!(pdu_header.padding, detonation_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut detonation_pdu = DetonationPdu::default();
        let mut buffer = BytesMut::new();
        detonation_pdu.serialize(&mut buffer);

        let new_detonation_pdu = DetonationPdu::deserialize(buffer).unwrap();
        assert_eq!(new_detonation_pdu.pdu_header, detonation_pdu.pdu_header);
    }
}
