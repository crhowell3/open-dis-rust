//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    entity_type::EntityType,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    vector3_float::Vector3Float,
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.9.4
pub struct MinefieldDataPdu {
    pub pdu_header: PduHeader,
    pub minefield_id: EntityId,
    pub requesting_entity_id: EntityId,
    pub minefield_sequence_number: u16,
    pub request_id: u8,
    pub pdu_sequence_number: u8,
    pub number_of_pdus: u8,
    pub number_of_mines_in_this_pdu: u8,
    pub number_of_sensor_types: u8,
    pub pad2: u8,
    pub data_filter: u32,
    pub mine_type: EntityType,
    pub sensor_types: Vec<u16>,
    pub pad3: u8,
    pub mine_location: Vec<Vector3Float>,
}

impl Default for MinefieldDataPdu {
    fn default() -> Self {
        MinefieldDataPdu {
            pdu_header: PduHeader::default(PduType::MinefieldData, ProtocolFamily::Minefield, 56),
            minefield_id: EntityId::default(1),
            requesting_entity_id: EntityId::default(2),
            minefield_sequence_number: 0,
            request_id: 0,
            pdu_sequence_number: 0,
            number_of_pdus: 0,
            number_of_mines_in_this_pdu: 0,
            number_of_sensor_types: 0,
            pad2: 0,
            data_filter: 0,
            mine_type: EntityType::default(),
            sensor_types: vec![0],
            pad3: 0,
            mine_location: vec![Vector3Float::new(0.0, 0.0, 0.0)],
        }
    }
}

impl Pdu for MinefieldDataPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.minefield_id.serialize(buf);
        self.requesting_entity_id.serialize(buf);
        buf.put_u16(self.minefield_sequence_number);
        buf.put_u8(self.request_id);
        buf.put_u8(self.pdu_sequence_number);
        buf.put_u8(self.number_of_pdus);
        buf.put_u8(self.number_of_mines_in_this_pdu);
        buf.put_u8(self.number_of_sensor_types);
        buf.put_u8(self.pad2);
        buf.put_u32(self.data_filter);
        self.mine_type.serialize(buf);
        for i in 0..self.sensor_types.len() {
            buf.put_u16(self.sensor_types[i]);
        }
        buf.put_u8(self.pad3);
        for i in 0..self.mine_location.len() {
            self.mine_location[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::MinefieldData {
            let minefield_id = EntityId::decode(&mut buffer);
            let requesting_entity_id = EntityId::decode(&mut buffer);
            let minefield_sequence_number = buffer.get_u16();
            let request_id = buffer.get_u8();
            let pdu_sequence_number = buffer.get_u8();
            let number_of_pdus = buffer.get_u8();
            let number_of_mines_in_this_pdu = buffer.get_u8();
            let number_of_sensor_types = buffer.get_u8();
            let pad2 = buffer.get_u8();
            let data_filter = buffer.get_u32();
            let mine_type = EntityType::decode(&mut buffer);
            let mut sensor_types: Vec<u16> = vec![];
            for _i in 0..number_of_sensor_types as usize {
                sensor_types.push(buffer.get_u16());
            }
            let pad3 = buffer.get_u8();
            let mut mine_location: Vec<Vector3Float> = vec![];
            for _i in 0..number_of_mines_in_this_pdu as usize {
                mine_location.push(Vector3Float::decode(&mut buffer));
            }

            Ok(MinefieldDataPdu {
                pdu_header,
                minefield_id,
                requesting_entity_id,
                minefield_sequence_number,
                request_id,
                pdu_sequence_number,
                number_of_pdus,
                number_of_mines_in_this_pdu,
                number_of_sensor_types,
                pad2,
                data_filter,
                mine_type,
                sensor_types,
                pad3,
                mine_location,
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
        let minefield_id = EntityId::decode(&mut buffer);
        let requesting_entity_id = EntityId::decode(&mut buffer);
        let minefield_sequence_number = buffer.get_u16();
        let request_id = buffer.get_u8();
        let pdu_sequence_number = buffer.get_u8();
        let number_of_pdus = buffer.get_u8();
        let number_of_mines_in_this_pdu = buffer.get_u8();
        let number_of_sensor_types = buffer.get_u8();
        let pad2 = buffer.get_u8();
        let data_filter = buffer.get_u32();
        let mine_type = EntityType::decode(&mut buffer);
        let mut sensor_types: Vec<u16> = vec![];
        for _i in 0..number_of_sensor_types as usize {
            sensor_types.push(buffer.get_u16());
        }
        let pad3 = buffer.get_u8();
        let mut mine_location: Vec<Vector3Float> = vec![];
        for _i in 0..number_of_mines_in_this_pdu as usize {
            mine_location.push(Vector3Float::decode(&mut buffer));
        }

        Ok(MinefieldDataPdu {
            pdu_header,
            minefield_id,
            requesting_entity_id,
            minefield_sequence_number,
            request_id,
            pdu_sequence_number,
            number_of_pdus,
            number_of_mines_in_this_pdu,
            number_of_sensor_types,
            pad2,
            data_filter,
            mine_type,
            sensor_types,
            pad3,
            mine_location,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MinefieldDataPdu;
    use crate::common::pdu_header::{PduHeader, PduType, ProtocolFamily};

    #[test]
    fn create_header() {
        let minefield_data_pdu = MinefieldDataPdu::default();
        let pdu_header =
            PduHeader::default(PduType::MinefieldData, ProtocolFamily::Minefield, 448 / 8);

        assert_eq!(
            pdu_header.protocol_version,
            minefield_data_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            minefield_data_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, minefield_data_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            minefield_data_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, minefield_data_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, minefield_data_pdu.pdu_header.padding);
    }
}
