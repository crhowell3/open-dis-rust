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
    entity_type::EntityType,
    enums::{PduType, ProtocolFamily},
    euler_angles::EulerAngles,
    pdu::Pdu,
    pdu_header::PduHeader,
    vector3_float::Vector3Float,
};

use super::data_types::{aggregate_id::AggregateId, aggregate_marking::AggregateMarking};

#[derive(Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.8.2
pub struct AggregateStatePdu {
    pdu_header: PduHeader,
    pub aggregate_id: EntityId,
    pub force_id: u8,
    pub aggregate_state: u8,
    pub aggregate_type: EntityType,
    pub formation: u32,
    pub aggregate_marking: AggregateMarking,
    pub dimensions: Vector3Float,
    pub orientation: EulerAngles,
    pub center_of_mass: WorldCoordinate,
    pub velocity: LinearVelocity,
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

impl Pdu for AggregateStatePdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH
            + EntityId::LENGTH
            + EntityType::LENGTH
            + Vector3Float::LENGTH
            + AggregateMarking::LENGTH
            + LinearVelocity::LENGTH
            + EulerAngles::LENGTH
            + WorldCoordinate::LENGTH
            + std::mem::size_of::<u8>() * 2
            + std::mem::size_of::<u16>() * 4
            + std::mem::size_of::<u32>() * 2;

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
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::AggregateState {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type AggregateState, got {:?}",
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

impl AggregateStatePdu {
    /// Creates a new `AggregateStatePdu`
    ///
    /// # Examples
    ///
    /// Initializing an `AggregateStatePdu`:
    /// ```
    /// use open_dis_rust::entity_management::AggregateStatePdu;
    /// let pdu = AggregateStatePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::AggregateState;
        pdu.pdu_header.protocol_family = ProtocolFamily::EntityManagement;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let aggregate_id = EntityId::deserialize(buf);
        let force_id = buf.get_u8();
        let aggregate_state = buf.get_u8();
        let aggregate_type = EntityType::deserialize(buf);
        let formation = buf.get_u32();
        let aggregate_marking = AggregateMarking::deserialize(buf);
        let dimensions = Vector3Float::deserialize(buf);
        let orientation = EulerAngles::deserialize(buf);
        let center_of_mass = WorldCoordinate::deserialize(buf);
        let velocity = LinearVelocity::deserialize(buf);
        let number_of_dis_aggregates = buf.get_u16();
        let number_of_dis_entities = buf.get_u16();
        let number_of_silent_aggregate_types = buf.get_u16();
        let number_of_silent_entity_types = buf.get_u16();
        let mut aggregate_id_list: Vec<AggregateId> = vec![];
        for _i in 0..number_of_dis_aggregates {
            aggregate_id_list.push(AggregateId::deserialize(buf));
        }
        let mut entity_id_list: Vec<EntityId> = vec![];
        for _i in 0..number_of_dis_entities {
            entity_id_list.push(EntityId::deserialize(buf));
        }
        let pad2 = buf.get_u8();
        let mut silent_aggregate_system_list: Vec<EntityType> = vec![];
        for _i in 0..number_of_silent_aggregate_types {
            silent_aggregate_system_list.push(EntityType::deserialize(buf));
        }
        let mut silent_entity_system_list: Vec<EntityType> = vec![];
        for _i in 0..number_of_silent_entity_types {
            silent_entity_system_list.push(EntityType::deserialize(buf));
        }
        let number_of_variable_datum_records = buf.get_u32();
        let mut variable_datum_list: Vec<u64> = vec![];
        for _i in 0..number_of_variable_datum_records {
            variable_datum_list.push(buf.get_u64());
        }

        AggregateStatePdu {
            pdu_header: PduHeader::default(),
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AggregateStatePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = AggregateStatePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<AggregateStatePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = AggregateStatePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = AggregateStatePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 1088 / BITS_PER_BYTE;
        let pdu = AggregateStatePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
