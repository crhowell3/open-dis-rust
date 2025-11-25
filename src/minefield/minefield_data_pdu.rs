//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::{
    common::{
        ClockTime, EntityCoordinateVector, EulerAngles, SerializedLength,
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

#[derive(Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.9.4
pub struct MinefieldDataPdu {
    pdu_header: PduHeader,
    pub minefield_id: MinefieldIdentifier,
    pub requesting_entity_id: EntityId,
    pub minefield_sequence_number: u16,
    pub request_id: u8,
    pub pdu_sequence_number: u8,
    pub number_of_pdus: u8,
    pub number_of_mines_in_this_pdu: u8,
    pub number_of_sensor_types: u8,
    padding: u8,
    pub data_filter: u32,
    pub mine_type: EntityType,
    pub sensor_types: Vec<MinefieldSensorTypes>,
    pub mine_location: Vec<EntityCoordinateVector>,
    pub ground_burial_depth_offset: Vec<Option<f32>>,
    pub water_burial_depth_offset: Vec<Option<f32>>,
    pub snow_burial_depth_offset: Vec<Option<f32>>,
    pub mine_orientation: Vec<Option<EulerAngles>>,
    pub thermal_contrast: Vec<Option<f32>>,
    pub reflectance: Vec<Option<f32>>,
    pub mine_emplacement_time: Vec<Option<ClockTime>>,
    pub mine_entity_id: Vec<Option<u16>>,
    pub fusing: Vec<Option<u16>>,
    pub scalar_detection_coefficient: Vec<Option<u8>>,
    pub paint_scheme: Vec<Option<u8>>,
    pub number_of_trip_wires: Vec<Option<u8>>,
    pub number_of_vertices: Vec<Option<u8>>,
    pub vertices: Vec<Option<Vec<EntityCoordinateVector>>>,
}

impl Pdu for MinefieldDataPdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH
            + MinefieldIdentifier::LENGTH
            + EntityId::LENGTH
            + EntityType::LENGTH
            + std::mem::size_of::<u8>() * 6
            + std::mem::size_of::<u16>()
            + std::mem::size_of::<u32>();

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
        self.requesting_entity_id.serialize(buf);
        buf.put_u16(self.minefield_sequence_number);
        buf.put_u8(self.request_id);
        buf.put_u8(self.pdu_sequence_number);
        buf.put_u8(self.number_of_pdus);
        buf.put_u8(self.number_of_mines_in_this_pdu);
        buf.put_u8(self.number_of_sensor_types);
        buf.put_u8(self.padding);
        buf.put_u32(self.data_filter);
        self.mine_type.serialize(buf);
        for i in 0..self.sensor_types.len() {
            buf.put_u16(self.sensor_types[i] as u16);
        }
        for i in 0..self.mine_location.len() {
            self.mine_location[i].serialize(buf);
        }
        // TODO(@crhowell3): Finish serialization logic
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::MinefieldData {
            return Err(DISError::invalid_header(
                format!("Expected PDU type MinefieldData, got {:?}", header.pdu_type),
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

impl MinefieldDataPdu {
    #[must_use]
    /// Creates a new `MinefieldDataPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `MinefieldDataPdu`:
    /// ```
    /// use open_dis_rust::minefield::MinefieldDataPdu;
    /// let pdu = MinefieldDataPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::MinefieldData;
        pdu.pdu_header.protocol_family = ProtocolFamily::Minefield;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let minefield_id = MinefieldIdentifier::deserialize(buf);
        let requesting_entity_id = EntityId::deserialize(buf);
        let minefield_sequence_number = buf.get_u16();
        let request_id = buf.get_u8();
        let pdu_sequence_number = buf.get_u8();
        let number_of_pdus = buf.get_u8();
        let number_of_mines_in_this_pdu = buf.get_u8();
        let number_of_sensor_types = buf.get_u8();
        let padding = buf.get_u8();
        let data_filter = buf.get_u32();
        let mine_type = EntityType::deserialize(buf);
        let mut sensor_types: Vec<MinefieldSensorTypes> = vec![];
        for _ in 0..number_of_sensor_types as usize {
            sensor_types.push(MinefieldSensorTypes::deserialize(buf));
        }
        let mut mine_location: Vec<EntityCoordinateVector> = vec![];
        let mut ground_burial_depth_offset: Vec<Option<f32>> = vec![];
        let mut water_burial_depth_offset: Vec<Option<f32>> = vec![];
        let mut snow_burial_depth_offset: Vec<Option<f32>> = vec![];
        let mut mine_orientation: Vec<Option<EulerAngles>> = vec![];
        let mut thermal_contrast: Vec<Option<f32>> = vec![];
        let mut reflectance: Vec<Option<f32>> = vec![];
        let mut mine_emplacement_time: Vec<Option<ClockTime>> = vec![];
        let mut mine_entity_id: Vec<Option<u16>> = vec![];
        let mut fusing: Vec<Option<u16>> = vec![];
        let mut scalar_detection_coefficient: Vec<Option<u8>> = vec![];
        let mut paint_scheme: Vec<Option<u8>> = vec![];
        let mut number_of_trip_wires: Vec<Option<u8>> = vec![];
        let mut number_of_vertices: Vec<Option<u8>> = vec![];
        let mut vertices: Vec<Option<Vec<EntityCoordinateVector>>> = vec![];
        for i in 0..number_of_mines_in_this_pdu as usize {
            mine_location.push(EntityCoordinateVector::deserialize(buf));
            ground_burial_depth_offset.push(Some(buf.get_f32()));
            water_burial_depth_offset.push(Some(buf.get_f32()));
            snow_burial_depth_offset.push(Some(buf.get_f32()));
            mine_orientation.push(Some(EulerAngles::deserialize(buf)));
            thermal_contrast.push(Some(buf.get_f32()));
            reflectance.push(Some(buf.get_f32()));
            mine_emplacement_time.push(Some(ClockTime::deserialize(buf)));
            mine_entity_id.push(Some(buf.get_u16()));
            fusing.push(Some(buf.get_u16()));
            scalar_detection_coefficient.push(Some(buf.get_u8()));
            paint_scheme.push(Some(buf.get_u8()));
            number_of_trip_wires.push(Some(buf.get_u8()));
            number_of_vertices.push(Some(buf.get_u8()));
            let mut vertices_vector: Vec<EntityCoordinateVector> = vec![];
            for _ in 0..number_of_vertices[i].unwrap() {
                vertices_vector.push(EntityCoordinateVector::deserialize(buf));
            }
            vertices.push(Some(vertices_vector));
        }

        MinefieldDataPdu {
            pdu_header: PduHeader::default(),
            minefield_id,
            requesting_entity_id,
            minefield_sequence_number,
            request_id,
            pdu_sequence_number,
            number_of_pdus,
            number_of_mines_in_this_pdu,
            number_of_sensor_types,
            padding,
            data_filter,
            mine_type,
            sensor_types,
            mine_location,
            ground_burial_depth_offset,
            water_burial_depth_offset,
            snow_burial_depth_offset,
            mine_orientation,
            thermal_contrast,
            reflectance,
            mine_emplacement_time,
            mine_entity_id,
            fusing,
            scalar_detection_coefficient,
            paint_scheme,
            number_of_trip_wires,
            number_of_vertices,
            vertices,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MinefieldDataPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = MinefieldDataPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<MinefieldDataPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = MinefieldDataPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = MinefieldDataPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 352 / BITS_PER_BYTE;
        let pdu = MinefieldDataPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
