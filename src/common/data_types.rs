//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::enums::{GridAxisDescriptorAxisType, GriddedDataDataRepresentation};

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Standard Variable record. `record_length` is the whole record in octets,
/// a multiple of 8.
pub struct StandardVariableRecords {
    pub record_type: u32,
    #[deku(
        update = "crate::common::wire::padded_record_length(self.record_specific_fields.len())?",
        assert = "crate::common::wire::valid_record_length(*record_length)"
    )]
    pub record_length: u16,
    /// Everything after the record length. When decoded, this includes the
    /// padding to the 64-bit boundary; when encoded, padding is added to
    /// reach it.
    #[deku(
        count = "crate::common::wire::record_fields_len(*record_length)",
        writer = "crate::common::wire::write_padded_fields(deku::writer, record_specific_fields)"
    )]
    pub record_specific_fields: Vec<u8>,
}

impl StandardVariableRecords {
    #[must_use]
    pub const fn new(
        record_type: u32,
        record_length: u16,
        record_specific_fields: Vec<u8>,
    ) -> Self {
        Self {
            record_type,
            record_length,
            record_specific_fields,
        }
    }
}

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct RecordSpecificationElement {
    pub record_id: u32,
    pub record_set_serial_number: u32,
    pub record_length: u16,
    #[deku(assert = "crate::common::wire::fits_in_pdu(values_len(*record_length, *record_count))")]
    pub record_count: u16,
    /// Exactly `record_length * record_count` octets; encoding fails otherwise.
    #[deku(
        count = "values_len(*record_length, *record_count)",
        assert = "record_values.len() == values_len(*record_length, *record_count)"
    )]
    pub record_values: Vec<u8>,
}

impl RecordSpecificationElement {
    #[must_use]
    pub const fn new(
        record_id: u32,
        record_set_serial_number: u32,
        record_length: u16,
        record_count: u16,
        record_values: Vec<u8>,
    ) -> Self {
        Self {
            record_id,
            record_set_serial_number,
            record_length,
            record_count,
            record_values,
        }
    }
}

/// The number of octets in `record_values`: `record_length * record_count`.
/// Can't overflow: two 16-bit values multiply to at most 32 bits.
fn values_len(record_length: u16, record_count: u16) -> usize {
    usize::from(record_length).saturating_mul(usize::from(record_count))
}

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
