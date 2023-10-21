use crate::dis::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};
use bytes::{Buf, BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Copy, Clone, Debug)]
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

impl EventReportPdu {
    pub fn default() -> Self {
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
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u32(self.event_type as u32);
        buf.put_u32(self.padding as u32);
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

            return Ok(EventReportPdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                event_type,
                padding,
                number_of_fixed_datum_records,
                number_of_variable_datum_records,
                fixed_datum_records,
                variable_datum_records,
            });
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

        return Ok(EventReportPdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            event_type,
            padding,
            number_of_fixed_datum_records,
            number_of_variable_datum_records,
            fixed_datum_records,
            variable_datum_records,
        });
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    Other = 0,
    RanOutOfAmmunition = 2,
    KilledInAction = 3,
    Damage = 4,
    MobilityDisabled = 5,
    FireDisabled = 6,
    RanOutOfFuel = 7,
    EntityInitialization = 8,
    RequestForIndirectFireOrCASMission = 9,
    IndirectFireOrCASFire = 10,
    MinefieldEntry = 11,
    MinefieldDetonation = 12,
    VehicleMasterPowerOn = 13,
    VehicleMasterPowerOff = 14,
    AggregateStateChangeRequested = 15,
    PreventCollisionDetonation = 16,
    OwnershipReport = 17,
    RadarPerception = 18,
    Detect = 19,
}

impl EventType {
    pub fn decode(buf: &mut BytesMut) -> EventType {
        match buf.get_u32() {
            0 => EventType::Other,
            2 => EventType::RanOutOfAmmunition,
            3 => EventType::KilledInAction,
            4 => EventType::Damage,
            5 => EventType::MobilityDisabled,
            6 => EventType::FireDisabled,
            7 => EventType::RanOutOfFuel,
            8 => EventType::EntityInitialization,
            9 => EventType::RequestForIndirectFireOrCASMission,
            10 => EventType::IndirectFireOrCASFire,
            11 => EventType::MinefieldEntry,
            12 => EventType::MinefieldDetonation,
            13 => EventType::VehicleMasterPowerOn,
            14 => EventType::VehicleMasterPowerOff,
            15 => EventType::AggregateStateChangeRequested,
            16 => EventType::PreventCollisionDetonation,
            17 => EventType::OwnershipReport,
            18 => EventType::RadarPerception,
            19 => EventType::Detect,
            _ => EventType::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EventReportPdu;
    use crate::dis::common::{
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
        let event_report_pdu = EventReportPdu::default();
        let mut buffer = BytesMut::new();
        event_report_pdu.serialize(&mut buffer);

        let new_event_report_pdu = EventReportPdu::deserialize(buffer).unwrap();
        assert_eq!(new_event_report_pdu.pdu_header, event_report_pdu.pdu_header);
    }
}
