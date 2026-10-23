//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Information Operations protocol family

use crate::common::{
    data_types::{EntityId, SimulationIdentifier, StandardVariableSpecification},
    enums::{
        IOActionIOActionPhase, IOActionIOActionType, IOActionIOSimulationSource,
        IOActionIOWarfareType, IOReportIOReportType, PduType, ProtocolFamily,
    },
    generic_header::GenericHeader,
    pdu::Pdu,
    pdu_header::PduHeader,
};

/// Implemented according to IEEE 1278.1-2012 §7.12.3
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct InformationOperationsReportPdu {
    #[deku(skip)]
    header: PduHeader,
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
crate::impl_pdu!(
    InformationOperationsReportPdu,
    PduHeader,
    PduType::InformationOperationsReport,
    ProtocolFamily::InformationOperations
);

/// Implemented according to IEEE 1278.1-2012 §7.12.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct InformationOperationsActionPdu {
    #[deku(skip)]
    header: PduHeader,
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
crate::impl_pdu!(
    InformationOperationsActionPdu,
    PduHeader,
    PduType::InformationOperationsAction,
    ProtocolFamily::InformationOperations
);

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
