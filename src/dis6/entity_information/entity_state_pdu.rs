//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation 
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use std::any::Any;
use bytes::{BytesMut, BufMut, Buf};

#[derive(Clone, Debug)]
pub struct EntityStatePdu {
    pub pdu_header: PduHeader,
    pub entity_id: EntityId,
    pub force_id: ForceId,
    pub number_of_articulation_parameters_field: u8,
    pub entity_type: EntityType,
    pub alternative_entity_type: EntityType,
    pub entity_linear_velocity: LinearVelocity,
    pub entity_location: WorldCoordinate,
    pub entity_orientation: EulerAngles,
    pub entity_appearance: EntityAppearance,
    pub dead_reckoning_parameters: DeadReckoningParameters,
    pub entity_marking: EntityMarking,
    pub entity_capabilities: EntityCapabilities,
    pub articulation_parameter: f32
}

impl EntityStatePdu {
    pub fn default() -> Self {
        EntityStatePdu {
            pdu_header: PduHeader::default(PduType::EntityState, ProtocolFamily::EntityInformation, 864/8),
            entity_id: EntityId::default(2),
            force_id: ForceId::Other,
            number_of_articulation_parameters: 0,
            entity_type: EntityType {
                kind: Kind::Munition,
                domain: 1,
                country: Country::Other,
                category: 3,
                subcategory: 0,
                specific: 0,
                extra: 0
            },
            alternative_entity_type: EntityType {
                kind: Kind::Munition,
                domain: 1,
                country: Country::Other,
                category: 0,
                subcategory: 0,
                specific: 0,
                extra: 0
            },
            entity_linear_velocity: LinearVelocity::new(0.0, 0.0, 0.0),
            entity_location_record: WorldCoordinate::new(0.0, 0.0, 0.0),
            entity_orientation: EulerAngles::new(0.0, 0.0, 0.0),
            entity_appearance: EntityAppearance::default(),
            dead_reckoning_parameters: DeadReckoningParameters::default(),
            entity_marking: EntityMarking::default("".to_string()),
            entity_capabilities: EntityCapabilities::default(),
            articulation_parameter: 0.0
        }
    }
}

impl Pdu for EntityStatePdu {
    fn deserialize(mut buffer: BytesMut) -> Result<EntityStatePdu, DISError> {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::EntityState {
            let entity_id = EntityIdRecord::decode(&mut buffer);
            let force_id = ForceId::decode(&mut buffer);
            let articulation_params = &buffer.get_u8();
            let entity_type = EntityType::decode(&mut buffer);
            let alt_entity_type = EntityType::decode(&mut buffer);
            let linear_velocity = LinearVelocity::decode(&mut buffer);
            let world_coordinate = WorldCoordinate::decode(&mut buffer);
            let orienation = EulerAngles::decode(&mut buffer);
            let appearance = EntityAppearance::decode(&mut buffer);
            let dead_reckoning = DeadReckoningParameters::decode(&mut buffer);
            let entity_marking = EntityMarking::decode(&mut buffer);
            let entity_capabilities = EntityCapabilities::decode(&mut buffer);

            return Ok(EntityStatePdu {
                pdu_header: pdu_header,
                entity_id: entity_id,
                force_id: force_id,
                number_of_articulation_parameters: *articulation_params,
                entity_type: entity_type,
                alternative_entity_type: alt_entity_type,
                entity_linear_velocity: linear_velocity,
                entity_location: world_coordinate,
                entity_orientation: orientation,
                entity_appearance: appearance,
                dead_reckoning_parameters: dead_reckoning,
                entity_marking: entity_marking,
                entity_capabilities: entity_capabilities,
                articulation_parameter: 0.0,
            }) 
        } else {
            Err(DISError::InvalidDISHeader)
        }
    }

    fn serialize(&self, buf: &mut BytesMut) {
        unimplemented!();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
/// Enum to represent the Force the entity is part of during the simulation.
pub enum ForceId {
    Other = 0,
    Friendly = 1,
    Opposing = 2,
    Neutral = 3
}

impl ForceId {
    pub fn decode(buf: &mut BytesMut) -> ForceId {
        match buf.get_u8() {
            1 => ForceId::Friendly,
            2 => ForceId::Opposing,
            3 => ForceId::Neutral,
            _ => ForceId::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::{common::{pdu_header::{PduHeaderRecord, PduType, ProtocolFamily}, pdu::Pdu}, entity_information::entity_state_pdu::EntityStatePDU};

    #[test]
    fn header_creation() {
        let entity_state_pdu = EntityStatePdu::default();
        let header = PduHeaderRecord::default(PduType::EntityState, ProtocolFamily::EntityInformation, 864/8);
        assert_eq!(header.protocol_version, entity_state_pdu.pdu_header.protocol_version);
        assert_eq!(header.exercise_id, entity_state_pdu.pdu_header.exercise_id);
        assert_eq!(header.pdu_type, entity_state_pdu.pdu_header.pdu_type);
        assert_eq!(header.protocol_family, entity_state_pdu.pdu_header.protocol_family);
        assert_eq!(header.timestamp, entity_state_pdu.pdu_header.timestamp);
        assert_eq!(header.length, entity_state_pdu.pdu_header.length);
        assert_eq!(header.padding, entity_state_pdu .pdu_header.padding);
    }

    #[test]
    fn header_deserialise() {
        let entity_state_pdu = EntityStatePdu::default();
        let mut buffer = BytesMut::new();
        entity_state_pdu.serialise(&mut buffer);

        let new_entity_state_pdu = EntityStatePdu::deserialise(buffer).unwrap();
        assert_eq!(new_entity_state_pdu.pdu_header, entity_state_pdu.pdu_header);
    }
}