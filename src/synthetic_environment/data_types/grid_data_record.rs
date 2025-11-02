//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::enums::{GriddedDataDataRepresentation, GriddedDataSampleType};

#[derive(Clone, Debug, PartialEq)]
pub enum DataRepresentationType {
    Type0 {
        number_of_octets: u16,
        data_values: Vec<u8>,
        padding: Vec<u8>,
    },
    Type1 {
        field_scale: f32,
        field_offset: f32,
        number_of_values: u16,
        data_values: Vec<u16>,
        padding: Vec<u8>,
    },
    Type2 {
        number_of_values: u16,
        padding: u16,
        data_values: Vec<f32>,
    },
}

#[derive(Clone, Debug)]
pub struct GridDataRecord {
    pub sample_type: GriddedDataSampleType,
    pub data_representation: GriddedDataDataRepresentation,
    pub data: DataRepresentationType,
}

impl GridDataRecord {
    #[must_use]
    pub fn new(
        sample_type: GriddedDataSampleType,
        data_representation: GriddedDataDataRepresentation,
        data: DataRepresentationType,
    ) -> Self {
        GridDataRecord {
            sample_type,
            data_representation,
            data,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.sample_type as u16);
        buf.put_u16(self.data_representation as u16);
        match &self.data {
            DataRepresentationType::Type0 {
                number_of_octets,
                data_values,
                padding,
            } => {
                buf.put_u16(*number_of_octets);
                for i in data_values {
                    buf.put_u8(*i);
                }
                for p in padding {
                    buf.put_u8(*p);
                }
            }
            DataRepresentationType::Type1 {
                field_scale,
                field_offset,
                number_of_values,
                data_values,
                padding,
            } => {
                buf.put_f32(*field_scale);
                buf.put_f32(*field_offset);
                buf.put_u16(*number_of_values);
                for i in data_values {
                    buf.put_u16(*i);
                }
                for p in padding {
                    buf.put_u8(*p);
                }
            }
            DataRepresentationType::Type2 {
                number_of_values,
                padding,
                data_values,
            } => {
                buf.put_u16(*number_of_values);
                buf.put_u16(*padding);
                for i in data_values {
                    buf.put_f32(*i);
                }
            }
        }
    }

    pub fn deserialize(buf: &mut BytesMut) -> GridDataRecord {
        let sample_type = GriddedDataSampleType::deserialize(buf);
        let data_representation = GriddedDataDataRepresentation::deserialize(buf);
        let data = match data_representation {
            GriddedDataDataRepresentation::Type0 => {
                let number_of_octets = buf.get_u16();
                let mut data_values: Vec<u8> = vec![];
                for _ in 0..number_of_octets {
                    data_values.push(buf.get_u8());
                }
                let mut padding: Vec<u8> = vec![];
                while buf.has_remaining() {
                    padding.push(buf.get_u8());
                }

                DataRepresentationType::Type0 {
                    number_of_octets,
                    data_values,
                    padding,
                }
            }
            GriddedDataDataRepresentation::Type1 => {
                let field_scale = buf.get_f32();
                let field_offset = buf.get_f32();
                let number_of_values = buf.get_u16();
                let mut data_values: Vec<u16> = vec![];
                for _ in 0..number_of_values {
                    data_values.push(buf.get_u16());
                }
                let mut padding: Vec<u8> = vec![];
                while buf.has_remaining() {
                    padding.push(buf.get_u8());
                }

                DataRepresentationType::Type1 {
                    field_scale,
                    field_offset,
                    number_of_values,
                    data_values,
                    padding,
                }
            }
            GriddedDataDataRepresentation::Type2 => {
                let number_of_values = buf.get_u16();
                let padding = buf.get_u16();
                let mut data_values: Vec<f32> = vec![];
                for _ in 0..number_of_values {
                    data_values.push(buf.get_f32());
                }

                DataRepresentationType::Type2 {
                    number_of_values,
                    padding,
                    data_values,
                }
            }
        };

        GridDataRecord {
            sample_type,
            data_representation,
            data,
        }
    }
}
