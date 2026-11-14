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

use crate::common::data_types::{
    DataRepresentationType, GridAxisType, IFFDataRecord, RecordSpecificationElement,
    StandardVariableRecords,
};
use crate::common::enums::{
    AggregateStateAggregateKind, BeamStatusBeamState, ComponentDamageStatus,
    ComponentIdentification, ComponentVisualSmokeColor, Country, DEPrecisionAimpointBeamSpotType,
    EEAttributeStateIndicator, EmitterName, EmitterSystemFunction,
    EntityAssociationAssociationType, EntityKind, EntityMarkingCharacterSet,
    EnvironmentalProcessRecordType, ExplosiveMaterialCategories, ForceId, FuelLocation,
    FuelMeasurementUnits, GridAxisDescriptorAxisType, GriddedDataDataRepresentation,
    GriddedDataSampleType, IFFApplicableModes, IFFSimulationMode, IOCommsNodeRecordCommsNodeType,
    IOEffectsRecordIOEffect, IOEffectsRecordIOLinkType, IOEffectsRecordIOProcess,
    IOEffectsRecordIOStatus, IntercomControlCommunicationsType, IntercomControlRecordType,
    MinefieldSensorTypesFLIR, MinefieldSensorTypesLaser, MinefieldSensorTypesMagnetic,
    MinefieldSensorTypesMultispectral, MinefieldSensorTypesOptical, MinefieldSensorTypesPhysical,
    MinefieldSensorTypesRADAR, MinefieldSensorTypesSONAR, MunitionDescriptorFuse,
    MunitionDescriptorWarhead, MunitionExpendableStatus, OtherKinds, SupplyFuelType,
    TransponderInterrogatorIndicator, UAAcousticEmitterSystemFunction, UAAcousticSystemName,
    VariableRecordTypes,
};
use crate::common::wire;

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

/// Implemented according to IEEE 1278.1-2012 §6.2.5
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AggregateType {
    pub aggregate_kind: AggregateStateAggregateKind,
    pub domain: OtherKinds,
    pub country: Country,
    pub category: u8,
    pub subcategory: u8,
    pub specific: u8,
    pub extra: u8,
}

impl AggregateType {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            aggregate_kind: AggregateStateAggregateKind::Other,
            domain: OtherKinds::Other,
            country: Country::Other,
            category: 0,
            subcategory: 0,
            specific: 0,
            extra: 0,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.6
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct Angle(f32);

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

/// Implemented according to IEEE 1278.1-2012 §6.2.12
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct BeamStatus {
    raw: u8,
}

impl BeamStatus {
    const BEAM_STATE: u8 = 0b0000_0001;

    /// A Beam Status record with the given state and the padding bits clear.
    #[must_use]
    pub const fn new(beam_state: BeamStatusBeamState) -> Self {
        let mut status = Self { raw: 0 };
        status.set_beam_state(beam_state);
        status
    }

    /// Whether the beam is active or deactivated
    #[must_use]
    pub const fn beam_state(self) -> BeamStatusBeamState {
        if self.raw & Self::BEAM_STATE == 0 {
            BeamStatusBeamState::Active
        } else {
            BeamStatusBeamState::Deactivated
        }
    }

    /// Sets the Beam State bit, leaving the padding bits as they are
    pub const fn set_beam_state(&mut self, beam_state: BeamStatusBeamState) {
        self.raw = match beam_state {
            BeamStatusBeamState::Active => self.raw & !Self::BEAM_STATE,
            BeamStatusBeamState::Deactivated => self.raw | Self::BEAM_STATE,
        };
    }

    /// A Beam Status record from its wire octet. The padding bits are kept so
    /// a decoded record encodes back to the same octet
    #[must_use]
    pub const fn from_raw(raw: u8) -> Self {
        Self { raw }
    }

    /// The record's wire octet
    #[must_use]
    pub const fn to_raw(self) -> u8 {
        self.raw
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.13 & §B.2.4
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ChangeOptions {
    raw: u8,
}

impl ChangeOptions {
    /// Change Indicator (bit 0): `true` when this is the entity's first IFF PDU, its data
    /// changed since the last one, or it's the first heartbeat after a PDU that had this
    /// bit set. Every system type implements it.
    #[must_use]
    pub const fn initial_report_or_change(self) -> bool {
        self.raw & (1 << 0) != 0
    }

    /// Sets bit 0. See [`initial_report_or_change`](Self::initial_report_or_change).
    pub const fn set_initial_report_or_change(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 0;
        } else {
            self.raw &= !(1 << 0);
        }
    }

    /// System-Specific Field 1 (bit 1). Its meaning depends on the system type: for
    /// a system with a Mode 4 capability, it's the Alternate Mode 4 Indicator; `true`
    /// when present. Systems that don't implement it set this to `false`.
    #[must_use]
    pub const fn system_specific_1(self) -> bool {
        self.raw & (1 << 1) != 0
    }

    /// Sets bit 1. See [`system_specific_1`](Self::system_specific_1).
    pub const fn set_system_specific_1(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 1;
        } else {
            self.raw &= !(1 << 1);
        }
    }

    /// System-Specific Field 2 (bit 2). Its meaning depends on the system type: for
    /// a system with a Mode C capability, it's the Alternate Mode C Indicator; `true`
    /// when present. Systems that don't implement it set this to `false`.
    #[must_use]
    pub const fn system_specific_2(self) -> bool {
        self.raw & (1 << 2) != 0
    }

    /// Sets bit 2. See [`system_specific_2`](Self::system_specific_2).
    pub const fn set_system_specific_2(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 2;
        } else {
            self.raw &= !(1 << 2);
        }
    }

    /// Heartbeat Indicator (bit 3): `true` when this IFF PDU is a heartbeat. Optional for
    /// system types 1 to 5, required for the rest.
    #[must_use]
    pub const fn heartbeat(self) -> bool {
        self.raw & (1 << 3) != 0
    }

    /// Sets bit 3. See [`heartbeat`](Self::heartbeat).
    pub const fn set_heartbeat(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 3;
        } else {
            self.raw &= !(1 << 3);
        }
    }

    const TRANSPONDER_INTERROGATOR: u8 = 1 << 4;
    const SIMULATION_MODE: u8 = 1 << 5;

    /// Transponder/Interrogator Indicator (bit 4) \[UID 337\]. Optional for system types
    /// 1 to 5, required for the rest. A combined system can be either, so the system type
    /// and name along do not say which.
    #[must_use]
    pub const fn transponder_interrogator(self) -> TransponderInterrogatorIndicator {
        if self.raw & Self::TRANSPONDER_INTERROGATOR == 0 {
            TransponderInterrogatorIndicator::Transponder
        } else {
            TransponderInterrogatorIndicator::Interrogator
        }
    }

    /// Sets bit 4. See [`transponder_interrogator`](Self::transponder_interrogator).
    pub const fn set_transponder_interrogator(&mut self, value: TransponderInterrogatorIndicator) {
        self.raw = match value {
            TransponderInterrogatorIndicator::Transponder => {
                self.raw & !Self::TRANSPONDER_INTERROGATOR
            }
            TransponderInterrogatorIndicator::Interrogator => {
                self.raw | Self::TRANSPONDER_INTERROGATOR
            }
        };
    }

    /// Simulation Mode (bit 5) \[UID 338\]: Regeneration or Interactive. Every system type
    /// implements it.
    #[must_use]
    pub const fn simulation_mode(self) -> IFFSimulationMode {
        if self.raw & Self::SIMULATION_MODE == 0 {
            IFFSimulationMode::Regeneration
        } else {
            IFFSimulationMode::Interactive
        }
    }

    /// Sets bit 5. See [`simulation_mode`](Self::simulation_mode).
    pub const fn set_simulation_mode(&mut self, value: IFFSimulationMode) {
        self.raw = match value {
            IFFSimulationMode::Regeneration => self.raw & !Self::SIMULATION_MODE,
            IFFSimulationMode::Interactive => self.raw | Self::SIMULATION_MODE,
        }
    }

    /// Interactive Capable (bit 6): `true` when the IFF system is capable of the
    /// Interactive simulation mode.
    #[must_use]
    pub const fn interactive_capable(self) -> bool {
        self.raw & (1 << 6) != 0
    }

    /// Sets bit 6. See [`interactive_capable`](Self::interactive_capable).
    pub const fn set_interactive_capable(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 6;
        } else {
            self.raw &= !(1 << 6);
        }
    }

    /// Test Mode (bit 7): `true` when the IFF system is in equipment test mode.
    #[must_use]
    pub const fn test_mode(self) -> bool {
        self.raw & (1 << 7) != 0
    }

    /// Sets bit 7. See [`test_mode`](Self::test_mode).
    pub const fn set_test_mode(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 7;
        } else {
            self.raw &= !(1 << 7);
        }
    }

    /// A Change/Options record from its wire octet.
    #[must_use]
    pub const fn from_raw(raw: u8) -> Self {
        Self { raw }
    }

    /// The record's wire octet.
    #[must_use]
    pub const fn to_raw(self) -> u8 {
        self.raw
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

/// Implemented according to IEEE 1278.1-2012 §6.2.15
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct DirectedEnergyDamage {
    pub record_type: u32,
    pub record_length: u16,
    padding: u16,
    pub damage_location: Vector,
    pub damage_diameter: f32,
    pub temperature: f32,
    pub component_identification: ComponentIdentification,
    pub component_damage_status: ComponentDamageStatus,
    pub component_visual_damage_status: u8, // TODO(cameron): implement ComponentVisualDamageStatus record
    pub component_visual_smoke_color: ComponentVisualSmokeColor,
    pub fire_event_id: EventIdentifier,
    padding2: u16,
}

impl DirectedEnergyDamage {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        damage_location: Vector,
        damage_diameter: f32,
        temperature: f32,
        component_identification: ComponentIdentification,
        component_damage_status: ComponentDamageStatus,
        component_visual_damage_status: u8,
        component_visual_smoke_color: ComponentVisualSmokeColor,
        fire_event_id: EventIdentifier,
    ) -> Self {
        Self {
            record_type: 4500,
            record_length: 40,
            padding: 0_u16,
            damage_location,
            damage_diameter,
            temperature,
            component_identification,
            component_damage_status,
            component_visual_damage_status,
            component_visual_smoke_color,
            fire_event_id,
            padding2: 0_u16,
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.16
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct DataFilter {
    raw: u32,
}

impl DataFilter {
    /// Ground Burial Depth Offet (bit 0).
    #[must_use]
    pub const fn ground_burial_depth_offset(self) -> bool {
        self.raw & (1 << 0) != 0
    }

    /// Sets bit 0. See [`ground_burial_depth_offset`](Self::ground_burial_depth_offset).
    pub const fn set_ground_burial_depth_offset(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 0;
        } else {
            self.raw &= !(1 << 0);
        }
    }

    /// Water Burial Depth Offset (bit 1).
    #[must_use]
    pub const fn water_burial_depth_offset(self) -> bool {
        self.raw & (1 << 1) != 0
    }

    /// Sets bit 1. See [`water_burial_depth_offset`](Self::water_burial_depth_offset).
    pub const fn set_water_burial_depth_offset(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 1;
        } else {
            self.raw &= !(1 << 1);
        }
    }

    /// Snow Burial Depth Offset (bit 2).
    #[must_use]
    pub const fn snow_burial_depth_offset(self) -> bool {
        self.raw & (1 << 2) != 0
    }

    /// Sets bit 2. See [`snow_burial_depth_offset`](Self::snow_burial_depth_offset).
    pub const fn set_snow_burial_depth_offset(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 2;
        } else {
            self.raw &= !(1 << 2);
        }
    }

    /// Mine Orientation (bit 3).
    #[must_use]
    pub const fn mine_orientation(self) -> bool {
        self.raw & (1 << 3) != 0
    }

    /// Sets bit 3. See [`mine_orientation`](Self::mine_orientation).
    pub const fn set_mine_orientation(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 3;
        } else {
            self.raw &= !(1 << 3);
        }
    }

    /// Thermal Contrast (bit 4).
    #[must_use]
    pub const fn thermal_contrast(self) -> bool {
        self.raw & (1 << 4) != 0
    }

    /// Sets bit 4. See [`thermal_contrast`](Self::thermal_contrast).
    pub const fn set_thermal_contrast(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 4;
        } else {
            self.raw &= !(1 << 4);
        }
    }

    /// Reflectance (bit 5).
    #[must_use]
    pub const fn reflectance(self) -> bool {
        self.raw & (1 << 5) != 0
    }

    /// Sets bit 5. See [`reflectance`](Self::reflectance).
    pub const fn set_reflectance(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 5;
        } else {
            self.raw &= !(1 << 5);
        }
    }

    /// Mine Emplacement Time (bit 6).
    #[must_use]
    pub const fn mine_emplacement_time(self) -> bool {
        self.raw & (1 << 6) != 0
    }

    /// Sets bit 6. See [`mine_emplacement_time`](Self::mine_emplacement_time).
    pub const fn set_mine_emplacement_time(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 6;
        } else {
            self.raw &= !(1 << 6);
        }
    }

    /// Trip/Detonation Wire (bit 7).
    #[must_use]
    pub const fn trip_detonation_wire(self) -> bool {
        self.raw & (1 << 7) != 0
    }

    /// Sets bit 7. See [`trip_detonation_wire`](Self::trip_detonation_wire).
    pub const fn set_trip_detonation_wire(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 7;
        } else {
            self.raw &= !(1 << 7);
        }
    }

    /// Fusing (bit 8).
    #[must_use]
    pub const fn fusing(self) -> bool {
        self.raw & (1 << 8) != 0
    }

    /// Sets bit 8. See [`fusing`](Self::fusing).
    pub const fn set_fusing(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 8;
        } else {
            self.raw &= !(1 << 8);
        }
    }

    /// Scalar Detection Coefficient (bit 9).
    #[must_use]
    pub const fn scalar_detection_coefficient(self) -> bool {
        self.raw & (1 << 9) != 0
    }

    /// Sets bit 9. See [`scalar_detection_coefficient`](Self::scalar_detection_coefficient).
    pub const fn set_scalar_detection_coefficient(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 9;
        } else {
            self.raw &= !(1 << 9);
        }
    }

    /// Paint Scheme (bit 10).
    #[must_use]
    pub const fn paint_scheme(self) -> bool {
        self.raw & (1 << 10) != 0
    }

    /// Sets bit 10. See [`paint_scheme`](Self::paint_scheme).
    pub const fn set_paint_scheme(&mut self, value: bool) {
        if value {
            self.raw |= 1 << 10;
        } else {
            self.raw &= !(1 << 10);
        }
    }

    /// A Data Filter record from its wire octets.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The record's wire octets.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.17
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct DataQueryDatumSpecification {
    #[deku(
        update = "self.fixed_datums.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_fixed_datums)"
    )]
    pub number_of_fixed_datums: u32,
    #[deku(
        update = "self.variable_datums.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_variable_datums)"
    )]
    pub number_of_variable_datums: u32,
    #[deku(count = "number_of_fixed_datums")]
    pub fixed_datums: Vec<VariableRecordTypes>,
    #[deku(count = "number_of_variable_datums")]
    pub variable_datums: Vec<VariableRecordTypes>,
}

impl DataQueryDatumSpecification {
    #[must_use]
    pub fn new() -> Self {
        Self {
            number_of_fixed_datums: 0,
            number_of_variable_datums: 0,
            fixed_datums: Vec::new(),
            variable_datums: Vec::new(),
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.18
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct DatumSpecification {
    #[deku(
        update = "self.fixed_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_fixed_datum_records)"
    )]
    pub number_of_fixed_datum_records: u32,
    #[deku(
        update = "self.variable_datum_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_variable_datum_records)"
    )]
    pub number_of_variable_datum_records: u32,
    #[deku(count = "number_of_fixed_datum_records")]
    pub fixed_datum_records: Vec<FixedDatumRecord>,
    #[deku(count = "number_of_variable_datum_records")]
    pub variable_datum_records: Vec<VariableDatumRecord>,
}

impl DatumSpecification {
    #[must_use]
    pub fn new() -> Self {
        Self {
            number_of_fixed_datum_records: 0,
            number_of_variable_datum_records: 0,
            fixed_datum_records: Vec::new(),
            variable_datum_records: Vec::new(),
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.19.2
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct MunitionDescriptor {
    pub munition_type: EntityType,
    pub warhead: MunitionDescriptorWarhead,
    pub fuse: MunitionDescriptorFuse,
    pub quantity: u16,
    pub rate: u16,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.19.3
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ExplosionDescriptor {
    pub exploding_object_type: EntityType,
    pub explosive_material: ExplosiveMaterialCategories,
    padding: u16,
    pub explosive_force: f32,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.20.2
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct DEAreaAimpoint {
    pub record_type: u32,
    pub record_length: u16,
    padding: u16,
    pub beam_antenna_pattern_record_count: u16,
    pub de_target_energy_deposition_record_count: u16,
    pub beam_antenna_pattern_records: Vec<BeamAntennaPattern>,
    pub de_target_energy_deposition_records: Vec<DETargetEnergyDeposition>,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.20.3
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct DEPrecisionAimpoint {
    pub record_type: u32,
    pub record_length: u16,
    padding: u16,
    pub target_spot_world_location: WorldCoordinates,
    pub target_spot_entity_location: Vector,
    pub target_spot_velocity: Vector,
    pub target_spot_acceleration: Vector,
    pub target_entity_id: EntityIdentifier,
    pub target_component_identifier: u8,
    pub beam_spot_type: DEPrecisionAimpointBeamSpotType,
    pub beam_spot_cross_section_semi_major_axis: f32,
    pub beam_spot_cross_section_semi_minor_axis: f32,
    pub beam_spot_cross_section_orientation_angle: f32,
    pub peak_irradiance: f32,
    padding2: u32,
}

impl DEPrecisionAimpoint {
    #[must_use]
    pub fn new() -> Self {
        Self {
            record_type: 4000,
            record_length: 96,
            ..Default::default()
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.20.4
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct DETargetEnergyDeposition {
    pub target_entity_id: EntityIdentifier,
    padding: u16,
    pub peak_irradiance: f32,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.21.1
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct BlankingSectorAttribute {
    pub record_type: u32,
    pub record_length: u16,
    padding: u16,
    pub emitter_number: u8,
    pub beam_number: u8,
    pub state_indicator: EEAttributeStateIndicator,
    padding2: u8,
    pub left_azimuth: f32,
    pub right_azimuth: f32,
    pub lower_elevation: f32,
    pub upper_elevation: f32,
    pub residual_power: f32,
    padding3: u64,
}

impl BlankingSectorAttribute {
    #[must_use]
    pub fn new() -> Self {
        Self {
            record_type: 3500,
            record_length: 40,
            ..Default::default()
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.21.2
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AngleDepositionAttribute {
    pub record_type: u32,
    pub record_length: u16,
    padding: u16,
    pub emitter_number: u8,
    pub beam_number: u8,
    pub state_indicator: EEAttributeStateIndicator,
    padding2: u8,
    pub azimuth_offset: f32,
    pub azimuth_width: f32,
    pub azimuth_pull_rate: f32,
    pub azimuth_pull_acceleration: f32,
    pub elevation_offset: f32,
    pub elevation_width: f32,
    pub elevation_pull_rate: f32,
    pub elevation_pull_acceleration: f32,
    padding3: u32,
}

impl AngleDepositionAttribute {
    #[must_use]
    pub fn new() -> Self {
        Self {
            record_type: 3501,
            record_length: 48,
            ..Default::default()
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.21.3
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct FalseTargetsAttribute {
    pub record_type: u32,
    pub record_length: u16,
    padding: u16,
    pub emitter_number: u8,
    pub beam_number: u8,
    pub state_indicator: EEAttributeStateIndicator,
    padding2: u8,
    padding3: u16,
    pub false_target_count: u16,
    pub walk_speed: f32,
    pub walk_acceleration: f32,
    pub maximum_walk_distance: f32,
    pub keep_time: f32,
    pub echo_spacing: f32,
    pub first_target_offset: f32,
}

impl FalseTargetsAttribute {
    #[must_use]
    pub fn new() -> Self {
        Self {
            record_type: 3502,
            record_length: 40,
            ..Default::default()
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.22
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EEFundamentalParameterData {
    pub frequency: f32,
    pub frequency_range: f32,
    pub effective_radiated_power: f32,
    pub pulse_repetition_frequency: f32,
    pub pulse_width: f32,
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

/// Implemented according to IEEE 1278.1-2012 §6.2.24
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EngineFuel {
    pub fuel_quantity: u32,
    pub fuel_measurement_units: FuelMeasurementUnits,
    pub fuel_type: SupplyFuelType,
    pub fuel_location: FuelLocation,
    padding: u8,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.25
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EngineFuelReload {
    pub standard_quantity: u32,
    pub maximum_quantity: u32,
    pub standard_quantity_reload_time: u32,
    pub maximum_quantity_reload_time: u32,
    pub fuel_measurement_units: FuelMeasurementUnits,
    pub fuel_type: SupplyFuelType,
    pub fuel_location: FuelLocation,
    padding: u8,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.26
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EntityAppearance {}

/// Implemented according to IEEE 1278.1-2012 §6.2.27
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EntityCapabilities {}

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

/// Implemented according to IEEE 1278.1-2012 §6.2.34
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ExerciseIdentifier(u8);

/// Implemented according to IEEE 1278.1-2012 §6.2.35
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct Expendable {
    pub expendable_type: EntityType,
    pub station_launcher: u32,
    pub quantity: u16,
    pub expendable_status: MunitionExpendableStatus,
    padding: u8,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.36
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ExpendableReload {
    pub expendable_type: EntityType,
    pub station_launcher: u32,
    pub standard_quantity: u16,
    pub maximum_quantity: u16,
    pub standard_quantity_reload_time: u32,
    pub maximum_quantity_reload_time: u32,
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

/// Implemented according to IEEE 1278.1-2012 §6.2.38
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ForceIdentification(ForceId);

/// Implemented according to IEEE 1278.1-2012 §6.2.39
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct FundamentalOperationalData {
    pub system_status: SystemStatus,
    pub data_field1: u8,
    pub information_layers: InformationLayers,
    pub data_field2: u8,
    pub parameter1: u16,
    pub parameter2: u16,
    pub parameter3: u16,
    pub parameter4: u16,
    pub parameter5: u16,
    pub parameter6: u16,
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

/// Implemented according to IEEE 1278.1-2012 §6.2.42
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct GroupIdentifier {
    pub simulation_address: SimulationAddress,
    pub group_number: u16,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.43
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct IFFDataSpecification {
    #[deku(
        update = "self.iff_data_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_iff_data_records)"
    )]
    pub number_of_iff_data_records: u16,
    #[deku(count = "number_of_iff_data_records")]
    pub iff_data_records: Vec<IFFDataRecord>,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.44
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
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
    pub burst_length: f32,
    pub applicable_modes: IFFApplicableModes,
    #[deku(bytes = 3)]
    pub system_specific_data: u32,
}

bitflags::bitflags! {
    /// Implemented according to IEEE 1278.1-2012 §6.2.45
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub struct InformationLayers: u8 {
        const LAYER_1 = 1 << 1;
        const LAYER_2 = 1 << 2;
        const LAYER_3 = 1 << 3;
        const LAYER_4 = 1 << 4;
        const LAYER_5 = 1 << 5;
        const LAYER_6 = 1 << 6;
        const LAYER_7 = 1 << 7;
    }
}

crate::common::enums::impl_deku_bitflags!(InformationLayers, u8);

impl InformationLayers {
    #[must_use]
    pub const fn layer(number: u8) -> Option<Self> {
        match number {
            1..=7 => Some(Self::from_bits_retain(1 << number)),
            _ => None,
        }
    }
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

/// Implemented according to IEEE 1278.1-2012 §6.2.48.2
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct IOCommunicationsNode {
    pub record_type: u32,
    pub record_length: u16,
    pub communications_node_type: IOCommsNodeRecordCommsNodeType,
    padding: u8,
    pub communications_node_id: CommunicationsNodeID,
}

impl IOCommunicationsNode {
    #[must_use]
    pub fn new() -> Self {
        Self {
            record_type: 5501,
            record_length: 16,
            communications_node_type: IOCommsNodeRecordCommsNodeType::default(),
            padding: 0u8,
            communications_node_id: CommunicationsNodeID::default(),
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.48.3
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct IOEffect {
    pub record_type: u32,
    pub record_length: u16,
    pub io_status: IOEffectsRecordIOStatus,
    pub io_link_type: IOEffectsRecordIOLinkType,
    pub io_effect: IOEffectsRecordIOEffect,
    pub io_effect_duty_cycle: u8,
    pub io_effect_duration: u16,
    pub io_process: IOEffectsRecordIOProcess,
    padding: u16,
}

impl IOEffect {
    #[must_use]
    pub fn new() -> Self {
        Self {
            record_type: 5500,
            record_length: 16,
            ..Default::default()
        }
    }
}

/// Implemented according to IEEE 1278.1-2012 §6.2.48.4
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct CommunicationsNodeID {
    pub entity_id: EntityIdentifier,
    pub element_id: u16,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.49
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct JammingTechnique {
    pub kind: u8,
    pub category: u8,
    pub subcategory: u8,
    pub specific: u8,
}

impl JammingTechnique {
    #[must_use]
    pub const fn new(kind: u8, category: u8, subcategory: u8, specific: u8) -> Self {
        Self {
            kind,
            category,
            subcategory,
            specific,
        }
    }

    /// The technique's name as defined in SISO-REF-010, or `None` if the code is not
    /// in the table.
    #[must_use]
    pub fn name(self) -> Option<&'static str> {
        NAMES
            .iter()
            .find(|(technique, _)| *technique == self)
            .map(|&(_, name)| name)
    }

    /// Whether this technique is `parent` or falls under it in the hierarchy.
    ///
    /// Every level that `parent` specifies must match; the levels it leaves as 0
    /// match anything. So every noise technique is within [`NOISE`](Self::NOISE)
    /// and [`CLICK`](Self::CLICK) is within [`BARRAGE_NOISE`](Self::BARRAGE_NOISE).
    #[must_use]
    pub const fn is_within(self, parent: Self) -> bool {
        const fn level_matches(level: u8, parent: u8) -> bool {
            parent == 0 || level == parent
        }

        self.kind == parent.kind
            && level_matches(self.category, parent.category)
            && level_matches(self.subcategory, parent.subcategory)
            && level_matches(self.specific, parent.specific)
    }
}

/// Every technique in SISO-REF-010-2023 Jamming Technique [UID 284], with its name.
const NAMES: [(JammingTechnique, &str); 134] = [
    (JammingTechnique::new(1, 0, 0, 0), "Noise"),
    (
        JammingTechnique::new(1, 5, 0, 0),
        "Amplitude Modulation (AM) Noise",
    ),
    (JammingTechnique::new(1, 10, 0, 0), "Barrage Noise"),
    (JammingTechnique::new(1, 10, 5, 0), "Click"),
    (JammingTechnique::new(1, 10, 10, 0), "Source Noise"),
    (JammingTechnique::new(1, 15, 0, 0), "Bistatic Clutter"),
    (JammingTechnique::new(1, 20, 0, 0), "Comb"),
    (
        JammingTechnique::new(1, 25, 0, 0),
        "Cooperative Blinked Noise (CBN)",
    ),
    (JammingTechnique::new(1, 30, 0, 0), "Doppler Noise"),
    (
        JammingTechnique::new(1, 35, 0, 0),
        "Frequency Modulation (FM) by Noise",
    ),
    (JammingTechnique::new(1, 40, 0, 0), "Impulse Noise"),
    (JammingTechnique::new(1, 45, 0, 0), "Partial Band"),
    (JammingTechnique::new(1, 50, 0, 0), "Pseudorandom AM"),
    (JammingTechnique::new(1, 55, 0, 0), "Pulse Noise"),
    (
        JammingTechnique::new(1, 60, 0, 0),
        "Quasi-Noise (aka Pseudorandom)",
    ),
    (
        JammingTechnique::new(1, 65, 0, 0),
        "Range Bin Masking (RBM) (aka Cover Pulse)",
    ),
    (
        JammingTechnique::new(1, 65, 5, 0),
        "Range Bin Masking with Velocity Bin Masking",
    ),
    (JammingTechnique::new(1, 70, 0, 0), "Repeater Noise"),
    (
        JammingTechnique::new(1, 70, 5, 0),
        "Narrowband Repeater Noise",
    ),
    (
        JammingTechnique::new(1, 70, 10, 0),
        "Wide Band Repeater Noise",
    ),
    (JammingTechnique::new(1, 75, 0, 0), "Spot Noise"),
    (
        JammingTechnique::new(1, 75, 5, 0),
        "Automatic Spot Noise (ASJ)",
    ),
    (JammingTechnique::new(1, 75, 10, 0), "Blinking Spot Noise"),
    (JammingTechnique::new(1, 75, 15, 0), "Burst Spot Noise"),
    (JammingTechnique::new(1, 75, 20, 0), "Doppler Spot Noise"),
    (JammingTechnique::new(1, 75, 25, 0), "Skirt Frequency"),
    (
        JammingTechnique::new(1, 80, 0, 0),
        "Swept Noise (aka Swept Spot Noise, Sweep)",
    ),
    (JammingTechnique::new(1, 80, 5, 0), "Frequency Swept"),
    (JammingTechnique::new(1, 80, 10, 0), "Swept AM"),
    (
        JammingTechnique::new(1, 85, 0, 0),
        "Velocity Bin Masking (VBM)",
    ),
    (JammingTechnique::new(2, 0, 0, 0), "Deception"),
    (JammingTechnique::new(2, 5, 0, 0), "Analyzer"),
    (JammingTechnique::new(2, 10, 0, 0), "Angle"),
    (JammingTechnique::new(2, 10, 5, 0), "Angle Gate Walk-Off"),
    (
        JammingTechnique::new(2, 10, 10, 0),
        "Cooperative Angle (CAJ)",
    ),
    (JammingTechnique::new(2, 10, 15, 0), "Cross-Eye"),
    (JammingTechnique::new(2, 10, 20, 0), "Cross-Polarization"),
    (JammingTechnique::new(2, 10, 25, 0), "Delta"),
    (
        JammingTechnique::new(2, 10, 30, 0),
        "Inverse Gain (aka Inverse Amplitude)",
    ),
    (JammingTechnique::new(2, 10, 35, 0), "Sea-Bounced"),
    (
        JammingTechnique::new(2, 10, 40, 0),
        "Swept Square Wave (SSW)",
    ),
    (JammingTechnique::new(2, 10, 45, 0), "Terrain Bounce"),
    (JammingTechnique::new(2, 15, 0, 0), "Angle and Gate Stealer"),
    (
        JammingTechnique::new(2, 15, 5, 0),
        "Cross-Polarization and Range Gate Pull-Off (RGPO)",
    ),
    (
        JammingTechnique::new(2, 15, 10, 0),
        "Cross-Polarization and Velocity Gate Pull-Off (VGPO)",
    ),
    (
        JammingTechnique::new(2, 15, 15, 0),
        "Cross-Polarization, RGPO and VGPO",
    ),
    (JammingTechnique::new(2, 15, 20, 0), "Inverse Gain and RGPO"),
    (
        JammingTechnique::new(2, 15, 25, 0),
        "Inverse Gain and RGPO and VGPO",
    ),
    (JammingTechnique::new(2, 15, 30, 0), "Inverse Gain and VGPO"),
    (JammingTechnique::new(2, 15, 35, 0), "RGPO and SSW"),
    (JammingTechnique::new(2, 15, 40, 0), "SSW and VGPO"),
    (JammingTechnique::new(2, 20, 0, 0), "Angle and False Target"),
    (
        JammingTechnique::new(2, 20, 5, 0),
        "Angle and Velocity False Targets (VFT)",
    ),
    (JammingTechnique::new(2, 20, 5, 5), "Inverse Gain and VFT"),
    (JammingTechnique::new(2, 20, 5, 10), "SSW and VFT"),
    (
        JammingTechnique::new(2, 20, 10, 0),
        "Range False Targets (RFT) and Inverse Gain",
    ),
    (JammingTechnique::new(2, 20, 15, 0), "RFT and SSW"),
    (
        JammingTechnique::new(2, 25, 0, 0),
        "Angle and Random Range Programs (RANRAP)",
    ),
    (JammingTechnique::new(2, 25, 5, 0), "RANRAP and SSW"),
    (JammingTechnique::new(2, 30, 0, 0), "Angle and Velocity"),
    (JammingTechnique::new(2, 30, 5, 0), "Inverse Gain and VBM"),
    (JammingTechnique::new(2, 30, 10, 0), "SSW and VBM"),
    (
        JammingTechnique::new(2, 35, 0, 0),
        "Automatic Gain Control (AGC)",
    ),
    (JammingTechnique::new(2, 40, 0, 0), "AGC and Gate Stealer"),
    (JammingTechnique::new(2, 40, 5, 0), "AGC and RGPO"),
    (JammingTechnique::new(2, 40, 10, 0), "AGC and VGPO"),
    (JammingTechnique::new(2, 40, 15, 0), "AGC and RGPO and VGPO"),
    (JammingTechnique::new(2, 45, 0, 0), "Colinear"),
    (
        JammingTechnique::new(2, 50, 0, 0),
        "Constant False Alarm Rate",
    ),
    (JammingTechnique::new(2, 55, 0, 0), "Double Cross"),
    (JammingTechnique::new(2, 60, 0, 0), "Down Link"),
    (JammingTechnique::new(2, 65, 0, 0), "False Target"),
    (JammingTechnique::new(2, 65, 5, 0), "Coherent False Targets"),
    (
        JammingTechnique::new(2, 65, 10, 0),
        "False Doppler Target (FDT)",
    ),
    (
        JammingTechnique::new(2, 65, 15, 0),
        "Multiple False Targets",
    ),
    (
        JammingTechnique::new(2, 65, 20, 0),
        "Range False Targets (RFT)",
    ),
    (JammingTechnique::new(2, 65, 25, 0), "Transponder"),
    (
        JammingTechnique::new(2, 65, 30, 0),
        "Velocity False Targets (VFT)",
    ),
    (JammingTechnique::new(2, 70, 0, 0), "Figure Eight"),
    (JammingTechnique::new(2, 75, 0, 0), "Gate Stealer"),
    (
        JammingTechnique::new(2, 75, 5, 0),
        "Chirp Gate Stealer (CGS)",
    ),
    (
        JammingTechnique::new(2, 75, 10, 0),
        "Range Gate Pull-Off (RGPO)",
    ),
    (JammingTechnique::new(2, 75, 15, 0), "RGPO and VGPO"),
    (
        JammingTechnique::new(2, 75, 20, 0),
        "VGPO (aka Velocity Gate Stealer, VGS)",
    ),
    (
        JammingTechnique::new(2, 80, 0, 0),
        "Gate Stealer and Repeater",
    ),
    (
        JammingTechnique::new(2, 80, 5, 0),
        "Repeater Swept Amplitude Modulation (RSAM) and VGPO",
    ),
    (JammingTechnique::new(2, 85, 0, 0), "Glint Enhance"),
    (JammingTechnique::new(2, 90, 0, 0), "Image Frequency"),
    (
        JammingTechnique::new(2, 95, 0, 0),
        "Jittered Pulse Repetition Frequency",
    ),
    (JammingTechnique::new(2, 100, 0, 0), "Jittered Pulse Width"),
    (
        JammingTechnique::new(2, 105, 0, 0),
        "Pseudorandom Noise (PRN)",
    ),
    (JammingTechnique::new(2, 110, 0, 0), "Pulse"),
    (
        JammingTechnique::new(2, 115, 0, 0),
        "Pulse Compression Deception",
    ),
    (
        JammingTechnique::new(2, 120, 0, 0),
        "Random Range Programs (RANRAP)",
    ),
    (JammingTechnique::new(2, 125, 0, 0), "Refraction"),
    (JammingTechnique::new(2, 130, 0, 0), "Repeater"),
    (
        JammingTechnique::new(2, 130, 5, 0),
        "Continuous Wave Repeater",
    ),
    (JammingTechnique::new(2, 130, 10, 0), "Repeater Noise"),
    (
        JammingTechnique::new(2, 130, 15, 0),
        "Multiple Frequency Repeater (MFR)",
    ),
    (
        JammingTechnique::new(2, 130, 20, 0),
        "Narrow Band Repeater Noise (NBRN)",
    ),
    (JammingTechnique::new(2, 130, 25, 0), "Random Doppler (RD)"),
    (
        JammingTechnique::new(2, 130, 30, 0),
        "Repeater Digital Radio Frequency Memory (DRFM)",
    ),
    (JammingTechnique::new(2, 130, 30, 1), "DRFM Level A"),
    (JammingTechnique::new(2, 130, 30, 2), "DRFM Level B"),
    (JammingTechnique::new(2, 130, 30, 3), "DRFM Level C"),
    (JammingTechnique::new(2, 130, 30, 4), "DRFM Level D"),
    (JammingTechnique::new(2, 130, 30, 5), "DRFM Level E"),
    (
        JammingTechnique::new(2, 130, 35, 0),
        "Repeater Swept Amplitude Modulation (RSAM)",
    ),
    (JammingTechnique::new(2, 135, 0, 0), "Scintillation"),
    (JammingTechnique::new(2, 140, 0, 0), "Serrodyne"),
    (JammingTechnique::new(2, 145, 0, 0), "Velocity"),
    (JammingTechnique::new(3, 0, 0, 0), "Deception and Noise"),
    (JammingTechnique::new(3, 5, 0, 0), "Angle and Noise"),
    (JammingTechnique::new(3, 5, 5, 0), "Angle and Barrage"),
    (
        JammingTechnique::new(3, 5, 5, 5),
        "Barrage and Inverse Gain",
    ),
    (JammingTechnique::new(3, 5, 5, 10), "Barrage and SSW"),
    (JammingTechnique::new(3, 10, 0, 0), "Angle and FM by Noise"),
    (
        JammingTechnique::new(3, 10, 5, 0),
        "FM by Noise and Inverse Gain",
    ),
    (JammingTechnique::new(3, 10, 10, 0), "FM by Noise and SSW"),
    (
        JammingTechnique::new(3, 15, 0, 0),
        "Angle and Pseudorandom AM",
    ),
    (
        JammingTechnique::new(3, 15, 5, 0),
        "Inverse Gain and Pseudorandom AM",
    ),
    (
        JammingTechnique::new(3, 15, 10, 0),
        "Pseudorandom AM and SSW",
    ),
    (JammingTechnique::new(3, 20, 0, 0), "Angle and Spot"),
    (
        JammingTechnique::new(3, 20, 5, 0),
        "Inverse Gain and Low Level Noise",
    ),
    (
        JammingTechnique::new(3, 20, 10, 0),
        "Inverse Gain and Spot Noise",
    ),
    (JammingTechnique::new(3, 20, 15, 0), "Spot and SSW"),
    (JammingTechnique::new(3, 25, 0, 0), "Gate Stealer and Noise"),
    (JammingTechnique::new(3, 25, 5, 0), "Noise and RGPO"),
    (
        JammingTechnique::new(3, 25, 5, 5),
        "Low Level Noise and RGPO",
    ),
    (JammingTechnique::new(3, 25, 10, 0), "Noise and VGPO"),
    (
        JammingTechnique::new(3, 25, 10, 5),
        "Low Level Noise and VGPO",
    ),
    (
        JammingTechnique::new(3, 30, 0, 0),
        "False Target Deception and Swept Noise",
    ),
    (JammingTechnique::new(4, 0, 0, 0), "Special"),
    (JammingTechnique::new(4, 1, 0, 0), "Super Jam"),
];

/// Formats the code in SISO-REF-010's dotted form, without trailing zero levels,
/// followed by the name when it's known: for example `1.10.5 Click`.
impl fmt::Display for JammingTechnique {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let levels = [self.kind, self.category, self.subcategory, self.specific];
        let shown = levels
            .iter()
            .rposition(|&level| level != 0)
            .map_or(1, |last| last.saturating_add(1));
        for (i, level) in levels.iter().take(shown).enumerate() {
            if i > 0 {
                f.write_str(".")?;
            }
            write!(f, "{level}")?;
        }
        if let Some(name) = self.name() {
            write!(f, " {name}")?;
        }
        Ok(())
    }
}

/// The techniques defined in SISO-REF-010-2023 Jamming Technique [UID 284].
impl JammingTechnique {
    /// `1` Noise.
    pub const NOISE: Self = Self::new(1, 0, 0, 0);
    /// `1.5` Amplitude Modulation (AM) Noise, under Noise.
    pub const AMPLITUDE_MODULATION_NOISE: Self = Self::new(1, 5, 0, 0);
    /// `1.10` Barrage Noise, under Noise.
    pub const BARRAGE_NOISE: Self = Self::new(1, 10, 0, 0);
    /// `1.10.5` Click, under Noise → Barrage Noise.
    pub const CLICK: Self = Self::new(1, 10, 5, 0);
    /// `1.10.10` Source Noise, under Noise → Barrage Noise.
    pub const SOURCE_NOISE: Self = Self::new(1, 10, 10, 0);
    /// `1.15` Bistatic Clutter, under Noise.
    pub const BISTATIC_CLUTTER: Self = Self::new(1, 15, 0, 0);
    /// `1.20` Comb, under Noise.
    pub const COMB: Self = Self::new(1, 20, 0, 0);
    /// `1.25` Cooperative Blinked Noise (CBN), under Noise.
    pub const COOPERATIVE_BLINKED_NOISE: Self = Self::new(1, 25, 0, 0);
    /// `1.30` Doppler Noise, under Noise.
    pub const DOPPLER_NOISE: Self = Self::new(1, 30, 0, 0);
    /// `1.35` Frequency Modulation (FM) by Noise, under Noise.
    pub const FREQUENCY_MODULATION_BY_NOISE: Self = Self::new(1, 35, 0, 0);
    /// `1.40` Impulse Noise, under Noise.
    pub const IMPULSE_NOISE: Self = Self::new(1, 40, 0, 0);
    /// `1.45` Partial Band, under Noise.
    pub const PARTIAL_BAND: Self = Self::new(1, 45, 0, 0);
    /// `1.50` Pseudorandom AM, under Noise.
    pub const PSEUDORANDOM_AM: Self = Self::new(1, 50, 0, 0);
    /// `1.55` Pulse Noise, under Noise.
    pub const PULSE_NOISE: Self = Self::new(1, 55, 0, 0);
    /// `1.60` Quasi-Noise (aka Pseudorandom), under Noise.
    pub const QUASI_NOISE: Self = Self::new(1, 60, 0, 0);
    /// `1.65` Range Bin Masking (RBM) (aka Cover Pulse), under Noise.
    pub const RANGE_BIN_MASKING: Self = Self::new(1, 65, 0, 0);
    /// `1.65.5` Range Bin Masking with Velocity Bin Masking, under Noise → Range Bin Masking (RBM) (aka Cover Pulse).
    pub const RANGE_BIN_MASKING_WITH_VELOCITY_BIN_MASKING: Self = Self::new(1, 65, 5, 0);
    /// `1.70` Repeater Noise, under Noise.
    pub const NOISE_REPEATER_NOISE: Self = Self::new(1, 70, 0, 0);
    /// `1.70.5` Narrowband Repeater Noise, under Noise → Repeater Noise.
    pub const NARROWBAND_REPEATER_NOISE: Self = Self::new(1, 70, 5, 0);
    /// `1.70.10` Wide Band Repeater Noise, under Noise → Repeater Noise.
    pub const WIDE_BAND_REPEATER_NOISE: Self = Self::new(1, 70, 10, 0);
    /// `1.75` Spot Noise, under Noise.
    pub const SPOT_NOISE: Self = Self::new(1, 75, 0, 0);
    /// `1.75.5` Automatic Spot Noise (ASJ), under Noise → Spot Noise.
    pub const AUTOMATIC_SPOT_NOISE: Self = Self::new(1, 75, 5, 0);
    /// `1.75.10` Blinking Spot Noise, under Noise → Spot Noise.
    pub const BLINKING_SPOT_NOISE: Self = Self::new(1, 75, 10, 0);
    /// `1.75.15` Burst Spot Noise, under Noise → Spot Noise.
    pub const BURST_SPOT_NOISE: Self = Self::new(1, 75, 15, 0);
    /// `1.75.20` Doppler Spot Noise, under Noise → Spot Noise.
    pub const DOPPLER_SPOT_NOISE: Self = Self::new(1, 75, 20, 0);
    /// `1.75.25` Skirt Frequency, under Noise → Spot Noise.
    pub const SKIRT_FREQUENCY: Self = Self::new(1, 75, 25, 0);
    /// `1.80` Swept Noise (aka Swept Spot Noise, Sweep), under Noise.
    pub const SWEPT_NOISE: Self = Self::new(1, 80, 0, 0);
    /// `1.80.5` Frequency Swept, under Noise → Swept Noise (aka Swept Spot Noise, Sweep).
    pub const FREQUENCY_SWEPT: Self = Self::new(1, 80, 5, 0);
    /// `1.80.10` Swept AM, under Noise → Swept Noise (aka Swept Spot Noise, Sweep).
    pub const SWEPT_AM: Self = Self::new(1, 80, 10, 0);
    /// `1.85` Velocity Bin Masking (VBM), under Noise.
    pub const VELOCITY_BIN_MASKING: Self = Self::new(1, 85, 0, 0);
    /// `2` Deception.
    pub const DECEPTION: Self = Self::new(2, 0, 0, 0);
    /// `2.5` Analyzer, under Deception.
    pub const ANALYZER: Self = Self::new(2, 5, 0, 0);
    /// `2.10` Angle, under Deception.
    pub const ANGLE: Self = Self::new(2, 10, 0, 0);
    /// `2.10.5` Angle Gate Walk-Off, under Deception → Angle.
    pub const ANGLE_GATE_WALK_OFF: Self = Self::new(2, 10, 5, 0);
    /// `2.10.10` Cooperative Angle (CAJ), under Deception → Angle.
    pub const COOPERATIVE_ANGLE: Self = Self::new(2, 10, 10, 0);
    /// `2.10.15` Cross-Eye, under Deception → Angle.
    pub const CROSS_EYE: Self = Self::new(2, 10, 15, 0);
    /// `2.10.20` Cross-Polarization, under Deception → Angle.
    pub const CROSS_POLARIZATION: Self = Self::new(2, 10, 20, 0);
    /// `2.10.25` Delta, under Deception → Angle.
    pub const DELTA: Self = Self::new(2, 10, 25, 0);
    /// `2.10.30` Inverse Gain (aka Inverse Amplitude), under Deception → Angle.
    pub const INVERSE_GAIN: Self = Self::new(2, 10, 30, 0);
    /// `2.10.35` Sea-Bounced, under Deception → Angle.
    pub const SEA_BOUNCED: Self = Self::new(2, 10, 35, 0);
    /// `2.10.40` Swept Square Wave (SSW), under Deception → Angle.
    pub const SWEPT_SQUARE_WAVE: Self = Self::new(2, 10, 40, 0);
    /// `2.10.45` Terrain Bounce, under Deception → Angle.
    pub const TERRAIN_BOUNCE: Self = Self::new(2, 10, 45, 0);
    /// `2.15` Angle and Gate Stealer, under Deception.
    pub const ANGLE_AND_GATE_STEALER: Self = Self::new(2, 15, 0, 0);
    /// `2.15.5` Cross-Polarization and Range Gate Pull-Off (RGPO), under Deception → Angle and Gate Stealer.
    pub const CROSS_POLARIZATION_AND_RANGE_GATE_PULL_OFF: Self = Self::new(2, 15, 5, 0);
    /// `2.15.10` Cross-Polarization and Velocity Gate Pull-Off (VGPO), under Deception → Angle and Gate Stealer.
    pub const CROSS_POLARIZATION_AND_VELOCITY_GATE_PULL_OFF: Self = Self::new(2, 15, 10, 0);
    /// `2.15.15` Cross-Polarization, RGPO and VGPO, under Deception → Angle and Gate Stealer.
    pub const CROSS_POLARIZATION_RGPO_AND_VGPO: Self = Self::new(2, 15, 15, 0);
    /// `2.15.20` Inverse Gain and RGPO, under Deception → Angle and Gate Stealer.
    pub const INVERSE_GAIN_AND_RGPO: Self = Self::new(2, 15, 20, 0);
    /// `2.15.25` Inverse Gain and RGPO and VGPO, under Deception → Angle and Gate Stealer.
    pub const INVERSE_GAIN_AND_RGPO_AND_VGPO: Self = Self::new(2, 15, 25, 0);
    /// `2.15.30` Inverse Gain and VGPO, under Deception → Angle and Gate Stealer.
    pub const INVERSE_GAIN_AND_VGPO: Self = Self::new(2, 15, 30, 0);
    /// `2.15.35` RGPO and SSW, under Deception → Angle and Gate Stealer.
    pub const RGPO_AND_SSW: Self = Self::new(2, 15, 35, 0);
    /// `2.15.40` SSW and VGPO, under Deception → Angle and Gate Stealer.
    pub const SSW_AND_VGPO: Self = Self::new(2, 15, 40, 0);
    /// `2.20` Angle and False Target, under Deception.
    pub const ANGLE_AND_FALSE_TARGET: Self = Self::new(2, 20, 0, 0);
    /// `2.20.5` Angle and Velocity False Targets (VFT), under Deception → Angle and False Target.
    pub const ANGLE_AND_VELOCITY_FALSE_TARGETS: Self = Self::new(2, 20, 5, 0);
    /// `2.20.5.5` Inverse Gain and VFT, under Deception → Angle and False Target → Angle and Velocity False Targets (VFT).
    pub const INVERSE_GAIN_AND_VFT: Self = Self::new(2, 20, 5, 5);
    /// `2.20.5.10` SSW and VFT, under Deception → Angle and False Target → Angle and Velocity False Targets (VFT).
    pub const SSW_AND_VFT: Self = Self::new(2, 20, 5, 10);
    /// `2.20.10` Range False Targets (RFT) and Inverse Gain, under Deception → Angle and False Target.
    pub const RANGE_FALSE_TARGETS_AND_INVERSE_GAIN: Self = Self::new(2, 20, 10, 0);
    /// `2.20.15` RFT and SSW, under Deception → Angle and False Target.
    pub const RFT_AND_SSW: Self = Self::new(2, 20, 15, 0);
    /// `2.25` Angle and Random Range Programs (RANRAP), under Deception.
    pub const ANGLE_AND_RANDOM_RANGE_PROGRAMS: Self = Self::new(2, 25, 0, 0);
    /// `2.25.5` RANRAP and SSW, under Deception → Angle and Random Range Programs (RANRAP).
    pub const RANRAP_AND_SSW: Self = Self::new(2, 25, 5, 0);
    /// `2.30` Angle and Velocity, under Deception.
    pub const ANGLE_AND_VELOCITY: Self = Self::new(2, 30, 0, 0);
    /// `2.30.5` Inverse Gain and VBM, under Deception → Angle and Velocity.
    pub const INVERSE_GAIN_AND_VBM: Self = Self::new(2, 30, 5, 0);
    /// `2.30.10` SSW and VBM, under Deception → Angle and Velocity.
    pub const SSW_AND_VBM: Self = Self::new(2, 30, 10, 0);
    /// `2.35` Automatic Gain Control (AGC), under Deception.
    pub const AUTOMATIC_GAIN_CONTROL: Self = Self::new(2, 35, 0, 0);
    /// `2.40` AGC and Gate Stealer, under Deception.
    pub const AGC_AND_GATE_STEALER: Self = Self::new(2, 40, 0, 0);
    /// `2.40.5` AGC and RGPO, under Deception → AGC and Gate Stealer.
    pub const AGC_AND_RGPO: Self = Self::new(2, 40, 5, 0);
    /// `2.40.10` AGC and VGPO, under Deception → AGC and Gate Stealer.
    pub const AGC_AND_VGPO: Self = Self::new(2, 40, 10, 0);
    /// `2.40.15` AGC and RGPO and VGPO, under Deception → AGC and Gate Stealer.
    pub const AGC_AND_RGPO_AND_VGPO: Self = Self::new(2, 40, 15, 0);
    /// `2.45` Colinear, under Deception.
    pub const COLINEAR: Self = Self::new(2, 45, 0, 0);
    /// `2.50` Constant False Alarm Rate, under Deception.
    pub const CONSTANT_FALSE_ALARM_RATE: Self = Self::new(2, 50, 0, 0);
    /// `2.55` Double Cross, under Deception.
    pub const DOUBLE_CROSS: Self = Self::new(2, 55, 0, 0);
    /// `2.60` Down Link, under Deception.
    pub const DOWN_LINK: Self = Self::new(2, 60, 0, 0);
    /// `2.65` False Target, under Deception.
    pub const FALSE_TARGET: Self = Self::new(2, 65, 0, 0);
    /// `2.65.5` Coherent False Targets, under Deception → False Target.
    pub const COHERENT_FALSE_TARGETS: Self = Self::new(2, 65, 5, 0);
    /// `2.65.10` False Doppler Target (FDT), under Deception → False Target.
    pub const FALSE_DOPPLER_TARGET: Self = Self::new(2, 65, 10, 0);
    /// `2.65.15` Multiple False Targets, under Deception → False Target.
    pub const MULTIPLE_FALSE_TARGETS: Self = Self::new(2, 65, 15, 0);
    /// `2.65.20` Range False Targets (RFT), under Deception → False Target.
    pub const RANGE_FALSE_TARGETS: Self = Self::new(2, 65, 20, 0);
    /// `2.65.25` Transponder, under Deception → False Target.
    pub const TRANSPONDER: Self = Self::new(2, 65, 25, 0);
    /// `2.65.30` Velocity False Targets (VFT), under Deception → False Target.
    pub const VELOCITY_FALSE_TARGETS: Self = Self::new(2, 65, 30, 0);
    /// `2.70` Figure Eight, under Deception.
    pub const FIGURE_EIGHT: Self = Self::new(2, 70, 0, 0);
    /// `2.75` Gate Stealer, under Deception.
    pub const GATE_STEALER: Self = Self::new(2, 75, 0, 0);
    /// `2.75.5` Chirp Gate Stealer (CGS), under Deception → Gate Stealer.
    pub const CHIRP_GATE_STEALER: Self = Self::new(2, 75, 5, 0);
    /// `2.75.10` Range Gate Pull-Off (RGPO), under Deception → Gate Stealer.
    pub const RANGE_GATE_PULL_OFF: Self = Self::new(2, 75, 10, 0);
    /// `2.75.15` RGPO and VGPO, under Deception → Gate Stealer.
    pub const RGPO_AND_VGPO: Self = Self::new(2, 75, 15, 0);
    /// `2.75.20` VGPO (aka Velocity Gate Stealer, VGS), under Deception → Gate Stealer.
    pub const VGPO: Self = Self::new(2, 75, 20, 0);
    /// `2.80` Gate Stealer and Repeater, under Deception.
    pub const GATE_STEALER_AND_REPEATER: Self = Self::new(2, 80, 0, 0);
    /// `2.80.5` Repeater Swept Amplitude Modulation (RSAM) and VGPO, under Deception → Gate Stealer and Repeater.
    pub const REPEATER_SWEPT_AMPLITUDE_MODULATION_AND_VGPO: Self = Self::new(2, 80, 5, 0);
    /// `2.85` Glint Enhance, under Deception.
    pub const GLINT_ENHANCE: Self = Self::new(2, 85, 0, 0);
    /// `2.90` Image Frequency, under Deception.
    pub const IMAGE_FREQUENCY: Self = Self::new(2, 90, 0, 0);
    /// `2.95` Jittered Pulse Repetition Frequency, under Deception.
    pub const JITTERED_PULSE_REPETITION_FREQUENCY: Self = Self::new(2, 95, 0, 0);
    /// `2.100` Jittered Pulse Width, under Deception.
    pub const JITTERED_PULSE_WIDTH: Self = Self::new(2, 100, 0, 0);
    /// `2.105` Pseudorandom Noise (PRN), under Deception.
    pub const PSEUDORANDOM_NOISE: Self = Self::new(2, 105, 0, 0);
    /// `2.110` Pulse, under Deception.
    pub const PULSE: Self = Self::new(2, 110, 0, 0);
    /// `2.115` Pulse Compression Deception, under Deception.
    pub const PULSE_COMPRESSION_DECEPTION: Self = Self::new(2, 115, 0, 0);
    /// `2.120` Random Range Programs (RANRAP), under Deception.
    pub const RANDOM_RANGE_PROGRAMS: Self = Self::new(2, 120, 0, 0);
    /// `2.125` Refraction, under Deception.
    pub const REFRACTION: Self = Self::new(2, 125, 0, 0);
    /// `2.130` Repeater, under Deception.
    pub const REPEATER: Self = Self::new(2, 130, 0, 0);
    /// `2.130.5` Continuous Wave Repeater, under Deception → Repeater.
    pub const CONTINUOUS_WAVE_REPEATER: Self = Self::new(2, 130, 5, 0);
    /// `2.130.10` Repeater Noise, under Deception → Repeater.
    pub const DECEPTION_REPEATER_NOISE: Self = Self::new(2, 130, 10, 0);
    /// `2.130.15` Multiple Frequency Repeater (MFR), under Deception → Repeater.
    pub const MULTIPLE_FREQUENCY_REPEATER: Self = Self::new(2, 130, 15, 0);
    /// `2.130.20` Narrow Band Repeater Noise (NBRN), under Deception → Repeater.
    pub const NARROW_BAND_REPEATER_NOISE: Self = Self::new(2, 130, 20, 0);
    /// `2.130.25` Random Doppler (RD), under Deception → Repeater.
    pub const RANDOM_DOPPLER: Self = Self::new(2, 130, 25, 0);
    /// `2.130.30` Repeater Digital Radio Frequency Memory (DRFM), under Deception → Repeater.
    pub const REPEATER_DIGITAL_RADIO_FREQUENCY_MEMORY: Self = Self::new(2, 130, 30, 0);
    /// `2.130.30.1` DRFM Level A, under Deception → Repeater → Repeater Digital Radio Frequency Memory (DRFM).
    pub const DRFM_LEVEL_A: Self = Self::new(2, 130, 30, 1);
    /// `2.130.30.2` DRFM Level B, under Deception → Repeater → Repeater Digital Radio Frequency Memory (DRFM).
    pub const DRFM_LEVEL_B: Self = Self::new(2, 130, 30, 2);
    /// `2.130.30.3` DRFM Level C, under Deception → Repeater → Repeater Digital Radio Frequency Memory (DRFM).
    pub const DRFM_LEVEL_C: Self = Self::new(2, 130, 30, 3);
    /// `2.130.30.4` DRFM Level D, under Deception → Repeater → Repeater Digital Radio Frequency Memory (DRFM).
    pub const DRFM_LEVEL_D: Self = Self::new(2, 130, 30, 4);
    /// `2.130.30.5` DRFM Level E, under Deception → Repeater → Repeater Digital Radio Frequency Memory (DRFM).
    pub const DRFM_LEVEL_E: Self = Self::new(2, 130, 30, 5);
    /// `2.130.35` Repeater Swept Amplitude Modulation (RSAM), under Deception → Repeater.
    pub const REPEATER_SWEPT_AMPLITUDE_MODULATION: Self = Self::new(2, 130, 35, 0);
    /// `2.135` Scintillation, under Deception.
    pub const SCINTILLATION: Self = Self::new(2, 135, 0, 0);
    /// `2.140` Serrodyne, under Deception.
    pub const SERRODYNE: Self = Self::new(2, 140, 0, 0);
    /// `2.145` Velocity, under Deception.
    pub const VELOCITY: Self = Self::new(2, 145, 0, 0);
    /// `3` Deception and Noise.
    pub const DECEPTION_AND_NOISE: Self = Self::new(3, 0, 0, 0);
    /// `3.5` Angle and Noise, under Deception and Noise.
    pub const ANGLE_AND_NOISE: Self = Self::new(3, 5, 0, 0);
    /// `3.5.5` Angle and Barrage, under Deception and Noise → Angle and Noise.
    pub const ANGLE_AND_BARRAGE: Self = Self::new(3, 5, 5, 0);
    /// `3.5.5.5` Barrage and Inverse Gain, under Deception and Noise → Angle and Noise → Angle and Barrage.
    pub const BARRAGE_AND_INVERSE_GAIN: Self = Self::new(3, 5, 5, 5);
    /// `3.5.5.10` Barrage and SSW, under Deception and Noise → Angle and Noise → Angle and Barrage.
    pub const BARRAGE_AND_SSW: Self = Self::new(3, 5, 5, 10);
    /// `3.10` Angle and FM by Noise, under Deception and Noise.
    pub const ANGLE_AND_FM_BY_NOISE: Self = Self::new(3, 10, 0, 0);
    /// `3.10.5` FM by Noise and Inverse Gain, under Deception and Noise → Angle and FM by Noise.
    pub const FM_BY_NOISE_AND_INVERSE_GAIN: Self = Self::new(3, 10, 5, 0);
    /// `3.10.10` FM by Noise and SSW, under Deception and Noise → Angle and FM by Noise.
    pub const FM_BY_NOISE_AND_SSW: Self = Self::new(3, 10, 10, 0);
    /// `3.15` Angle and Pseudorandom AM, under Deception and Noise.
    pub const ANGLE_AND_PSEUDORANDOM_AM: Self = Self::new(3, 15, 0, 0);
    /// `3.15.5` Inverse Gain and Pseudorandom AM, under Deception and Noise → Angle and Pseudorandom AM.
    pub const INVERSE_GAIN_AND_PSEUDORANDOM_AM: Self = Self::new(3, 15, 5, 0);
    /// `3.15.10` Pseudorandom AM and SSW, under Deception and Noise → Angle and Pseudorandom AM.
    pub const PSEUDORANDOM_AM_AND_SSW: Self = Self::new(3, 15, 10, 0);
    /// `3.20` Angle and Spot, under Deception and Noise.
    pub const ANGLE_AND_SPOT: Self = Self::new(3, 20, 0, 0);
    /// `3.20.5` Inverse Gain and Low Level Noise, under Deception and Noise → Angle and Spot.
    pub const INVERSE_GAIN_AND_LOW_LEVEL_NOISE: Self = Self::new(3, 20, 5, 0);
    /// `3.20.10` Inverse Gain and Spot Noise, under Deception and Noise → Angle and Spot.
    pub const INVERSE_GAIN_AND_SPOT_NOISE: Self = Self::new(3, 20, 10, 0);
    /// `3.20.15` Spot and SSW, under Deception and Noise → Angle and Spot.
    pub const SPOT_AND_SSW: Self = Self::new(3, 20, 15, 0);
    /// `3.25` Gate Stealer and Noise, under Deception and Noise.
    pub const GATE_STEALER_AND_NOISE: Self = Self::new(3, 25, 0, 0);
    /// `3.25.5` Noise and RGPO, under Deception and Noise → Gate Stealer and Noise.
    pub const NOISE_AND_RGPO: Self = Self::new(3, 25, 5, 0);
    /// `3.25.5.5` Low Level Noise and RGPO, under Deception and Noise → Gate Stealer and Noise → Noise and RGPO.
    pub const LOW_LEVEL_NOISE_AND_RGPO: Self = Self::new(3, 25, 5, 5);
    /// `3.25.10` Noise and VGPO, under Deception and Noise → Gate Stealer and Noise.
    pub const NOISE_AND_VGPO: Self = Self::new(3, 25, 10, 0);
    /// `3.25.10.5` Low Level Noise and VGPO, under Deception and Noise → Gate Stealer and Noise → Noise and VGPO.
    pub const LOW_LEVEL_NOISE_AND_VGPO: Self = Self::new(3, 25, 10, 5);
    /// `3.30` False Target Deception and Swept Noise, under Deception and Noise.
    pub const FALSE_TARGET_DECEPTION_AND_SWEPT_NOISE: Self = Self::new(3, 30, 0, 0);
    /// `4` Special.
    pub const SPECIAL: Self = Self::new(4, 0, 0, 0);
    /// `4.1` Super Jam, under Special.
    pub const SUPER_JAM: Self = Self::new(4, 1, 0, 0);
}

/// Implemented according to IEEE 1278.1-2012 §6.2.50
#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LaunchedMunition {
    pub fire_event_id: EventIdentifier,
    padding: u16,
    pub firing_entity_id: EntityIdentifier,
    padding2: u16,
    pub targeting_entity_id: EntityIdentifier,
    padding3: u16,
    pub target_location_in_world_coordinates: WorldCoordinates,
}

impl LaunchedMunition {
    #[must_use]
    pub fn new() -> Self {
        Self {
            fire_event_id: EventIdentifier::default(),
            padding: 0u16,
            firing_entity_id: EntityIdentifier::default(),
            padding2: 0u16,
            targeting_entity_id: EntityIdentifier::default(),
            padding3: 0u16,
            target_location_in_world_coordinates: WorldCoordinates::default(),
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

/// Implemented according to IEEE 1278.1-2012 §6.2.53
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LiveEntityIdentifier {
    pub live_simulation_address: LiveSimulationAddress,
    pub entity_number: u16,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.54
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LiveSimulationAddress {
    pub live_site_number: u8,
    pub live_application_number: u8,
}

/// Implemented according to IEEE 1278.1-2012 §6.2.55
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct MineEntityIdentifier {
    pub simulation_address: SimulationAddress,
    pub mine_entity_number: u16,
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

/// Implemented according to IEEE 1278.1-2012 §6.2.57
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MinefieldSensorType {
    Other(u16),
    Optical(MinefieldSensorTypesOptical),
    Flir(MinefieldSensorTypesFLIR),
    Radar(MinefieldSensorTypesRADAR),
    Magnetic(MinefieldSensorTypesMagnetic),
    Laser(MinefieldSensorTypesLaser),
    Sonar(MinefieldSensorTypesSONAR),
    Physical(MinefieldSensorTypesPhysical),
    Multispectral(MinefieldSensorTypesMultispectral),
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
