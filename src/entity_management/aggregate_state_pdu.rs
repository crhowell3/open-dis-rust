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
    entity_type::EntityType,
    euler_angles::EulerAngles,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    vector3_double::Vector3Double,
    vector3_float::Vector3Float,
};

use super::data_types::{aggregate_id::AggregateId, aggregate_marking::AggregateMarking};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.8.2
pub struct AggregateStatePdu {
    pub pdu_header: PduHeader,
    pub aggregate_id: EntityId,
    pub force_id: u8,
    pub aggregate_state: u8,
    pub aggregate_type: EntityType,
    pub formation: u32,
    pub aggregate_marking: AggregateMarking,
    pub dimensions: Vector3Float,
    pub orientation: EulerAngles,
    pub center_of_mass: Vector3Double,
    pub velocity: Vector3Float,
    pub number_of_dis_aggregates: u16,
    pub number_of_dis_entities: u16,
    pub number_of_silent_aggregate_types: u16,
    pub number_of_silent_entity_types: u16,
    pub aggregate_id_list: Vec<AggregateId>,
    pub entity_id_list: Vec<EntityId>,
    pub pad2: u8,
    pub silent_aggregate_system_list: Vec<EntityType>,
    pub silent_entity_system_list: Vec<EntityType>,
    pub number_of_variable_datum_records: u32,
    pub variable_datum_list: Vec<u64>,
}

impl Default for AggregateStatePdu {
    /// Creates a default Aggregate State PDU with arbitrary aggregate ID
    ///
    /// # Examples
    ///
    /// Initializing an Aggregate State PDU:
    /// ```
    /// use open_dis_rust::entity_management::aggregate_state_pdu::AggregateStatePdu;
    /// let aggregate_state_pdu = AggregateStatePdu::default();
    /// ```
    ///
    fn default() -> Self {
        AggregateStatePdu {
            pdu_header: PduHeader::default(
                PduType::AggregateState,
                ProtocolFamily::EntityManagement,
                56,
            ),
            aggregate_id: EntityId::default(1),
            force_id: 0,
            aggregate_state: 0,
            aggregate_type: EntityType::default(),
            formation: 0,
            aggregate_marking: AggregateMarking::default(),
            dimensions: Vector3Float::default(),
            orientation: EulerAngles::default(),
            center_of_mass: Vector3Double::default(),
            velocity: Vector3Float::default(),
            number_of_dis_aggregates: 0,
            number_of_dis_entities: 0,
            number_of_silent_aggregate_types: 0,
            number_of_silent_entity_types: 0,
            aggregate_id_list: vec![],
            entity_id_list: vec![],
            pad2: 0,
            silent_aggregate_system_list: vec![],
            silent_entity_system_list: vec![],
            number_of_variable_datum_records: 0,
            variable_datum_list: vec![],
        }
    }
}

impl Pdu for AggregateStatePdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.aggregate_id.serialize(buf);
        buf.put_u8(self.force_id);
        buf.put_u8(self.aggregate_state);
        self.aggregate_type.serialize(buf);
        buf.put_u32(self.formation);
        self.aggregate_marking.serialize(buf);
        self.dimensions.serialize(buf);
        self.orientation.serialize(buf);
        self.center_of_mass.serialize(buf);
        self.velocity.serialize(buf);
        buf.put_u16(self.number_of_dis_aggregates);
        buf.put_u16(self.number_of_dis_entities);
        buf.put_u16(self.number_of_silent_aggregate_types);
        buf.put_u16(self.number_of_silent_entity_types);
        for i in 0..self.aggregate_id_list.len() {
            self.aggregate_id_list[i].serialize(buf);
        }
        for i in 0..self.entity_id_list.len() {
            self.entity_id_list[i].serialize(buf);
        }
        buf.put_u8(self.pad2);
        for i in 0..self.silent_aggregate_system_list.len() {
            self.silent_aggregate_system_list[i].serialize(buf);
        }
        for i in 0..self.silent_entity_system_list.len() {
            self.silent_entity_system_list[i].serialize(buf);
        }
        buf.put_u32(self.number_of_variable_datum_records);
        for i in 0..self.variable_datum_list.len() {
            buf.put_u64(self.variable_datum_list[i]);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::AggregateState {
            let aggregate_id = EntityId::deserialize(&mut buffer);
            let force_id = buffer.get_u8();
            let aggregate_state = buffer.get_u8();
            let aggregate_type = EntityType::deserialize(&mut buffer);
            let formation = buffer.get_u32();
            let aggregate_marking = AggregateMarking::deserialize(&mut buffer);
            let dimensions = Vector3Float::deserialize(&mut buffer);
            let orientation = EulerAngles::deserialize(&mut buffer);
            let center_of_mass = Vector3Double::deserialize(&mut buffer);
            let velocity = Vector3Float::deserialize(&mut buffer);
            let number_of_dis_aggregates = buffer.get_u16();
            let number_of_dis_entities = buffer.get_u16();
            let number_of_silent_aggregate_types = buffer.get_u16();
            let number_of_silent_entity_types = buffer.get_u16();
            let mut aggregate_id_list: Vec<AggregateId> = vec![];
            for _i in 0..number_of_dis_aggregates {
                aggregate_id_list.push(AggregateId::deserialize(&mut buffer));
            }
            let mut entity_id_list: Vec<EntityId> = vec![];
            for _i in 0..number_of_dis_entities {
                entity_id_list.push(EntityId::deserialize(&mut buffer));
            }
            let pad2 = buffer.get_u8();
            let mut silent_aggregate_system_list: Vec<EntityType> = vec![];
            for _i in 0..number_of_silent_aggregate_types {
                silent_aggregate_system_list.push(EntityType::deserialize(&mut buffer));
            }
            let mut silent_entity_system_list: Vec<EntityType> = vec![];
            for _i in 0..number_of_silent_entity_types {
                silent_entity_system_list.push(EntityType::deserialize(&mut buffer));
            }
            let number_of_variable_datum_records = buffer.get_u32();
            let mut variable_datum_list: Vec<u64> = vec![];
            for _i in 0..number_of_variable_datum_records {
                variable_datum_list.push(buffer.get_u64());
            }
            Ok(AggregateStatePdu {
                pdu_header,
                aggregate_id,
                force_id,
                aggregate_state,
                aggregate_type,
                formation,
                aggregate_marking,
                dimensions,
                orientation,
                center_of_mass,
                velocity,
                number_of_dis_aggregates,
                number_of_dis_entities,
                number_of_silent_aggregate_types,
                number_of_silent_entity_types,
                aggregate_id_list,
                entity_id_list,
                pad2,
                silent_aggregate_system_list,
                silent_entity_system_list,
                number_of_variable_datum_records,
                variable_datum_list,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type AggregateState, got {:?}",
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
        let aggregate_id = EntityId::deserialize(&mut buffer);
        let force_id = buffer.get_u8();
        let aggregate_state = buffer.get_u8();
        let aggregate_type = EntityType::deserialize(&mut buffer);
        let formation = buffer.get_u32();
        let aggregate_marking = AggregateMarking::deserialize(&mut buffer);
        let dimensions = Vector3Float::deserialize(&mut buffer);
        let orientation = EulerAngles::deserialize(&mut buffer);
        let center_of_mass = Vector3Double::deserialize(&mut buffer);
        let velocity = Vector3Float::deserialize(&mut buffer);
        let number_of_dis_aggregates = buffer.get_u16();
        let number_of_dis_entities = buffer.get_u16();
        let number_of_silent_aggregate_types = buffer.get_u16();
        let number_of_silent_entity_types = buffer.get_u16();
        let mut aggregate_id_list: Vec<AggregateId> = vec![];
        for _i in 0..number_of_dis_aggregates {
            aggregate_id_list.push(AggregateId::deserialize(&mut buffer));
        }
        let mut entity_id_list: Vec<EntityId> = vec![];
        for _i in 0..number_of_dis_entities {
            entity_id_list.push(EntityId::deserialize(&mut buffer));
        }
        let pad2 = buffer.get_u8();
        let mut silent_aggregate_system_list: Vec<EntityType> = vec![];
        for _i in 0..number_of_silent_aggregate_types {
            silent_aggregate_system_list.push(EntityType::deserialize(&mut buffer));
        }
        let mut silent_entity_system_list: Vec<EntityType> = vec![];
        for _i in 0..number_of_silent_entity_types {
            silent_entity_system_list.push(EntityType::deserialize(&mut buffer));
        }
        let number_of_variable_datum_records = buffer.get_u32();
        let mut variable_datum_list: Vec<u64> = vec![];
        for _i in 0..number_of_variable_datum_records {
            variable_datum_list.push(buffer.get_u64());
        }
        Ok(AggregateStatePdu {
            pdu_header,
            aggregate_id,
            force_id,
            aggregate_state,
            aggregate_type,
            formation,
            aggregate_marking,
            dimensions,
            orientation,
            center_of_mass,
            velocity,
            number_of_dis_aggregates,
            number_of_dis_entities,
            number_of_silent_aggregate_types,
            number_of_silent_entity_types,
            aggregate_id_list,
            entity_id_list,
            pad2,
            silent_aggregate_system_list,
            silent_entity_system_list,
            number_of_variable_datum_records,
            variable_datum_list,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::AggregateStatePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let aggregate_state_pdu = AggregateStatePdu::default();
        let pdu_header = PduHeader::default(
            PduType::AggregateState,
            ProtocolFamily::EntityManagement,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            aggregate_state_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            aggregate_state_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, aggregate_state_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            aggregate_state_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, aggregate_state_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, aggregate_state_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut aggregate_state_pdu = AggregateStatePdu::default();
        let mut buffer = BytesMut::new();
        aggregate_state_pdu.serialize(&mut buffer);

        let new_aggregate_state_pdu = AggregateStatePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_aggregate_state_pdu.pdu_header,
            aggregate_state_pdu.pdu_header
        );
    }
}
