//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::generic_header::GenericHeader;

use crate::{
    common::{
        SerializedLength,
        data_types::{
            SimulationIdentifier, entity_id::EntityId,
            standard_variable_specification::StandardVariableSpecification,
        },
        enums::{IOActionIOSimulationSource, IOReportIOReportType, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

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

#[cfg(test)]
mod tests {
    use super::InformationOperationsReportPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = InformationOperationsReportPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<InformationOperationsReportPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = InformationOperationsReportPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu =
            InformationOperationsReportPdu::deserialize(&mut deserialize_buf).expect("Some");
        assert_eq!(new_pdu.header, pdu.header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
        let pdu = InformationOperationsReportPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
