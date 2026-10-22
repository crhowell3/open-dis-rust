//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Information Operations protocol family

use crate::common::enums::{
    IOActionIOActionPhase, IOActionIOActionType, IOActionIOSimulationSource, IOActionIOWarfareType,
    IOReportIOReportType, PduType, ProtocolFamily,
};
use crate::common::{
    SerializedLength,
    data_types::{EntityId, SimulationIdentifier, StandardVariableSpecification},
    generic_header::GenericHeader,
    pdu::Pdu,
    pdu_header::PduHeader,
};
use crate::define_pdu;

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.12.3
    pub struct InformationOperationsReportPdu {
        header: PduHeader,
        pdu_type: PduType::InformationOperationsReport,
        protocol_family: ProtocolFamily::InformationOperations,
        fields: {
            pub originating_simulation_id: SimulationIdentifier,
            pub io_simulation_source: IOActionIOSimulationSource,
            pub io_report_type: IOReportIOReportType,
            padding: u8,
            pub io_attacker_entity_id: EntityId,
            pub primary_target_entity_id: EntityId,
            padding2: u16,
            padding3: u16,
            pub io_records: StandardVariableSpecification,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.12.2
    pub struct InformationOperationsActionPdu {
        header: PduHeader,
        pdu_type: PduType::InformationOperationsAction,
        protocol_family: ProtocolFamily::InformationOperations,
        fields: {
            pub originating_simulation_id: SimulationIdentifier,
            pub receiving_simulation_id: SimulationIdentifier,
            pub request_id: u32,
            pub io_warfare_type: IOActionIOWarfareType,
            pub io_simulation_source: IOActionIOSimulationSource,
            pub io_action_type: IOActionIOActionType,
            pub io_action_phase: IOActionIOActionPhase,
            padding: u32,
            pub io_attacker_entity_id: EntityId,
            pub io_primary_target_entity_id: EntityId,
            padding2: u16,
            pub io_records: StandardVariableSpecification,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{common::pdu::Pdu, pdu_tests};

    pdu_tests!(
        information_operations_actions_pdu, InformationOperationsActionPdu,
        pdu_type: PduType::InformationOperationsAction,
        default_bits: 448,
        wrong_type: InformationOperationsReportPdu,
    );
    pdu_tests!(
        information_operations_report_pdu, InformationOperationsReportPdu,
        pdu_type: PduType::InformationOperationsReport,
        default_bits: 320,
        wrong_type: InformationOperationsActionPdu,
    );
}
