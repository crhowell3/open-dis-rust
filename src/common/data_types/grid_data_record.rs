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
use num_traits::FromPrimitive;

use crate::common::{enums::GriddedDataDataRepresentation, wire};

/// The values of a Grid Data record, by data representation. The number of
/// values is `data_values.len()`.
#[derive(Clone, Debug, PartialEq)]
pub enum DataRepresentationType {
    /// Representation type 0 (IEEE 1278.1-2012 Table 68): octets, padded to
    /// a 16-bit boundary on the wire.
    Type0 { data_values: Vec<u8> },
    /// Representation type 1 (Table 69): scaled 16-bit values, padded to a
    /// 32-bit boundary on the wire.
    Type1 {
        field_scale: f32,
        field_offset: f32,
        data_values: Vec<u16>,
    },
    /// Representation type 2 (Table 70): 32-bit floating point values.
    Type2 { data_values: Vec<f32> },
}

impl Default for DataRepresentationType {
    fn default() -> Self {
        Self::Type0 {
            data_values: Vec::new(),
        }
    }
}

impl DataRepresentationType {
    /// The data representation these values are sent as.
    #[must_use]
    pub const fn data_representation(&self) -> GriddedDataDataRepresentation {
        match self {
            Self::Type0 { .. } => GriddedDataDataRepresentation::Type0,
            Self::Type1 { .. } => GriddedDataDataRepresentation::Type1,
            Self::Type2 { .. } => GriddedDataDataRepresentation::Type2,
        }
    }
}

/// Grid Data record (IEEE 1278.1-2012 §6.2.41).
///
/// `data_representation` follows `data`: it is set from `data` when
/// decoding and by `serialize`, and the encoded representation always
/// matches `data`.
#[derive(Clone, Debug, Default)]
pub struct GridDataRecord {
    /// SISO-REF-010 UID 246, a 16-bit enumeration.
    pub sample_type: u16,
    pub data_representation: GriddedDataDataRepresentation,
    pub data: DataRepresentationType,
}

impl GridDataRecord {
    #[must_use]
    pub const fn new(
        sample_type: u16,
        data_representation: GriddedDataDataRepresentation,
        data: DataRepresentationType,
    ) -> Self {
        Self {
            sample_type,
            data_representation,
            data,
        }
    }
}

impl DekuReader<'_, Endian> for GridDataRecord {
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        endian: Endian,
    ) -> Result<Self, DekuError> {
        let sample_type = u16::from_reader_with_ctx(reader, endian)?;
        let representation = u16::from_reader_with_ctx(reader, endian)?;
        let data_representation = GriddedDataDataRepresentation::from_u16(representation)
            .ok_or_else(|| {
                DekuError::Parse(
                    format!(
                        "{representation} is not a defined GriddedDataDataRepresentation value"
                    )
                    .into(),
                )
            })?;
        let data = match data_representation {
            GriddedDataDataRepresentation::Type0 => {
                let count = u16::from_reader_with_ctx(reader, endian)?;
                let data_values: Vec<u8> = wire::read_n(reader, usize::from(count), endian)?;
                wire::skip(reader, wire::padding_to(data_values.len(), 2))?;
                DataRepresentationType::Type0 { data_values }
            }
            GriddedDataDataRepresentation::Type1 => {
                let field_scale = f32::from_reader_with_ctx(reader, endian)?;
                let field_offset = f32::from_reader_with_ctx(reader, endian)?;
                let count = u16::from_reader_with_ctx(reader, endian)?;
                let data_values: Vec<u16> = wire::read_n(reader, usize::from(count), endian)?;
                wire::skip(reader, type_1_padding(data_values.len()))?;
                DataRepresentationType::Type1 {
                    field_scale,
                    field_offset,
                    data_values,
                }
            }
            GriddedDataDataRepresentation::Type2 => {
                let count = u16::from_reader_with_ctx(reader, endian)?;
                wire::skip(reader, 2)?;
                let data_values = wire::read_n(reader, usize::from(count), endian)?;
                DataRepresentationType::Type2 { data_values }
            }
        };
        Ok(Self {
            sample_type,
            data_representation,
            data,
        })
    }
}

impl DekuWriter<Endian> for GridDataRecord {
    fn to_writer<W: Write + Seek>(
        &self,
        writer: &mut Writer<W>,
        endian: Endian,
    ) -> Result<(), DekuError> {
        self.sample_type.to_writer(writer, endian)?;
        // Lossless: a fieldless enum whose values all fit in 16 bits.
        #[allow(clippy::as_conversions)]
        let representation = u16::from(self.data.data_representation() as u8);
        representation.to_writer(writer, endian)?;
        match &self.data {
            DataRepresentationType::Type0 { data_values } => {
                value_count(data_values.len())?.to_writer(writer, endian)?;
                data_values.to_writer(writer, endian)?;
                wire::write_zeros(writer, wire::padding_to(data_values.len(), 2))
            }
            DataRepresentationType::Type1 {
                field_scale,
                field_offset,
                data_values,
            } => {
                field_scale.to_writer(writer, endian)?;
                field_offset.to_writer(writer, endian)?;
                value_count(data_values.len())?.to_writer(writer, endian)?;
                data_values.to_writer(writer, endian)?;
                wire::write_zeros(writer, type_1_padding(data_values.len()))
            }
            DataRepresentationType::Type2 { data_values } => {
                value_count(data_values.len())?.to_writer(writer, endian)?;
                wire::write_zeros(writer, 2)?;
                data_values.to_writer(writer, endian)
            }
        }
    }
}

impl DekuUpdate for GridDataRecord {
    fn update(&mut self) -> Result<(), DekuError> {
        self.data_representation = self.data.data_representation();
        Ok(())
    }
}

crate::impl_deku_default_endian!(GridDataRecord);

fn value_count(len: usize) -> Result<u16, DekuError> {
    u16::try_from(len)
        .map_err(|_| DekuError::Assertion("grid data record has more than 65,535 values".into()))
}

/// Padding after `count` 16-bit values in a type 1 record: the record is 14
/// octets plus the values, padded to a 32-bit boundary.
const fn type_1_padding(count: usize) -> usize {
    wire::padding_to(count.saturating_mul(2).saturating_add(14), 4)
}
