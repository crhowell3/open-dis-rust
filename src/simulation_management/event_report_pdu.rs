//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    enums::EventType,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Copy, Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.5.12
pub struct EventReportPdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub event_type: EventType,
    pub padding: u32,
    pub number_of_fixed_datum_records: u32,
    pub number_of_variable_datum_records: u32,
    pub fixed_datum_records: u64,
    pub variable_datum_records: u64,
}

impl Default for EventReportPdu {
    fn default() -> Self {
        EventReportPdu {
            pdu_header: PduHeader::default(
                PduType::EventReport,
                ProtocolFamily::SimulationManagement,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            event_type: EventType::Other,
            padding: 0,
            number_of_fixed_datum_records: 0,
            number_of_variable_datum_records: 0,
            fixed_datum_records: 0,
            variable_datum_records: 0,
        }
    }
}

impl Pdu for EventReportPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u32(self.event_type as u32);
        buf.put_u32(self.padding);
        buf.put_u32(self.number_of_fixed_datum_records);
        buf.put_u32(self.number_of_variable_datum_records);
        buf.put_u64(self.fixed_datum_records);
        buf.put_u64(self.variable_datum_records);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::EventReport {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let event_type = EventType::decode(&mut buffer);
            let padding = buffer.get_u32();
            let number_of_fixed_datum_records = buffer.get_u32();
            let number_of_variable_datum_records = buffer.get_u32();
            let mut fixed_datum_records: u64 = 0;
            for _record in 0..number_of_fixed_datum_records as usize {
                fixed_datum_records += buffer.get_u64();
            }
            let mut variable_datum_records: u64 = 0;
            for _record in 0..number_of_variable_datum_records as usize {
                variable_datum_records += buffer.get_u64();
            }

            Ok(EventReportPdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                event_type,
                padding,
                number_of_fixed_datum_records,
                number_of_variable_datum_records,
                fixed_datum_records,
                variable_datum_records,
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
        let originating_entity_id = EntityId::decode(&mut buffer);
        let receiving_entity_id = EntityId::decode(&mut buffer);
        let event_type = EventType::decode(&mut buffer);
        let padding = buffer.get_u32();
        let number_of_fixed_datum_records = buffer.get_u32();
        let number_of_variable_datum_records = buffer.get_u32();
        let mut fixed_datum_records: u64 = 0;
        for _record in 0..number_of_fixed_datum_records as usize {
            fixed_datum_records += buffer.get_u64();
        }
        let mut variable_datum_records: u64 = 0;
        for _record in 0..number_of_variable_datum_records as usize {
            variable_datum_records += buffer.get_u64();
        }

        Ok(EventReportPdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            event_type,
            padding,
            number_of_fixed_datum_records,
            number_of_variable_datum_records,
            fixed_datum_records,
            variable_datum_records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::EventReportPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let event_report_pdu = EventReportPdu::default();
        let pdu_header = PduHeader::default(
            PduType::EventReport,
            ProtocolFamily::SimulationManagement,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            event_report_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            event_report_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, event_report_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            event_report_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, event_report_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, event_report_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut event_report_pdu = EventReportPdu::default();
        let mut buffer = BytesMut::new();
        event_report_pdu.serialize(&mut buffer);

        let new_event_report_pdu = EventReportPdu::deserialize(buffer).unwrap();
        assert_eq!(new_event_report_pdu.pdu_header, event_report_pdu.pdu_header);
    }
}
