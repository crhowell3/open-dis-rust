//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

//! The Live Entity (LE) Information/Interaction protocol family

use bitflags::bitflags;
use deku::{
    DekuError, DekuReader, DekuWriter,
    no_std_io::{Read, Seek, Write},
    reader::Reader,
    writer::Writer,
};

use crate::{
    common::{
        LiveEntityPduHeader,
        data_types::{
            EntityType, VariableParameter, entity_marking::EntityMarking,
            fixed_binary_16::FixedBinary16, munition_descriptor::MunitionDescriptor,
        },
        enums::{ForceId, PduType, ProtocolFamily, impl_deku_bitflags},
        live_entity_records::{
            LEDeadReckoningParameters, LEEntityId, LEEulerAngles, LELinearVelocity,
            LiveEntityOrientation16, LiveEventIdentifier, OrientationError, PositionError,
            RelativeWorldCoordinates,
        },
    },
    impl_pdu,
};

bitflags! {
    /// TSPI Flag (IEEE 1278.1-2012 §9.4.2.5): which optional TSPI fields are
    /// present. Bit 0 is the least significant bit.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct TspiFlag: u8 {
        const LINEAR_VELOCITY      = 1 << 0;
        const ORIENTATION          = 1 << 1;
        const POSITION_ERROR       = 1 << 2;
        const ORIENTATION_ERROR    = 1 << 3;
        const DEAD_RECKONING       = 1 << 4;
        const MEASURED_SPEED       = 1 << 5;
        const SYSTEM_SPECIFIC_DATA = 1 << 6;
    }
}
impl_deku_bitflags!(TspiFlag, u8);

/// Implemented according to IEEE 1278.1-2012 §9.4.2.5
///
/// Only the entity ID, flag and location are always present. Set the
/// optional fields you want to send; `serialize` sets `tspi_flag` to match.
#[derive(Debug, Default, Clone, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct TimeSpacePositionInformationPdu {
    #[deku(skip)]
    header: LiveEntityPduHeader,
    pub live_entity_id: LEEntityId,
    #[deku(update = "self.derived_tspi_flag()")]
    pub tspi_flag: TspiFlag,
    pub entity_location: RelativeWorldCoordinates,
    #[deku(cond = "tspi_flag.contains(TspiFlag::LINEAR_VELOCITY)")]
    pub entity_linear_velocity: Option<LELinearVelocity>,
    #[deku(cond = "tspi_flag.contains(TspiFlag::ORIENTATION)")]
    pub entity_orientation: Option<LEEulerAngles>,
    #[deku(cond = "tspi_flag.contains(TspiFlag::POSITION_ERROR)")]
    pub position_error: Option<PositionError>,
    #[deku(cond = "tspi_flag.contains(TspiFlag::ORIENTATION_ERROR)")]
    pub orientation_error: Option<OrientationError>,
    #[deku(cond = "tspi_flag.contains(TspiFlag::DEAD_RECKONING)")]
    pub dead_reckoning_parameters: Option<LEDeadReckoningParameters>,
    #[deku(cond = "tspi_flag.contains(TspiFlag::MEASURED_SPEED)")]
    pub measured_speed: Option<FixedBinary16>,
    /// Up to 255 octets. Sent as a one-octet length followed by the data.
    #[deku(
        cond = "tspi_flag.contains(TspiFlag::SYSTEM_SPECIFIC_DATA)",
        reader = "read_system_specific_data(deku::reader)",
        writer = "write_system_specific_data(deku::writer, system_specific_data.as_deref())"
    )]
    pub system_specific_data: Option<Vec<u8>>,
}
impl_pdu!(
    TimeSpacePositionInformationPdu,
    LiveEntityPduHeader,
    PduType::TimeSpacePositionInformation,
    ProtocolFamily::LiveEntityInformationInteraction
);

impl TimeSpacePositionInformationPdu {
    /// The flag implied by which optional fields are present. Bits the
    /// flag doesn't name are kept.
    fn derived_tspi_flag(&self) -> TspiFlag {
        let mut flag = self.tspi_flag;
        flag.set(
            TspiFlag::LINEAR_VELOCITY,
            self.entity_linear_velocity.is_some(),
        );
        flag.set(TspiFlag::ORIENTATION, self.entity_orientation.is_some());
        flag.set(TspiFlag::POSITION_ERROR, self.position_error.is_some());
        flag.set(
            TspiFlag::ORIENTATION_ERROR,
            self.orientation_error.is_some(),
        );
        flag.set(
            TspiFlag::DEAD_RECKONING,
            self.dead_reckoning_parameters.is_some(),
        );
        flag.set(TspiFlag::MEASURED_SPEED, self.measured_speed.is_some());
        flag.set(
            TspiFlag::SYSTEM_SPECIFIC_DATA,
            self.system_specific_data.is_some(),
        );
        flag
    }
}

fn read_system_specific_data<R: Read + Seek>(
    reader: &mut Reader<R>,
) -> Result<Option<Vec<u8>>, DekuError> {
    let length = u8::from_reader_with_ctx(reader, ())?;
    let mut data = vec![0; usize::from(length)];
    for octet in &mut data {
        *octet = u8::from_reader_with_ctx(reader, ())?;
    }
    Ok(Some(data))
}

fn write_system_specific_data<W: Write + Seek>(
    writer: &mut Writer<W>,
    data: Option<&[u8]>,
) -> Result<(), DekuError> {
    let Some(data) = data else {
        return Ok(());
    };
    let length = u8::try_from(data.len()).map_err(|_| {
        DekuError::Assertion("TSPI system-specific data is longer than 255 octets".into())
    })?;
    length.to_writer(writer, ())?;
    data.to_writer(writer, ())
}

// ---------------------------------------------------------------------------
// Appearance
// ---------------------------------------------------------------------------

bitflags! {
    /// Appearance Flags 1 (IEEE 1278.1-2012 §9.4.3.5)
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct AppearanceFlags1: u8 {
        const FORCE_ID_INCLUDED          = 0b0000_0001;
        const ENTITY_TYPE_INCLUDED       = 0b0000_0010;
        const ALT_ENTITY_TYPE_INCLUDED   = 0b0000_0100;
        const ENTITY_MARKING_INCLUDED    = 0b0000_1000;
        const CAPABILITIES_INCLUDED      = 0b0001_0000;
        const VISUAL_INCLUDED            = 0b0010_0000;
        const IR_INCLUDED                = 0b0100_0000;
        const FLAG2_INCLUDED             = 0b1000_0000;
    }
}
impl_deku_bitflags!(AppearanceFlags1, u8);

bitflags! {
    /// Appearance Flags 2 (IEEE 1278.1-2012 §9.4.3.5). Bits it doesn't name
    /// are kept.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct AppearanceFlags2: u8 {
        const EM_INCLUDED     = 0b0000_0010;
        const AUDIO_INCLUDED  = 0b0000_0100;
    }
}
impl_deku_bitflags!(AppearanceFlags2, u8);

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct PlatformAppearance: u32 {
        const PaintScheme = 0b0000_0000_0000_0001;
    }
}

/// A 32-bit entity appearance record (SISO-REF-010 UIDs 31-43).
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EntityAppearance {
    data: u32,
}

#[allow(dead_code)]
impl EntityAppearance {
    #[must_use]
    pub const fn new(data: u32) -> Self {
        Self { data }
    }

    #[must_use]
    pub const fn data(&self) -> u32 {
        self.data
    }

    pub const fn set_data(&mut self, value: u32) {
        self.data = value;
    }

    const fn get_bits(self, shift: u8, mask: u32) -> u32 {
        (self.data >> shift) & mask
    }

    const fn set_bits(&mut self, shift: u8, mask: u32, value: u32) {
        self.data &= !(mask << shift);
        self.data |= (value & mask) << shift;
    }
}

/// Implemented according to IEEE 1278.1-2012 §9.4.3.5
///
/// Set the optional fields you want to send; `serialize` sets the presence
/// flags to match.
#[derive(Debug, Default, Clone, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct AppearancePdu {
    #[deku(skip)]
    header: LiveEntityPduHeader,
    pub live_entity_id: LEEntityId,
    #[deku(update = "self.derived_flags1()")]
    pub appearance_flags1: AppearanceFlags1,
    #[deku(
        cond = "appearance_flags1.contains(AppearanceFlags1::FLAG2_INCLUDED)",
        update = "self.derived_flags2()"
    )]
    pub appearance_flags2: Option<AppearanceFlags2>,
    #[deku(cond = "appearance_flags1.contains(AppearanceFlags1::FORCE_ID_INCLUDED)")]
    pub force_id: Option<ForceId>,
    #[deku(cond = "appearance_flags1.contains(AppearanceFlags1::ENTITY_TYPE_INCLUDED)")]
    pub entity_type: Option<EntityType>,
    #[deku(cond = "appearance_flags1.contains(AppearanceFlags1::ALT_ENTITY_TYPE_INCLUDED)")]
    pub alternate_entity_type: Option<EntityType>,
    #[deku(cond = "appearance_flags1.contains(AppearanceFlags1::ENTITY_MARKING_INCLUDED)")]
    pub entity_marking: Option<EntityMarking>,
    /// SISO-REF-010 UID 55: a set of capability bits whose meaning depends
    /// on the entity kind and domain.
    #[deku(cond = "appearance_flags1.contains(AppearanceFlags1::CAPABILITIES_INCLUDED)")]
    pub capabilities: Option<u32>,
    #[deku(cond = "appearance_flags1.contains(AppearanceFlags1::VISUAL_INCLUDED)")]
    pub appearance_visual: Option<EntityAppearance>,
    #[deku(cond = "appearance_flags1.contains(AppearanceFlags1::IR_INCLUDED)")]
    pub appearance_ir: Option<EntityAppearance>,
    #[deku(cond = "appearance_flags2.is_some_and(|f| f.contains(AppearanceFlags2::EM_INCLUDED))")]
    pub appearance_em: Option<EntityAppearance>,
    #[deku(
        cond = "appearance_flags2.is_some_and(|f| f.contains(AppearanceFlags2::AUDIO_INCLUDED))"
    )]
    pub appearance_audio: Option<EntityAppearance>,
}
impl_pdu!(
    AppearancePdu,
    LiveEntityPduHeader,
    PduType::Appearance,
    ProtocolFamily::LiveEntityInformationInteraction
);

impl AppearancePdu {
    /// Flags 2 as implied by the EM and audio fields. A Flags 2 record that
    /// is already present is kept, with any bits it doesn't name.
    fn derived_flags2(&self) -> Option<AppearanceFlags2> {
        let em = self.appearance_em.is_some();
        let audio = self.appearance_audio.is_some();
        if self.appearance_flags2.is_none() && !em && !audio {
            return None;
        }
        let mut flags = self.appearance_flags2.unwrap_or_default();
        flags.set(AppearanceFlags2::EM_INCLUDED, em);
        flags.set(AppearanceFlags2::AUDIO_INCLUDED, audio);
        Some(flags)
    }

    /// Flags 1 as implied by which optional fields are present.
    fn derived_flags1(&self) -> AppearanceFlags1 {
        let mut flags = AppearanceFlags1::empty();
        flags.set(AppearanceFlags1::FORCE_ID_INCLUDED, self.force_id.is_some());
        flags.set(
            AppearanceFlags1::ENTITY_TYPE_INCLUDED,
            self.entity_type.is_some(),
        );
        flags.set(
            AppearanceFlags1::ALT_ENTITY_TYPE_INCLUDED,
            self.alternate_entity_type.is_some(),
        );
        flags.set(
            AppearanceFlags1::ENTITY_MARKING_INCLUDED,
            self.entity_marking.is_some(),
        );
        flags.set(
            AppearanceFlags1::CAPABILITIES_INCLUDED,
            self.capabilities.is_some(),
        );
        flags.set(
            AppearanceFlags1::VISUAL_INCLUDED,
            self.appearance_visual.is_some(),
        );
        flags.set(AppearanceFlags1::IR_INCLUDED, self.appearance_ir.is_some());
        flags.set(
            AppearanceFlags1::FLAG2_INCLUDED,
            self.derived_flags2().is_some(),
        );
        flags
    }
}

/// Implemented according to IEEE 1278.1-2012 §9.4.4.5
#[derive(Debug, Default, Clone, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct ArticulatedPartsPdu {
    #[deku(skip)]
    header: LiveEntityPduHeader,
    pub live_entity_id: LEEntityId,
    /// Set by `serialize` from `variable_parameter_records`.
    #[deku(update = "self.variable_parameter_records.len()")]
    pub number_of_variable_parameter_records: u8,
    #[deku(count = "number_of_variable_parameter_records")]
    pub variable_parameter_records: Vec<VariableParameter>,
}
impl_pdu!(
    ArticulatedPartsPdu,
    LiveEntityPduHeader,
    PduType::ArticulatedParts,
    ProtocolFamily::LiveEntityInformationInteraction
);

/// Implemented according to IEEE 1278.1-2012 §9.4.5.6
#[derive(Debug, Default, Clone, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct LiveEntityFirePdu {
    #[deku(skip)]
    header: LiveEntityPduHeader,
    pub firing_live_entity_id: LEEntityId,
    pub flags: u8,
    pub padding: u8,
    pub target_live_entity_id: LEEntityId,
    pub munition_live_entity_id: LEEntityId,
    pub event_id: LiveEventIdentifier,
    pub location: RelativeWorldCoordinates,
    pub munition_descriptor: MunitionDescriptor,
    pub velocity: LELinearVelocity,
    pub range: u16,
}
impl_pdu!(
    LiveEntityFirePdu,
    LiveEntityPduHeader,
    PduType::LiveEntityFire,
    ProtocolFamily::LiveEntityInformationInteraction
);

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LiveEntityRelativeLocation {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LiveMunitionDescriptor {
    pub munition_type: EntityType,
    pub warhead: u16,
    pub fuse: u16,
    pub quantity: u16,
    pub rate: u16,
}

/// Implemented according to IEEE 1278.1-2012 §9.4.6.8
#[derive(Debug, Default, Clone, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct LiveEntityDetonationPdu {
    #[deku(skip)]
    header: LiveEntityPduHeader,
    pub firing_live_entity_id: LEEntityId,
    pub detonation_flag_1: u8,
    pub detonation_flag_2: u8,
    pub target_live_entity_id: LEEntityId,
    pub munition_live_entity_id: LEEntityId,
    pub event_id: LiveEventIdentifier,
    pub location_in_world_coordinates: RelativeWorldCoordinates,
    pub velocity: LELinearVelocity,
    pub munition_orientation: LiveEntityOrientation16,
    pub munition_descriptor: LiveMunitionDescriptor,
    pub location_in_entity_coordinates: LiveEntityRelativeLocation,
    pub detonation_result: u8,
    pub padding: u8,
}
impl_pdu!(
    LiveEntityDetonationPdu,
    LiveEntityPduHeader,
    PduType::LiveEntityDetonation,
    ProtocolFamily::LiveEntityInformationInteraction
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::{
            constants::BITS_PER_BYTE,
            pdu::{Pdu, to_bytes},
        },
        pdu_tests,
    };
    use bytes::Buf;

    pdu_tests!(
        time_space_position_information_pdu_tests, TimeSpacePositionInformationPdu,
        pdu_type: PduType::TimeSpacePositionInformation,
        default_bits: 432,
        wrong_type: AppearancePdu,
    );
    pdu_tests!(
        appearance_pdu_tests, AppearancePdu,
        pdu_type: PduType::Appearance,
        default_bits: 136,
        wrong_type: TimeSpacePositionInformationPdu,
    );
    pdu_tests!(
        articulated_parts_pdu_tests, ArticulatedPartsPdu,
        pdu_type: PduType::ArticulatedParts,
        default_bits: 136,
        wrong_type: AppearancePdu,
    );
    pdu_tests!(
        live_entity_fire_pdu_tests, LiveEntityFirePdu,
        pdu_type: PduType::LiveEntityFire,
        default_bits: 496,
        wrong_type: LiveEntityDetonationPdu,
    );
    pdu_tests!(
        live_entity_detonation_pdu_tests, LiveEntityDetonationPdu,
        pdu_type: PduType::LiveEntityDetonation,
        default_bits: 592,
        wrong_type: LiveEntityFirePdu,
    );
}
