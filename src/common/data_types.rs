//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::{
    enums::{
        DeadReckoningAlgorithm, GridAxisDescriptorAxisType, GriddedDataDataRepresentation,
        VariableRecordTypes,
    },
    records::{
        AcousticEmitterSystem, AngularVelocityVector, Attribute, EmitterSystem, EntityIdentifier,
        TrackJamData, Vector,
    },
};

pub mod fixed_binary;
pub mod point;

pub use fixed_binary::{FixedBinary8, FixedBinary16};
pub use point::Point;

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AcousticBeamData {
    pub beam_data_length: u8,
    pub beam_id_number: u8,
    pub pad2: u16,
    pub fundamental_data_parameters: AcousticBeamFundamentalParameter,
}

impl AcousticBeamData {
    #[must_use]
    pub const fn new(
        beam_data_length: u8,
        beam_id_number: u8,
        pad2: u16,
        fundamental_data_parameters: AcousticBeamFundamentalParameter,
    ) -> Self {
        Self {
            beam_data_length,
            beam_id_number,
            pad2,
            fundamental_data_parameters,
        }
    }
}

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

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §7.6.4 Table 163
pub struct ApaData {
    pub parameter_index: u16,
    pub parameter_value: i16,
}

impl ApaData {
    #[must_use]
    pub const fn new(parameter_index: u16, parameter_value: i16) -> Self {
        Self {
            parameter_index,
            parameter_value,
        }
    }
}

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct RadioEntityType {
    pub entity_kind: u8,
    pub domain: u8,
    pub country: u16,
    pub category: u8,
    pub nomenclature_version: u8,
    pub nomenclature: u16,
}

impl RadioEntityType {
    #[must_use]
    pub const fn new(
        entity_kind: u8,
        domain: u8,
        country: u16,
        category: u8,
        nomenclature_version: u8,
        nomenclature: u16,
    ) -> Self {
        Self {
            entity_kind,
            domain,
            country,
            category,
            nomenclature_version,
            nomenclature,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §7.6.4(j-l)
pub struct ShaftRPMs {
    pub current_shaft_rpms: i16,
    pub ordered_shaft_rpms: i16,
    pub shaft_rpm_rate_of_change: i32,
}

impl ShaftRPMs {
    #[must_use]
    pub const fn new(
        current_shaft_rpms: i16,
        ordered_shaft_rpms: i16,
        shaft_rpm_rate_of_change: i32,
    ) -> Self {
        Self {
            current_shaft_rpms,
            ordered_shaft_rpms,
            shaft_rpm_rate_of_change,
        }
    }
}

#[derive(Copy, Clone, Debug, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct IFFFundamentalParameterData {
    pub erp: f32,
    pub frequency: f32,
    pub pgrf: f32,
    pub pulse_width: f32,
    pub burst_length: u32,
    pub applicable_modes: u8,
    pub system_specific_data: [u8; 3],
}

impl Default for IFFFundamentalParameterData {
    fn default() -> Self {
        Self {
            erp: 0.0,
            frequency: 0.0,
            pgrf: 0.0,
            pulse_width: 0.0,
            burst_length: 0,
            applicable_modes: 0,
            system_specific_data: [0; 3],
        }
    }
}

impl IFFFundamentalParameterData {
    #[must_use]
    pub const fn new(
        erp: f32,
        frequency: f32,
        pgrf: f32,
        pulse_width: f32,
        burst_length: u32,
        applicable_modes: u8,
        system_specific_data: [u8; 3],
    ) -> Self {
        Self {
            erp,
            frequency,
            pgrf,
            pulse_width,
            burst_length,
            applicable_modes,
            system_specific_data,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct FundamentalParameterData {
    pub frequency: f32,
    pub frequency_range: f32,
    pub effective_radiated_power: f32,
    pub pulse_repetition_frequency: f32,
    pub pulse_width: f32,
    pub beam_azimuth_center: f32,
    pub beam_azimuth_sweep: f32,
    pub beam_elevation_center: f32,
    pub beam_elevation_sweep: f32,
    pub beam_sweep_sync: f32,
}

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ElectromagneticEmissionBeamData {
    pub beam_data_length: u8,
    pub beam_id_number: u8,
    pub beam_parameter_index: u16,
    pub fundamental_parameter_data: FundamentalParameterData,
    pub beam_function: u8,
    #[deku(update = "self.track_jam_targets.len()")]
    pub number_of_track_jam_targets: u8,
    pub high_density_track_jam: u8,
    pub pad4: u8,
    pub jamming_mode_sequence: u32,
    #[deku(count = "number_of_track_jam_targets")]
    pub track_jam_targets: Vec<TrackJamData>,
}

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ElectromagneticEmissionSystemData {
    pub system_data_length: u8,
    #[deku(update = "crate::common::wire::update_each(&mut self.beam_data_records)?")]
    pub number_of_beams: u8,
    pub emissionspadding2: u16,
    pub emitter_system: EmitterSystem,
    pub location: Vector,
    #[deku(count = "number_of_beams")]
    pub beam_data_records: Vec<ElectromagneticEmissionBeamData>,
}

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct DeadReckoningParameters {
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    pub dead_reckoning_other_parameters: [u8; 15],
    pub entity_linear_acceleration: Vector,
    pub entity_angular_velocity: AngularVelocityVector,
}

impl DeadReckoningParameters {
    #[must_use]
    pub const fn new(
        dead_reckoning_algorithm: DeadReckoningAlgorithm,
        entity_linear_acceleration: Vector,
        entity_angular_velocity: AngularVelocityVector,
    ) -> Self {
        Self {
            dead_reckoning_algorithm,
            dead_reckoning_other_parameters: [0u8; 15],
            entity_linear_acceleration,
            entity_angular_velocity,
        }
    }
}

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AttributeRecordSet {
    pub entity_id: EntityIdentifier,
    #[deku(
        update = "crate::common::wire::update_each(&mut self.attribute_records)?",
        assert = "crate::common::wire::fits_in_pdu(*number_of_attribute_records)"
    )]
    pub number_of_attribute_records: u16,
    #[deku(count = "number_of_attribute_records")]
    pub attribute_records: Vec<Attribute>,
}

impl AttributeRecordSet {
    #[must_use]
    pub fn new() -> Self {
        Self {
            entity_id: EntityIdentifier::default(),
            number_of_attribute_records: 0u16,
            attribute_records: vec![],
        }
    }
}

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AcousticEmitterSystemData {
    pub emitter_system_data_length: u8,
    #[deku(update = "self.beam_records.len()")]
    pub number_of_beams: u8,
    pub pad2: u16,
    pub acoustic_emitter_system: AcousticEmitterSystem,
    pub emitter_location: Vector,
    #[deku(count = "number_of_beams")]
    pub beam_records: Vec<AcousticBeamData>,
}

impl AcousticEmitterSystemData {
    #[must_use]
    pub const fn new(
        emitter_system_data_length: u8,
        pad2: u16,
        acoustic_emitter_system: AcousticEmitterSystem,
        emitter_location: Vector,
        beam_records: Vec<AcousticBeamData>,
    ) -> Self {
        Self {
            emitter_system_data_length,
            number_of_beams: 0,
            pad2,
            acoustic_emitter_system,
            emitter_location,
            beam_records,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AcousticBeamFundamentalParameter {
    pub active_emission_parameter_index: u16,
    pub scan_pattern: u16,
    pub beam_center_azimuth: f32,
    pub azimuthal_beamwidth: f32,
    pub beam_center_de: f32,
    pub de_beamwidth: f32,
}

impl AcousticBeamFundamentalParameter {
    #[must_use]
    pub const fn new(
        active_emission_parameter_index: u16,
        scan_pattern: u16,
        beam_center_azimuth: f32,
        azimuthal_beamwidth: f32,
        beam_center_de: f32,
        de_beamwidth: f32,
    ) -> Self {
        Self {
            active_emission_parameter_index,
            scan_pattern,
            beam_center_azimuth,
            azimuthal_beamwidth,
            beam_center_de,
            de_beamwidth,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct IFFDataRecord {
    pub record_type: VariableRecordTypes,
    #[deku(
        update = "crate::common::wire::padded_record_length_to(self.record_specific_fields.len(), 4)?",
        assert = "crate::common::wire::valid_aligned_record_length(*record_length, 4)"
    )]
    pub record_length: u16,
    #[deku(
        count = "crate::common::wire::record_fields_len(*record_length)",
        writer = "crate::common::wire::write_padded_fields_to(deku::writer, record_specific_fields, 4)"
    )]
    pub record_specific_fields: Vec<u8>,
}
