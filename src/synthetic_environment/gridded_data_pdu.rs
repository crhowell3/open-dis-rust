//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
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
    pub pdu_header: PduHeader,
    pub environmental_simulation_id: EntityId,
    pub field_number: u16,
    pub pdu_number: u16,
    pub pdu_total: u16,
    pub coordinate_system: GriddedDataCoordinateSystem,
    pub number_of_grid_axes: u8,
    pub constant_grid: GriddedDataConstantGrid,
    pub environment_type: EntityType,
    pub orientation: EulerAngles,
    pub sample_time: u64,
    pub total_values: u32,
    pub vector_dimension: u8,
    pub padding1: u8,
    pub padding2: u16,
    pub grid_axis_descriptors: Vec<GridAxisDescriptor>,
    pub grid_data_list: Vec<GridDataRecord>,
}

impl Default for GriddedDataPdu {
    /// Creates a default Gridded Data PDU with arbitrary environmental process ID
    ///
    /// # Examples
    ///
    /// Initializing a Gridded Data PDU:
    /// ```
    /// use open_dis_rust::synthetic_environment::gridded_data_pdu::GriddedDataPdu;
    /// let gridded_data_pdu = GriddedDataPdu::default();
    /// ```
    ///
    fn default() -> Self {
        GriddedDataPdu {
            pdu_header: PduHeader::default(
                PduType::GriddedData,
                ProtocolFamily::SyntheticEnvironment,
                112,
            ),
            environmental_simulation_id: EntityId::default(0),
            field_number: 0,
            pdu_number: 0,
            pdu_total: 0,
            coordinate_system: GriddedDataCoordinateSystem::default(),
            number_of_grid_axes: 0,
            constant_grid: GriddedDataConstantGrid::default(),
            environment_type: EntityType::default(),
            orientation: EulerAngles::default(),
            sample_time: 0,
            total_values: 0,
            vector_dimension: 0,
            padding1: 0,
            padding2: 0,
            grid_axis_descriptors: vec![],
            grid_data_list: vec![],
        }
    }
}

impl Pdu for GriddedDataPdu {
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
        buf.put_u64(self.sample_time);
        buf.put_u32(self.total_values);
        buf.put_u8(self.vector_dimension);
        buf.put_u8(self.padding1);
        buf.put_u16(self.padding2);
        for i in 0..self.grid_axis_descriptors.len() {
            self.grid_axis_descriptors[i].serialize(buf);
        }
        for i in 0..self.grid_data_list.len() {
            self.grid_data_list[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::GriddedData {
            let environmental_simulation_id = EntityId::deserialize(&mut buffer);
            let field_number = buffer.get_u16();
            let pdu_number = buffer.get_u16();
            let pdu_total = buffer.get_u16();
            let coordinate_system = GriddedDataCoordinateSystem::deserialize(&mut buffer);
            let number_of_grid_axes = buffer.get_u8();
            let constant_grid = GriddedDataConstantGrid::deserialize(&mut buffer);
            let environment_type = EntityType::deserialize(&mut buffer);
            let orientation = EulerAngles::deserialize(&mut buffer);
            let sample_time = buffer.get_u64();
            let total_values = buffer.get_u32();
            let vector_dimension = buffer.get_u8();
            let padding1 = buffer.get_u8();
            let padding2 = buffer.get_u16();
            let mut grid_axis_descriptors: Vec<GridAxisDescriptor> = vec![];
            for _ in 0..number_of_grid_axes {
                grid_axis_descriptors.push(GridAxisDescriptor::deserialize(&mut buffer));
            }
            let mut grid_data_list: Vec<GridDataRecord> = vec![];
            while buffer.has_remaining() {
                grid_data_list.push(GridDataRecord::deserialize(&mut buffer));
            }

            Ok(GriddedDataPdu {
                pdu_header,
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
                padding1,
                padding2,
                grid_axis_descriptors,
                grid_data_list,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type GriddedData, got {:?}",
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
        let environmental_simulation_id = EntityId::deserialize(&mut buffer);
        let field_number = buffer.get_u16();
        let pdu_number = buffer.get_u16();
        let pdu_total = buffer.get_u16();
        let coordinate_system = GriddedDataCoordinateSystem::deserialize(&mut buffer);
        let number_of_grid_axes = buffer.get_u8();
        let constant_grid = GriddedDataConstantGrid::deserialize(&mut buffer);
        let environment_type = EntityType::deserialize(&mut buffer);
        let orientation = EulerAngles::deserialize(&mut buffer);
        let sample_time = buffer.get_u64();
        let total_values = buffer.get_u32();
        let vector_dimension = buffer.get_u8();
        let padding1 = buffer.get_u8();
        let padding2 = buffer.get_u16();
        let mut grid_axis_descriptors: Vec<GridAxisDescriptor> = vec![];
        for _ in 0..number_of_grid_axes {
            grid_axis_descriptors.push(GridAxisDescriptor::deserialize(&mut buffer));
        }
        let mut grid_data_list: Vec<GridDataRecord> = vec![];
        while buffer.has_remaining() {
            grid_data_list.push(GridDataRecord::deserialize(&mut buffer));
        }
        Ok(GriddedDataPdu {
            pdu_header,
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
            padding1,
            padding2,
            grid_axis_descriptors,
            grid_data_list,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::GriddedDataPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let gridded_data_pdu = GriddedDataPdu::default();
        let pdu_header = PduHeader::default(
            PduType::GriddedData,
            ProtocolFamily::SyntheticEnvironment,
            112,
        );

        assert_eq!(
            pdu_header.protocol_version,
            gridded_data_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            gridded_data_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, gridded_data_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            gridded_data_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, gridded_data_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, gridded_data_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut gridded_data_pdu = GriddedDataPdu::default();
        let mut buffer = BytesMut::new();
        gridded_data_pdu.serialize(&mut buffer);

        let new_gridded_data_pdu = GriddedDataPdu::deserialize(buffer).unwrap();
        assert_eq!(new_gridded_data_pdu.pdu_header, gridded_data_pdu.pdu_header);
    }

    #[test]
    fn check_pdu_size() {
        let gridded_data_pdu = GriddedDataPdu::default();

        assert_eq!(
            gridded_data_pdu.pdu_header.length,
            gridded_data_pdu.pdu_header.length
        );
    }
}
