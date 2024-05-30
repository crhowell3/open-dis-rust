//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::{
    common::{
        dis_error::DISError,
        entity_id::EntityId,
        enums::LandPlatformAppearance,
        euler_angles::EulerAngles,
        linear_velocity::LinearVelocity,
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
        world_coordinate::WorldCoordinate,
    },
    warfare::data_types::variable_parameter::VariableParameter,
};

#[derive(Clone, Debug)]
pub struct EntityStateUpdatePdu {
    pub pdu_header: PduHeader,
    pub entity_id: EntityId,
    pub padding: u8,
    pub number_of_variable_parameters: u8,
    pub entity_linear_velocity: LinearVelocity,
    pub entity_location: WorldCoordinate,
    pub entity_orientation: EulerAngles,
    pub entity_appearance: EntityAppearance,
    pub variable_parameter_records: Vec<VariableParameter>,
}

impl Default for EntityStateUpdatePdu {
    fn default() -> Self {
        EntityStateUpdatePdu {
            pdu_header: PduHeader::default(
                PduType::EntityStateUpdate,
                ProtocolFamily::EntityInformation,
                864 / 8,
            ),
            entity_id: EntityId::default(1),
            padding: 0,
            number_of_variable_parameters: 0,
            entity_linear_velocity: LinearVelocity::default(),
            entity_location: WorldCoordinate::default(),
            entity_orientation: EulerAngles::default(),
            entity_appearance: 0,
            variable_parameter_records: vec![],
        }
    }
}

impl Pdu for EntityStateUpdatePdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.entity_id.serialize(buf);
        buf.put_u8(self.padding);
        buf.put_u8(self.number_of_variable_parameters);
        self.entity_linear_velocity.serialize(buf);
        self.entity_location.serialize(buf);
        self.entity_orientation.serialize(buf);
        self.entity_appearance.as_u32(buf);
        for i in 0..self.variable_parameter_records.len() {
            self.variable_parameter_records[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<EntityStateUpdatePdu, DISError> {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::EntityStateUpdate {
            let entity_id = EntityId::decode(&mut buffer);
            let padding = buffer.get_u8();
            let number_of_variable_parameters = buffer.get_u8();
            let entity_linear_velocity = LinearVelocity::decode(&mut buffer);
            let entity_location = WorldCoordinate::decode(&mut buffer);
            let entity_orientation = EulerAngles::decode(&mut buffer);
            let entity_appearance = LandPlatformAppearance::decode(&mut buffer);
            let mut variable_parameter_records: Vec<VariableParameter> = vec![];
            for _i in 0..number_of_variable_parameters {
                variable_parameter_records.push(VariableParameter::decode(&mut buffer));
            }
            Ok(EntityStateUpdatePdu {
                pdu_header,
                entity_id,
                padding,
                number_of_variable_parameters,
                entity_linear_velocity,
                entity_location,
                entity_orientation,
                entity_appearance,
                variable_parameter_records,
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
        let entity_id = EntityId::decode(&mut buffer);
        let padding = buffer.get_u8();
        let number_of_variable_parameters = buffer.get_u8();
        let entity_linear_velocity = LinearVelocity::decode(&mut buffer);
        let entity_location = WorldCoordinate::decode(&mut buffer);
        let entity_orientation = EulerAngles::decode(&mut buffer);
        let entity_appearance = LandPlatformAppearance::decode(&mut buffer);
        let mut variable_parameter_records: Vec<VariableParameter> = vec![];
        for _i in 0..number_of_variable_parameters {
            variable_parameter_records.push(VariableParameter::decode(&mut buffer));
        }
        Ok(EntityStateUpdatePdu {
            pdu_header,
            entity_id,
            padding,
            number_of_variable_parameters,
            entity_linear_velocity,
            entity_location,
            entity_orientation,
            entity_appearance,
            variable_parameter_records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::EntityStateUpdatePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let entity_state_update_pdu = EntityStateUpdatePdu::default();
        let header = PduHeader::default(
            PduType::EntityStateUpdate,
            ProtocolFamily::EntityInformation,
            864 / 8,
        );
        assert_eq!(
            header.protocol_version,
            entity_state_update_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            header.exercise_id,
            entity_state_update_pdu.pdu_header.exercise_id
        );
        assert_eq!(header.pdu_type, entity_state_update_pdu.pdu_header.pdu_type);
        assert_eq!(
            header.protocol_family,
            entity_state_update_pdu.pdu_header.protocol_family
        );
        assert_eq!(header.length, entity_state_update_pdu.pdu_header.length);
        assert_eq!(header.padding, entity_state_update_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let entity_state_update_pdu = EntityStateUpdatePdu::default();
        let mut buffer = BytesMut::new();
        entity_state_update_pdu.serialize(&mut buffer);

        let new_entity_state_update_pdu = EntityStateUpdatePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_entity_state_update_pdu.pdu_header,
            entity_state_update_pdu.pdu_header
        );
    }
}
