//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Simulation Management (SIMAN) protocol family

use crate::common::enums::{ActionResponseRequestStatus, EventType, FrozenBehavior, Reason};
use crate::common::generic_header::GenericHeader;
use crate::{
    common::{
        SerializedLength,
        data_types::{ClockTime, EntityId, FixedDatumRecord, VariableDatumRecord},
        enums::{AcknowledgeFlag, AcknowledgeResponseFlag, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.5.2
    pub struct CreateEntityPdu {
        header: PduHeader,
        pdu_type: PduType::CreateEntity,
        protocol_family: ProtocolFamily::SimulationManagement,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub request_id: u32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.5.3
    pub struct RemoveEntityPdu {
        header: PduHeader,
        pdu_type: PduType::RemoveEntity,
        protocol_family: ProtocolFamily::SimulationManagement,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub request_id: u32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.5.4
    pub struct StartResumePdu {
        header: PduHeader,
        pdu_type: PduType::StartResume,
        protocol_family: ProtocolFamily::SimulationManagement,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub real_world_time: ClockTime,
            pub simulation_time: ClockTime,
            pub request_id: u32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.5.5
    pub struct StopFreezePdu {
        header: PduHeader,
        pdu_type: PduType::StopFreeze,
        protocol_family: ProtocolFamily::SimulationManagement,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub real_world_time: ClockTime,
            pub reason: Reason,
            pub frozen_behavior: FrozenBehavior,
            padding: u16,
            pub request_id: u32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.5.6
    pub struct AcknowledgePdu {
        header: PduHeader,
        pdu_type: PduType::Acknowledge,
        protocol_family: ProtocolFamily::SimulationManagement,
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
    /// Implemented according to IEEE 1278.1-2012 §7.5.7
    pub struct ActionRequestPdu {
        header: PduHeader,
        pdu_type: PduType::ActionRequest,
        protocol_family: ProtocolFamily::SimulationManagement,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub request_id: u32,
            pub action_id: u32,
            pub number_of_fixed_datum_records: u32,
            pub number_of_variable_datum_records: u32,
            pub fixed_datum_records: Vec<FixedDatumRecord>,
            pub variable_datum_records: Vec<VariableDatumRecord>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.5.8
    pub struct ActionResponsePdu {
        header: PduHeader,
        pdu_type: PduType::ActionResponse,
        protocol_family: ProtocolFamily::SimulationManagement,
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
    /// Implemented according to IEEE 1278.1-2012 §7.5.9
    pub struct DataQueryPdu {
        header: PduHeader,
        pdu_type: PduType::DataQuery,
        protocol_family: ProtocolFamily::SimulationManagement,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub request_id: u32,
            pub time_interval: u32,
            pub number_of_fixed_datum_records: u32,
            pub number_of_variable_datum_records: u32,
            pub fixed_datum_records: Vec<FixedDatumRecord>,
            pub variable_datum_records: Vec<VariableDatumRecord>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.5.10
    pub struct SetDataPdu {
        header: PduHeader,
        pdu_type: PduType::SetData,
        protocol_family: ProtocolFamily::SimulationManagement,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub request_id: u32,
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
    /// Implemented according to IEEE 1278.1-2012 §7.5.11
    pub struct DataPdu {
        header: PduHeader,
        pdu_type: PduType::Data,
        protocol_family: ProtocolFamily::SimulationManagement,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub request_id: u32,
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
    /// Implemented according to IEEE 1278.1-2012 §7.5.12
    pub struct EventReportPdu {
        header: PduHeader,
        pdu_type: PduType::EventReport,
        protocol_family: ProtocolFamily::SimulationManagement,
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
    /// Implemented according to IEEE 1278.1-2012 §7.5.13
    pub struct CommentPdu {
        header: PduHeader,
        pdu_type: PduType::Comment,
        protocol_family: ProtocolFamily::SimulationManagement,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{common::pdu::Pdu, pdu_tests};

    pdu_tests!(
        create_entity_pdu_tests, CreateEntityPdu,
        pdu_type: PduType::CreateEntity,
        default_bits: 224,
        wrong_type: RemoveEntityPdu,
    );
    pdu_tests!(
        remove_entity_pdu_tests, RemoveEntityPdu,
        pdu_type: PduType::RemoveEntity,
        default_bits: 224,
        wrong_type: StopFreezePdu,
    );
    pdu_tests!(
        stop_freeze_pdu_tests, StopFreezePdu,
        pdu_type: PduType::StopFreeze,
        default_bits: 320,
        wrong_type: StartResumePdu,
    );
    pdu_tests!(
        start_resume_pdu_tests, StartResumePdu,
        pdu_type: PduType::StartResume,
        default_bits: 352,
        wrong_type: AcknowledgePdu,
    );
    pdu_tests!(
        acknowledge_pdu_tests, AcknowledgePdu,
        pdu_type: PduType::Acknowledge,
        default_bits: 256,
        wrong_type: ActionRequestPdu,
    );
    pdu_tests!(
        action_request_pdu_tests, ActionRequestPdu,
        pdu_type: PduType::ActionRequest,
        default_bits: 320,
        wrong_type: ActionResponsePdu,
    );
    pdu_tests!(
        action_response_pdu_tests, ActionResponsePdu,
        pdu_type: PduType::ActionResponse,
        default_bits: 320,
        wrong_type: DataPdu,
    );
    pdu_tests!(
        data_pdu_tests, DataPdu,
        pdu_type: PduType::Data,
        default_bits: 320,
        wrong_type: SetDataPdu,
    );
    pdu_tests!(
        set_data_pdu_tests, SetDataPdu,
        pdu_type: PduType::SetData,
        default_bits: 320,
        wrong_type: DataQueryPdu,
    );
    pdu_tests!(
        data_query_pdu_tests, DataQueryPdu,
        pdu_type: PduType::DataQuery,
        default_bits: 320,
        wrong_type: EventReportPdu,
    );
    pdu_tests!(
        event_report_pdu_tests, EventReportPdu,
        pdu_type: PduType::EventReport,
        default_bits: 320,
        wrong_type: CommentPdu,
    );
    pdu_tests!(
        comment_pdu_tests, CommentPdu,
        pdu_type: PduType::Comment,
        default_bits: 256,
        wrong_type: DataQueryPdu,
    );
}
