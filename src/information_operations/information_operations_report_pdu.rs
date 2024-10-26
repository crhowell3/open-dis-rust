//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::{
    common::{
        dis_error::DISError,
        entity_id::EntityId,
        enums::{IOActionIOSimulationSource, IOReportIOReportType},
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    },
    warfare::data_types::standard_variable_specification::StandardVariableSpecification,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.12.3
pub struct InformationOperationsReportPdu {
    pub pdu_header: PduHeader,
    pub originating_simulation_id: EntityId,
    pub io_simulation_source: IOActionIOSimulationSource,
    pub io_report_type: IOReportIOReportType,
    pub padding1: u8,
    pub io_attacker_entity_id: EntityId,
    pub primary_target_entity_id: EntityId,
    pub padding2: u16,
    pub padding3: u16,
    pub io_records: StandardVariableSpecification,
}

impl Default for InformationOperationsReportPdu {
    /// Creates default-initialized Information Operations Report PDU
    ///
    /// # Examples
    ///
    /// Initializing an Information Operations Report PDU:
    /// ```
    /// use open_dis_rust::information_operations::information_operations_report_pdu::InformationOperationsReportPdu;
    /// let mut io_report_pdu = InformationOperationsReportPdu::default();
    /// ```
    ///
    fn default() -> Self {
        InformationOperationsReportPdu {
            pdu_header: PduHeader::default(
                PduType::InformationOperationsReport,
                ProtocolFamily::InformationOperations,
                32,
            ),
            originating_simulation_id: EntityId::default(1),
            io_simulation_source: IOActionIOSimulationSource::default(),
            io_report_type: IOReportIOReportType::default(),
            padding1: 0,
            io_attacker_entity_id: EntityId::default(3),
            primary_target_entity_id: EntityId::default(4),
            padding2: 0,
            padding3: 0,
            io_records: StandardVariableSpecification::default(),
        }
    }
}

impl Pdu for InformationOperationsReportPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_simulation_id.serialize(buf);
        buf.put_u16(self.io_simulation_source as u16);
        buf.put_u8(self.io_report_type as u8);
        buf.put_u8(self.padding1);
        self.io_attacker_entity_id.serialize(buf);
        self.primary_target_entity_id.serialize(buf);
        buf.put_u16(self.padding2);
        buf.put_u16(self.padding3);
        self.io_records.serialize(buf);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::InformationOperationsReport {
            let originating_simulation_id = EntityId::decode(&mut buffer);
            let io_simulation_source = IOActionIOSimulationSource::decode(&mut buffer);
            let io_report_type = IOReportIOReportType::decode(&mut buffer);
            let padding1 = buffer.get_u8();
            let io_attacker_entity_id = EntityId::decode(&mut buffer);
            let primary_target_entity_id = EntityId::decode(&mut buffer);
            let padding2 = buffer.get_u16();
            let padding3 = buffer.get_u16();
            let io_records = StandardVariableSpecification::decode(&mut buffer);
            Ok(InformationOperationsReportPdu {
                pdu_header,
                originating_simulation_id,
                io_simulation_source,
                io_report_type,
                padding1,
                io_attacker_entity_id,
                primary_target_entity_id,
                padding2,
                padding3,
                io_records,
            })
        } else {
            Err(DISError::InvalidDISHeader)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn deserialize_without_header(
        mut buffer: BytesMut,
        pdu_header: PduHeader,
    ) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let originating_simulation_id = EntityId::decode(&mut buffer);
        let io_simulation_source = IOActionIOSimulationSource::decode(&mut buffer);
        let io_report_type = IOReportIOReportType::decode(&mut buffer);
        let padding1 = buffer.get_u8();
        let io_attacker_entity_id = EntityId::decode(&mut buffer);
        let primary_target_entity_id = EntityId::decode(&mut buffer);
        let padding2 = buffer.get_u16();
        let padding3 = buffer.get_u16();
        let io_records = StandardVariableSpecification::decode(&mut buffer);
        Ok(InformationOperationsReportPdu {
            pdu_header,
            originating_simulation_id,
            io_simulation_source,
            io_report_type,
            padding1,
            io_attacker_entity_id,
            primary_target_entity_id,
            padding2,
            padding3,
            io_records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::InformationOperationsReportPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let information_operations_report_pdu = InformationOperationsReportPdu::default();
        let pdu_header = PduHeader::default(
            PduType::InformationOperationsReport,
            ProtocolFamily::InformationOperations,
            32,
        );

        assert_eq!(
            pdu_header.protocol_version,
            information_operations_report_pdu
                .pdu_header
                .protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            information_operations_report_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            information_operations_report_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            information_operations_report_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            information_operations_report_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.padding,
            information_operations_report_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let mut information_operations_report_pdu = InformationOperationsReportPdu::default();
        let mut buffer = BytesMut::new();
        information_operations_report_pdu.serialize(&mut buffer);

        let new_information_operations_report_pdu =
            InformationOperationsReportPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_information_operations_report_pdu.pdu_header,
            information_operations_report_pdu.pdu_header
        );
    }
}