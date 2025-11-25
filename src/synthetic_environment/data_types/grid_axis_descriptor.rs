//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::enums::GridAxisDescriptorAxisType;

#[derive(Clone, Debug, PartialEq)]
pub enum GridAxisType {
    FixedSpacing {
        number_of_points_on_x_axis: u16,
        initial_index: u16,
    },
    VariableSpacing {
        number_of_points_on_x_axis: u16,
        initial_index: u16,
        coordinate_scale_x: f64,
        coordinate_offset_x: f64,
        x_values: Vec<u16>,
        padding: Vec<u8>,
    },
}

#[derive(Clone, Debug)]
pub struct GridAxisDescriptor {
    pub domain_initial: f64,
    pub domain_final: f64,
    pub domain_points: u16,
    pub interleaf_factor: u8,
    pub axis_type: GridAxisDescriptorAxisType,
    pub data: GridAxisType,
}

impl GridAxisDescriptor {
    #[must_use]
    pub fn new(
        domain_initial: f64,
        domain_final: f64,
        domain_points: u16,
        interleaf_factor: u8,
        axis_type: GridAxisDescriptorAxisType,
        data: GridAxisType,
    ) -> Self {
        GridAxisDescriptor {
            domain_initial,
            domain_final,
            domain_points,
            interleaf_factor,
            axis_type,
            data,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_f64(self.domain_initial);
        buf.put_f64(self.domain_final);
        buf.put_u16(self.domain_points);
        buf.put_u8(self.interleaf_factor);
        buf.put_u8(self.axis_type as u8);
        match &self.data {
            GridAxisType::FixedSpacing {
                number_of_points_on_x_axis,
                initial_index,
            } => {
                buf.put_u16(*number_of_points_on_x_axis);
                buf.put_u16(*initial_index);
            }
            GridAxisType::VariableSpacing {
                number_of_points_on_x_axis,
                initial_index,
                coordinate_scale_x,
                coordinate_offset_x,
                x_values,
                padding,
            } => {
                buf.put_u16(*number_of_points_on_x_axis);
                buf.put_u16(*initial_index);
                buf.put_f64(*coordinate_scale_x);
                buf.put_f64(*coordinate_offset_x);
                for x in x_values {
                    buf.put_u16(*x);
                }
                for p in padding {
                    buf.put_u8(*p);
                }
            }
        }
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let domain_initial = buf.get_f64();
        let domain_final = buf.get_f64();
        let domain_points = buf.get_u16();
        let interleaf_factor = buf.get_u8();
        let axis_type = GridAxisDescriptorAxisType::deserialize(buf);
        let data = match axis_type {
            GridAxisDescriptorAxisType::RegularAxis => GridAxisType::FixedSpacing {
                number_of_points_on_x_axis: buf.get_u16(),
                initial_index: buf.get_u16(),
            },
            GridAxisDescriptorAxisType::IrregularAxis => {
                let number_of_points_on_x_axis = buf.get_u16();
                let initial_index = buf.get_u16();
                let coordinate_scale_x = buf.get_f64();
                let coordinate_offset_x = buf.get_f64();
                let mut x_values: Vec<u16> = vec![];
                for _ in 0..number_of_points_on_x_axis {
                    x_values.push(buf.get_u16());
                }
                let mut padding: Vec<u8> = vec![];
                while buf.has_remaining() {
                    padding.push(buf.get_u8());
                }

                GridAxisType::VariableSpacing {
                    number_of_points_on_x_axis,
                    initial_index,
                    coordinate_scale_x,
                    coordinate_offset_x,
                    x_values,
                    padding,
                }
            }
        };

        GridAxisDescriptor {
            domain_initial,
            domain_final,
            domain_points,
            interleaf_factor,
            axis_type,
            data,
        }
    }
}
