//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use deku::{
    DekuError, DekuReader, DekuUpdate, DekuWriter,
    ctx::Endian,
    no_std_io::{Read, Seek, Write},
    reader::Reader,
    writer::Writer,
};

use crate::common::{enums::GridAxisDescriptorAxisType, wire};

/// The spacing of a grid axis, which decides the rest of the Grid Axis
/// Descriptor record.
#[derive(Clone, Debug, PartialEq)]
pub enum GridAxisType {
    /// Regular (fixed spacing) axis data (IEEE 1278.1-2012 Table 66).
    FixedSpacing {
        number_of_points_on_x_axis: u16,
        initial_index: u16,
    },
    /// Irregular (variable spacing) axis data (IEEE 1278.1-2012 Table 67).
    /// The number of points is `x_values.len()`; the record is padded to a
    /// 64-bit boundary on the wire.
    VariableSpacing {
        initial_index: u16,
        coordinate_scale_x: f64,
        coordinate_offset_x: f64,
        x_values: Vec<u16>,
    },
}

impl Default for GridAxisType {
    fn default() -> Self {
        Self::FixedSpacing {
            number_of_points_on_x_axis: 0,
            initial_index: 0,
        }
    }
}

impl GridAxisType {
    /// The axis type this spacing is sent as.
    #[must_use]
    pub const fn axis_type(&self) -> GridAxisDescriptorAxisType {
        match self {
            Self::FixedSpacing { .. } => GridAxisDescriptorAxisType::RegularAxis,
            Self::VariableSpacing { .. } => GridAxisDescriptorAxisType::IrregularAxis,
        }
    }
}

/// Grid Axis Descriptor record (IEEE 1278.1-2012 §6.2.40).
///
/// `axis_type` follows `data`: it is set from `data` when decoding and by
/// `serialize`, and the encoded axis type always matches `data`.
#[derive(Clone, Debug, Default)]
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
    pub const fn new(
        domain_initial: f64,
        domain_final: f64,
        domain_points: u16,
        interleaf_factor: u8,
        axis_type: GridAxisDescriptorAxisType,
        data: GridAxisType,
    ) -> Self {
        Self {
            domain_initial,
            domain_final,
            domain_points,
            interleaf_factor,
            axis_type,
            data,
        }
    }
}

impl DekuReader<'_, Endian> for GridAxisDescriptor {
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        endian: Endian,
    ) -> Result<Self, DekuError> {
        let domain_initial = f64::from_reader_with_ctx(reader, endian)?;
        let domain_final = f64::from_reader_with_ctx(reader, endian)?;
        let domain_points = u16::from_reader_with_ctx(reader, endian)?;
        let interleaf_factor = u8::from_reader_with_ctx(reader, endian)?;
        let axis_type = GridAxisDescriptorAxisType::from_reader_with_ctx(reader, endian)?;
        let number_of_points = u16::from_reader_with_ctx(reader, endian)?;
        let initial_index = u16::from_reader_with_ctx(reader, endian)?;
        let data = match axis_type {
            GridAxisDescriptorAxisType::RegularAxis => GridAxisType::FixedSpacing {
                number_of_points_on_x_axis: number_of_points,
                initial_index,
            },
            GridAxisDescriptorAxisType::IrregularAxis => {
                let coordinate_scale_x = f64::from_reader_with_ctx(reader, endian)?;
                let coordinate_offset_x = f64::from_reader_with_ctx(reader, endian)?;
                let x_values = wire::read_n(reader, usize::from(number_of_points), endian)?;
                wire::skip(reader, x_values_padding(x_values.len()))?;
                GridAxisType::VariableSpacing {
                    initial_index,
                    coordinate_scale_x,
                    coordinate_offset_x,
                    x_values,
                }
            }
        };
        Ok(Self {
            domain_initial,
            domain_final,
            domain_points,
            interleaf_factor,
            axis_type,
            data,
        })
    }
}

impl DekuWriter<Endian> for GridAxisDescriptor {
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        endian: Endian,
    ) -> Result<(), DekuError> {
        self.domain_initial.to_writer(writer, endian)?;
        self.domain_final.to_writer(writer, endian)?;
        self.domain_points.to_writer(writer, endian)?;
        self.interleaf_factor.to_writer(writer, endian)?;
        self.data.axis_type().to_writer(writer, endian)?;
        match &self.data {
            GridAxisType::FixedSpacing {
                number_of_points_on_x_axis,
                initial_index,
            } => {
                number_of_points_on_x_axis.to_writer(writer, endian)?;
                initial_index.to_writer(writer, endian)
            }
            GridAxisType::VariableSpacing {
                initial_index,
                coordinate_scale_x,
                coordinate_offset_x,
                x_values,
            } => {
                let points = u16::try_from(x_values.len()).map_err(|_| {
                    DekuError::Assertion("grid axis has more than 65,535 points".into())
                })?;
                points.to_writer(writer, endian)?;
                initial_index.to_writer(writer, endian)?;
                coordinate_scale_x.to_writer(writer, endian)?;
                coordinate_offset_x.to_writer(writer, endian)?;
                x_values.to_writer(writer, endian)?;
                wire::write_zeros(writer, x_values_padding(x_values.len()))
            }
        }
    }
}

impl DekuUpdate for GridAxisDescriptor {
    fn update(&mut self) -> Result<(), DekuError> {
        self.axis_type = self.data.axis_type();
        Ok(())
    }
}

crate::impl_deku_default_endian!(GridAxisDescriptor);

/// Padding after `points` 16-bit x values, to a 64-bit boundary. The rest of
/// the irregular-axis record is 40 octets, already a multiple of 8.
const fn x_values_padding(points: usize) -> usize {
    wire::padding_to(points.saturating_mul(2), 8)
}
