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
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use super::data_types::point::Point;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.9.3
pub struct MinefieldQueryPdu {
    pub pdu_header: PduHeader,
    pub minefield_id: EntityId,
    pub requesting_entity_id: EntityId,
    pub request_id: u8,
    pub number_of_perimeter_points: u8,
    pub pad2: u8,
    pub number_of_sensor_types: u8,
    pub data_filter: u32,
    pub requested_mine_type: EntityType,
    pub requested_perimeter_points: Vec<Point>,
    pub sensor_types: Vec<u16>,
}

impl Default for MinefieldQueryPdu {
    fn default() -> Self {
        MinefieldQueryPdu {
            pdu_header: PduHeader::default(PduType::MinefieldQuery, ProtocolFamily::Minefield, 56),
            minefield_id: EntityId::default(1),
            requesting_entity_id: EntityId::default(2),
            request_id: 0,
            number_of_perimeter_points: 0,
            pad2: 0,
            number_of_sensor_types: 0,
            data_filter: 0,
            requested_mine_type: EntityType::default(),
            requested_perimeter_points: vec![Point::default()],
            sensor_types: vec![0],
        }
    }
}

impl Pdu for MinefieldQueryPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.minefield_id.serialize(buf);
        self.requesting_entity_id.serialize(buf);
        buf.put_u8(self.request_id);
        buf.put_u8(self.number_of_perimeter_points);
        buf.put_u8(self.pad2);
        buf.put_u8(self.number_of_sensor_types);
        buf.put_u32(self.data_filter);
        self.requested_mine_type.serialize(buf);
        for i in 0..self.requested_perimeter_points.len() {
            self.requested_perimeter_points[i].serialize(buf);
        }
        for i in 0..self.sensor_types.len() {
            buf.put_u16(self.sensor_types[i]);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::MinefieldQuery {
            let minefield_id = EntityId::deserialize(&mut buffer);
            let requesting_entity_id = EntityId::deserialize(&mut buffer);
            let request_id = buffer.get_u8();
            let number_of_perimeter_points = buffer.get_u8();
            let pad2 = buffer.get_u8();
            let number_of_sensor_types = buffer.get_u8();
            let data_filter = buffer.get_u32();
            let requested_mine_type = EntityType::deserialize(&mut buffer);
            let mut requested_perimeter_points: Vec<Point> = vec![];
            for _i in 0..number_of_perimeter_points as usize {
                requested_perimeter_points.push(Point::deserialize(&mut buffer));
            }
            let mut sensor_types: Vec<u16> = vec![];
            for _i in 0..number_of_sensor_types as usize {
                sensor_types.push(buffer.get_u16());
            }

            Ok(MinefieldQueryPdu {
                pdu_header,
                minefield_id,
                requesting_entity_id,
                request_id,
                number_of_perimeter_points,
                pad2,
                number_of_sensor_types,
                data_filter,
                requested_mine_type,
                requested_perimeter_points,
                sensor_types,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type MinefieldQuery, got {:?}",
                    pdu_header.pdu_type
                ),
                None,
            ))
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
        let minefield_id = EntityId::deserialize(&mut buffer);
        let requesting_entity_id = EntityId::deserialize(&mut buffer);
        let request_id = buffer.get_u8();
        let number_of_perimeter_points = buffer.get_u8();
        let pad2 = buffer.get_u8();
        let number_of_sensor_types = buffer.get_u8();
        let data_filter = buffer.get_u32();
        let requested_mine_type = EntityType::deserialize(&mut buffer);
        let mut requested_perimeter_points: Vec<Point> = vec![];
        for _i in 0..number_of_perimeter_points as usize {
            requested_perimeter_points.push(Point::deserialize(&mut buffer));
        }
        let mut sensor_types: Vec<u16> = vec![];
        for _i in 0..number_of_sensor_types as usize {
            sensor_types.push(buffer.get_u16());
        }

        Ok(MinefieldQueryPdu {
            pdu_header,
            minefield_id,
            requesting_entity_id,
            request_id,
            number_of_perimeter_points,
            pad2,
            number_of_sensor_types,
            data_filter,
            requested_mine_type,
            requested_perimeter_points,
            sensor_types,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MinefieldQueryPdu;
    use crate::common::pdu_header::{PduHeader, PduType, ProtocolFamily};

    #[test]
    fn create_header() {
        let minefield_query_pdu = MinefieldQueryPdu::default();
        let pdu_header =
            PduHeader::default(PduType::MinefieldQuery, ProtocolFamily::Minefield, 448 / 8);

        assert_eq!(
            pdu_header.protocol_version,
            minefield_query_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            minefield_query_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, minefield_query_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            minefield_query_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, minefield_query_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, minefield_query_pdu.pdu_header.padding);
    }
}
