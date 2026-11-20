//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Entity Management protocol family

use crate::records::{
    AggregateIdentifier, AggregateMarking, AggregateType, EntityIdentifier, EntityType,
    EulerAngles, NamedLocationIdentification, PduHeader, RecordSpecification, Relationship,
    SilentEntitySystem, SimulationIdentifier, VariableDatumRecord, Vector, WorldCoordinates,
};
use crate::siso::enums::{
    AggregateStateAggregateState, AggregateStateFormation, ForceId, IsGroupOfGroupedEntityCategory,
    PduType, ProtocolFamily, RequiredReliabilityService, TransferControlTransferType,
};
use crate::siso::grouped_entity::GroupedEntityDescription;
use deku::DekuError;

/// One Silent Aggregate System in an Aggregate State PDU (IEEE 1278.1-2012
/// §7.8.2 item r, Table 183): a number of subaggregates of one type that
/// are not sending their own Aggregate State PDU
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct SilentAggregateSystem {
    pub number_of_aggregates: u16,
    padding: u16,
    pub aggregate_type: AggregateType,
}

impl SilentAggregateSystem {
    #[must_use]
    pub const fn new(number_of_aggregates: u16, aggregate_type: AggregateType) -> Self {
        Self {
            number_of_aggregates,
            padding: 0,
            aggregate_type,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §7.8.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct AggregateStatePdu {
    #[deku(skip)]
    header: PduHeader,
    pub aggregate_id: AggregateIdentifier,
    pub force_id: ForceId,
    pub aggregate_state: AggregateStateAggregateState,
    pub aggregate_type: EntityType,
    pub formation: AggregateStateFormation,
    pub aggregate_marking: AggregateMarking,
    pub dimensions: Vector,
    pub orientation: EulerAngles,
    pub center_of_mass: WorldCoordinates,
    pub velocity: Vector,
    #[deku(
        update = "self.aggregate_id_list.len()",
        assert = "crate::wire::fits_in_pdu(*number_of_dis_aggregates)"
    )]
    pub number_of_dis_aggregates: u16,
    #[deku(
        update = "self.entity_id_list.len()",
        assert = "crate::wire::fits_in_pdu(*number_of_dis_entities)"
    )]
    pub number_of_dis_entities: u16,
    #[deku(
        update = "self.silent_aggregate_system_list.len()",
        assert = "crate::wire::fits_in_pdu(*number_of_silent_aggregate_types)"
    )]
    pub number_of_silent_aggregate_types: u16,
    #[deku(
        update = "crate::wire::update_each(&mut self.silent_entity_system_list)?",
        assert = "crate::wire::fits_in_pdu(*number_of_silent_entity_types)"
    )]
    pub number_of_silent_entity_types: u16,
    #[deku(count = "number_of_dis_aggregates")]
    pub aggregate_id_list: Vec<AggregateIdentifier>,
    #[deku(count = "number_of_dis_entities")]
    pub entity_id_list: Vec<EntityIdentifier>,
    /// Zero octets that bring the two ID lists to a 32-bit boundary.
    #[deku(
        count = "id_list_padding(usize::from(*number_of_dis_aggregates), usize::from(*number_of_dis_entities))",
        writer = "crate::wire::write_zeros(deku::writer, id_list_padding(aggregate_id_list.len(), entity_id_list.len()))"
    )]
    padding: Vec<u8>,
    #[deku(count = "number_of_silent_aggregate_types")]
    pub silent_aggregate_system_list: Vec<SilentAggregateSystem>,
    #[deku(count = "number_of_silent_entity_types")]
    pub silent_entity_system_list: Vec<SilentEntitySystem>,
    #[deku(
        update = "self.variable_datum_list.len()",
        assert = "crate::wire::fits_in_pdu(*number_of_variable_datum_records)"
    )]
    pub number_of_variable_datum_records: u32,
    #[deku(count = "number_of_variable_datum_records")]
    pub variable_datum_list: Vec<VariableDatumRecord>,
}
crate::pdu::impl_pdu!(
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
    pub group_entity_id: EntityIdentifier,
    /// The type of every entity in the group, which picks the record type of
    /// `grouped_entity_descriptions`. `serialize` sets it from them when there
    /// are any.
    #[deku(update = "self.derived_category()?")]
    pub grouped_entity_category: IsGroupOfGroupedEntityCategory,
    #[deku(update = "self.grouped_entity_descriptions.len()")]
    pub number_of_grouped_entities: u8,
    padding: u32,
    pub latitude: f64,
    pub longitude: f64,
    /// One record per entity, all of the same category
    #[deku(count = "number_of_grouped_entities", ctx = "*grouped_entity_category")]
    pub grouped_entity_descriptions: Vec<GroupedEntityDescription>,
}
crate::pdu::impl_pdu!(
    IsGroupOfPdu,
    PduHeader,
    PduType::IsGroupOf,
    ProtocolFamily::EntityManagement
);

impl IsGroupOfPdu {
    /// The category of the grouped entity descriptions, which must all share
    /// one, or the current category when there are none.
    fn derived_category(&self) -> Result<IsGroupOfGroupedEntityCategory, DekuError> {
        let mut categories = self
            .grouped_entity_descriptions
            .iter()
            .map(GroupedEntityDescription::category);
        let Some(first) = categories.next() else {
            return Ok(self.grouped_entity_category);
        };
        if categories.any(|category| category != first) {
            return Err(DekuError::Assertion(
                "an IsGroupOf PDU's grouped entity descriptions must all be of one category".into(),
            ));
        }
        Ok(first)
    }
}

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
    pub transfer_entity_id: EntityIdentifier,
    pub record_information: RecordSpecification,
}
crate::pdu::impl_pdu!(
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
    pub originating_entity_id: EntityIdentifier,
    pub receiving_entity_id: EntityIdentifier,
    pub relationship: Relationship,
    pub part_location: Vector,
    pub named_location_id: NamedLocationIdentification,
    pub part_entity_type: EntityType,
}
crate::pdu::impl_pdu!(
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
    use crate::{pdu::Pdu, pdu::pdu_tests};

    /// Serialize a PDU, failing the test if serialization returns an error.
    pub fn to_bytes<P: Pdu>(pdu: &mut P) -> Vec<u8> {
        pdu.to_bytes().expect("serialize failed")
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
