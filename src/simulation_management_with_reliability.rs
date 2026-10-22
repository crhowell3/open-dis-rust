//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Simulation Management with Reliability (SIMAN-R) protocol family

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            ClockTime, EntityId, RecordSpecification, {FixedDatumRecord, VariableDatumRecord},
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
    use crate::{common::pdu::Pdu, pdu_tests};

    pdu_tests!(
        create_entity_reliable_pdu_tests, CreateEntityReliablePdu,
        pdu_type: PduType::CreateEntityReliable,
        default_bits: 256,
        wrong_type: RemoveEntityReliablePdu,
    );
    pdu_tests!(
        remove_entity_reliable_pdu_tests, RemoveEntityReliablePdu,
        pdu_type: PduType::RemoveEntityReliable,
        default_bits: 256,
        wrong_type: StartResumeReliablePdu,
    );
    pdu_tests!(
        start_resume_reliable_pdu_tests, StartResumeReliablePdu,
        pdu_type: PduType::StartResumeReliable,
        default_bits: 384,
        wrong_type: StopFreezeReliablePdu,
    );
    pdu_tests!(
        stop_freeze_reliable_pdu_tests, StopFreezeReliablePdu,
        pdu_type: PduType::StopFreezeReliable,
        default_bits: 320,
        wrong_type: AcknowledgeReliablePdu,
    );
    pdu_tests!(
        acknowledge_reliable_pdu_tests, AcknowledgeReliablePdu,
        pdu_type: PduType::AcknowledgeReliable,
        default_bits: 256,
        wrong_type: ActionRequestReliablePdu,
    );
    pdu_tests!(
        action_request_reliable_pdu_tests, ActionRequestReliablePdu,
        pdu_type: PduType::ActionRequestReliable,
        default_bits: 384,
        wrong_type: ActionResponseReliablePdu,
    );
    pdu_tests!(
        action_response_reliable_pdu_tests, ActionResponseReliablePdu,
        pdu_type: PduType::ActionResponseReliable,
        default_bits: 320,
        wrong_type: DataQueryReliablePdu,
    );
    pdu_tests!(
        data_query_reliable_pdu_tests, DataQueryReliablePdu,
        pdu_type: PduType::DataQueryReliable,
        default_bits: 352,
        wrong_type: SetDataReliablePdu,
    );
    pdu_tests!(
        set_data_reliable_pdu_tests, SetDataReliablePdu,
        pdu_type: PduType::SetDataReliable,
        default_bits: 320,
        wrong_type: DataReliablePdu,
    );
    pdu_tests!(
        data_reliable_pdu_tests, DataReliablePdu,
        pdu_type: PduType::DataReliable,
        default_bits: 320,
        wrong_type: EventReportReliablePdu,
    );
    pdu_tests!(
        event_report_reliable_pdu_tests, EventReportReliablePdu,
        pdu_type: PduType::EventReportReliable,
        default_bits: 320,
        wrong_type: CommentReliablePdu,
    );
    pdu_tests!(
        comment_reliable_pdu_tests, CommentReliablePdu,
        pdu_type: PduType::CommentReliable,
        default_bits: 256,
        wrong_type: RecordQueryReliablePdu,
    );
    pdu_tests!(
        record_query_reliable_pdu_tests, RecordQueryReliablePdu,
        pdu_type: PduType::RecordQueryReliable,
        default_bits: 320,
        wrong_type: SetRecordReliablePdu,
    );
    pdu_tests!(
        set_record_reliable_pdu_tests, SetRecordReliablePdu,
        pdu_type: PduType::SetRecordReliable,
        default_bits: 320,
        wrong_type: CommentReliablePdu,
    );
}
