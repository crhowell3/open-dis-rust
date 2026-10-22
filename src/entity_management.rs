//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Entity Management protocol family

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            AggregateId, AggregateMarking, EntityId, EntityType, EulerAngles, LinearVelocity,
            NamedLocation, RecordSpecification, Relationship, SimulationIdentifier,
            VariableDatumRecord, Vector3Float, WorldCoordinate,
        },
        enums::{
            AggregateStateAggregateState, AggregateStateFormation, ForceId,
            IsGroupOfGroupedEntityCategory, PduType, ProtocolFamily, RequiredReliabilityService,
            TransferControlTransferType,
        },
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.8.2
    pub struct AggregateStatePdu {
        header: PduHeader,
        pdu_type: PduType::AggregateState,
        protocol_family: ProtocolFamily::EntityManagement,
        fields: {
            pub aggregate_id: AggregateId,
            pub force_id: ForceId,
            pub aggregate_state: AggregateStateAggregateState,
            pub aggregate_type: EntityType,
            pub formation: AggregateStateFormation,
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
            padding: Vec<u8>,
            pub silent_aggregate_system_list: Vec<EntityType>,
            pub silent_entity_system_list: Vec<EntityType>,
            pub number_of_variable_datum_records: u32,
            pub variable_datum_list: Vec<VariableDatumRecord>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.8.3
    pub struct IsGroupOfPdu {
        header: PduHeader,
        pdu_type: PduType::IsGroupOf,
        protocol_family: ProtocolFamily::EntityManagement,
        fields: {
            pub group_entity_id: EntityId,
            pub grouped_entity_category: IsGroupOfGroupedEntityCategory,
            pub number_of_grouped_entities: u8,
            padding: u32,
            pub latitude: f64,
            pub longitude: f64,
            pub grouped_entity_descriptions: Vec<u64>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.8.4
    pub struct TransferOwnershipPdu {
        header: PduHeader,
        pdu_type: PduType::TransferOwnership,
        protocol_family: ProtocolFamily::EntityManagement,
        fields: {
            pub originating_id: SimulationIdentifier,
            pub receiving_id: SimulationIdentifier,
            pub request_id: u32,
            pub required_reliability_service: RequiredReliabilityService,
            pub transfer_type: TransferControlTransferType,
            pub transfer_entity_id: EntityId,
            pub record_information: RecordSpecification,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.8.5
    pub struct IsPartOfPdu {
        header: PduHeader,
        pdu_type: PduType::IsPartOf,
        protocol_family: ProtocolFamily::EntityManagement,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub relationship: Relationship,
            pub part_location: Vector3Float,
            pub named_location_id: NamedLocation,
            pub part_entity_type: EntityType,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::{constants::BITS_PER_BYTE, pdu::Pdu},
        pdu_tests,
    };
    use bytes::{Buf, Bytes, BytesMut};

    /// Serialize a PDU, failing the test if serialization returns an error.
    fn to_bytes<P: Pdu>(pdu: &mut P) -> Bytes {
        let mut buf = BytesMut::new();
        pdu.serialize(&mut buf).expect("serialize failed");
        buf.freeze()
    }

    pdu_tests!(
        aggregate_state_pdu_tests, AggregateStatePdu,
        pdu_type: PduType::AggregateState,
        default_bits: 1088,
        wrong_type: TransferOwnershipPdu,
    );
    pdu_tests!(
        transfer_ownership_pdu_tests, TransferOwnershipPdu,
        pdu_type: PduType::TransferOwnership,
        default_bits: 320,
        wrong_type: IsGroupOfPdu,
    );
    pdu_tests!(
        is_group_of_pdu_tests, IsGroupOfPdu,
        pdu_type: PduType::IsGroupOf,
        default_bits: 320,
        wrong_type: IsPartOfPdu,
    );
    pdu_tests!(
        is_part_of_pdu_tests, IsPartOfPdu,
        pdu_type: PduType::IsPartOf,
        default_bits: 416,
        wrong_type: TransferOwnershipPdu,
    );
}
