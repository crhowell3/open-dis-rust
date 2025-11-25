//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::{
    common::{
        SerializedLength,
        constants::MAX_PDU_SIZE_OCTETS,
        dis_error::DISError,
        entity_id::EntityId,
        entity_type::EntityType,
        enums::{MinefieldSensorTypes, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    minefield::data_types::minefield_identifier::MinefieldIdentifier,
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use super::data_types::point::Point;

#[derive(Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.9.3
pub struct MinefieldQueryPdu {
    pdu_header: PduHeader,
    pub minefield_id: MinefieldIdentifier,
    pub requesting_entity_id: EntityId,
    pub request_id: u8,
    pub number_of_perimeter_points: u8,
    padding: u8,
    pub number_of_sensor_types: u8,
    pub data_filter: u32,
    pub requested_mine_type: EntityType,
    pub requested_perimeter_points: Vec<Point>,
    pub sensor_types: Vec<MinefieldSensorTypes>,
}

impl Pdu for MinefieldQueryPdu {
    fn length(&self) -> Result<u16, DISError> {
        let length = PduHeader::LENGTH
            + MinefieldIdentifier::LENGTH
            + EntityId::LENGTH
            + std::mem::size_of::<u8>() * 4
            + std::mem::size_of::<u32>()
            + EntityType::LENGTH;

        u16::try_from(length).map_err(|_| DISError::PduSizeExceeded {
            size: length,
            max_size: MAX_PDU_SIZE_OCTETS,
        })
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
        self.minefield_id.serialize(buf);
        self.requesting_entity_id.serialize(buf);
        buf.put_u8(self.request_id);
        buf.put_u8(self.number_of_perimeter_points);
        buf.put_u8(self.padding);
        buf.put_u8(self.number_of_sensor_types);
        buf.put_u32(self.data_filter);
        self.requested_mine_type.serialize(buf);
        for i in 0..self.requested_perimeter_points.len() {
            self.requested_perimeter_points[i].serialize(buf);
        }
        for i in 0..self.sensor_types.len() {
            buf.put_u16(self.sensor_types[i] as u16);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::MinefieldQuery {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type MinefieldQuery, got {:?}",
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

impl MinefieldQueryPdu {
    #[must_use]
    /// Creates a new `MinefieldQueryPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `MinefieldQueryPdu`:
    /// ```
    /// use open_dis_rust::minefield::MinefieldQueryPdu;
    /// let pdu = MinefieldQueryPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::MinefieldQuery;
        pdu.pdu_header.protocol_family = ProtocolFamily::Minefield;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let minefield_id = MinefieldIdentifier::deserialize(buf);
        let requesting_entity_id = EntityId::deserialize(buf);
        let request_id = buf.get_u8();
        let number_of_perimeter_points = buf.get_u8();
        let padding = buf.get_u8();
        let number_of_sensor_types = buf.get_u8();
        let data_filter = buf.get_u32();
        let requested_mine_type = EntityType::deserialize(buf);
        let mut requested_perimeter_points: Vec<Point> = vec![];
        for _i in 0..number_of_perimeter_points as usize {
            requested_perimeter_points.push(Point::deserialize(buf));
        }
        let mut sensor_types: Vec<MinefieldSensorTypes> = vec![];
        for _i in 0..number_of_sensor_types as usize {
            sensor_types.push(MinefieldSensorTypes::deserialize(buf));
        }

        MinefieldQueryPdu {
            pdu_header: PduHeader::default(),
            minefield_id,
            requesting_entity_id,
            request_id,
            number_of_perimeter_points,
            padding,
            number_of_sensor_types,
            data_filter,
            requested_mine_type,
            requested_perimeter_points,
            sensor_types,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MinefieldQueryPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = MinefieldQueryPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<MinefieldQueryPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = MinefieldQueryPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = MinefieldQueryPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
        let pdu = MinefieldQueryPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
