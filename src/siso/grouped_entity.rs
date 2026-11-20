//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! SISO-REF-010-2023 Grouped Entity Description records (§20.4.3, UIDs 215–223), which
//! describe each entity in an [`IsGroupOfPdu`].
//!
//! The PDU's Grouped Entity Category (UID 213) picks the record type, and every entity in
//! the PDU uses the same one. [`GroupedEntityDescription`] holds any of them.
//!
//! The site and application numbers of each entity are the group's, so only the entity
//! number is sent. Angles are in 25 milliradian increments and rates in 25 milliradians
//! per second.

#[cfg(doc)]
use crate::entity_management::IsGroupOfPdu;
use crate::records::EntityAppearance;
use crate::siso::enums::{IsGroupOfGroupedEntityCategory, IsGroupOfRestStatus};

/// A grouped entity's location as offsets in meters from the [`IsGroupOfPdu`]'s Group
/// Reference Point
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct GroupedEntityLocation {
    pub x_offset: i16,
    pub y_offset: i16,
    pub z_offset: i16,
}

/// A grouped entity's orientation as Euler angles in 25 milliradian increments
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct GroupedEntityOrientation {
    pub psi: i8,
    pub theta: i8,
    pub phi: i8,
}

/// Basic Ground Combat Vehicle record (SISO-REF-010-2023 §20.4.3.1, UID 215): 20 octets
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct BasicGroundCombatVehicle {
    pub entity_number: u16,
    pub location: GroupedEntityLocation,
    pub appearance: EntityAppearance,
    pub orientation: GroupedEntityOrientation,
    /// Meters per second; negative when moving backwards
    pub speed: i8,
    pub turret_azimuth: i8,
    pub gun_elevation: i8,
    pub turret_slew_rate: i8,
    pub gun_elevation_rate: i8,
}

/// Enhanced Ground Combat Vehicle record (SISO-REF-010-2023 §20.4.3.2, UID 216): 24 octets
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EnhancedGroundCombatVehicle {
    pub basic: BasicGroundCombatVehicle,
    /// Fuel remaining in 10 liter increments
    pub fuel_status: u8,
    /// Distance traveled since the last failure, in tens of kilometers
    pub ground_maintenance_status: u8,
    pub primary_ammunition: u8,
    pub secondary_ammunition: u8,
}

/// Basic Ground Combat Soldier record (SISO-REF-010-2023 §20.4.3.3, UID 217): 20 octets
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct BasicGroundCombatSoldier {
    pub entity_number: u16,
    pub location: GroupedEntityLocation,
    pub appearance: EntityAppearance,
    pub orientation: GroupedEntityOrientation,
    /// Meters per second; negative when moving backwards
    pub speed: i8,
    pub head_azimuth: i8,
    pub head_elevation: i8,
    pub head_scan_rate: i8,
    pub head_elevation_rate: i8,
}

/// Enhanced Ground Combat Soldier record (SISO-REF-010-2023 §20.4.3.4, UID 218): 24 octets
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EnhancedGroundCombatSoldier {
    pub basic: BasicGroundCombatSoldier,
    /// Water on hand in one ounce increments
    pub water_status: u8,
    pub rest_status: IsGroupOfRestStatus,
    pub primary_ammunition: u8,
    pub secondary_ammunition: u8,
}

/// Basic Rotor Wing Aircraft record (SISO-REF-010-2023 §20.4.3.5, UID 219): 24 octets
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct BasicRotorWingAircraft {
    pub entity_number: u16,
    pub location: GroupedEntityLocation,
    pub appearance: EntityAppearance,
    pub orientation: GroupedEntityOrientation,
    /// Primary fuel remaining, in the system's natural units
    pub fuel_status: u8,
    /// Deviation of the movement direction from the orientation
    pub horizontal_deviation: i8,
    pub vertical_deviation: i8,
    /// 0.1 meters per second; negative when moving backwards
    pub movement_speed: i16,
    pub turret_azimuth: i8,
    pub gun_elevation: i8,
    pub turret_slew_rate: i8,
    pub gun_elevation_rate: i8,
}

/// Enhanced Rotor Wing Aircraft record (SISO-REF-010-2023 §20.4.3.6, UID 220): 28 octets
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EnhancedRotorWingAircraft {
    pub basic: BasicRotorWingAircraft,
    /// Supplemental fuel remaining, in the system's natural units
    pub supplemental_fuel_status: u8,
    /// Hours since the last maintenance was completed
    pub air_maintenance_status: u8,
    pub primary_ammunition: u8,
    pub secondary_ammunition: u8,
}

/// Basic Fixed Wing Aircraft record (SISO-REF-010-2023 §20.4.3.7, UID 221): 20 octets
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct BasicFixedWingAircraft {
    pub entity_number: u16,
    pub location: GroupedEntityLocation,
    pub appearance: EntityAppearance,
    pub orientation: GroupedEntityOrientation,
    /// Primary fuel remaining, in the system's natural units
    pub fuel_status: u8,
    /// Deviation of the movement direction from the orientation
    pub horizontal_deviation: i8,
    pub vertical_deviation: i8,
    /// 0.1 meters per second; negative when moving backwards
    pub movement_speed: i16,
}

/// Enhanced Fixed Wing Aircraft record (SISO-REF-010-2023 §20.4.3.8, UID 222): 24 octets
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EnhancedFixedWingAircraft {
    pub basic: BasicFixedWingAircraft,
    /// Supplemental fuel remaining, in the system's natural units
    pub supplemental_fuel_status: u8,
    /// Hours since the last maintenance was completed
    pub air_maintenance_status: u8,
    pub primary_ammunition: u8,
    pub secondary_ammunition: u8,
}

/// Ground Logistics Vehicle record (SISO-REF-010-2023 §20.4.3.9, UID 223): 16 octets
///
/// The speed is 8 bits, as the record's field list says; its format table lists 16.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct GroundLogisticsVehicle {
    pub entity_number: u16,
    pub location: GroupedEntityLocation,
    pub appearance: EntityAppearance,
    pub orientation: GroupedEntityOrientation,
    /// Meters per second; negative when moving backwards
    pub speed: i8,
}

/// Any Grouped Entity Description record (SISO-REF-010-2023 §20.4.3), one variant per
/// Grouped Entity Category (UID 213)
///
/// Category 0 (Undefined) has no record format, so an [`IsGroupOfPdu`] of that category can
/// only be empty.
#[derive(Copy, Clone, Debug, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian, category: IsGroupOfGroupedEntityCategory",
    id = "category"
)]
pub enum GroupedEntityDescription {
    #[deku(id = "IsGroupOfGroupedEntityCategory::BasicGroundCombatVehicle")]
    BasicGroundCombatVehicle(BasicGroundCombatVehicle),
    #[deku(id = "IsGroupOfGroupedEntityCategory::EnhancedGroundCombatVehicle")]
    EnhancedGroundCombatVehicle(EnhancedGroundCombatVehicle),
    #[deku(id = "IsGroupOfGroupedEntityCategory::BasicGroundCombatSoldier")]
    BasicGroundCombatSoldier(BasicGroundCombatSoldier),
    #[deku(id = "IsGroupOfGroupedEntityCategory::EnhancedGroundCombatSoldier")]
    EnhancedGroundCombatSoldier(EnhancedGroundCombatSoldier),
    #[deku(id = "IsGroupOfGroupedEntityCategory::BasicRotorWingAircraft")]
    BasicRotorWingAircraft(BasicRotorWingAircraft),
    #[deku(id = "IsGroupOfGroupedEntityCategory::EnhancedRotorWingAircraft")]
    EnhancedRotorWingAircraft(EnhancedRotorWingAircraft),
    #[deku(id = "IsGroupOfGroupedEntityCategory::BasicFixedWingAircraft")]
    BasicFixedWingAircraft(BasicFixedWingAircraft),
    #[deku(id = "IsGroupOfGroupedEntityCategory::EnhancedFixedWingAircraft")]
    EnhancedFixedWingAircraft(EnhancedFixedWingAircraft),
    #[deku(id = "IsGroupOfGroupedEntityCategory::GroundLogisticsVehicle")]
    GroundLogisticsVehicle(GroundLogisticsVehicle),
}

impl GroupedEntityDescription {
    /// The Grouped Entity Category this record belongs to
    #[must_use]
    pub const fn category(&self) -> IsGroupOfGroupedEntityCategory {
        match self {
            Self::BasicGroundCombatVehicle(_) => {
                IsGroupOfGroupedEntityCategory::BasicGroundCombatVehicle
            }
            Self::EnhancedGroundCombatVehicle(_) => {
                IsGroupOfGroupedEntityCategory::EnhancedGroundCombatVehicle
            }
            Self::BasicGroundCombatSoldier(_) => {
                IsGroupOfGroupedEntityCategory::BasicGroundCombatSoldier
            }
            Self::EnhancedGroundCombatSoldier(_) => {
                IsGroupOfGroupedEntityCategory::EnhancedGroundCombatSoldier
            }
            Self::BasicRotorWingAircraft(_) => {
                IsGroupOfGroupedEntityCategory::BasicRotorWingAircraft
            }
            Self::EnhancedRotorWingAircraft(_) => {
                IsGroupOfGroupedEntityCategory::EnhancedRotorWingAircraft
            }
            Self::BasicFixedWingAircraft(_) => {
                IsGroupOfGroupedEntityCategory::BasicFixedWingAircraft
            }
            Self::EnhancedFixedWingAircraft(_) => {
                IsGroupOfGroupedEntityCategory::EnhancedFixedWingAircraft
            }
            Self::GroundLogisticsVehicle(_) => {
                IsGroupOfGroupedEntityCategory::GroundLogisticsVehicle
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::wire::encoded_len;

    /// The sizes the SISO-REF-010-2023 §20.4.3 field lists add up to
    #[test]
    fn record_sizes_follow_the_field_lists() {
        assert_eq!(
            encoded_len(&BasicGroundCombatVehicle::default()).unwrap(),
            20
        );
        assert_eq!(
            encoded_len(&EnhancedGroundCombatVehicle::default()).unwrap(),
            24
        );
        assert_eq!(
            encoded_len(&BasicGroundCombatSoldier::default()).unwrap(),
            20
        );
        assert_eq!(
            encoded_len(&EnhancedGroundCombatSoldier::default()).unwrap(),
            24
        );
        assert_eq!(encoded_len(&BasicRotorWingAircraft::default()).unwrap(), 24);
        assert_eq!(
            encoded_len(&EnhancedRotorWingAircraft::default()).unwrap(),
            28
        );
        assert_eq!(encoded_len(&BasicFixedWingAircraft::default()).unwrap(), 20);
        assert_eq!(
            encoded_len(&EnhancedFixedWingAircraft::default()).unwrap(),
            24
        );
        assert_eq!(encoded_len(&GroundLogisticsVehicle::default()).unwrap(), 16);
    }
}
