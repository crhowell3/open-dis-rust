//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Simulation Management (SIMAN) protocol family

use crate::common::data_types::ClockTime;
use crate::common::data_types::datum_records::{FixedDatumRecord, VariableDatumRecord};
use crate::common::enums::{ActionResponseRequestStatus, EventType, FrozenBehavior, Reason};
use crate::common::generic_header::GenericHeader;
use crate::{
    common::{
        SerializedLength,
        data_types::entity_id::EntityId,
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
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    mod create_entity_pdu_tests {
        use super::*;
        #[test]
        fn cast_to_any() {
            let pdu = CreateEntityPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<CreateEntityPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = CreateEntityPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = CreateEntityPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
            let pdu = CreateEntityPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod remove_entity_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = RemoveEntityPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<RemoveEntityPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = RemoveEntityPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = RemoveEntityPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
            let pdu = RemoveEntityPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod stop_freeze_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = StopFreezePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<StopFreezePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = StopFreezePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = StopFreezePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = StopFreezePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod start_resume_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = StartResumePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<StartResumePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = StartResumePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = StartResumePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 352 / BITS_PER_BYTE;
            let pdu = StartResumePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod acknowledge_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = AcknowledgePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<AcknowledgePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = AcknowledgePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = AcknowledgePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
            let pdu = AcknowledgePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod action_request_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = ActionRequestPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<ActionRequestPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = ActionRequestPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = ActionRequestPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = ActionRequestPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod action_response_pdu_tests {
        use super::*;
        #[test]
        fn cast_to_any() {
            let pdu = ActionResponsePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<ActionResponsePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = ActionResponsePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = ActionResponsePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = ActionResponsePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod data_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = DataPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<DataPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = DataPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = DataPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = DataPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod set_data_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = SetDataPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<SetDataPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = SetDataPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = SetDataPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = SetDataPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod data_query_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = DataQueryPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<DataQueryPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = DataQueryPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = DataQueryPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = DataQueryPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod event_report_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = EventReportPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<EventReportPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = EventReportPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = EventReportPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = EventReportPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod comment_pdu_tests {
        use super::*;
        #[test]
        fn cast_to_any() {
            let pdu = CommentPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<CommentPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = CommentPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = CommentPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
            let pdu = CommentPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }
}
