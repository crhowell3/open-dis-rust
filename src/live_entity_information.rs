//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Live Entity (LE) Information/Interaction protocol family

use bitflags::bitflags;
use deku::{
    DekuError, DekuReader, DekuWriter,
    no_std_io::{Read, Seek, Write},
    reader::Reader,
    writer::Writer,
};

use crate::siso::enums::{DetonationResult, ForceId, PduType, ProtocolFamily};
use crate::wire::impl_deku_bitflags;
use crate::{
    live_entity_records::{
        FixedBinary16, LEDeadReckoningParameters, LEEntityId, LEEulerAngles, LELinearVelocity,
        LiveEntityOrientation16, LiveEntityPduHeader, LiveEntityRelativeLocation,
        LiveEventIdentifier, LiveMunitionDescriptor, OrientationError, PositionError,
        RelativeWorldCoordinates,
    },
    pdu::impl_pdu,
    records::{EntityAppearance, EntityCapabilities, EntityMarking, EntityType, VariableParameter},
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
    pub measured_speed: Option<FixedBinary16<0>>,
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
    /// Appearance Flag-2 (IEEE 1278.1-2012 §9.4.3.5, Table 222). Bits it
    /// does not name are kept.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct AppearanceFlags2: u8 {
        const EM_INCLUDED     = 0b0000_0001;
        const AUDIO_INCLUDED  = 0b0000_0010;
    }
}
impl_deku_bitflags!(AppearanceFlags2, u8);

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
    pub capabilities: Option<EntityCapabilities>,
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

bitflags! {
    /// LE Fire Flag (IEEE 1278.1-2012 §9.4.5.2, Table 225): which optional
    /// LE Fire PDU fields are present. `serialize` sets it from the fields
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct LEFireFlags: u8 {
        const TARGET_ID                 = 1 << 0;
        const MUNITION_SITE_APPLICATION = 1 << 1;
        const MUNITION_ID               = 1 << 2;
        const EVENT_SITE_APPLICATION    = 1 << 3;
        const WARHEAD_FUSE              = 1 << 4;
        const QUANTITY_RATE             = 1 << 5;
        const LOCATION                  = 1 << 6;
    }
}
impl_deku_bitflags!(LEFireFlags, u8);

/// Implemented according to IEEE 1278.1-2012 §9.4.5.6
///
/// Set the optional fields you want to send; `serialize` sets `flags` to
/// match. The munition and event IDs' site and application numbers are sent
/// only when they differ from the firing entity's, as the standard requires,
/// and are filled in from it when they are not sent.
///
/// If `target_live_entity_id` is `None`, the standard requires `location`
/// (§9.4.5.6 g)
#[derive(Debug, Default, Clone, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct LiveEntityFirePdu {
    #[deku(skip)]
    header: LiveEntityPduHeader,
    pub firing_live_entity_id: LEEntityId,
    #[deku(update = "self.derived_flags()")]
    pub flags: LEFireFlags,
    #[deku(cond = "flags.contains(LEFireFlags::TARGET_ID)")]
    pub target_live_entity_id: Option<LEEntityId>,
    #[deku(
        reader = "read_compact_entity_id(deku::reader, flags.contains(LEFireFlags::MUNITION_ID), flags.contains(LEFireFlags::MUNITION_SITE_APPLICATION), *firing_live_entity_id)",
        writer = "write_compact_entity_id(deku::writer, *munition_live_entity_id, flags.contains(LEFireFlags::MUNITION_SITE_APPLICATION))"
    )]
    pub munition_live_entity_id: Option<LEEntityId>,
    #[deku(
        reader = "read_event_id(deku::reader, flags.contains(LEFireFlags::EVENT_SITE_APPLICATION), *firing_live_entity_id)",
        writer = "write_event_id(deku::writer, *event_id, flags.contains(LEFireFlags::EVENT_SITE_APPLICATION))"
    )]
    pub event_id: LiveEventIdentifier,
    #[deku(cond = "flags.contains(LEFireFlags::LOCATION)")]
    pub location: Option<RelativeWorldCoordinates>,
    #[deku(
        ctx = "flags.contains(LEFireFlags::WARHEAD_FUSE), flags.contains(LEFireFlags::QUANTITY_RATE)"
    )]
    pub munition_descriptor: LiveMunitionDescriptor,
    pub velocity: LELinearVelocity,
    pub range: u16,
}
impl_pdu!(
    LiveEntityFirePdu,
    LiveEntityPduHeader,
    PduType::LiveEntityFire,
    ProtocolFamily::LiveEntityInformationInteraction
);

impl LiveEntityFirePdu {
    /// The flags implied by which optional fields are present and whether
    /// the munition and event IDs' site and application differ from the
    /// firing entity's
    fn derived_flags(&self) -> LEFireFlags {
        let firing = self.firing_live_entity_id;
        let mut flags = LEFireFlags::empty();
        flags.set(LEFireFlags::TARGET_ID, self.target_live_entity_id.is_some());
        flags.set(
            LEFireFlags::MUNITION_ID,
            self.munition_live_entity_id.is_some(),
        );
        flags.set(
            LEFireFlags::MUNITION_SITE_APPLICATION,
            self.munition_live_entity_id.is_some_and(|id| {
                !same_site_application(id.site_number, id.application_number, firing)
            }),
        );
        flags.set(
            LEFireFlags::EVENT_SITE_APPLICATION,
            !same_site_application(
                self.event_id.site_number,
                self.event_id.application_number,
                firing,
            ),
        );
        flags.set(
            LEFireFlags::WARHEAD_FUSE,
            self.munition_descriptor.warhead_and_fuse.is_some(),
        );
        flags.set(
            LEFireFlags::QUANTITY_RATE,
            self.munition_descriptor.quantity_and_rate.is_some(),
        );
        flags.set(LEFireFlags::LOCATION, self.location.is_some());
        flags
    }
}

/// Whether a site and application are the firing entity's, so they are left
/// out of a munition or event ID
const fn same_site_application(site: u8, application: u8, firing: LEEntityId) -> bool {
    site == firing.site_number && application == firing.application_number
}

/// Reads a site and application number, or takes the firing entity's when
/// they aren't sent.
fn read_site_application<R: Read + Seek>(
    reader: &mut Reader<R>,
    included: bool,
    firing: LEEntityId,
) -> Result<(u8, u8), DekuError> {
    if included {
        Ok((
            u8::from_reader_with_ctx(reader, ())?,
            u8::from_reader_with_ctx(reader, ())?,
        ))
    } else {
        Ok((firing.site_number, firing.application_number))
    }
}

fn write_site_application<W: Write + Seek>(
    writer: &mut Writer<W>,
    included: bool,
    site: u8,
    application: u8,
) -> Result<(), DekuError> {
    if included {
        site.to_writer(writer, ())?;
        application.to_writer(writer, ())?;
    }
    Ok(())
}

/// A munition ID: absent, or an optional site and application number then
/// the entity number.
fn read_compact_entity_id<R: Read + Seek>(
    reader: &mut Reader<R>,
    present: bool,
    site_application_included: bool,
    firing: LEEntityId,
) -> Result<Option<LEEntityId>, DekuError> {
    if !present {
        return Ok(None);
    }
    let (site_number, application_number) =
        read_site_application(reader, site_application_included, firing)?;
    let entity_number = u16::from_reader_with_ctx(reader, deku::ctx::Endian::Big)?;
    Ok(Some(LEEntityId {
        site_number,
        application_number,
        entity_number,
    }))
}

fn write_compact_entity_id<W: Write + Seek>(
    writer: &mut Writer<W>,
    id: Option<LEEntityId>,
    site_application_included: bool,
) -> Result<(), DekuError> {
    let Some(id) = id else {
        return Ok(());
    };
    write_site_application(
        writer,
        site_application_included,
        id.site_number,
        id.application_number,
    )?;
    id.entity_number.to_writer(writer, deku::ctx::Endian::Big)
}

/// An event ID: an optional site and application number then the event
/// number.
fn read_event_id<R: Read + Seek>(
    reader: &mut Reader<R>,
    site_application_included: bool,
    firing: LEEntityId,
) -> Result<LiveEventIdentifier, DekuError> {
    let (site_number, application_number) =
        read_site_application(reader, site_application_included, firing)?;
    let event_number = u16::from_reader_with_ctx(reader, deku::ctx::Endian::Big)?;
    Ok(LiveEventIdentifier::new(
        site_number,
        application_number,
        event_number,
    ))
}

fn write_event_id<W: Write + Seek>(
    writer: &mut Writer<W>,
    id: LiveEventIdentifier,
    site_application_included: bool,
) -> Result<(), DekuError> {
    write_site_application(
        writer,
        site_application_included,
        id.site_number,
        id.application_number,
    )?;
    id.event_number.to_writer(writer, deku::ctx::Endian::Big)
}

bitflags! {
    /// LE Detonation Flag-1 (IEEE 1278.1-2012 §9.4.6.2, Table 227): which
    /// optional LE Detonation PDU fields are present. `serialize` sets it
    /// from the fields.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct LEDetonationFlags1: u8 {
        const TARGET_ID                 = 1 << 0;
        const MUNITION_SITE_APPLICATION = 1 << 1;
        const MUNITION_ID               = 1 << 2;
        const EVENT_SITE_APPLICATION    = 1 << 3;
        const WARHEAD_FUSE              = 1 << 4;
        const QUANTITY_RATE             = 1 << 5;
        /// Location in Entity's Coordinates is included and Location in
        /// Relative World Coordinates is not.
        const ENTITY_LOCATION           = 1 << 6;
        const FLAG_2                    = 1 << 7;
    }
}
impl_deku_bitflags!(LEDetonationFlags1, u8);

bitflags! {
    /// LE Detonation Flag-2 (IEEE 1278.1-2012 §9.4.6.2, Table 228). Bits it
    /// does not name are kept
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct LEDetonationFlags2: u8 {
        const MUNITION_ORIENTATION = 1 << 0;
        const EVENT_ID             = 1 << 1;
    }
}
impl_deku_bitflags!(LEDetonationFlags2, u8);

/// Implemented according to IEEE 1278.1-2012 §9.4.6.8
///
/// Set the optional fields you want to send; `serialize` sets the flags to
/// match. The munition and event IDs' site and application numbers are sent
/// only when they differ from the firing entity's, as the standard requires,
/// and are filled in from it when they are not sent.
///
/// Exactly one of `location_in_world_coordinates` and
/// `location_in_entity_coordinates` must be set, and the entity-coordinates
/// location needs a target (§9.4.6.8 k). `serialize` returns an error
/// otherwise. `new()` sets the world-coordinates location.
#[derive(Debug, Clone, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct LiveEntityDetonationPdu {
    #[deku(skip)]
    header: LiveEntityPduHeader,
    pub firing_live_entity_id: LEEntityId,
    #[deku(update = "self.derived_flags_1()?")]
    pub flags_1: LEDetonationFlags1,
    #[deku(
        cond = "flags_1.contains(LEDetonationFlags1::FLAG_2)",
        update = "self.derived_flags_2()"
    )]
    pub flags_2: Option<LEDetonationFlags2>,
    #[deku(cond = "flags_1.contains(LEDetonationFlags1::TARGET_ID)")]
    pub target_live_entity_id: Option<LEEntityId>,
    #[deku(
        reader = "read_compact_entity_id(deku::reader, flags_1.contains(LEDetonationFlags1::MUNITION_ID), flags_1.contains(LEDetonationFlags1::MUNITION_SITE_APPLICATION), *firing_live_entity_id)",
        writer = "write_compact_entity_id(deku::writer, *munition_live_entity_id, flags_1.contains(LEDetonationFlags1::MUNITION_SITE_APPLICATION))"
    )]
    pub munition_live_entity_id: Option<LEEntityId>,
    #[deku(
        reader = "read_optional_event_id(deku::reader, flags_2.is_some_and(|f| f.contains(LEDetonationFlags2::EVENT_ID)), flags_1.contains(LEDetonationFlags1::EVENT_SITE_APPLICATION), *firing_live_entity_id)",
        writer = "write_optional_event_id(deku::writer, *event_id, flags_1.contains(LEDetonationFlags1::EVENT_SITE_APPLICATION))"
    )]
    pub event_id: Option<LiveEventIdentifier>,
    #[deku(cond = "!flags_1.contains(LEDetonationFlags1::ENTITY_LOCATION)")]
    pub location_in_world_coordinates: Option<RelativeWorldCoordinates>,
    pub velocity: LELinearVelocity,
    #[deku(cond = "flags_2.is_some_and(|f| f.contains(LEDetonationFlags2::MUNITION_ORIENTATION))")]
    pub munition_orientation: Option<LiveEntityOrientation16>,
    #[deku(
        ctx = "flags_1.contains(LEDetonationFlags1::WARHEAD_FUSE), flags_1.contains(LEDetonationFlags1::QUANTITY_RATE)"
    )]
    pub munition_descriptor: LiveMunitionDescriptor,
    #[deku(cond = "flags_1.contains(LEDetonationFlags1::ENTITY_LOCATION)")]
    pub location_in_entity_coordinates: Option<LiveEntityRelativeLocation>,
    pub detonation_result: DetonationResult,
}
impl_pdu!(
    LiveEntityDetonationPdu,
    LiveEntityPduHeader,
    PduType::LiveEntityDetonation,
    ProtocolFamily::LiveEntityInformationInteraction
);

impl Default for LiveEntityDetonationPdu {
    fn default() -> Self {
        Self {
            header: LiveEntityPduHeader::default(),
            firing_live_entity_id: LEEntityId::default(),
            flags_1: LEDetonationFlags1::empty(),
            flags_2: None,
            target_live_entity_id: None,
            munition_live_entity_id: None,
            event_id: None,
            location_in_world_coordinates: Some(RelativeWorldCoordinates::default()),
            velocity: LELinearVelocity::default(),
            munition_orientation: None,
            munition_descriptor: LiveMunitionDescriptor::default(),
            location_in_entity_coordinates: None,
            detonation_result: DetonationResult::default(),
        }
    }
}

impl LiveEntityDetonationPdu {
    /// Flag-2 as implied by the munition orientation and event ID. A Flag-2
    /// that is already present is kept, with any bits it doesn't name.
    fn derived_flags_2(&self) -> Option<LEDetonationFlags2> {
        let orientation = self.munition_orientation.is_some();
        let event = self.event_id.is_some();
        if self.flags_2.is_none() && !orientation && !event {
            return None;
        }
        let mut flags = self.flags_2.unwrap_or_default();
        flags.set(LEDetonationFlags2::MUNITION_ORIENTATION, orientation);
        flags.set(LEDetonationFlags2::EVENT_ID, event);
        Some(flags)
    }

    /// Flag-1 as implied by the optional fields, after checking that exactly
    /// one location is set and the entity-coordinates one has a target.
    fn derived_flags_1(&self) -> Result<LEDetonationFlags1, DekuError> {
        let entity_location = match (
            self.location_in_world_coordinates.is_some(),
            self.location_in_entity_coordinates.is_some(),
        ) {
            (true, false) => false,
            (false, true) => true,
            _ => {
                return Err(DekuError::Assertion(
                        "an LE Detonation PDU needs exactly one of location_in_world_coordinates and location_in_entity_coordinates".into(),
                    ));
            }
        };
        if entity_location && self.target_live_entity_id.is_none() {
            return Err(DekuError::Assertion(
                    "an LE Detonation PDU's location_in_entity_coordinates needs a target_live_entity_id".into(),
                ));
        }
        let firing = self.firing_live_entity_id;
        let mut flags = LEDetonationFlags1::empty();
        flags.set(
            LEDetonationFlags1::TARGET_ID,
            self.target_live_entity_id.is_some(),
        );
        flags.set(
            LEDetonationFlags1::MUNITION_ID,
            self.munition_live_entity_id.is_some(),
        );
        flags.set(
            LEDetonationFlags1::MUNITION_SITE_APPLICATION,
            self.munition_live_entity_id.is_some_and(|id| {
                !same_site_application(id.site_number, id.application_number, firing)
            }),
        );
        flags.set(
            LEDetonationFlags1::EVENT_SITE_APPLICATION,
            self.event_id.is_some_and(|id| {
                !same_site_application(id.site_number, id.application_number, firing)
            }),
        );
        flags.set(
            LEDetonationFlags1::WARHEAD_FUSE,
            self.munition_descriptor.warhead_and_fuse.is_some(),
        );
        flags.set(
            LEDetonationFlags1::QUANTITY_RATE,
            self.munition_descriptor.quantity_and_rate.is_some(),
        );
        flags.set(LEDetonationFlags1::ENTITY_LOCATION, entity_location);
        flags.set(LEDetonationFlags1::FLAG_2, self.derived_flags_2().is_some());
        Ok(flags)
    }
}

fn read_optional_event_id<R: Read + Seek>(
    reader: &mut Reader<R>,
    present: bool,
    site_application_included: bool,
    firing: LEEntityId,
) -> Result<Option<LiveEventIdentifier>, DekuError> {
    if present {
        read_event_id(reader, site_application_included, firing).map(Some)
    } else {
        Ok(None)
    }
}

fn write_optional_event_id<W: Write + Seek>(
    writer: &mut Writer<W>,
    id: Option<LiveEventIdentifier>,
    site_application_included: bool,
) -> Result<(), DekuError> {
    id.map_or(Ok(()), |id| {
        write_event_id(writer, id, site_application_included)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{pdu::Pdu, pdu::pdu_tests};

    /// Serialize a PDU, failing the test if serialization returns an error.
    pub fn to_bytes<P: Pdu>(pdu: &mut P) -> Vec<u8> {
        pdu.to_bytes().expect("serialize failed")
    }

    pdu_tests!(
        time_space_position_information_pdu_tests, TimeSpacePositionInformationPdu,
        pdu_type: PduType::TimeSpacePositionInformation,
        default_bits: 200,
        wrong_type: AppearancePdu,
    );
    pdu_tests!(
        appearance_pdu_tests, AppearancePdu,
        pdu_type: PduType::Appearance,
        default_bits: 136,
        wrong_type: TimeSpacePositionInformationPdu,
    );

    /// Table 222: Flag-2 bit 0 marks Appearance-EM and bit 1 Appearance-Audio
    #[test]
    fn appearance_flag_2_bits_follow_table_222() {
        let mut pdu = AppearancePdu::new();
        pdu.appearance_em = Some(EntityAppearance::default());
        let em_only = to_bytes(&mut pdu);
        // LE header (12) + Live Entity ID (4) + Flag-1 (1), then Flag-2
        assert_eq!(em_only[16], 0b1000_0000);
        assert_eq!(em_only[17], 0b0000_0001);

        pdu.appearance_em = None;
        pdu.appearance_audio = Some(EntityAppearance::default());
        assert_eq!(to_bytes(&mut pdu)[17], 0b0000_0010);
    }

    pdu_tests!(
        articulated_parts_pdu_tests, ArticulatedPartsPdu,
        pdu_type: PduType::ArticulatedParts,
        default_bits: 136,
        wrong_type: AppearancePdu,
    );

    use crate::live_entity_records::{QuantityAndRate, WarheadAndFuse};

    fn le_id(site_number: u8, application_number: u8, entity_number: u16) -> LEEntityId {
        LEEntityId {
            site_number,
            application_number,
            entity_number,
        }
    }

    /// Table 226 with every flag set: 280 + 32F0 + (16F1 + 16)F2 + 16F3 +
    /// 64F6 + 32F4 + 32F5 = 488 bits, with no padding after the flag.
    #[test]
    fn le_fire_with_every_optional_field_follows_table_226() {
        let mut pdu = LiveEntityFirePdu::new();
        pdu.firing_live_entity_id = le_id(1, 2, 10);
        pdu.target_live_entity_id = Some(le_id(3, 4, 5));
        pdu.munition_live_entity_id = Some(le_id(7, 8, 9));
        pdu.event_id = LiveEventIdentifier::new(7, 8, 11);
        pdu.location = Some(RelativeWorldCoordinates::default());
        pdu.munition_descriptor.warhead_and_fuse = Some(WarheadAndFuse::default());
        pdu.munition_descriptor.quantity_and_rate = Some(QuantityAndRate {
            quantity: 6,
            rate: 600,
        });
        pdu.range = 0xBEEF;
        let bytes = to_bytes(&mut pdu);

        assert_eq!(bytes.len(), 488 / 8);
        assert_eq!(bytes[16], 0b0111_1111, "flags");
        assert_eq!(&bytes[17..21], &[3, 4, 0, 5], "target");
        assert_eq!(&bytes[21..25], &[7, 8, 0, 9], "munition ID");
        assert_eq!(&bytes[25..29], &[7, 8, 0, 11], "event ID");
        assert_eq!(&bytes[49..53], &[0, 6, 2, 88], "quantity and rate");
        assert_eq!(&bytes[59..61], &[0xBE, 0xEF], "range");

        let decoded = LiveEntityFirePdu::deserialize(&mut bytes.as_slice()).expect("deserialize");
        assert_eq!(decoded.munition_live_entity_id, Some(le_id(7, 8, 9)));
        assert_eq!(decoded.event_id, LiveEventIdentifier::new(7, 8, 11));
        assert_eq!(decoded.munition_descriptor, pdu.munition_descriptor);
    }

    /// §9.4.5.6 e) and f): site and application numbers equal to the firing
    /// entity's "shall not be included" and their flag bits are zero.
    #[test]
    fn le_fire_leaves_out_the_firing_entitys_site_and_application() {
        let mut pdu = LiveEntityFirePdu::new();
        pdu.firing_live_entity_id = le_id(1, 2, 10);
        pdu.munition_live_entity_id = Some(le_id(1, 2, 9));
        pdu.event_id = LiveEventIdentifier::new(1, 2, 11);
        pdu.location = Some(RelativeWorldCoordinates::default());
        let bytes = to_bytes(&mut pdu);

        // 280 + 16 (munition entity number) + 64 (location) bits
        assert_eq!(bytes.len(), 360 / 8);
        assert_eq!(bytes[16], 0b0100_0100, "flags");
        assert_eq!(&bytes[17..21], &[0, 9, 0, 11], "munition and event numbers");

        let decoded = LiveEntityFirePdu::deserialize(&mut bytes.as_slice()).expect("deserialize");
        assert_eq!(decoded.munition_live_entity_id, Some(le_id(1, 2, 9)));
        assert_eq!(decoded.event_id, LiveEventIdentifier::new(1, 2, 11));
    }

    /// Table 229: Flag-2 follows Flag-1 bit 7; the Event ID needs Flag-2 bit 1;
    /// Location in Entity's Coordinates replaces Location in Relative World
    /// Coordinates when Flag-1 bit 6 is set; no padding after the result.
    #[test]
    fn le_detonation_follows_table_229() {
        let mut pdu = LiveEntityDetonationPdu::new();
        pdu.firing_live_entity_id = le_id(1, 2, 10);
        pdu.target_live_entity_id = Some(le_id(3, 4, 5));
        pdu.event_id = Some(LiveEventIdentifier::new(5, 6, 11));
        pdu.location_in_world_coordinates = None;
        pdu.location_in_entity_coordinates = Some(LiveEntityRelativeLocation { x: 1, y: 2, z: 3 });
        pdu.munition_orientation = Some(LiveEntityOrientation16::new(4, 5, 6));
        pdu.detonation_result =
            num_traits::FromPrimitive::from_u8(5).expect("a defined detonation result");
        let bytes = to_bytes(&mut pdu);

        // 256 + 8F7 + 32F0 + (16F3 + 16)G1 + 48G0 + 48F6 bits.
        assert_eq!(bytes.len(), 424 / 8);
        assert_eq!(bytes[16], 0b1100_1001, "flag 1");
        assert_eq!(bytes[17], 0b0000_0011, "flag 2");
        assert_eq!(&bytes[18..22], &[3, 4, 0, 5], "target");
        assert_eq!(&bytes[22..26], &[5, 6, 0, 11], "event ID");
        assert_eq!(&bytes[32..38], &[0, 4, 0, 5, 0, 6], "munition orientation");
        assert_eq!(
            &bytes[46..52],
            &[0, 1, 0, 2, 0, 3],
            "location in entity coordinates"
        );
        assert_eq!(bytes[52], 5, "detonation result");

        let decoded =
            LiveEntityDetonationPdu::deserialize(&mut bytes.as_slice()).expect("deserialize");
        assert_eq!(decoded.event_id, Some(LiveEventIdentifier::new(5, 6, 11)));
        assert!(decoded.location_in_world_coordinates.is_none());
        assert_eq!(
            decoded.location_in_entity_coordinates,
            Some(LiveEntityRelativeLocation { x: 1, y: 2, z: 3 })
        );
    }

    #[test]
    fn le_detonation_needs_exactly_one_location() {
        let mut pdu = LiveEntityDetonationPdu::new();
        pdu.location_in_world_coordinates = None;
        assert!(pdu.to_bytes().is_err());

        pdu.location_in_world_coordinates = Some(RelativeWorldCoordinates::default());
        pdu.location_in_entity_coordinates = Some(LiveEntityRelativeLocation::default());
        assert!(pdu.to_bytes().is_err());

        // §9.4.6.8 k): no entity-coordinates location without a target.
        pdu.location_in_world_coordinates = None;
        assert!(pdu.to_bytes().is_err());
        pdu.target_live_entity_id = Some(LEEntityId::default());
        assert!(pdu.to_bytes().is_ok());
    }

    pdu_tests!(
        live_entity_fire_pdu_tests, LiveEntityFirePdu,
        pdu_type: PduType::LiveEntityFire,
        default_bits: 280,
        wrong_type: LiveEntityDetonationPdu,
    );
    pdu_tests!(
        live_entity_detonation_pdu_tests, LiveEntityDetonationPdu,
        pdu_type: PduType::LiveEntityDetonation,
        default_bits: 320,
        wrong_type: LiveEntityFirePdu,
    );
}
