use crate::dis::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Copy, Clone, Debug)]
pub struct DataPdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub padding: u32,
    pub number_of_fixed_datum_records: u32,
    pub number_of_variable_datum_records: u32,
    pub fixed_datum_records: u64,
    pub variable_datum_records: u64,
}

impl DataPdu {
    pub fn default() -> Self {
        DataPdu {
            pdu_header: PduHeader::default(PduType::Data, ProtocolFamily::SimulationManagement, 56),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            request_id: 0,
            padding: 0,
            number_of_fixed_datum_records: 0,
            number_of_variable_datum_records: 0,
            fixed_datum_records: 0,
            variable_datum_records: 0,
        }
    }
}

impl Pdu for DataPdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u32(self.request_id as u32);
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
        if pdu_header.pdu_type == PduType::Data {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let request_id = buffer.get_u32();
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

            return Ok(DataPdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                request_id,
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
        let request_id = buffer.get_u32();
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

        return Ok(DataPdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            request_id,
            padding,
            number_of_fixed_datum_records,
            number_of_variable_datum_records,
            fixed_datum_records,
            variable_datum_records,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::DataPdu;
    use crate::dis::common::pdu_header::{PduHeader, PduType, ProtocolFamily};

    #[test]
    fn create_header() {
        let action_request_pdu = DataPdu::default();
        let pdu_header =
            PduHeader::default(PduType::Data, ProtocolFamily::SimulationManagement, 448 / 8);

        assert_eq!(
            pdu_header.protocol_version,
            action_request_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            action_request_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, action_request_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            action_request_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, action_request_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, action_request_pdu.pdu_header.padding);
    }
}
