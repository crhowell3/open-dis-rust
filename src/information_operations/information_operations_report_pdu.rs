//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::{
    common::{
        SimulationIdentifier,
        constants::MAX_PDU_SIZE_OCTETS,
        dis_error::DISError,
        entity_id::EntityId,
        enums::{IOActionIOSimulationSource, IOReportIOReportType, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    warfare::data_types::standard_variable_specification::StandardVariableSpecification,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.12.3
pub struct InformationOperationsReportPdu {
    pdu_header: PduHeader,
    pub originating_simulation_id: SimulationIdentifier,
    pub io_simulation_source: IOActionIOSimulationSource,
    pub io_report_type: IOReportIOReportType,
    _padding: u8,
    pub io_attacker_entity_id: EntityId,
    pub primary_target_entity_id: EntityId,
    _padding2: u16,
    _padding3: u16,
    pub io_records: StandardVariableSpecification,
}

impl Default for InformationOperationsReportPdu {
    fn default() -> Self {
        InformationOperationsReportPdu {
            pdu_header: PduHeader::default(),
            originating_simulation_id: SimulationIdentifier::default(),
            io_simulation_source: IOActionIOSimulationSource::default(),
            io_report_type: IOReportIOReportType::default(),
            _padding: 0u8,
            io_attacker_entity_id: EntityId::default(3),
            primary_target_entity_id: EntityId::default(4),
            _padding2: 0u16,
            _padding3: 0u16,
            io_records: StandardVariableSpecification::default(),
        }
    }
}

impl Pdu for InformationOperationsReportPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<SimulationIdentifier>()
            + std::mem::size_of::<EntityId>() * 2
            + std::mem::size_of::<u8>()
            + std::mem::size_of::<u16>() * 2
            + std::mem::size_of::<IOReportIOReportType>()
            + std::mem::size_of::<IOActionIOSimulationSource>()
            + std::mem::size_of::<StandardVariableSpecification>();

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let size = std::mem::size_of_val(self);
        self.pdu_header.length = u16::try_from(size).map_err(|_| DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })?;
        self.pdu_header.serialize(buf);
        self.originating_simulation_id.serialize(buf);
        buf.put_u16(self.io_simulation_source as u16);
        buf.put_u8(self.io_report_type as u8);
        buf.put_u8(self._padding);
        self.io_attacker_entity_id.serialize(buf);
        self.primary_target_entity_id.serialize(buf);
        buf.put_u16(self._padding2);
        buf.put_u16(self._padding3);
        self.io_records.serialize(buf);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::InformationOperationsReport {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type InformationOperationsReport, got {:?}",
                    header.pdu_type
                ),
                None,
            ));
        }
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn deserialize_without_header<B: Buf>(buf: &mut B, header: PduHeader) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }
}

impl InformationOperationsReportPdu {
    /// Creates a new `InformationOperationsReportPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `InformationOperationsReportPdu`:
    /// ```
    /// use open_dis_rust::information_operations::InformationOperationsReportPdu;
    /// let pdu = InformationOperationsReportPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::InformationOperationsReport;
        pdu.pdu_header.protocol_family = ProtocolFamily::InformationOperations;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_simulation_id = SimulationIdentifier::deserialize(buf);
        let io_simulation_source = IOActionIOSimulationSource::deserialize(buf);
        let io_report_type = IOReportIOReportType::deserialize(buf);
        let _padding = buf.get_u8();
        let io_attacker_entity_id = EntityId::deserialize(buf);
        let primary_target_entity_id = EntityId::deserialize(buf);
        let _padding2 = buf.get_u16();
        let _padding3 = buf.get_u16();
        let io_records = StandardVariableSpecification::deserialize(buf);

        InformationOperationsReportPdu {
            pdu_header: PduHeader::default(),
            originating_simulation_id,
            io_simulation_source,
            io_report_type,
            _padding,
            io_attacker_entity_id,
            primary_target_entity_id,
            _padding2,
            _padding3,
            io_records,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InformationOperationsReportPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn create_header() {
        let pdu = InformationOperationsReportPdu::new();
        let pdu_header = PduHeader::default();

        assert_eq!(pdu_header.protocol_version, pdu.pdu_header.protocol_version);
        assert_eq!(pdu_header.exercise_id, pdu.pdu_header.exercise_id);
        assert_eq!(pdu_header.pdu_type, pdu.pdu_header.pdu_type);
        assert_eq!(pdu_header.protocol_family, pdu.pdu_header.protocol_family);
        assert_eq!(pdu_header.length, pdu.pdu_header.length);
        assert_eq!(pdu_header.status_record, pdu.pdu_header.status_record);
    }

    #[test]
    fn cast_to_any() {
        let pdu = InformationOperationsReportPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<InformationOperationsReportPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = InformationOperationsReportPdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = InformationOperationsReportPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 320 / 8;
        let pdu = InformationOperationsReportPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
