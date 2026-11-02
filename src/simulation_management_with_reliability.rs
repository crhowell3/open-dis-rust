//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Simulation Management with Reliability (SIMAN-R) protocol family

use crate::common::{
    data_types::{
        ClockTime, EntityId, RecordSpecification, {FixedDatumRecord, VariableDatumRecord},
    },
    enums::{
        AcknowledgeFlag, AcknowledgeResponseFlag, ActionResponseRequestStatus, EventType,
        FrozenBehavior, PduType, ProtocolFamily, Reason, RecordQueryREventType, RecordREventType,
        RequiredReliabilityService, VariableRecordTypes,
    },
    pdu_header::PduHeader,
};

/// Implemented according to IEEE 1278.1-2012 §7.11.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct CreateEntityReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    padding: u8,
    padding2: u16,
    pub request_id: u32,
}
crate::impl_pdu!(
    CreateEntityReliablePdu,
    PduHeader,
    PduType::CreateEntityReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.3
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct RemoveEntityReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    padding: u8,
    padding2: u16,
    pub request_id: u32,
}
crate::impl_pdu!(
    RemoveEntityReliablePdu,
    PduHeader,
    PduType::RemoveEntityReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.4
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct StartResumeReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub real_world_time: ClockTime,
    pub simulation_time: ClockTime,
    pub required_reliability_service: RequiredReliabilityService,
    padding: u8,
    padding2: u16,
    pub request_id: u32,
}
crate::impl_pdu!(
    StartResumeReliablePdu,
    PduHeader,
    PduType::StartResumeReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.5
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct StopFreezeReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub real_world_time: ClockTime,
    pub reason: Reason,
    pub frozen_behavior: FrozenBehavior,
    pub required_reliability_service: u8,
    padding: u8,
    pub request_id: u32,
}
crate::impl_pdu!(
    StopFreezeReliablePdu,
    PduHeader,
    PduType::StopFreezeReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.6
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct AcknowledgeReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub acknowledge_flag: AcknowledgeFlag,
    pub response_flag: AcknowledgeResponseFlag,
    pub request_id: u32,
}
crate::impl_pdu!(
    AcknowledgeReliablePdu,
    PduHeader,
    PduType::AcknowledgeReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.7
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct ActionRequestReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    padding: u8,
    padding2: u16,
    pub request_id: u32,
    pub action_id: u32,
    padding3: u32,
    #[deku(
        update = "self.fixed_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_fixed_datum_records)"
    )]
    pub number_of_fixed_datum_records: u32,
    #[deku(
        update = "self.variable_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_variable_datum_records)"
    )]
    pub number_of_variable_datum_records: u32,
    #[deku(count = "number_of_fixed_datum_records")]
    pub fixed_datum_records: Vec<FixedDatumRecord>,
    #[deku(count = "number_of_variable_datum_records")]
    pub variable_datum_records: Vec<VariableDatumRecord>,
}
crate::impl_pdu!(
    ActionRequestReliablePdu,
    PduHeader,
    PduType::ActionRequestReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.8
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct ActionResponseReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub request_status: ActionResponseRequestStatus,
    #[deku(
        update = "self.fixed_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_fixed_datum_records)"
    )]
    pub number_of_fixed_datum_records: u32,
    #[deku(
        update = "self.variable_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_variable_datum_records)"
    )]
    pub number_of_variable_datum_records: u32,
    #[deku(count = "number_of_fixed_datum_records")]
    pub fixed_datum_records: Vec<FixedDatumRecord>,
    #[deku(count = "number_of_variable_datum_records")]
    pub variable_datum_records: Vec<VariableDatumRecord>,
}
crate::impl_pdu!(
    ActionResponseReliablePdu,
    PduHeader,
    PduType::ActionResponseReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.9
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct DataQueryReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    padding: u8,
    padding2: u16,
    pub request_id: u32,
    pub time_interval: u32,
    #[deku(
        update = "self.fixed_datum_ids.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_fixed_datum_records)"
    )]
    pub number_of_fixed_datum_records: u32,
    #[deku(
        update = "self.variable_datum_ids.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_variable_datum_records)"
    )]
    pub number_of_variable_datum_records: u32,
    #[deku(count = "number_of_fixed_datum_records")]
    pub fixed_datum_ids: Vec<VariableRecordTypes>,
    #[deku(count = "number_of_variable_datum_records")]
    pub variable_datum_ids: Vec<VariableRecordTypes>,
}
crate::impl_pdu!(
    DataQueryReliablePdu,
    PduHeader,
    PduType::DataQueryReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.10
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct SetDataReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub required_reliability_service: RequiredReliabilityService,
    padding: u8,
    padding2: u16,
    pub request_id: u32,
    #[deku(
        update = "self.fixed_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_fixed_datum_records)"
    )]
    pub number_of_fixed_datum_records: u32,
    #[deku(
        update = "self.variable_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_variable_datum_records)"
    )]
    pub number_of_variable_datum_records: u32,
    #[deku(count = "number_of_fixed_datum_records")]
    pub fixed_datum_records: Vec<FixedDatumRecord>,
    #[deku(count = "number_of_variable_datum_records")]
    pub variable_datum_records: Vec<VariableDatumRecord>,
}
crate::impl_pdu!(
    SetDataReliablePdu,
    PduHeader,
    PduType::SetDataReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.11
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct DataReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub required_reliability_service: RequiredReliabilityService,
    padding: u8,
    padding2: u16,
    #[deku(
        update = "self.fixed_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_fixed_datum_records)"
    )]
    pub number_of_fixed_datum_records: u32,
    #[deku(
        update = "self.variable_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_variable_datum_records)"
    )]
    pub number_of_variable_datum_records: u32,
    #[deku(count = "number_of_fixed_datum_records")]
    pub fixed_datum_records: Vec<FixedDatumRecord>,
    #[deku(count = "number_of_variable_datum_records")]
    pub variable_datum_records: Vec<VariableDatumRecord>,
}
crate::impl_pdu!(
    DataReliablePdu,
    PduHeader,
    PduType::DataReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.12
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct EventReportReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub event_type: EventType,
    padding: u32,
    #[deku(
        update = "self.fixed_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_fixed_datum_records)"
    )]
    pub number_of_fixed_datum_records: u32,
    #[deku(
        update = "self.variable_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_variable_datum_records)"
    )]
    pub number_of_variable_datum_records: u32,
    #[deku(count = "number_of_fixed_datum_records")]
    pub fixed_datum_records: Vec<FixedDatumRecord>,
    #[deku(count = "number_of_variable_datum_records")]
    pub variable_datum_records: Vec<VariableDatumRecord>,
}
crate::impl_pdu!(
    EventReportReliablePdu,
    PduHeader,
    PduType::EventReportReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.13
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct CommentReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    #[deku(
        update = "self.fixed_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_fixed_datum_records)"
    )]
    pub number_of_fixed_datum_records: u32,
    #[deku(
        update = "self.variable_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_variable_datum_records)"
    )]
    pub number_of_variable_datum_records: u32,
    #[deku(count = "number_of_fixed_datum_records")]
    pub fixed_datum_records: Vec<FixedDatumRecord>,
    #[deku(count = "number_of_variable_datum_records")]
    pub variable_datum_records: Vec<VariableDatumRecord>,
}
crate::impl_pdu!(
    CommentReliablePdu,
    PduHeader,
    PduType::CommentReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.14
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct RecordQueryReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub required_reliability_service: RequiredReliabilityService,
    padding: u8,
    pub event_type: RecordQueryREventType,
    pub time: u32,
    #[deku(
        update = "self.record_ids.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_records)"
    )]
    pub number_of_records: u32,
    #[deku(count = "number_of_records")]
    pub record_ids: Vec<u32>,
}
crate::impl_pdu!(
    RecordQueryReliablePdu,
    PduHeader,
    PduType::RecordQueryReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.15
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct SetRecordReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub required_reliability_service: RequiredReliabilityService,
    padding: u8,
    padding2: u16,
    padding3: u32,
    pub record_sets: RecordSpecification,
}
crate::impl_pdu!(
    SetRecordReliablePdu,
    PduHeader,
    PduType::SetRecordReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

/// Implemented according to IEEE 1278.1-2012 §7.11.15
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct RecordReliablePdu {
    #[deku(skip)]
    header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub required_reliability_service: RequiredReliabilityService,
    padding: u8,
    pub event_type: RecordREventType,
    pub response_serial_number: u32,
    pub record_sets: RecordSpecification,
}
crate::impl_pdu!(
    RecordReliablePdu,
    PduHeader,
    PduType::RecordReliable,
    ProtocolFamily::SimulationManagementWithReliability
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::{constants::BITS_PER_BYTE, pdu::Pdu},
        pdu_tests,
    };
    use bytes::{Buf, Bytes, BytesMut};

    /// Serialize a PDU, failing the test if serialization returns an error.
    pub fn to_bytes<P: Pdu>(pdu: &mut P) -> Bytes {
        let mut buf = BytesMut::new();
        pdu.serialize(&mut buf).expect("serialize failed");
        buf.freeze()
    }

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
    pdu_tests!(
        record_reliable_pdu_tests, RecordReliablePdu,
        pdu_type: PduType::RecordReliable,
        default_bits: 320,
        wrong_type: CommentReliablePdu,
    );
}
