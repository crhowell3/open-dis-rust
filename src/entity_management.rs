//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Entity Management protocol family

use crate::common::{
    GenericHeader,
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
};

/// Implemented according to IEEE 1278.1-2012 §7.8.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct AggregateStatePdu {
    #[deku(skip)]
    header: PduHeader,
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
    #[deku(
        update = "self.aggregate_id_list.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_dis_aggregates)"
    )]
    pub number_of_dis_aggregates: u16,
    #[deku(
        update = "self.entity_id_list.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_dis_entities)"
    )]
    pub number_of_dis_entities: u16,
    #[deku(
        update = "self.silent_aggregate_system_list.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_silent_aggregate_types)"
    )]
    pub number_of_silent_aggregate_types: u16,
    #[deku(
        update = "self.silent_entity_system_list.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_silent_entity_types)"
    )]
    pub number_of_silent_entity_types: u16,
    #[deku(count = "number_of_dis_aggregates")]
    pub aggregate_id_list: Vec<AggregateId>,
    #[deku(count = "number_of_dis_entities")]
    pub entity_id_list: Vec<EntityId>,
    /// Zero octets that bring the two ID lists to a 32-bit boundary.
    #[deku(
        count = "id_list_padding(usize::from(*number_of_dis_aggregates), usize::from(*number_of_dis_entities))",
        writer = "crate::common::wire::write_zeros(deku::writer, id_list_padding(aggregate_id_list.len(), entity_id_list.len()))"
    )]
    padding: Vec<u8>,
    #[deku(count = "number_of_silent_aggregate_types")]
    pub silent_aggregate_system_list: Vec<EntityType>,
    #[deku(count = "number_of_silent_entity_types")]
    pub silent_entity_system_list: Vec<EntityType>,
    #[deku(
        update = "self.variable_datum_list.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_variable_datum_records)"
    )]
    pub number_of_variable_datum_records: u32,
    #[deku(count = "number_of_variable_datum_records")]
    pub variable_datum_list: Vec<VariableDatumRecord>,
}
crate::impl_pdu!(
    AggregateStatePdu,
    PduHeader,
    PduType::AggregateState,
    ProtocolFamily::EntityManagement
);

/// Implemented according to IEEE 1278.1-2012 §7.8.3
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct IsGroupOfPdu {
    #[deku(skip)]
    header: PduHeader,
    pub group_entity_id: EntityId,
    pub grouped_entity_category: IsGroupOfGroupedEntityCategory,
    #[deku(update = "self.grouped_entity_descriptions.len()")]
    pub number_of_grouped_entities: u8,
    padding: u32,
    pub latitude: f64,
    pub longitude: f64,
    #[deku(count = "number_of_grouped_entities")]
    pub grouped_entity_descriptions: Vec<u64>,
}
crate::impl_pdu!(
    IsGroupOfPdu,
    PduHeader,
    PduType::IsGroupOf,
    ProtocolFamily::EntityManagement
);

/// Implemented according to IEEE 1278.1-2012 §7.8.4
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct TransferOwnershipPdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_id: SimulationIdentifier,
    pub receiving_id: SimulationIdentifier,
    pub request_id: u32,
    pub required_reliability_service: RequiredReliabilityService,
    pub transfer_type: TransferControlTransferType,
    pub transfer_entity_id: EntityId,
    pub record_information: RecordSpecification,
}
crate::impl_pdu!(
    TransferOwnershipPdu,
    PduHeader,
    PduType::TransferOwnership,
    ProtocolFamily::EntityManagement
);

/// Implemented according to IEEE 1278.1-2012 §7.8.5
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct IsPartOfPdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub relationship: Relationship,
    pub part_location: Vector3Float,
    pub named_location_id: NamedLocation,
    pub part_entity_type: EntityType,
}
crate::impl_pdu!(
    IsPartOfPdu,
    PduHeader,
    PduType::IsPartOf,
    ProtocolFamily::EntityManagement
);

/// Octets of padding after the Aggregate State PDU's aggregate and entity ID
/// lists. Each ID is 6 octets, so the lists end 2 octets short of a 32-bit
/// boundary when they hold an odd number of IDs between them.
const fn id_list_padding(aggregates: usize, entities: usize) -> usize {
    // The sum is odd exactly when the parities differ.
    if (aggregates ^ entities) & 1 == 1 {
        2
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::{
            constants::BITS_PER_BYTE,
            pdu::{Pdu, to_bytes},
        },
        pdu_tests,
    };
    use bytes::Buf;

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
