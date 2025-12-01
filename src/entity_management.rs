//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Entity Management protocol family

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            EntityId, EntityType, EulerAngles, LinearVelocity, SimulationIdentifier, Vector3Float,
            WorldCoordinate, datum_records::VariableDatumRecord, named_location::NamedLocation,
            record_specification::RecordSpecification, relationship::Relationship,
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

use crate::common::data_types::{aggregate_id::AggregateId, aggregate_marking::AggregateMarking};

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
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    mod aggregate_state_pdu_tests {
        use super::*;

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
            let new_pdu = AggregateStatePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 1088 / BITS_PER_BYTE;
            let pdu = AggregateStatePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod transfer_ownership_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = TransferOwnershipPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<TransferOwnershipPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = TransferOwnershipPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                TransferOwnershipPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = TransferOwnershipPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod is_group_of_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = IsGroupOfPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<IsGroupOfPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = IsGroupOfPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = IsGroupOfPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = IsGroupOfPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod is_part_of_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = IsPartOfPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<IsPartOfPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = IsPartOfPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = IsPartOfPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 416 / BITS_PER_BYTE;
            let pdu = IsPartOfPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }
}
