//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    ClockTime,
    dis_error::DISError,
    entity_id::EntityId,
    entity_type::EntityType,
    enums::{GriddedDataConstantGrid, GriddedDataCoordinateSystem},
    euler_angles::EulerAngles,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

use super::data_types::{
    grid_axis_descriptor::GridAxisDescriptor, grid_data_record::GridDataRecord,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.10.3
pub struct GriddedDataPdu {
    pdu_header: PduHeader,
    pub environmental_simulation_id: EntityId,
    pub field_number: u16,
    pub pdu_number: u16,
    pub pdu_total: u16,
    pub coordinate_system: GriddedDataCoordinateSystem,
    pub number_of_grid_axes: u8,
    pub constant_grid: GriddedDataConstantGrid,
    pub environment_type: EntityType,
    pub orientation: EulerAngles,
    pub sample_time: ClockTime,
    pub total_values: u32,
    pub vector_dimension: u8,
    _padding: u8,
    _padding2: u16,
    pub grid_axis_descriptors: Vec<GridAxisDescriptor>,
    pub grid_data_list: Vec<GridDataRecord>,
}

impl Default for GriddedDataPdu {
    fn default() -> Self {
        GriddedDataPdu {
            pdu_header: PduHeader::default(),
            environmental_simulation_id: EntityId::default(0),
            field_number: 0,
            pdu_number: 0,
            pdu_total: 0,
            coordinate_system: GriddedDataCoordinateSystem::default(),
            number_of_grid_axes: 0,
            constant_grid: GriddedDataConstantGrid::default(),
            environment_type: EntityType::default(),
            orientation: EulerAngles::default(),
            sample_time: ClockTime::default(),
            total_values: 0,
            vector_dimension: 0,
            _padding: 0u8,
            _padding2: 0,
            grid_axis_descriptors: vec![],
            grid_data_list: vec![],
        }
    }
}

impl Pdu for GriddedDataPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>() * 3
            + std::mem::size_of::<u16>() * 3;

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.environmental_simulation_id.serialize(buf);
        buf.put_u16(self.field_number);
        buf.put_u16(self.pdu_number);
        buf.put_u16(self.pdu_total);
        buf.put_u16(self.coordinate_system as u16);
        buf.put_u8(self.number_of_grid_axes);
        buf.put_u8(self.constant_grid as u8);
        self.environment_type.serialize(buf);
        self.orientation.serialize(buf);
        self.sample_time.serialize(buf);
        buf.put_u32(self.total_values);
        buf.put_u8(self.vector_dimension);
        buf.put_u8(self._padding);
        buf.put_u16(self._padding2);
        for i in 0..self.grid_axis_descriptors.len() {
            self.grid_axis_descriptors[i].serialize(buf);
        }
        for i in 0..self.grid_data_list.len() {
            self.grid_data_list[i].serialize(buf);
        }
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::GriddedData {
            return Err(DISError::invalid_header(
                format!("Expected PDU type GriddedData, got {:?}", header.pdu_type),
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

impl GriddedDataPdu {
    /// Creates a new Entity Damage Status PDU
    ///
    /// # Examples
    ///
    /// Initializing an Entity Damage Status PDU:
    /// ```
    /// use open_dis_rust::synthetic_environment::GriddedDataPdu;
    /// let pdu = GriddedDataPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::GriddedData;
        pdu.pdu_header.protocol_family = ProtocolFamily::SyntheticEnvironment;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let environmental_simulation_id = EntityId::deserialize(buf);
        let field_number = buf.get_u16();
        let pdu_number = buf.get_u16();
        let pdu_total = buf.get_u16();
        let coordinate_system = GriddedDataCoordinateSystem::deserialize(buf);
        let number_of_grid_axes = buf.get_u8();
        let constant_grid = GriddedDataConstantGrid::deserialize(buf);
        let environment_type = EntityType::deserialize(buf);
        let orientation = EulerAngles::deserialize(buf);
        let sample_time = ClockTime::deserialize(buf);
        let total_values = buf.get_u32();
        let vector_dimension = buf.get_u8();
        let _padding = buf.get_u8();
        let _padding2 = buf.get_u16();
        let mut grid_axis_descriptors: Vec<GridAxisDescriptor> = vec![];
        for _ in 0..number_of_grid_axes {
            grid_axis_descriptors.push(GridAxisDescriptor::deserialize(buf));
        }
        let mut grid_data_list: Vec<GridDataRecord> = vec![];
        while buf.has_remaining() {
            grid_data_list.push(GridDataRecord::deserialize(buf));
        }

        GriddedDataPdu {
            pdu_header: PduHeader::default(),
            environmental_simulation_id,
            field_number,
            pdu_number,
            pdu_total,
            coordinate_system,
            number_of_grid_axes,
            constant_grid,
            environment_type,
            orientation,
            sample_time,
            total_values,
            vector_dimension,
            _padding,
            _padding2,
            grid_axis_descriptors,
            grid_data_list,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GriddedDataPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn create_header() {
        let pdu = GriddedDataPdu::default();
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
        let pdu = GriddedDataPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<GriddedDataPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = GriddedDataPdu::default();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = GriddedDataPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 512 / 8;
        let pdu = GriddedDataPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
