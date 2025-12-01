//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Simulation Management with Reliability (SIMAN-R) protocol family

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            ClockTime,
            datum_records::{FixedDatumRecord, VariableDatumRecord},
            entity_id::EntityId,
            record_specification::RecordSpecification,
        },
        enums::{
            AcknowledgeFlag, AcknowledgeResponseFlag, ActionResponseRequestStatus, EventType,
            FrozenBehavior, PduType, ProtocolFamily, Reason, RecordQueryREventType,
            RequiredReliabilityService, VariableRecordTypes,
        },
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.2
    pub struct CreateEntityReliablePdu {
        header: PduHeader,
        pdu_type: PduType::CreateEntityReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub required_reliability_service: RequiredReliabilityService,
            padding: u8,
            padding2: u16,
            pub request_id: u32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.3
    pub struct RemoveEntityReliablePdu {
        header: PduHeader,
        pdu_type: PduType::RemoveEntityReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub required_reliability_service: RequiredReliabilityService,
            padding: u8,
            padding2: u16,
            pub request_id: u32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.4
    pub struct StartResumeReliablePdu {
        header: PduHeader,
        pdu_type: PduType::StartResumeReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub real_world_time: ClockTime,
            pub simulation_time: ClockTime,
            pub required_reliability_service: RequiredReliabilityService,
            padding: u8,
            padding2: u16,
            pub request_id: u32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.5
    pub struct StopFreezeReliablePdu {
        header: PduHeader,
        pdu_type: PduType::StopFreezeReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub real_world_time: ClockTime,
            pub reason: Reason,
            pub frozen_behavior: FrozenBehavior,
            pub required_reliability_service: u8,
            padding: u8,
            pub request_id: u32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.6
    pub struct AcknowledgeReliablePdu {
        header: PduHeader,
        pdu_type: PduType::AcknowledgeReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub acknowledge_flag: AcknowledgeFlag,
            pub response_flag: AcknowledgeResponseFlag,
            pub request_id: u32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.7
    pub struct ActionRequestReliablePdu {
        header: PduHeader,
        pdu_type: PduType::ActionRequestReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub required_reliability_service: RequiredReliabilityService,
            padding: u8,
            padding2: u16,
            pub request_id: u32,
            pub action_id: u32,
            padding3: u32,
            pub number_of_fixed_datum_records: u32,
            pub number_of_variable_datum_records: u32,
            pub fixed_datum_records: Vec<FixedDatumRecord>,
            pub variable_datum_records: Vec<VariableDatumRecord>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.8
    pub struct ActionResponseReliablePdu {
        header: PduHeader,
        pdu_type: PduType::ActionResponseReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub request_id: u32,
            pub request_status: ActionResponseRequestStatus,
            pub number_of_fixed_datum_records: u32,
            pub number_of_variable_datum_records: u32,
            pub fixed_datum_records: Vec<FixedDatumRecord>,
            pub variable_datum_records: Vec<VariableDatumRecord>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.9
    pub struct DataQueryReliablePdu {
        header: PduHeader,
        pdu_type: PduType::DataQueryReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub required_reliability_service: RequiredReliabilityService,
            padding: u8,
            padding2: u16,
            pub request_id: u32,
            pub time_interval: u32,
            pub number_of_fixed_datum_records: u32,
            pub number_of_variable_datum_records: u32,
            pub fixed_datum_ids: Vec<VariableRecordTypes>,
            pub variable_datum_ids: Vec<VariableRecordTypes>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.10
    pub struct SetDataReliablePdu {
        header: PduHeader,
        pdu_type: PduType::SetDataReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub required_reliability_service: RequiredReliabilityService,
            padding: u8,
            padding2: u16,
            pub request_id: u32,
            pub number_of_fixed_datum_records: u32,
            pub number_of_variable_datum_records: u32,
            pub fixed_datum_records: Vec<FixedDatumRecord>,
            pub variable_datum_records: Vec<VariableDatumRecord>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.11
    pub struct DataReliablePdu {
        header: PduHeader,
        pdu_type: PduType::DataReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub request_id: u32,
            pub required_reliability_service: RequiredReliabilityService,
            padding: u8,
            padding2: u16,
            pub number_of_fixed_datum_records: u32,
            pub number_of_variable_datum_records: u32,
            pub fixed_datum_records: Vec<FixedDatumRecord>,
            pub variable_datum_records: Vec<VariableDatumRecord>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.12
    pub struct EventReportReliablePdu {
        header: PduHeader,
        pdu_type: PduType::EventReportReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub event_type: EventType,
            padding: u32,
            pub number_of_fixed_datum_records: u32,
            pub number_of_variable_datum_records: u32,
            pub fixed_datum_records: Vec<FixedDatumRecord>,
            pub variable_datum_records: Vec<VariableDatumRecord>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.13
    pub struct CommentReliablePdu {
        header: PduHeader,
        pdu_type: PduType::CommentReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub number_of_fixed_datum_records: u32,
            pub number_of_variable_datum_records: u32,
            pub fixed_datum_records: Vec<FixedDatumRecord>,
            pub variable_datum_records: Vec<VariableDatumRecord>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.14
    pub struct RecordQueryReliablePdu {
        header: PduHeader,
        pdu_type: PduType::RecordQueryReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub request_id: u32,
            pub required_reliability_service: RequiredReliabilityService,
            padding: u8,
            pub event_type: RecordQueryREventType,
            pub time: u32,
            pub number_of_records: u32,
            pub record_ids: Vec<u32>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.11.15
    pub struct SetRecordReliablePdu {
        header: PduHeader,
        pdu_type: PduType::SetRecordReliable,
        protocol_family: ProtocolFamily::SimulationManagementWithReliability,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub request_id: u32,
            pub required_reliability_service: RequiredReliabilityService,
            padding: u8,
            padding2: u16,
            padding3: u32,
            pub record_sets: RecordSpecification,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    mod create_entity_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = CreateEntityReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<CreateEntityReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = CreateEntityReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                CreateEntityReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
            let pdu = CreateEntityReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod remove_entity_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = RemoveEntityReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<RemoveEntityReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = RemoveEntityReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                RemoveEntityReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
            let pdu = RemoveEntityReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod start_resume_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = StartResumeReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<StartResumeReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = StartResumeReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                StartResumeReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 384 / BITS_PER_BYTE;
            let pdu = StartResumeReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod stop_freeze_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = StopFreezeReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<StopFreezeReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = StopFreezeReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                StopFreezeReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = StopFreezeReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod acknowledge_reliable_pdu_tests {
        use super::*;
        #[test]
        fn cast_to_any() {
            let pdu = AcknowledgeReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<AcknowledgeReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = AcknowledgeReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                AcknowledgeReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
            let pdu = AcknowledgeReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod action_request_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = ActionRequestReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<ActionRequestReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = ActionRequestReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                ActionRequestReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 384 / BITS_PER_BYTE;
            let pdu = ActionRequestReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod action_response_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = ActionResponseReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<ActionResponseReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = ActionResponseReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                ActionResponseReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = ActionResponseReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod data_query_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = DataQueryReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<DataQueryReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = DataQueryReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                DataQueryReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 352 / BITS_PER_BYTE;
            let pdu = DataQueryReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod set_data_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = SetDataReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<SetDataReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = SetDataReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = SetDataReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = SetDataReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod data_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = DataReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<DataReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = DataReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = DataReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = DataReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod event_report_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = EventReportReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<EventReportReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = EventReportReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                EventReportReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = EventReportReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod comment_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = CommentReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<CommentReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = CommentReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = CommentReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
            let pdu = CommentReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod record_query_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = RecordQueryReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<RecordQueryReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = RecordQueryReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                RecordQueryReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = RecordQueryReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod set_record_reliable_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = SetRecordReliablePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<SetRecordReliablePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = SetRecordReliablePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                SetRecordReliablePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = SetRecordReliablePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }
}
