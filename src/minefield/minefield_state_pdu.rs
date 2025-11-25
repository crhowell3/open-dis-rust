//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    SerializedLength, WorldCoordinate,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_type::EntityType,
    enums::{ForceId, MinefieldStateProtocolMode, PduType, ProtocolFamily},
    euler_angles::EulerAngles,
    pdu::Pdu,
    pdu_header::PduHeader,
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use super::data_types::{minefield_identifier::MinefieldIdentifier, point::Point};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.9.2
#[derive(Default)]
pub struct MinefieldStatePdu {
    pdu_header: PduHeader,
    pub minefield_id: MinefieldIdentifier,
    pub minefield_sequence: u16,
    pub force_id: ForceId,
    pub number_of_perimeter_points: u8,
    pub minefield_type: EntityType,
    pub number_of_mine_types: u16,
    pub minefield_location: WorldCoordinate,
    pub minefield_orientation: EulerAngles,
    pub appearance: u16,
    pub protocol_mode: MinefieldStateProtocolMode,
    pub perimeter_points: Vec<Point>,
    pub mine_type: Vec<EntityType>,
}

impl Pdu for MinefieldStatePdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH
            + MinefieldIdentifier::LENGTH
            + std::mem::size_of::<u16>() * 4
            + std::mem::size_of::<u8>() * 2
            + EntityType::LENGTH
            + WorldCoordinate::LENGTH
            + EulerAngles::LENGTH;

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
        self.minefield_id.serialize(buf);
        buf.put_u16(self.minefield_sequence);
        buf.put_u8(self.force_id as u8);
        buf.put_u8(self.number_of_perimeter_points);
        self.minefield_type.serialize(buf);
        buf.put_u16(self.number_of_mine_types);
        self.minefield_location.serialize(buf);
        self.minefield_orientation.serialize(buf);
        buf.put_u16(self.appearance);
        buf.put_u16(self.protocol_mode as u16);
        for i in 0..self.perimeter_points.len() {
            self.perimeter_points[i].serialize(buf);
        }
        for i in 0..self.mine_type.len() {
            self.mine_type[i].serialize(buf);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::MinefieldState {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type MinefieldState, got {:?}",
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

impl MinefieldStatePdu {
    /// Creates a new `MinefieldStatePdu`
    ///
    /// # Examples
    ///
    /// Initializing an `MinefieldStatePdu`:
    /// ```
    /// use open_dis_rust::minefield::MinefieldStatePdu;
    /// let pdu = MinefieldStatePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::MinefieldState;
        pdu.pdu_header.protocol_family = ProtocolFamily::Minefield;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let minefield_id = MinefieldIdentifier::deserialize(buf);
        let minefield_sequence = buf.get_u16();
        let force_id = ForceId::deserialize(buf);
        let number_of_perimeter_points = buf.get_u8();
        let minefield_type = EntityType::deserialize(buf);
        let number_of_mine_types = buf.get_u16();
        let minefield_location = WorldCoordinate::deserialize(buf);
        let minefield_orientation = EulerAngles::deserialize(buf);
        let appearance = buf.get_u16();
        let protocol_mode = MinefieldStateProtocolMode::deserialize(buf);
        let mut perimeter_points: Vec<Point> = vec![];
        for _i in 0..number_of_perimeter_points {
            perimeter_points.push(Point::deserialize(buf));
        }
        let mut mine_type: Vec<EntityType> = vec![];
        for _i in 0..number_of_mine_types {
            mine_type.push(EntityType::deserialize(buf));
        }

        MinefieldStatePdu {
            pdu_header: PduHeader::default(),
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MinefieldStatePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = MinefieldStatePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<MinefieldStatePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = MinefieldStatePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = MinefieldStatePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 576 / BITS_PER_BYTE;
        let pdu = MinefieldStatePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
