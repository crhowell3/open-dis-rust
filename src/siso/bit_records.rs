//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use std::fmt;

use bitflags::bitflags;

use crate::siso::enums::{
    ComponentVisualDamageStatusSmoke, ComponentVisualDamageStatusSurfaceDamage, impl_deku_bitflags,
};

bitflags! {
    /// SISO-REF-010-2023 FrozenBehavior \[UID 68\]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct FrozenBehavior: u8 {
        const RunSimulationClock = 1 << 0;
        const TransmitUpdates = 1 << 1;
        const ProcessUpdates = 1 << 2;
    }
}

impl Default for FrozenBehavior {
    fn default() -> Self {
        Self::empty()
    }
}

impl_deku_bitflags!(FrozenBehavior, u8);

bitflags! {
    /// SISO-REF-010-2023 `ArealObjectStateModification` \[UID 242\]
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ArealObjectStateModification: u8 {
        /// Whether any of the object's locations have changed since the last update
        const IS_LOCATION_MODIFIED = 1 << 0;
    }
}

impl_deku_bitflags!(ArealObjectStateModification, u8);

/// SISO-REF-010-2023 `ComponentVisualDamageStatus` [UID 317]: What a damage site in a
/// [`DirectedEnergyDamage`](crate::common::records::DirectedEnergyDamage) record looks like
///
/// | Bits | Field |
/// | --- | --- |
/// | 0 | Is Fire Present |
/// | 1-2 | Smoke \[UID 375\] |
/// | 3-4 | Surface Damage \[UID 376\] |
///
/// Bit 0 is the least significant bit. Bits 5-7 are unused, and a decoded record keeps
/// them as received, so it encodes back to the same octet.
///
/// ```
/// use open_dis_rust::siso::enums::{ComponentVisualDamageStatusSmoke, ComponentVisualDamageStatusSurfaceDamage};
/// use open_dis_rust::siso::bit_records::ComponentVisualDamageStatus;
///
/// let mut status = ComponentVisualDamageStatus::new();
/// status.set_fire_present(true);
/// status.set_smoke(ComponentVisualDamageStatusSmoke::HeavySmoke);
/// status.set_surface_damage(ComponentVisualDamageStatusSurfaceDamage::LightCharring);
/// assert_eq!(status.to_raw(), 0b0000_1111);
/// ```
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct ComponentVisualDamageStatus {
    raw: u8,
}

impl ComponentVisualDamageStatus {
    const FIRE: u8 = 0b0000_0001;
    const SMOKE: u8 = 0b0000_0110;
    const SURFACE_DAMAGE: u8 = 0b0001_1000;

    /// A record with every field set to zero: no fire, no smoke, normal appearance
    #[must_use]
    pub const fn new() -> Self {
        Self { raw: 0 }
    }

    /// A view of the given octet
    #[must_use]
    pub const fn from_raw(raw: u8) -> Self {
        Self { raw }
    }

    /// The wire octet
    #[must_use]
    pub const fn to_raw(self) -> u8 {
        self.raw
    }

    const fn set_bits(&mut self, mask: u8, value: u8) {
        self.raw = (self.raw & !mask) | (value & mask);
    }

    /// Describes the presence of fire at the damage site (bit 0)
    #[must_use]
    pub const fn fire_present(self) -> bool {
        self.raw & Self::FIRE != 0
    }

    /// Sets bit 0. See [`fire_present`](Self::fire_present)
    pub const fn set_fire_present(&mut self, present: bool) {
        self.set_bits(Self::FIRE, if present { Self::FIRE } else { 0 });
    }

    /// Describes the presence of smoke emanating from the damage site (bits 1-2)
    #[must_use]
    pub const fn smoke(self) -> ComponentVisualDamageStatusSmoke {
        match (self.raw & Self::SMOKE) >> 1 {
            0 => ComponentVisualDamageStatusSmoke::NoSmoke,
            1 => ComponentVisualDamageStatusSmoke::LightSmoke,
            2 => ComponentVisualDamageStatusSmoke::ModerateSmoke,
            _ => ComponentVisualDamageStatusSmoke::HeavySmoke,
        }
    }

    /// Sets bits 1-2. See [`smoke`](Self::smoke)
    pub const fn set_smoke(&mut self, smoke: ComponentVisualDamageStatusSmoke) {
        let value: u8 = match smoke {
            ComponentVisualDamageStatusSmoke::NoSmoke => 0,
            ComponentVisualDamageStatusSmoke::LightSmoke => 1,
            ComponentVisualDamageStatusSmoke::ModerateSmoke => 2,
            ComponentVisualDamageStatusSmoke::HeavySmoke => 3,
        };
        self.set_bits(Self::SMOKE, value << 1);
    }

    /// Describes general surface appearance at the damage site (bits 3-4)
    #[must_use]
    pub const fn surface_damage(self) -> ComponentVisualDamageStatusSurfaceDamage {
        match (self.raw & Self::SURFACE_DAMAGE) >> 3 {
            0 => ComponentVisualDamageStatusSurfaceDamage::NormalAppearance,
            1 => ComponentVisualDamageStatusSurfaceDamage::LightCharring,
            2 => ComponentVisualDamageStatusSurfaceDamage::HeavyCharring,
            _ => ComponentVisualDamageStatusSurfaceDamage::OneorMoreHolesBurnedCompletelythroughSurface,
        }
    }

    /// Sets bits 3-4. See [`surface_damage`](Self::surface_damage)
    pub const fn set_surface_damage(&mut self, damage: ComponentVisualDamageStatusSurfaceDamage) {
        let value: u8 = match damage {
            ComponentVisualDamageStatusSurfaceDamage::NormalAppearance => 0,
            ComponentVisualDamageStatusSurfaceDamage::LightCharring => 1,
            ComponentVisualDamageStatusSurfaceDamage::HeavyCharring => 2,
            ComponentVisualDamageStatusSurfaceDamage::OneorMoreHolesBurnedCompletelythroughSurface => 3,
        };
        self.set_bits(Self::SURFACE_DAMAGE, value << 3);
    }
}

impl fmt::Debug for ComponentVisualDamageStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComponentVisualDamageStatus")
            .field("fire_present", &self.fire_present())
            .field("smoke", &self.smoke())
            .field("surface_damage", &self.surface_damage())
            .finish()
    }
}

impl From<u8> for ComponentVisualDamageStatus {
    fn from(raw: u8) -> Self {
        Self::from_raw(raw)
    }
}

impl From<ComponentVisualDamageStatus> for u8 {
    fn from(appearance: ComponentVisualDamageStatus) -> Self {
        appearance.raw
    }
}
