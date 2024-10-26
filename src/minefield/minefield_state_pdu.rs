//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    dis_error::DISError,
    entity_type::EntityType,
    euler_angles::EulerAngles,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    vector3_double::Vector3Double,
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use super::data_types::{minefield_identifier::MinefieldIdentifier, point::Point};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.9.2
pub struct MinefieldStatePdu {
    pub pdu_header: PduHeader,
    pub minefield_id: MinefieldIdentifier,
    pub minefield_sequence: u16,
    pub force_id: u8,
    pub number_of_perimeter_points: u8,
    pub minefield_type: EntityType,
    pub number_of_mine_types: u16,
    pub minefield_location: Vector3Double,
    pub minefield_orientation: EulerAngles,
    pub appearance: u16,
    pub protocol_mode: u16,
    pub perimeter_points: Vec<Point>,
    pub mine_type: Vec<EntityType>,
}

impl Default for MinefieldStatePdu {
    fn default() -> Self {
        MinefieldStatePdu {
            pdu_header: PduHeader::default(PduType::MinefieldState, ProtocolFamily::Minefield, 56),
            minefield_id: MinefieldIdentifier::default(),
            minefield_sequence: 0,
            force_id: 0,
            number_of_perimeter_points: 0,
            minefield_type: EntityType::default(),
            number_of_mine_types: 0,
            minefield_location: Vector3Double::default(),
            minefield_orientation: EulerAngles::default(),
            appearance: 0,
            protocol_mode: 0,
            perimeter_points: vec![Point::default()],
            mine_type: vec![EntityType::default()],
        }
    }
}

impl Pdu for MinefieldStatePdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.minefield_id.serialize(buf);
        buf.put_u16(self.minefield_sequence);
        buf.put_u8(self.force_id);
        buf.put_u8(self.number_of_perimeter_points);
        self.minefield_type.serialize(buf);
        buf.put_u16(self.number_of_mine_types);
        self.minefield_location.serialize(buf);
        self.minefield_orientation.serialize(buf);
        buf.put_u16(self.appearance);
        buf.put_u16(self.protocol_mode);
        for i in 0..self.perimeter_points.len() {
            self.perimeter_points[i].serialize(buf);
        }
        for i in 0..self.mine_type.len() {
            self.mine_type[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::MinefieldState {
            let minefield_id = MinefieldIdentifier::decode(&mut buffer);
            let minefield_sequence = buffer.get_u16();
            let force_id = buffer.get_u8();
            let number_of_perimeter_points = buffer.get_u8();
            let minefield_type = EntityType::decode(&mut buffer);
            let number_of_mine_types = buffer.get_u16();
            let minefield_location = Vector3Double::decode(&mut buffer);
            let minefield_orientation = EulerAngles::decode(&mut buffer);
            let appearance = buffer.get_u16();
            let protocol_mode = buffer.get_u16();
            let mut perimeter_points: Vec<Point> = vec![];
            for _i in 0..number_of_perimeter_points {
                perimeter_points.push(Point::decode(&mut buffer));
            }
            let mut mine_type: Vec<EntityType> = vec![];
            for _i in 0..number_of_mine_types {
                mine_type.push(EntityType::decode(&mut buffer));
            }

            Ok(MinefieldStatePdu {
                pdu_header,
                minefield_id,
                minefield_sequence,
                force_id,
                number_of_perimeter_points,
                minefield_type,
                number_of_mine_types,
                minefield_location,
                minefield_orientation,
                appearance,
                protocol_mode,
                perimeter_points,
                mine_type,
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
        let minefield_id = MinefieldIdentifier::decode(&mut buffer);
        let minefield_sequence = buffer.get_u16();
        let force_id = buffer.get_u8();
        let number_of_perimeter_points = buffer.get_u8();
        let minefield_type = EntityType::decode(&mut buffer);
        let number_of_mine_types = buffer.get_u16();
        let minefield_location = Vector3Double::decode(&mut buffer);
        let minefield_orientation = EulerAngles::decode(&mut buffer);
        let appearance = buffer.get_u16();
        let protocol_mode = buffer.get_u16();
        let mut perimeter_points: Vec<Point> = vec![];
        for _i in 0..number_of_perimeter_points {
            perimeter_points.push(Point::decode(&mut buffer));
        }
        let mut mine_type: Vec<EntityType> = vec![];
        for _i in 0..number_of_mine_types {
            mine_type.push(EntityType::decode(&mut buffer));
        }

        Ok(MinefieldStatePdu {
            pdu_header,
            minefield_id,
            minefield_sequence,
            force_id,
            number_of_perimeter_points,
            minefield_type,
            number_of_mine_types,
            minefield_location,
            minefield_orientation,
            appearance,
            protocol_mode,
            perimeter_points,
            mine_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MinefieldStatePdu;
    use crate::common::pdu_header::{PduHeader, PduType, ProtocolFamily};

    #[test]
    fn create_header() {
        let minefield_state_pdu = MinefieldStatePdu::default();
        let pdu_header =
            PduHeader::default(PduType::MinefieldState, ProtocolFamily::Minefield, 448 / 8);

        assert_eq!(
            pdu_header.protocol_version,
            minefield_state_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            minefield_state_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, minefield_state_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            minefield_state_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, minefield_state_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, minefield_state_pdu.pdu_header.padding);
    }
}
