//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use deku::{
    DekuError, DekuReader, DekuUpdate, DekuWriter,
    ctx::Endian,
    no_std_io::{Read, Seek, Write},
    reader::Reader,
    writer::Writer,
};
use num_traits::FromPrimitive;

use crate::common::data_types::*;
use crate::common::enums::*;
use crate::common::wire;

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

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §6.2.2
pub struct AcousticEmitterSystem {
    pub acoustic_name: UAAcousticSystemName,
    pub acoustic_function: UAAcousticEmitterSystemFunction,
    pub acoustic_id: u8,
}

impl AcousticEmitterSystem {
    #[must_use]
    pub const fn new(
        acoustic_name: UAAcousticSystemName,
        acoustic_function: UAAcousticEmitterSystemFunction,
        acoustic_id: u8,
    ) -> Self {
        Self {
            acoustic_name,
            acoustic_function,
            acoustic_id,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.3
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AggregateIdentifier {
    pub simulation_address: SimulationAddress,
    pub aggregate_id: u16,
}

impl AggregateIdentifier {
    #[must_use]
    pub const fn new(simulation_address: SimulationAddress, aggregate_id: u16) -> Self {
        Self {
            simulation_address,
            aggregate_id,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.4
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AggregateMarking {
    pub character_set: EntityMarkingCharacterSet,
    pub characters: [i8; 31],
}

impl AggregateMarking {
    #[must_use]
    pub const fn new(character_set: EntityMarkingCharacterSet, characters: [i8; 31]) -> Self {
        Self {
            character_set,
            characters,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.7
/// For all fields, assume right-hand rule for directionality
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AngularVelocityVector {
    /// The angular velocity in radians/second about the entity's X-axis
    pub rate_about_x_axis: f32,
    /// The angular velocity in radians/second about the entity's Y-axis
    pub rate_about_y_axis: f32,
    /// The angular velocity in radians/second about the entity's Z-axis
    pub rate_about_z_axis: f32,
}

impl AngularVelocityVector {
    /// Create a new `AngularVelocity` struct with existing values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.8
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AntennaPattern {
    pub record_specific_fields: Vec<u8>,
}

impl AntennaPattern {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            record_specific_fields: Vec::new(),
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.9
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct Association {
    /// Indicates the type of association
    pub association_type: EntityAssociationAssociationType,
    /// Padding field
    padding: u8,
    /// Identifies the associated entity
    associated_entity_id: EntityIdentifier,
    /// Specifies the associated location in world coordinates
    associated_location_in_world_coordinates: WorldCoordinates,
}

impl Association {
    /// Create a new `Association` struct with existing values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.10
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]

pub struct Attribute {
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

impl Attribute {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            record_type: 0u32,
            record_length: 0u16,
            record_specific_fields: vec![],
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.11
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct BeamData {
    pub beam_azimuth_center: f32,
    pub beam_azimuth_sweep: f32,
    pub beam_elevation_center: f32,
    pub beam_elevation_sweep: f32,
    pub beam_sweep_sync: f32,
}

impl BeamData {
    #[must_use]
    pub const fn new(
        beam_azimuth_center: f32,
        beam_azimuth_sweep: f32,
        beam_elevation_center: f32,
        beam_elevation_sweep: f32,
        beam_sweep_sync: f32,
    ) -> Self {
        Self {
            beam_azimuth_center,
            beam_azimuth_sweep,
            beam_elevation_center,
            beam_elevation_sweep,
            beam_sweep_sync,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.14
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ClockTime {
    /// The hours since 0000h 1 January 1970 UTC (The Epoch)
    pub hour: u32,
    /// Time past the hour indicated in the hour field
    pub time_past_hour: Timestamp,
}

impl ClockTime {
    #[must_use]
    pub const fn new(hour: u32, time_past_hour: Timestamp) -> Self {
        Self {
            hour,
            time_past_hour,
        }
    }

    /// The current UTC time, from the system clock, with an absolute timestamp
    #[must_use]
    pub fn now() -> Self {
        let since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let hour = u32::try_from(since_epoch.as_secs() / 3600).unwrap_or(u32::MAX);
        Self::new(hour, Timestamp::absolute(since_epoch))
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.23
#[derive(Copy, Clone, Debug, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EmitterSystem {
    pub emitter_name: EmitterName,
    pub function: EmitterSystemFunction,
    pub emitter_id_number: u8,
}

impl Default for EmitterSystem {
    fn default() -> Self {
        Self {
            emitter_name: EmitterName::default(),
            function: EmitterSystemFunction::Other,
            emitter_id_number: 0,
        }
    }
}

impl EmitterSystem {
    #[must_use]
    pub const fn new(name: EmitterName, function: EmitterSystemFunction, id: u8) -> Self {
        Self {
            emitter_name: name,
            function,
            emitter_id_number: id,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.28
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EntityIdentifier {
    /// The simulation's designation associated with all object identifiers
    pub simulation_address: SimulationAddress,
    /// The unique identification number of the entity
    pub entity_id: u16,
}

impl EntityIdentifier {
    #[must_use]
    pub const fn new(simulation_address: SimulationAddress, entity_id: u16) -> Self {
        Self {
            simulation_address,
            entity_id,
        }
    }
}

/// Number of character octets in an Entity Marking record.
const MARKING_CHARACTERS: usize = 11;

/// Implemented according to IEEE 1278.1-2012 §6.2.29
/// Entity Marking record: a character set followed by 11 character octets.
///
/// Always 12 octets on the wire. A longer `entity_marking_string` is truncated
/// to its first 11 octets when encoded, a shorter one is zero-padded, and the
/// padding is removed when decoded.
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EntityMarking {
    pub entity_marking_character_set: EntityMarkingCharacterSet,
    #[deku(
        reader = "read_characters(deku::reader)",
        writer = "write_characters(deku::writer, &self.entity_marking_string)"
    )]
    pub entity_marking_string: String,
}

impl EntityMarking {
    #[must_use]
    pub const fn new(
        entity_marking_character_set: EntityMarkingCharacterSet,
        entity_marking_string: String,
    ) -> Self {
        Self {
            entity_marking_character_set,
            entity_marking_string,
        }
    }
}

/// Read the 11 character octets, dropping trailing zero padding. Octets that
/// aren't valid UTF-8 become U+FFFD.
fn read_characters<R: Read + Seek>(reader: &mut Reader<R>) -> Result<String, DekuError> {
    let characters = <[u8; MARKING_CHARACTERS]>::from_reader_with_ctx(reader, ())?;
    let text = characters
        .iter()
        .position(|&b| b == 0)
        .and_then(|end| characters.get(..end))
        .unwrap_or(&characters);
    Ok(String::from_utf8_lossy(text).into_owned())
}

/// Write `text` as exactly 11 octets: truncated or zero-padded.
fn write_characters<W: Write + Seek>(writer: &mut Writer<W>, text: &str) -> Result<(), DekuError> {
    let mut characters = [0u8; MARKING_CHARACTERS];
    for (slot, byte) in characters.iter_mut().zip(text.bytes()) {
        *slot = byte;
    }
    characters.to_writer(writer, ())
}

/// Implemented according to IEEE 1278.1-2012 §6.2.30
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EntityType {
    pub kind: EntityKind,
    pub domain: u8,
    pub country: Country,
    pub category: u8,
    pub subcategory: u8,
    pub specific: u8,
    pub extra: u8,
}

impl EntityType {
    #[must_use]
    pub const fn new(
        kind: EntityKind,
        domain: u8,
        country: Country,
        category: u8,
        subcategory: u8,
        specific: u8,
        extra: u8,
    ) -> Self {
        Self {
            kind,
            domain,
            country,
            category,
            subcategory,
            specific,
            extra,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.31
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct Environment {
    pub environment_type: EnvironmentalProcessRecordType,
    pub length: u16,
    pub index: u8,
    pub padding: u8,
}

impl Environment {
    #[must_use]
    pub const fn new(
        environment_type: EnvironmentalProcessRecordType,
        length: u16,
        index: u8,
        padding: u8,
    ) -> Self {
        Self {
            environment_type,
            length,
            index,
            padding,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.32
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EulerAngles {
    /// Angle of rotation about the Z-axis
    pub psi: f32,
    /// Angle of rotation about the Y-axis
    pub theta: f32,
    /// Angle of rotation about the X-axis
    pub phi: f32,
}

impl EulerAngles {
    #[must_use]
    #[allow(clippy::similar_names)]
    pub const fn new(psi: f32, theta: f32, phi: f32) -> Self {
        Self { psi, theta, phi }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.33
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EventIdentifier {
    pub simulation_address: SimulationAddress,
    pub event_number: u16,
}

impl EventIdentifier {
    #[must_use]
    pub const fn new(simulation_address: SimulationAddress, event_number: u16) -> Self {
        Self {
            simulation_address,
            event_number,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.37
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct FixedDatumRecord {
    pub datum_id: u32,
    pub datum_value: u32,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.40
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

crate::common::wire::impl_deku_default_endian!(GridAxisDescriptor);

/// Padding after `points` 16-bit x values, to a 64-bit boundary. The rest of
/// the irregular-axis record is 40 octets, already a multiple of 8.
const fn x_values_padding(points: usize) -> usize {
    wire::padding_to(points.saturating_mul(2), 8)
}

/// Implemented according to IEEE 1278.1-2012 §6.2.41
///
/// `data_representation` follows `data`: it is set from `data` when
/// decoding and by `serialize`, and the encoded representation always
/// matches `data`.
#[derive(Clone, Debug, Default)]
pub struct GridData {
    /// SISO-REF-010 UID 246, a 16-bit enumeration.
    pub sample_type: GriddedDataSampleType,
    pub data_representation: GriddedDataDataRepresentation,
    pub data: DataRepresentationType,
}

impl GridData {
    #[must_use]
    pub const fn new(
        sample_type: GriddedDataSampleType,
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

impl DekuReader<'_, Endian> for GridData {
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        endian: Endian,
    ) -> Result<Self, DekuError> {
        let sample_type =
            GriddedDataSampleType::from_u16(u16::from_reader_with_ctx(reader, endian)?)
                .unwrap_or_default();
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

impl DekuWriter<Endian> for GridData {
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

impl DekuUpdate for GridData {
    fn update(&mut self) -> Result<(), DekuError> {
        self.data_representation = self.data.data_representation();
        Ok(())
    }
}

crate::common::wire::impl_deku_default_endian!(GridData);

fn value_count(len: usize) -> Result<u16, DekuError> {
    u16::try_from(len)
        .map_err(|_| DekuError::Assertion("grid data record has more than 65,535 values".into()))
}

/// Padding after `count` 16-bit values in a type 1 record: the record is 14
/// octets plus the values, padded to a 32-bit boundary.
const fn type_1_padding(count: usize) -> usize {
    wire::padding_to(count.saturating_mul(2).saturating_add(14), 4)
}

/// Implemented according to IEEE 1278.1-2012 §6.2.46
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct IntercomCommunicationsParameters {
    pub record_type: IntercomControlRecordType,
    pub record_length: u16,
    pub record_specific_field: u32,
}

impl IntercomCommunicationsParameters {
    #[must_use]
    pub const fn new(
        record_type: IntercomControlRecordType,
        record_length: u16,
        record_specific_field: u32,
    ) -> Self {
        Self {
            record_type,
            record_length,
            record_specific_field,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.47
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct IntercomIdentifier {
    pub intercom_reference_id: EntityIdentifier,
    pub event_number: u16,
}

impl IntercomIdentifier {
    #[must_use]
    pub const fn new(intercom_reference_id: EntityIdentifier, event_number: u16) -> Self {
        Self {
            intercom_reference_id,
            event_number,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.51
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LayerHeader {
    pub layer_number: u8,
    pub layer_specific_information: u8,
    pub length: u16,
}

impl LayerHeader {
    #[must_use]
    pub const fn new(layer_number: u8, layer_specific_information: u8, length: u16) -> Self {
        Self {
            layer_number,
            layer_specific_information,
            length,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.52
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LinearSegmentParameter {
    pub segment_number: u8,
    pub segment_modification: u8,
    pub general_segment_appearance: u32,
    pub specific_segment_appearance: u32, // TODO(@anyone) Implement Specific Object Appearance
    pub segment_location: WorldCoordinates,
    pub segment_orientation: EulerAngles,
    pub segment_length: f32,
    pub segment_width: f32,
    pub segment_height: f32,
    pub segment_depth: f32,
    padding: u32,
}

impl LinearSegmentParameter {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        segment_number: u8,
        segment_modification: u8,
        general_segment_appearance: u32,
        specific_segment_appearance: u32,
        segment_location: WorldCoordinates,
        segment_orientation: EulerAngles,
        segment_length: f32,
        segment_width: f32,
        segment_height: f32,
        segment_depth: f32,
    ) -> Self {
        Self {
            segment_number,
            segment_modification,
            general_segment_appearance,
            specific_segment_appearance,
            segment_location,
            segment_orientation,
            segment_length,
            segment_width,
            segment_height,
            segment_depth,
            padding: 0_u32,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.56
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct MinefieldIdentifier {
    pub simulation_address: SimulationAddress,
    pub minefield_number: u16,
}

impl MinefieldIdentifier {
    #[must_use]
    pub const fn new(simulation_address: SimulationAddress, minefield_number: u16) -> Self {
        Self {
            simulation_address,
            minefield_number,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.58
#[derive(Clone, Debug, Default)]
pub struct ModulationParameters {
    pub record_specific_fields: Vec<u8>,
}

impl ModulationParameters {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            record_specific_fields: Vec::new(),
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.59
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ModulationType {
    pub spread_spectrum: u16,
    pub major: u16,
    pub detail: u16,
    pub system: u16,
}

impl ModulationType {
    #[must_use]
    pub const fn new(spread_spectrum: u16, major: u16, detail: u16, system: u16) -> Self {
        Self {
            spread_spectrum,
            major,
            detail,
            system,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.62
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct NamedLocationIdentification {
    pub station_name: u16,
    pub station_number: u16,
}

impl NamedLocationIdentification {
    #[must_use]
    pub const fn new(station_name: u16, station_number: u16) -> Self {
        Self {
            station_name,
            station_number,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.63
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ObjectIdentifier {
    pub simulation_address: SimulationAddress,
    pub object_number: u16,
}

impl ObjectIdentifier {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.64
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ObjectType {
    pub domain: u8,
    pub object_kind: u8,
    pub category: u8,
    pub subcategory: u8,
}

impl ObjectType {
    #[must_use]
    pub const fn new(domain: u8, object_kind: u8, category: u8, subcategory: u8) -> Self {
        Self {
            domain,
            object_kind,
            category,
            subcategory,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.68
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct PropulsionSystemData {
    pub power_setting: f32,
    pub engine_rpm: f32,
}

impl PropulsionSystemData {
    #[must_use]
    pub const fn new(power_setting: f32, engine_rpm: f32) -> Self {
        Self {
            power_setting,
            engine_rpm,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.73
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct RecordSpecification {
    #[deku(
        update = "crate::common::wire::update_each(&mut self.record_sets)?",
        assert = "crate::common::wire::fits_in_pdu(*number_of_record_sets)"
    )]
    pub number_of_record_sets: u32,
    #[deku(count = "number_of_record_sets")]
    pub record_sets: Vec<RecordSpecificationElement>,
}

impl RecordSpecification {
    #[must_use]
    pub const fn new(
        number_of_record_sets: u32,
        record_sets: Vec<RecordSpecificationElement>,
    ) -> Self {
        Self {
            number_of_record_sets,
            record_sets,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.74
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct Relationship {
    pub nature: u16,
    pub position: u16,
}

impl Relationship {
    #[must_use]
    pub const fn new(nature: u16, position: u16) -> Self {
        Self { nature, position }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.76
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct SecondaryOperationalData {
    pub operational_data1: u8,
    pub operational_data2: u8,
    pub number_of_iff_fundamental_parameter_records: u16,
}

impl SecondaryOperationalData {
    #[must_use]
    pub const fn new(
        operational_data1: u8,
        operational_data2: u8,
        number_of_iff_fundamental_parameter_records: u16,
    ) -> Self {
        Self {
            operational_data1,
            operational_data2,
            number_of_iff_fundamental_parameter_records,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.80
#[derive(Copy, Clone, Debug, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct SimulationAddress {
    /// Identification number representing the site, which may be a facility,
    /// installation, organizational unit, or a geographical location. Valid
    /// site ID values range from 1 to 65,534
    pub site_id: u16,
    /// Identification number representing the software program that is used to
    /// generate and process distributed simulation data. Valid application ID
    /// values range from 1 to 65,534
    pub application_id: u16,
}

impl Default for SimulationAddress {
    fn default() -> Self {
        Self {
            site_id: 1,
            application_id: 1,
        }
    }
}

impl SimulationAddress {
    #[must_use]
    pub const fn new(site_id: u16, application_id: u16) -> Self {
        Self {
            site_id,
            application_id,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.81
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct SimulationIdentifier {
    pub simulation_address: SimulationAddress,
    pub reference_number: u16,
}

impl SimulationIdentifier {
    #[must_use]
    pub fn new() -> Self {
        Self {
            simulation_address: SimulationAddress::default(),
            reference_number: 0u16,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.83
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct StandardVariableSpecification {
    #[deku(
        update = "crate::common::wire::update_each(&mut self.standard_variable_records)?",
        assert = "crate::common::wire::fits_in_pdu(*number_of_standard_variable_records)"
    )]
    pub number_of_standard_variable_records: u16,
    #[deku(count = "number_of_standard_variable_records")]
    pub standard_variable_records: Vec<StandardVariableRecords>,
}

impl StandardVariableSpecification {
    #[must_use]
    pub const fn new(
        number_of_standard_variable_records: u16,
        standard_variable_records: Vec<StandardVariableRecords>,
    ) -> Self {
        Self {
            number_of_standard_variable_records,
            standard_variable_records,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.86
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct SupplyQuantity {
    pub supply_type: EntityType,
    pub quantity: f32,
}

impl SupplyQuantity {
    #[must_use]
    pub const fn new(supply_type: EntityType, quantity: f32) -> Self {
        Self {
            supply_type,
            quantity,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.87
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct SystemIdentifier {
    pub system_type: u16,
    pub system_name: u16,
    pub system_mode: u8,
    pub change_options: u8,
}

impl SystemIdentifier {
    #[must_use]
    pub const fn new(
        system_type: u16,
        system_name: u16,
        system_mode: u8,
        change_options: u8,
    ) -> Self {
        Self {
            system_type,
            system_name,
            system_mode,
            change_options,
        }
    }
}

/// Timestamp ticks in one hour: bits 31 to 1 count units of 3600/2^31 seconds
const TICKS_PER_HOUR: u128 = 1 << 31;
const NANOS_PER_HOUR: u128 = 3_600_000_000_000;
const SECONDS_PER_HOUR: u64 = 3600;
const ABSOLUTE_BIT: u32 = 1;

/// Implemented according to IEEE 1278.1-2012 §6.2.88
/// A DIS timestamp, as carried in every PDU header and in [`ClockTime`] records.
///
/// A timestamp counts the time since the start of the current hour in units of
/// about 1.676 microseconds.
///
/// Because the count restarts every hour, timestamps can't be ordered by
/// comparing them directly. Use [`duration_since`](Self::duration_since), which
/// allows for the hour wrapping around.
///
/// ```
/// use std::time::Duration;
/// use open_dis_rust::common::records::Timestamp;
///
/// let start = Timestamp::absolute(Duration::from_secs(3599)); // 59:59
/// let later = Timestamp::absolute(Duration::from_secs(1));    // 00:01, in the next hour
/// // Accurate to within a tick, about 1.7 µs.
/// assert_eq!(later.duration_since(start).as_secs_f64().round(), 2.0);
/// assert!(later.is_absolute());
/// ```
///
/// [`ClockTime`]: crate::common::records::ClockTime
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct Timestamp {
    raw: u32,
}

impl Timestamp {
    /// An absolute timestamp for the current time from the system clock
    ///
    /// The result is only as accurate as the system clock, so sync it with
    /// something like NTP before other simulators compare their timestamps
    /// with yours.
    #[must_use]
    pub fn now() -> Self {
        let since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        Self::absolute(since_epoch)
    }

    /// An absolute timestamp for the given time past the hour, in UTC.
    ///
    /// Whole hours are discarded, so a time since midnight, or since the Unix
    /// epoch, gives the same result as the time since the start of its hour.
    #[must_use]
    pub fn absolute(time_past_hour: Duration) -> Self {
        Self {
            raw: Self::ticks(time_past_hour) << 1 | ABSOLUTE_BIT,
        }
    }

    /// A relative timestamp for the given time on the simulation's own clock, for
    /// example the time since the simulation started.
    ///
    /// Whole hours are discarded, as for [`absolute`](Self::absolute).
    #[must_use]
    pub fn relative(time: Duration) -> Self {
        Self {
            raw: Self::ticks(time) << 1,
        }
    }

    /// A timestamp from its 32-bit wire value.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The timestamp's 32-bit wire value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// Whether the timestamp is absolute (measured against UTC) rather than relative.
    #[must_use]
    pub const fn is_absolute(self) -> bool {
        self.raw & ABSOLUTE_BIT == ABSOLUTE_BIT
    }

    /// The time since the start of the hour that the timestamp gives.
    #[must_use]
    pub fn time_past_hour(self) -> Duration {
        Self::duration(self.raw >> 1)
    }

    /// How long after `earlier` this timestamp is, allowing for the hour wrapping
    /// around.
    ///
    /// The result is always less than an hour. A timestamp from 59:59, compared
    /// with one from 00:01, is 2 seconds earlier, not 59 minutes and 58 seconds
    /// later. Like every timestamp, it's accurate to within about 1.7 µs, so a
    /// whole number of seconds can come out a fraction of a microsecond short.
    ///
    /// If `earlier` is actually slightly *after* this timestamp, for example
    /// because the sender's clock runs fast, the result is just under an hour.
    /// Treat results above about 30 minutes as meaning `earlier` is in the future.
    ///
    /// Only compare timestamps of the same kind: absolute with absolute, or
    /// relative with relative from the same simulation.
    #[must_use]
    pub fn duration_since(self, earlier: Self) -> Duration {
        let ticks = (self.raw >> 1).wrapping_sub(earlier.raw >> 1) & (u32::MAX >> 1);
        Self::duration(ticks)
    }

    /// Converts a duration to the nearest tick since the start of its hour. Always
    /// below 2³¹: a time that rounds up to the next hour becomes tick 0
    fn ticks(time: Duration) -> u32 {
        let past_hour = time.as_nanos().checked_rem(NANOS_PER_HOUR).unwrap_or(0);
        let ticks = past_hour
            .saturating_mul(TICKS_PER_HOUR)
            .saturating_add(NANOS_PER_HOUR / 2)
            .checked_div(NANOS_PER_HOUR)
            .unwrap_or(0)
            .checked_rem(TICKS_PER_HOUR)
            .unwrap_or(0);
        u32::try_from(ticks).unwrap_or(0)
    }

    /// Converts ticks to a duration. `ticks` is below 2³¹, so this never overflows.
    fn duration(ticks: u32) -> Duration {
        // 3600 s / 2³¹ per tick: whole seconds first, then the remainder in nanoseconds
        let total = u64::from(ticks).saturating_mul(SECONDS_PER_HOUR);
        let seconds = total >> 31;
        let remainder = total & ((1 << 31) - 1);
        // Round to the nearest nanosecond. `Duration::new` carries 10⁹ nanoseconds
        // into the seconds
        let nanos = remainder
            .saturating_mul(1_000_000_000)
            .saturating_add(1 << 30)
            >> 31;
        Duration::new(seconds, u32::try_from(nanos).unwrap_or(0))
    }
}

impl From<Timestamp> for u32 {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.raw
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timestamp")
            .field("time_past_hour", &self.time_past_hour())
            .field("absolute", &self.is_absolute())
            .finish()
    }
}

/// Formats the time past the hour as minutes and seconds, to the nearest microsecond,
/// followed by `abs` or `rel`: for example `35:26.232076 abs`. A timestamp's
/// resolution is about 1.676 µs, so the last digit isn't always exact.
impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Round to the nearest microsecond, but never up to 60:00
        let micros = self
            .time_past_hour()
            .as_nanos()
            .saturating_add(500)
            .checked_div(1000)
            .unwrap_or(0)
            .min(3_599_999_999);
        let seconds = micros / 1_000_000;
        write!(
            f,
            "{:02}:{:02}.{:06} {}",
            seconds / 60,
            seconds % 60,
            micros % 1_000_000,
            if self.is_absolute() { "abs" } else { "rel" }
        )
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.89
#[derive(Copy, Clone, Default, Debug, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct TotalRecordSets {
    pub total_record_sets: u16,
    padding: u16,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.90
#[derive(Copy, Clone, Default, Debug, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct TrackJamData {
    pub entity_id: EntityIdentifier,
    pub emitter_number: u8,
    pub beam_number: u8,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.92
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct UnattachedIdentifier {
    pub simulation_address: SimulationAddress,
    pub reference_number: u16,
}

impl UnattachedIdentifier {
    #[must_use]
    pub const fn new(simulation_address: SimulationAddress, reference_number: u16) -> Self {
        Self {
            simulation_address,
            reference_number,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.93
/// Variable Datum record: an ID, a length in bits, then the value, padded with
/// zeros to a 64-bit boundary.
///
/// `value` holds the value's octets, `ceil(length_bits / 8)` of them; the
/// padding is added when encoding and skipped when decoding.
#[derive(Debug, Clone, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct VariableDatumRecord {
    pub datum_id: u32,
    #[deku(assert = "wire::fits_in_pdu(value_len(*length_bits))")]
    pub length_bits: u32,
    /// Encoding fails unless this holds exactly `ceil(length_bits / 8)` octets.
    #[deku(
        reader = "read_value(deku::reader, *length_bits)",
        writer = "write_value(deku::writer, value, *length_bits)"
    )]
    pub value: Vec<u8>,
}

/// Octets needed to hold `length_bits` bits.
fn value_len(length_bits: u32) -> usize {
    usize::try_from(length_bits.div_ceil(8)).unwrap_or(usize::MAX)
}

/// Zero octets after the value, to reach the next 64-bit boundary.
fn padding_len(length_bits: u32) -> usize {
    let padded = usize::try_from(length_bits.div_ceil(64))
        .unwrap_or(usize::MAX)
        .saturating_mul(8);
    padded.saturating_sub(value_len(length_bits))
}

/// Read the value's octets, then skip its padding.
fn read_value<R: Read + Seek>(
    reader: &mut Reader<R>,
    length_bits: u32,
) -> Result<Vec<u8>, DekuError> {
    let mut value = vec![0; value_len(length_bits)];
    for octet in &mut value {
        *octet = u8::from_reader_with_ctx(reader, ())?;
    }
    for _ in 0..padding_len(length_bits) {
        u8::from_reader_with_ctx(reader, ())?;
    }
    Ok(value)
}

/// Write the value's octets, then zero padding to the 64-bit boundary.
fn write_value<W: Write + Seek>(
    writer: &mut Writer<W>,
    value: &[u8],
    length_bits: u32,
) -> Result<(), DekuError> {
    if value.len() != value_len(length_bits) {
        return Err(DekuError::Assertion(
            format!(
                "VariableDatumRecord: {length_bits} bits needs {} octets, but value has {}",
                value_len(length_bits),
                value.len()
            )
            .into(),
        ));
    }
    value.to_writer(writer, ())?;
    vec![0u8; padding_len(length_bits)].to_writer(writer, ())
}

/// Implemented according to IEEE 1278.1-2012 §6.2.94
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct VariableParameter {
    pub record_type: u8,
    pub variable_parameter_field1: f64,
    pub variable_parameter_field2: u32,
    pub variable_parameter_field3: u16,
    pub variable_parameter_field4: u8,
}

impl VariableParameter {
    #[must_use]
    pub const fn new(
        record_type: u8,
        variable_parameter_field1: f64,
        variable_parameter_field2: u32,
        variable_parameter_field3: u16,
        variable_parameter_field4: u8,
    ) -> Self {
        Self {
            record_type,
            variable_parameter_field1,
            variable_parameter_field2,
            variable_parameter_field3,
            variable_parameter_field4,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.95
/// `record_length` is the whole record in octets, a multiple of 8.
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct VariableTransmitterParameters {
    /// SISO-REF-010 UID 66.
    pub record_type: VariableRecordTypes,
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

impl VariableTransmitterParameters {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.96
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct Vector {
    pub first_vector_component: f32,
    pub second_vector_component: f32,
    pub third_vector_component: f32,
}

impl Vector {
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            first_vector_component: x,
            second_vector_component: y,
            third_vector_component: z,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.97
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct VectoringNozzleSystemData {
    pub horizontal_deflection_angle: f32,
    pub vertical_deflection_angle: f32,
}

impl VectoringNozzleSystemData {
    #[must_use]
    pub const fn new(horizontal_deflection_angle: f32, vertical_deflection_angle: f32) -> Self {
        Self {
            horizontal_deflection_angle,
            vertical_deflection_angle,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.98
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct WorldCoordinates {
    /// The coordinate value along the X-axis which passes through the prime meridian at the equator
    pub x_coordinate: f64,
    /// The coordinate value along the Y-axis which passes through 90°E longitude at the equator
    pub y_coordinate: f64,
    /// The coordinate value along the Z-axis which passes through the North Pole
    pub z_coordinate: f64,
}

impl WorldCoordinates {
    #[must_use]
    pub const fn new(x_coordinate: f64, y_coordinate: f64, z_coordinate: f64) -> Self {
        Self {
            x_coordinate,
            y_coordinate,
            z_coordinate,
        }
    }
}

#[cfg(test)]
mod tests;
