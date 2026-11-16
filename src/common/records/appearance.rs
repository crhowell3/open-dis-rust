//     open-dis-rust - Rust implementation of the IEEE-1278.1-2012 Distributed Interactive Simulation
//                     (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! Typed views of the [`EntityAppearance`] record, one for each layout in SISO-REF-010-2023
//! §17.11.1 (UIDs 31–43 and 480)
//!
//! Each view wraps the same 32 bits as the record it came from with a getter and setter
//! for each field. Bits that a layout does not use are kept as they are, so converting a view
//! back to an [`EntityAppearance`] gives the same bits. Bit 0 is the least significant bit.
//!
//! Getters for enumerations whose values do not fill their bits return an `Option`, which is
//! `None` for a value SISO-REF-010 does not define.

#![allow(
    clippy::as_conversions,
    reason = "fieldless enums whose values fit their fields, cast to u32"
)]

use std::fmt;

use num_traits::FromPrimitive;

use super::{EntityAppearance, EntityType};
use crate::common::enums::{
    AppearanceAntiCollisionDayNight, AppearanceCamouflageType, AppearanceCanopy,
    AppearanceConcealedMovement, AppearanceConcealedPosition, AppearanceDamage,
    AppearanceEntityorObjectState, AppearanceEnvironmentalDensity, AppearanceHatch,
    AppearanceLauncherOperational, AppearanceLifeFormComplianceStatus, AppearanceLifeFormPosture,
    AppearanceLifeFormWeaponImplement, AppearanceLifeformHealth, AppearanceNVGMode,
    AppearanceNavigationPositionBrightness, AppearancePaintScheme, AppearanceSubsurfaceHatch,
    AppearanceSupplyDeployed, AppearanceTrailingEffects, ClothingIRSignature, CoverShroudStatus,
    DamageArea, EntityKind, FlareSmokeColor, FlareSmokeStatus, Parachute, SpotChaffStatus,
};

/// The bits of `raw` from bit `lo`, `width` bits wide
const fn field(raw: u32, lo: u32, width: u32) -> u32 {
    (raw >> lo) & (1u32 << width).wrapping_sub(1)
}

/// `raw` with the `width` bits from bit `lo` replaced by `value`
const fn with_field(raw: u32, lo: u32, width: u32, value: u32) -> u32 {
    let mask = (1u32 << width).wrapping_sub(1) << lo;
    (raw & !mask) | ((value << lo) & mask)
}

/// Appearance of a platform in the land domain (SISO-REF-010 \[UID 31\])
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct LandPlatformAppearance {
    raw: u32,
}

impl LandPlatformAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The visual paint design (bit 0) \[UID 378\].
    #[must_use]
    pub fn paint_scheme(self) -> AppearancePaintScheme {
        AppearancePaintScheme::from_u32(field(self.raw, 0, 1)).unwrap_or_default()
    }

    /// Sets bit 0. See [`paint_scheme`](Self::paint_scheme).
    pub const fn set_paint_scheme(&mut self, value: AppearancePaintScheme) {
        self.raw = with_field(self.raw, 0, 1, value as u32);
    }

    /// Whether the entity can no longer move under its own power (bit 1).
    #[must_use]
    pub const fn mobility_killed(self) -> bool {
        field(self.raw, 1, 1) == 1
    }

    /// Sets bit 1. See [`mobility_killed`](Self::mobility_killed).
    pub const fn set_mobility_killed(&mut self, value: bool) {
        self.raw = with_field(self.raw, 1, 1, value as u32);
    }

    /// Whether the entity can no longer fire weapons (bit 2).
    #[must_use]
    pub const fn fire_power_killed(self) -> bool {
        field(self.raw, 2, 1) == 1
    }

    /// Sets bit 2. See [`fire_power_killed`](Self::fire_power_killed).
    pub const fn set_fire_power_killed(&mut self, value: bool) {
        self.raw = with_field(self.raw, 2, 1, value as u32);
    }

    /// The damaged appearance (bits 3–4) \[UID 379\].
    #[must_use]
    pub fn damage(self) -> AppearanceDamage {
        AppearanceDamage::from_u32(field(self.raw, 3, 2)).unwrap_or_default()
    }

    /// Sets bits 3–4. See [`damage`](Self::damage).
    pub const fn set_damage(&mut self, value: AppearanceDamage) {
        self.raw = with_field(self.raw, 3, 2, value as u32);
    }

    /// Whether smoke is emanating from the entity (bit 5).
    #[must_use]
    pub const fn is_smoke_emanating(self) -> bool {
        field(self.raw, 5, 1) == 1
    }

    /// Sets bit 5. See [`is_smoke_emanating`](Self::is_smoke_emanating).
    pub const fn set_is_smoke_emanating(&mut self, value: bool) {
        self.raw = with_field(self.raw, 5, 1, value as u32);
    }

    /// Whether the engine is emitting smoke (bit 6).
    #[must_use]
    pub const fn is_engine_emitting_smoke(self) -> bool {
        field(self.raw, 6, 1) == 1
    }

    /// Sets bit 6. See [`is_engine_emitting_smoke`](Self::is_engine_emitting_smoke).
    pub const fn set_is_engine_emitting_smoke(&mut self, value: bool) {
        self.raw = with_field(self.raw, 6, 1, value as u32);
    }

    /// The size of the dust cloud trailing the entity (bits 7–8) \[UID 381\].
    #[must_use]
    pub fn trailing_dust_cloud(self) -> AppearanceTrailingEffects {
        AppearanceTrailingEffects::from_u32(field(self.raw, 7, 2)).unwrap_or_default()
    }

    /// Sets bits 7–8. See [`trailing_dust_cloud`](Self::trailing_dust_cloud).
    pub const fn set_trailing_dust_cloud(&mut self, value: AppearanceTrailingEffects) {
        self.raw = with_field(self.raw, 7, 2, value as u32);
    }

    /// Whether the primary hatch is open or closed, and whether someone is visible (bits 9–11) \[UID 382\]. `None` if the bits hold a value that isn't defined.
    #[must_use]
    pub fn primary_hatch(self) -> Option<AppearanceHatch> {
        AppearanceHatch::from_u32(field(self.raw, 9, 3))
    }

    /// Sets bits 9–11. See [`primary_hatch`](Self::primary_hatch).
    pub const fn set_primary_hatch(&mut self, value: AppearanceHatch) {
        self.raw = with_field(self.raw, 9, 3, value as u32);
    }

    /// Whether the head lights are on (bit 12).
    #[must_use]
    pub const fn head_lights_on(self) -> bool {
        field(self.raw, 12, 1) == 1
    }

    /// Sets bit 12. See [`head_lights_on`](Self::head_lights_on).
    pub const fn set_head_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 12, 1, value as u32);
    }

    /// Whether the tail lights are on (bit 13).
    #[must_use]
    pub const fn tail_lights_on(self) -> bool {
        field(self.raw, 13, 1) == 1
    }

    /// Sets bit 13. See [`tail_lights_on`](Self::tail_lights_on).
    pub const fn set_tail_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 13, 1, value as u32);
    }

    /// Whether the brake lights are on (bit 14).
    #[must_use]
    pub const fn brake_lights_on(self) -> bool {
        field(self.raw, 14, 1) == 1
    }

    /// Sets bit 14. See [`brake_lights_on`](Self::brake_lights_on).
    pub const fn set_brake_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 14, 1, value as u32);
    }

    /// Whether the entity is burning and flames are visible (bit 15).
    #[must_use]
    pub const fn is_flaming(self) -> bool {
        field(self.raw, 15, 1) == 1
    }

    /// Sets bit 15. See [`is_flaming`](Self::is_flaming).
    pub const fn set_is_flaming(&mut self, value: bool) {
        self.raw = with_field(self.raw, 15, 1, value as u32);
    }

    /// The status of the mechanism the platform needs to be operational, such as its primary launcher being raised (bit 16) \[UID 383\].
    #[must_use]
    pub fn launcher_operational(self) -> AppearanceLauncherOperational {
        AppearanceLauncherOperational::from_u32(field(self.raw, 16, 1)).unwrap_or_default()
    }

    /// Sets bit 16. See [`launcher_operational`](Self::launcher_operational).
    pub const fn set_launcher_operational(&mut self, value: AppearanceLauncherOperational) {
        self.raw = with_field(self.raw, 16, 1, value as u32);
    }

    /// The camouflage color (bits 17–18) \[UID 384\].
    #[must_use]
    pub fn camouflage_type(self) -> AppearanceCamouflageType {
        AppearanceCamouflageType::from_u32(field(self.raw, 17, 2)).unwrap_or_default()
    }

    /// Sets bits 17–18. See [`camouflage_type`](Self::camouflage_type).
    pub const fn set_camouflage_type(&mut self, value: AppearanceCamouflageType) {
        self.raw = with_field(self.raw, 17, 2, value as u32);
    }

    /// The type of concealment (bit 19) \[UID 385\].
    #[must_use]
    pub fn concealed_position(self) -> AppearanceConcealedPosition {
        AppearanceConcealedPosition::from_u32(field(self.raw, 19, 1)).unwrap_or_default()
    }

    /// Sets bit 19. See [`concealed_position`](Self::concealed_position).
    pub const fn set_concealed_position(&mut self, value: AppearanceConcealedPosition) {
        self.raw = with_field(self.raw, 19, 1, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the power plant is on (bit 22).
    #[must_use]
    pub const fn power_plant_on(self) -> bool {
        field(self.raw, 22, 1) == 1
    }

    /// Sets bit 22. See [`power_plant_on`](Self::power_plant_on).
    pub const fn set_power_plant_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 22, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }

    /// Whether the tent is extended (bit 24).
    #[must_use]
    pub const fn tent_extended(self) -> bool {
        field(self.raw, 24, 1) == 1
    }

    /// Sets bit 24. See [`tent_extended`](Self::tent_extended).
    pub const fn set_tent_extended(&mut self, value: bool) {
        self.raw = with_field(self.raw, 24, 1, value as u32);
    }

    /// Whether the ramp is extended (bit 25).
    #[must_use]
    pub const fn ramp_extended(self) -> bool {
        field(self.raw, 25, 1) == 1
    }

    /// Sets bit 25. See [`ramp_extended`](Self::ramp_extended).
    pub const fn set_ramp_extended(&mut self, value: bool) {
        self.raw = with_field(self.raw, 25, 1, value as u32);
    }

    /// Whether the blackout lights are on (bit 26).
    #[must_use]
    pub const fn blackout_lights_on(self) -> bool {
        field(self.raw, 26, 1) == 1
    }

    /// Sets bit 26. See [`blackout_lights_on`](Self::blackout_lights_on).
    pub const fn set_blackout_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 26, 1, value as u32);
    }

    /// Whether the blackout brake lights are on (bit 27).
    #[must_use]
    pub const fn blackout_brake_lights_on(self) -> bool {
        field(self.raw, 27, 1) == 1
    }

    /// Sets bit 27. See [`blackout_brake_lights_on`](Self::blackout_brake_lights_on).
    pub const fn set_blackout_brake_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 27, 1, value as u32);
    }

    /// Whether spot/search light #1 is on. Without a record giving more detail, it covers all spot/search lights (bit 28).
    #[must_use]
    pub const fn spot_search_light_1_on(self) -> bool {
        field(self.raw, 28, 1) == 1
    }

    /// Sets bit 28. See [`spot_search_light_1_on`](Self::spot_search_light_1_on).
    pub const fn set_spot_search_light_1_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 28, 1, value as u32);
    }

    /// Whether the forward interior lights are on. Without a record giving more detail, it covers all interior lights (bit 29).
    #[must_use]
    pub const fn interior_lights_forward_on(self) -> bool {
        field(self.raw, 29, 1) == 1
    }

    /// Sets bit 29. See [`interior_lights_forward_on`](Self::interior_lights_forward_on).
    pub const fn set_interior_lights_forward_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 29, 1, value as u32);
    }

    /// Whether the vehicle's occupants have surrendered (bit 30).
    #[must_use]
    pub const fn occupants_surrendered(self) -> bool {
        field(self.raw, 30, 1) == 1
    }

    /// Sets bit 30. See [`occupants_surrendered`](Self::occupants_surrendered).
    pub const fn set_occupants_surrendered(&mut self, value: bool) {
        self.raw = with_field(self.raw, 30, 1, value as u32);
    }

    /// Whether the entity is masked or cloaked (bit 31).
    #[must_use]
    pub const fn masked_cloaked(self) -> bool {
        field(self.raw, 31, 1) == 1
    }

    /// Sets bit 31. See [`masked_cloaked`](Self::masked_cloaked).
    pub const fn set_masked_cloaked(&mut self, value: bool) {
        self.raw = with_field(self.raw, 31, 1, value as u32);
    }
}

impl fmt::Debug for LandPlatformAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LandPlatformAppearance")
            .field("paint_scheme", &self.paint_scheme())
            .field("mobility_killed", &self.mobility_killed())
            .field("fire_power_killed", &self.fire_power_killed())
            .field("damage", &self.damage())
            .field("is_smoke_emanating", &self.is_smoke_emanating())
            .field("is_engine_emitting_smoke", &self.is_engine_emitting_smoke())
            .field("trailing_dust_cloud", &self.trailing_dust_cloud())
            .field("primary_hatch", &self.primary_hatch())
            .field("head_lights_on", &self.head_lights_on())
            .field("tail_lights_on", &self.tail_lights_on())
            .field("brake_lights_on", &self.brake_lights_on())
            .field("is_flaming", &self.is_flaming())
            .field("launcher_operational", &self.launcher_operational())
            .field("camouflage_type", &self.camouflage_type())
            .field("concealed_position", &self.concealed_position())
            .field("is_frozen", &self.is_frozen())
            .field("power_plant_on", &self.power_plant_on())
            .field("state", &self.state())
            .field("tent_extended", &self.tent_extended())
            .field("ramp_extended", &self.ramp_extended())
            .field("blackout_lights_on", &self.blackout_lights_on())
            .field("blackout_brake_lights_on", &self.blackout_brake_lights_on())
            .field("spot_search_light_1_on", &self.spot_search_light_1_on())
            .field(
                "interior_lights_forward_on",
                &self.interior_lights_forward_on(),
            )
            .field("occupants_surrendered", &self.occupants_surrendered())
            .field("masked_cloaked", &self.masked_cloaked())
            .finish()
    }
}

impl From<EntityAppearance> for LandPlatformAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<LandPlatformAppearance> for EntityAppearance {
    fn from(view: LandPlatformAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of a platform in the air domain (SISO-REF-010 \[UID 32\])
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct AirPlatformAppearance {
    raw: u32,
}

impl AirPlatformAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The visual paint design (bit 0) \[UID 378\].
    #[must_use]
    pub fn paint_scheme(self) -> AppearancePaintScheme {
        AppearancePaintScheme::from_u32(field(self.raw, 0, 1)).unwrap_or_default()
    }

    /// Sets bit 0. See [`paint_scheme`](Self::paint_scheme).
    pub const fn set_paint_scheme(&mut self, value: AppearancePaintScheme) {
        self.raw = with_field(self.raw, 0, 1, value as u32);
    }

    /// Whether the entity can no longer move under its own power (bit 1).
    #[must_use]
    pub const fn propulsion_killed(self) -> bool {
        field(self.raw, 1, 1) == 1
    }

    /// Sets bit 1. See [`propulsion_killed`](Self::propulsion_killed).
    pub const fn set_propulsion_killed(&mut self, value: bool) {
        self.raw = with_field(self.raw, 1, 1, value as u32);
    }

    /// Whether the platform's lighting is in covert or overt mode (bit 2) \[UID 400\].
    #[must_use]
    pub fn nvg_mode(self) -> AppearanceNVGMode {
        AppearanceNVGMode::from_u32(field(self.raw, 2, 1)).unwrap_or_default()
    }

    /// Sets bit 2. See [`nvg_mode`](Self::nvg_mode).
    pub const fn set_nvg_mode(&mut self, value: AppearanceNVGMode) {
        self.raw = with_field(self.raw, 2, 1, value as u32);
    }

    /// The damaged appearance (bits 3–4) \[UID 379\].
    #[must_use]
    pub fn damage(self) -> AppearanceDamage {
        AppearanceDamage::from_u32(field(self.raw, 3, 2)).unwrap_or_default()
    }

    /// Sets bits 3–4. See [`damage`](Self::damage).
    pub const fn set_damage(&mut self, value: AppearanceDamage) {
        self.raw = with_field(self.raw, 3, 2, value as u32);
    }

    /// Whether smoke is emanating from the entity (bit 5).
    #[must_use]
    pub const fn is_smoke_emanating(self) -> bool {
        field(self.raw, 5, 1) == 1
    }

    /// Sets bit 5. See [`is_smoke_emanating`](Self::is_smoke_emanating).
    pub const fn set_is_smoke_emanating(&mut self, value: bool) {
        self.raw = with_field(self.raw, 5, 1, value as u32);
    }

    /// Whether the engine is emitting smoke (bit 6).
    #[must_use]
    pub const fn is_engine_emitting_smoke(self) -> bool {
        field(self.raw, 6, 1) == 1
    }

    /// Sets bit 6. See [`is_engine_emitting_smoke`](Self::is_engine_emitting_smoke).
    pub const fn set_is_engine_emitting_smoke(&mut self, value: bool) {
        self.raw = with_field(self.raw, 6, 1, value as u32);
    }

    /// The size of the contrails or ionization trailing the entity (bits 7–8) \[UID 381\].
    #[must_use]
    pub fn trailing_effects(self) -> AppearanceTrailingEffects {
        AppearanceTrailingEffects::from_u32(field(self.raw, 7, 2)).unwrap_or_default()
    }

    /// Sets bits 7–8. See [`trailing_effects`](Self::trailing_effects).
    pub const fn set_trailing_effects(&mut self, value: AppearanceTrailingEffects) {
        self.raw = with_field(self.raw, 7, 2, value as u32);
    }

    /// The state of the canopy or troop door (bits 9–11) \[UID 387\]. `None` if the bits hold a value that isn't defined.
    #[must_use]
    pub fn canopy_troop_door(self) -> Option<AppearanceCanopy> {
        AppearanceCanopy::from_u32(field(self.raw, 9, 3))
    }

    /// Sets bits 9–11. See [`canopy_troop_door`](Self::canopy_troop_door).
    pub const fn set_canopy_troop_door(&mut self, value: AppearanceCanopy) {
        self.raw = with_field(self.raw, 9, 3, value as u32);
    }

    /// Whether the landing lights are on (bit 12).
    #[must_use]
    pub const fn landing_lights_on(self) -> bool {
        field(self.raw, 12, 1) == 1
    }

    /// Sets bit 12. See [`landing_lights_on`](Self::landing_lights_on).
    pub const fn set_landing_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 12, 1, value as u32);
    }

    /// Whether the navigation lights are on (bit 13).
    #[must_use]
    pub const fn navigation_lights_on(self) -> bool {
        field(self.raw, 13, 1) == 1
    }

    /// Sets bit 13. See [`navigation_lights_on`](Self::navigation_lights_on).
    pub const fn set_navigation_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 13, 1, value as u32);
    }

    /// Whether the anti-collision lights are on (bit 14).
    #[must_use]
    pub const fn anti_collision_lights_on(self) -> bool {
        field(self.raw, 14, 1) == 1
    }

    /// Sets bit 14. See [`anti_collision_lights_on`](Self::anti_collision_lights_on).
    pub const fn set_anti_collision_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 14, 1, value as u32);
    }

    /// Whether the entity is burning and flames are visible (bit 15).
    #[must_use]
    pub const fn is_flaming(self) -> bool {
        field(self.raw, 15, 1) == 1
    }

    /// Sets bit 15. See [`is_flaming`](Self::is_flaming).
    pub const fn set_is_flaming(&mut self, value: bool) {
        self.raw = with_field(self.raw, 15, 1, value as u32);
    }

    /// Whether the afterburner is on (bit 16).
    #[must_use]
    pub const fn afterburner_on(self) -> bool {
        field(self.raw, 16, 1) == 1
    }

    /// Sets bit 16. See [`afterburner_on`](Self::afterburner_on).
    pub const fn set_afterburner_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 16, 1, value as u32);
    }

    /// Whether the lower anti-collision light is on (bit 17).
    #[must_use]
    pub const fn lower_anti_collision_light_on(self) -> bool {
        field(self.raw, 17, 1) == 1
    }

    /// Sets bit 17. See [`lower_anti_collision_light_on`](Self::lower_anti_collision_light_on).
    pub const fn set_lower_anti_collision_light_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 17, 1, value as u32);
    }

    /// Whether the upper anti-collision light is on (bit 18).
    #[must_use]
    pub const fn upper_anti_collision_light_on(self) -> bool {
        field(self.raw, 18, 1) == 1
    }

    /// Sets bit 18. See [`upper_anti_collision_light_on`](Self::upper_anti_collision_light_on).
    pub const fn set_upper_anti_collision_light_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 18, 1, value as u32);
    }

    /// The day/night status of the anti-collision lights. Day (0) if both the upper and lower anti-collision lights are off (bit 19) \[UID 397\].
    #[must_use]
    pub fn anti_collision_light_day_night(self) -> AppearanceAntiCollisionDayNight {
        AppearanceAntiCollisionDayNight::from_u32(field(self.raw, 19, 1)).unwrap_or_default()
    }

    /// Sets bit 19. See [`anti_collision_light_day_night`](Self::anti_collision_light_day_night).
    pub const fn set_anti_collision_light_day_night(
        &mut self,
        value: AppearanceAntiCollisionDayNight,
    ) {
        self.raw = with_field(self.raw, 19, 1, value as u32);
    }

    /// Whether any of the platform's lights are blinking (bit 20).
    #[must_use]
    pub const fn is_blinking(self) -> bool {
        field(self.raw, 20, 1) == 1
    }

    /// Sets bit 20. See [`is_blinking`](Self::is_blinking).
    pub const fn set_is_blinking(&mut self, value: bool) {
        self.raw = with_field(self.raw, 20, 1, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the power plant is on (bit 22).
    #[must_use]
    pub const fn power_plant_on(self) -> bool {
        field(self.raw, 22, 1) == 1
    }

    /// Sets bit 22. See [`power_plant_on`](Self::power_plant_on).
    pub const fn set_power_plant_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 22, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }

    /// Whether the formation lights are on (bit 24).
    #[must_use]
    pub const fn formation_lights_on(self) -> bool {
        field(self.raw, 24, 1) == 1
    }

    /// Sets bit 24. See [`formation_lights_on`](Self::formation_lights_on).
    pub const fn set_formation_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 24, 1, value as u32);
    }

    /// Whether the landing gear is extended, rather than wholly retracted (bit 25).
    #[must_use]
    pub const fn landing_gear_extended(self) -> bool {
        field(self.raw, 25, 1) == 1
    }

    /// Sets bit 25. See [`landing_gear_extended`](Self::landing_gear_extended).
    pub const fn set_landing_gear_extended(&mut self, value: bool) {
        self.raw = with_field(self.raw, 25, 1, value as u32);
    }

    /// Whether the cargo doors (main door) are open (bit 26).
    #[must_use]
    pub const fn cargo_doors_opened(self) -> bool {
        field(self.raw, 26, 1) == 1
    }

    /// Sets bit 26. See [`cargo_doors_opened`](Self::cargo_doors_opened).
    pub const fn set_cargo_doors_opened(&mut self, value: bool) {
        self.raw = with_field(self.raw, 26, 1, value as u32);
    }

    /// The brightness of the navigation/position lights (bit 27) \[UID 398\].
    #[must_use]
    pub fn navigation_position_brightness(self) -> AppearanceNavigationPositionBrightness {
        AppearanceNavigationPositionBrightness::from_u32(field(self.raw, 27, 1)).unwrap_or_default()
    }

    /// Sets bit 27. See [`navigation_position_brightness`](Self::navigation_position_brightness).
    pub const fn set_navigation_position_brightness(
        &mut self,
        value: AppearanceNavigationPositionBrightness,
    ) {
        self.raw = with_field(self.raw, 27, 1, value as u32);
    }

    /// Whether spot/search light #1 is on. Without a record giving more detail, it covers all spot/search lights (bit 28).
    #[must_use]
    pub const fn spot_search_light_1_on(self) -> bool {
        field(self.raw, 28, 1) == 1
    }

    /// Sets bit 28. See [`spot_search_light_1_on`](Self::spot_search_light_1_on).
    pub const fn set_spot_search_light_1_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 28, 1, value as u32);
    }

    /// Whether the interior lights are on (bit 29).
    #[must_use]
    pub const fn interior_lights_on(self) -> bool {
        field(self.raw, 29, 1) == 1
    }

    /// Sets bit 29. See [`interior_lights_on`](Self::interior_lights_on).
    pub const fn set_interior_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 29, 1, value as u32);
    }

    /// Whether reverse thrust is engaged (bit 30).
    #[must_use]
    pub const fn reverse_thrust_engaged(self) -> bool {
        field(self.raw, 30, 1) == 1
    }

    /// Sets bit 30. See [`reverse_thrust_engaged`](Self::reverse_thrust_engaged).
    pub const fn set_reverse_thrust_engaged(&mut self, value: bool) {
        self.raw = with_field(self.raw, 30, 1, value as u32);
    }

    /// Whether the platform has weight on its main landing gear (bit 31).
    #[must_use]
    pub const fn weight_on_wheels(self) -> bool {
        field(self.raw, 31, 1) == 1
    }

    /// Sets bit 31. See [`weight_on_wheels`](Self::weight_on_wheels).
    pub const fn set_weight_on_wheels(&mut self, value: bool) {
        self.raw = with_field(self.raw, 31, 1, value as u32);
    }
}

impl fmt::Debug for AirPlatformAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AirPlatformAppearance")
            .field("paint_scheme", &self.paint_scheme())
            .field("propulsion_killed", &self.propulsion_killed())
            .field("nvg_mode", &self.nvg_mode())
            .field("damage", &self.damage())
            .field("is_smoke_emanating", &self.is_smoke_emanating())
            .field("is_engine_emitting_smoke", &self.is_engine_emitting_smoke())
            .field("trailing_effects", &self.trailing_effects())
            .field("canopy_troop_door", &self.canopy_troop_door())
            .field("landing_lights_on", &self.landing_lights_on())
            .field("navigation_lights_on", &self.navigation_lights_on())
            .field("anti_collision_lights_on", &self.anti_collision_lights_on())
            .field("is_flaming", &self.is_flaming())
            .field("afterburner_on", &self.afterburner_on())
            .field(
                "lower_anti_collision_light_on",
                &self.lower_anti_collision_light_on(),
            )
            .field(
                "upper_anti_collision_light_on",
                &self.upper_anti_collision_light_on(),
            )
            .field(
                "anti_collision_light_day_night",
                &self.anti_collision_light_day_night(),
            )
            .field("is_blinking", &self.is_blinking())
            .field("is_frozen", &self.is_frozen())
            .field("power_plant_on", &self.power_plant_on())
            .field("state", &self.state())
            .field("formation_lights_on", &self.formation_lights_on())
            .field("landing_gear_extended", &self.landing_gear_extended())
            .field("cargo_doors_opened", &self.cargo_doors_opened())
            .field(
                "navigation_position_brightness",
                &self.navigation_position_brightness(),
            )
            .field("spot_search_light_1_on", &self.spot_search_light_1_on())
            .field("interior_lights_on", &self.interior_lights_on())
            .field("reverse_thrust_engaged", &self.reverse_thrust_engaged())
            .field("weight_on_wheels", &self.weight_on_wheels())
            .finish()
    }
}

impl From<EntityAppearance> for AirPlatformAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<AirPlatformAppearance> for EntityAppearance {
    fn from(view: AirPlatformAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of a platform in the surface domain (SISO-REF-010 \[UID 33\])
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct SurfacePlatformAppearance {
    raw: u32,
}

impl SurfacePlatformAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The visual paint design (bit 0) \[UID 378\].
    #[must_use]
    pub fn paint_scheme(self) -> AppearancePaintScheme {
        AppearancePaintScheme::from_u32(field(self.raw, 0, 1)).unwrap_or_default()
    }

    /// Sets bit 0. See [`paint_scheme`](Self::paint_scheme).
    pub const fn set_paint_scheme(&mut self, value: AppearancePaintScheme) {
        self.raw = with_field(self.raw, 0, 1, value as u32);
    }

    /// Whether the entity can no longer move under its own power (bit 1).
    #[must_use]
    pub const fn mobility_killed(self) -> bool {
        field(self.raw, 1, 1) == 1
    }

    /// Sets bit 1. See [`mobility_killed`](Self::mobility_killed).
    pub const fn set_mobility_killed(&mut self, value: bool) {
        self.raw = with_field(self.raw, 1, 1, value as u32);
    }

    /// The damaged appearance (bits 3–4) \[UID 379\].
    #[must_use]
    pub fn damage(self) -> AppearanceDamage {
        AppearanceDamage::from_u32(field(self.raw, 3, 2)).unwrap_or_default()
    }

    /// Sets bits 3–4. See [`damage`](Self::damage).
    pub const fn set_damage(&mut self, value: AppearanceDamage) {
        self.raw = with_field(self.raw, 3, 2, value as u32);
    }

    /// Whether smoke is emanating from the entity (bit 5).
    #[must_use]
    pub const fn is_smoke_emanating(self) -> bool {
        field(self.raw, 5, 1) == 1
    }

    /// Sets bit 5. See [`is_smoke_emanating`](Self::is_smoke_emanating).
    pub const fn set_is_smoke_emanating(&mut self, value: bool) {
        self.raw = with_field(self.raw, 5, 1, value as u32);
    }

    /// Whether the engine is emitting smoke (bit 6).
    #[must_use]
    pub const fn is_engine_emitting_smoke(self) -> bool {
        field(self.raw, 6, 1) == 1
    }

    /// Sets bit 6. See [`is_engine_emitting_smoke`](Self::is_engine_emitting_smoke).
    pub const fn set_is_engine_emitting_smoke(&mut self, value: bool) {
        self.raw = with_field(self.raw, 6, 1, value as u32);
    }

    /// The size of the wake trailing the entity (bits 7–8) \[UID 381\].
    #[must_use]
    pub fn wake_size(self) -> AppearanceTrailingEffects {
        AppearanceTrailingEffects::from_u32(field(self.raw, 7, 2)).unwrap_or_default()
    }

    /// Sets bits 7–8. See [`wake_size`](Self::wake_size).
    pub const fn set_wake_size(&mut self, value: AppearanceTrailingEffects) {
        self.raw = with_field(self.raw, 7, 2, value as u32);
    }

    /// Whether the running lights are on (bit 12).
    #[must_use]
    pub const fn running_lights_on(self) -> bool {
        field(self.raw, 12, 1) == 1
    }

    /// Sets bit 12. See [`running_lights_on`](Self::running_lights_on).
    pub const fn set_running_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 12, 1, value as u32);
    }

    /// Whether the entity is burning and flames are visible (bit 15).
    #[must_use]
    pub const fn is_flaming(self) -> bool {
        field(self.raw, 15, 1) == 1
    }

    /// Sets bit 15. See [`is_flaming`](Self::is_flaming).
    pub const fn set_is_flaming(&mut self, value: bool) {
        self.raw = with_field(self.raw, 15, 1, value as u32);
    }

    /// Whether the accommodation ladder is lowered (bit 16).
    #[must_use]
    pub const fn is_accommodation_ladder_lowered(self) -> bool {
        field(self.raw, 16, 1) == 1
    }

    /// Sets bit 16. See [`is_accommodation_ladder_lowered`](Self::is_accommodation_ladder_lowered).
    pub const fn set_is_accommodation_ladder_lowered(&mut self, value: bool) {
        self.raw = with_field(self.raw, 16, 1, value as u32);
    }

    /// Whether the safety fence around a helicopter landing deck is raised (bit 17).
    #[must_use]
    pub const fn is_fence_raised(self) -> bool {
        field(self.raw, 17, 1) == 1
    }

    /// Sets bit 17. See [`is_fence_raised`](Self::is_fence_raised).
    pub const fn set_is_fence_raised(&mut self, value: bool) {
        self.raw = with_field(self.raw, 17, 1, value as u32);
    }

    /// Whether the national identification flag is raised (bit 18).
    #[must_use]
    pub const fn is_flag_raised(self) -> bool {
        field(self.raw, 18, 1) == 1
    }

    /// Sets bit 18. See [`is_flag_raised`](Self::is_flag_raised).
    pub const fn set_is_flag_raised(&mut self, value: bool) {
        self.raw = with_field(self.raw, 18, 1, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the power plant is on (bit 22).
    #[must_use]
    pub const fn power_plant_on(self) -> bool {
        field(self.raw, 22, 1) == 1
    }

    /// Sets bit 22. See [`power_plant_on`](Self::power_plant_on).
    pub const fn set_power_plant_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 22, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }

    /// Whether the spot lights are on (bit 28).
    #[must_use]
    pub const fn spot_lights_on(self) -> bool {
        field(self.raw, 28, 1) == 1
    }

    /// Sets bit 28. See [`spot_lights_on`](Self::spot_lights_on).
    pub const fn set_spot_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 28, 1, value as u32);
    }

    /// Whether the interior lights are on (bit 29).
    #[must_use]
    pub const fn interior_lights_on(self) -> bool {
        field(self.raw, 29, 1) == 1
    }

    /// Sets bit 29. See [`interior_lights_on`](Self::interior_lights_on).
    pub const fn set_interior_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 29, 1, value as u32);
    }
}

impl fmt::Debug for SurfacePlatformAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SurfacePlatformAppearance")
            .field("paint_scheme", &self.paint_scheme())
            .field("mobility_killed", &self.mobility_killed())
            .field("damage", &self.damage())
            .field("is_smoke_emanating", &self.is_smoke_emanating())
            .field("is_engine_emitting_smoke", &self.is_engine_emitting_smoke())
            .field("wake_size", &self.wake_size())
            .field("running_lights_on", &self.running_lights_on())
            .field("is_flaming", &self.is_flaming())
            .field(
                "is_accommodation_ladder_lowered",
                &self.is_accommodation_ladder_lowered(),
            )
            .field("is_fence_raised", &self.is_fence_raised())
            .field("is_flag_raised", &self.is_flag_raised())
            .field("is_frozen", &self.is_frozen())
            .field("power_plant_on", &self.power_plant_on())
            .field("state", &self.state())
            .field("spot_lights_on", &self.spot_lights_on())
            .field("interior_lights_on", &self.interior_lights_on())
            .finish()
    }
}

impl From<EntityAppearance> for SurfacePlatformAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<SurfacePlatformAppearance> for EntityAppearance {
    fn from(view: SurfacePlatformAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of a platform in the subsurface domain (SISO-REF-010 \[UID 34\]).
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct SubsurfacePlatformAppearance {
    raw: u32,
}

impl SubsurfacePlatformAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The visual paint design (bit 0) \[UID 378\].
    #[must_use]
    pub fn paint_scheme(self) -> AppearancePaintScheme {
        AppearancePaintScheme::from_u32(field(self.raw, 0, 1)).unwrap_or_default()
    }

    /// Sets bit 0. See [`paint_scheme`](Self::paint_scheme).
    pub const fn set_paint_scheme(&mut self, value: AppearancePaintScheme) {
        self.raw = with_field(self.raw, 0, 1, value as u32);
    }

    /// Whether the entity can no longer move under its own power (bit 1).
    #[must_use]
    pub const fn mobility_killed(self) -> bool {
        field(self.raw, 1, 1) == 1
    }

    /// Sets bit 1. See [`mobility_killed`](Self::mobility_killed).
    pub const fn set_mobility_killed(&mut self, value: bool) {
        self.raw = with_field(self.raw, 1, 1, value as u32);
    }

    /// The damaged appearance (bits 3–4) \[UID 379\].
    #[must_use]
    pub fn damage(self) -> AppearanceDamage {
        AppearanceDamage::from_u32(field(self.raw, 3, 2)).unwrap_or_default()
    }

    /// Sets bits 3–4. See [`damage`](Self::damage).
    pub const fn set_damage(&mut self, value: AppearanceDamage) {
        self.raw = with_field(self.raw, 3, 2, value as u32);
    }

    /// Whether smoke is emanating from the entity (bit 5).
    #[must_use]
    pub const fn is_smoke_emanating(self) -> bool {
        field(self.raw, 5, 1) == 1
    }

    /// Sets bit 5. See [`is_smoke_emanating`](Self::is_smoke_emanating).
    pub const fn set_is_smoke_emanating(&mut self, value: bool) {
        self.raw = with_field(self.raw, 5, 1, value as u32);
    }

    /// Whether the engine is emitting smoke (bit 6).
    #[must_use]
    pub const fn is_engine_emitting_smoke(self) -> bool {
        field(self.raw, 6, 1) == 1
    }

    /// Sets bit 6. See [`is_engine_emitting_smoke`](Self::is_engine_emitting_smoke).
    pub const fn set_is_engine_emitting_smoke(&mut self, value: bool) {
        self.raw = with_field(self.raw, 6, 1, value as u32);
    }

    /// Whether the hatch is open or closed (bits 9–11) \[UID 388\]. `None` if the bits hold a value that isn't defined.
    #[must_use]
    pub fn hatch(self) -> Option<AppearanceSubsurfaceHatch> {
        AppearanceSubsurfaceHatch::from_u32(field(self.raw, 9, 3))
    }

    /// Sets bits 9–11. See [`hatch`](Self::hatch).
    pub const fn set_hatch(&mut self, value: AppearanceSubsurfaceHatch) {
        self.raw = with_field(self.raw, 9, 3, value as u32);
    }

    /// Whether the running lights are on (bit 12).
    #[must_use]
    pub const fn running_lights_on(self) -> bool {
        field(self.raw, 12, 1) == 1
    }

    /// Sets bit 12. See [`running_lights_on`](Self::running_lights_on).
    pub const fn set_running_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 12, 1, value as u32);
    }

    /// Whether the entity is burning and flames are visible (bit 15).
    #[must_use]
    pub const fn is_flaming(self) -> bool {
        field(self.raw, 15, 1) == 1
    }

    /// Sets bit 15. See [`is_flaming`](Self::is_flaming).
    pub const fn set_is_flaming(&mut self, value: bool) {
        self.raw = with_field(self.raw, 15, 1, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the power plant is on (bit 22).
    #[must_use]
    pub const fn power_plant_on(self) -> bool {
        field(self.raw, 22, 1) == 1
    }

    /// Sets bit 22. See [`power_plant_on`](Self::power_plant_on).
    pub const fn set_power_plant_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 22, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }
}

impl fmt::Debug for SubsurfacePlatformAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SubsurfacePlatformAppearance")
            .field("paint_scheme", &self.paint_scheme())
            .field("mobility_killed", &self.mobility_killed())
            .field("damage", &self.damage())
            .field("is_smoke_emanating", &self.is_smoke_emanating())
            .field("is_engine_emitting_smoke", &self.is_engine_emitting_smoke())
            .field("hatch", &self.hatch())
            .field("running_lights_on", &self.running_lights_on())
            .field("is_flaming", &self.is_flaming())
            .field("is_frozen", &self.is_frozen())
            .field("power_plant_on", &self.power_plant_on())
            .field("state", &self.state())
            .finish()
    }
}

impl From<EntityAppearance> for SubsurfacePlatformAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<SubsurfacePlatformAppearance> for EntityAppearance {
    fn from(view: SubsurfacePlatformAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of a platform in the space domain (SISO-REF-010 \[UID 35\]).
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct SpacePlatformAppearance {
    raw: u32,
}

impl SpacePlatformAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The visual paint design (bit 0) \[UID 378\].
    #[must_use]
    pub fn paint_scheme(self) -> AppearancePaintScheme {
        AppearancePaintScheme::from_u32(field(self.raw, 0, 1)).unwrap_or_default()
    }

    /// Sets bit 0. See [`paint_scheme`](Self::paint_scheme).
    pub const fn set_paint_scheme(&mut self, value: AppearancePaintScheme) {
        self.raw = with_field(self.raw, 0, 1, value as u32);
    }

    /// Whether the entity can no longer move under its own power (bit 1).
    #[must_use]
    pub const fn mobility_killed(self) -> bool {
        field(self.raw, 1, 1) == 1
    }

    /// Sets bit 1. See [`mobility_killed`](Self::mobility_killed).
    pub const fn set_mobility_killed(&mut self, value: bool) {
        self.raw = with_field(self.raw, 1, 1, value as u32);
    }

    /// The damaged appearance (bits 3–4) \[UID 379\].
    #[must_use]
    pub fn damage(self) -> AppearanceDamage {
        AppearanceDamage::from_u32(field(self.raw, 3, 2)).unwrap_or_default()
    }

    /// Sets bits 3–4. See [`damage`](Self::damage).
    pub const fn set_damage(&mut self, value: AppearanceDamage) {
        self.raw = with_field(self.raw, 3, 2, value as u32);
    }

    /// Whether smoke or vapor is emanating from the entity (bit 5).
    #[must_use]
    pub const fn is_smoke_vapor_emanating(self) -> bool {
        field(self.raw, 5, 1) == 1
    }

    /// Sets bit 5. See [`is_smoke_vapor_emanating`](Self::is_smoke_vapor_emanating).
    pub const fn set_is_smoke_vapor_emanating(&mut self, value: bool) {
        self.raw = with_field(self.raw, 5, 1, value as u32);
    }

    /// Whether the engine is emitting smoke (bit 6).
    #[must_use]
    pub const fn is_engine_emitting_smoke(self) -> bool {
        field(self.raw, 6, 1) == 1
    }

    /// Sets bit 6. See [`is_engine_emitting_smoke`](Self::is_engine_emitting_smoke).
    pub const fn set_is_engine_emitting_smoke(&mut self, value: bool) {
        self.raw = with_field(self.raw, 6, 1, value as u32);
    }

    /// Whether the entity is burning and flames are visible (bit 15).
    #[must_use]
    pub const fn is_flaming(self) -> bool {
        field(self.raw, 15, 1) == 1
    }

    /// Sets bit 15. See [`is_flaming`](Self::is_flaming).
    pub const fn set_is_flaming(&mut self, value: bool) {
        self.raw = with_field(self.raw, 15, 1, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the power plant is on (bit 22).
    #[must_use]
    pub const fn power_plant_on(self) -> bool {
        field(self.raw, 22, 1) == 1
    }

    /// Sets bit 22. See [`power_plant_on`](Self::power_plant_on).
    pub const fn set_power_plant_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 22, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }
}

impl fmt::Debug for SpacePlatformAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpacePlatformAppearance")
            .field("paint_scheme", &self.paint_scheme())
            .field("mobility_killed", &self.mobility_killed())
            .field("damage", &self.damage())
            .field("is_smoke_vapor_emanating", &self.is_smoke_vapor_emanating())
            .field("is_engine_emitting_smoke", &self.is_engine_emitting_smoke())
            .field("is_flaming", &self.is_flaming())
            .field("is_frozen", &self.is_frozen())
            .field("power_plant_on", &self.power_plant_on())
            .field("state", &self.state())
            .finish()
    }
}

impl From<EntityAppearance> for SpacePlatformAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<SpacePlatformAppearance> for EntityAppearance {
    fn from(view: SpacePlatformAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of a munition (SISO-REF-010 \[UID 36\]).
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct MunitionAppearance {
    raw: u32,
}

impl MunitionAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The damaged appearance (bits 3–4) \[UID 379\].
    #[must_use]
    pub fn damage(self) -> AppearanceDamage {
        AppearanceDamage::from_u32(field(self.raw, 3, 2)).unwrap_or_default()
    }

    /// Sets bits 3–4. See [`damage`](Self::damage).
    pub const fn set_damage(&mut self, value: AppearanceDamage) {
        self.raw = with_field(self.raw, 3, 2, value as u32);
    }

    /// Whether smoke or vapor is emanating from the entity (bit 5).
    #[must_use]
    pub const fn is_smoke_vapor_emanating(self) -> bool {
        field(self.raw, 5, 1) == 1
    }

    /// Sets bit 5. See [`is_smoke_vapor_emanating`](Self::is_smoke_vapor_emanating).
    pub const fn set_is_smoke_vapor_emanating(&mut self, value: bool) {
        self.raw = with_field(self.raw, 5, 1, value as u32);
    }

    /// Whether the engine is emitting smoke (bit 6).
    #[must_use]
    pub const fn is_engine_emitting_smoke(self) -> bool {
        field(self.raw, 6, 1) == 1
    }

    /// Sets bit 6. See [`is_engine_emitting_smoke`](Self::is_engine_emitting_smoke).
    pub const fn set_is_engine_emitting_smoke(&mut self, value: bool) {
        self.raw = with_field(self.raw, 6, 1, value as u32);
    }

    /// The size of the vapor trail (bits 7–8) \[UID 381\].
    #[must_use]
    pub fn vapor_trail_size(self) -> AppearanceTrailingEffects {
        AppearanceTrailingEffects::from_u32(field(self.raw, 7, 2)).unwrap_or_default()
    }

    /// Sets bits 7–8. See [`vapor_trail_size`](Self::vapor_trail_size).
    pub const fn set_vapor_trail_size(&mut self, value: AppearanceTrailingEffects) {
        self.raw = with_field(self.raw, 7, 2, value as u32);
    }

    /// Whether the entity is burning and flames are visible (bit 15).
    #[must_use]
    pub const fn is_flaming(self) -> bool {
        field(self.raw, 15, 1) == 1
    }

    /// Sets bit 15. See [`is_flaming`](Self::is_flaming).
    pub const fn set_is_flaming(&mut self, value: bool) {
        self.raw = with_field(self.raw, 15, 1, value as u32);
    }

    /// Whether the guided munition's launch flash is present (bit 16).
    #[must_use]
    pub const fn launch_flash_present(self) -> bool {
        field(self.raw, 16, 1) == 1
    }

    /// Sets bit 16. See [`launch_flash_present`](Self::launch_flash_present).
    pub const fn set_launch_flash_present(&mut self, value: bool) {
        self.raw = with_field(self.raw, 16, 1, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the power plant is on (bit 22).
    #[must_use]
    pub const fn power_plant_on(self) -> bool {
        field(self.raw, 22, 1) == 1
    }

    /// Sets bit 22. See [`power_plant_on`](Self::power_plant_on).
    pub const fn set_power_plant_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 22, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }

    /// The status of the cover or shroud (bits 24–25) \[UID 426\].
    #[must_use]
    pub fn cover_shroud_status(self) -> CoverShroudStatus {
        CoverShroudStatus::from_u32(field(self.raw, 24, 2)).unwrap_or_default()
    }

    /// Sets bits 24–25. See [`cover_shroud_status`](Self::cover_shroud_status).
    pub const fn set_cover_shroud_status(&mut self, value: CoverShroudStatus) {
        self.raw = with_field(self.raw, 24, 2, value as u32);
    }

    /// Whether the entity is masked or cloaked (bit 31).
    #[must_use]
    pub const fn masked_cloaked(self) -> bool {
        field(self.raw, 31, 1) == 1
    }

    /// Sets bit 31. See [`masked_cloaked`](Self::masked_cloaked).
    pub const fn set_masked_cloaked(&mut self, value: bool) {
        self.raw = with_field(self.raw, 31, 1, value as u32);
    }
}

impl fmt::Debug for MunitionAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MunitionAppearance")
            .field("damage", &self.damage())
            .field("is_smoke_vapor_emanating", &self.is_smoke_vapor_emanating())
            .field("is_engine_emitting_smoke", &self.is_engine_emitting_smoke())
            .field("vapor_trail_size", &self.vapor_trail_size())
            .field("is_flaming", &self.is_flaming())
            .field("launch_flash_present", &self.launch_flash_present())
            .field("is_frozen", &self.is_frozen())
            .field("power_plant_on", &self.power_plant_on())
            .field("state", &self.state())
            .field("cover_shroud_status", &self.cover_shroud_status())
            .field("masked_cloaked", &self.masked_cloaked())
            .finish()
    }
}

impl From<EntityAppearance> for MunitionAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<MunitionAppearance> for EntityAppearance {
    fn from(view: MunitionAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of a human life form (SISO-REF-010 \[UID 37\]).
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct HumanLifeFormAppearance {
    raw: u32,
}

impl HumanLifeFormAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The visual paint design (bit 0) \[UID 378\].
    #[must_use]
    pub fn paint_scheme(self) -> AppearancePaintScheme {
        AppearancePaintScheme::from_u32(field(self.raw, 0, 1)).unwrap_or_default()
    }

    /// Sets bit 0. See [`paint_scheme`](Self::paint_scheme).
    pub const fn set_paint_scheme(&mut self, value: AppearancePaintScheme) {
        self.raw = with_field(self.raw, 0, 1, value as u32);
    }

    /// The visible severity of any injury (bits 3–4) \[UID 390\].
    #[must_use]
    pub fn health(self) -> AppearanceLifeformHealth {
        AppearanceLifeformHealth::from_u32(field(self.raw, 3, 2)).unwrap_or_default()
    }

    /// Sets bits 3–4. See [`health`](Self::health).
    pub const fn set_health(&mut self, value: AppearanceLifeformHealth) {
        self.raw = with_field(self.raw, 3, 2, value as u32);
    }

    /// How, and how far, the life form is complying (bits 5–8) \[UID 391\].
    #[must_use]
    pub fn compliance_status(self) -> AppearanceLifeFormComplianceStatus {
        AppearanceLifeFormComplianceStatus::from_u32(field(self.raw, 5, 4)).unwrap_or_default()
    }

    /// Sets bits 5–8. See [`compliance_status`](Self::compliance_status).
    pub const fn set_compliance_status(&mut self, value: AppearanceLifeFormComplianceStatus) {
        self.raw = with_field(self.raw, 5, 4, value as u32);
    }

    /// The general nature of the IR signature from clothing or covering (bits 9–10) \[UID 802\].
    #[must_use]
    pub fn clothing_ir_signature(self) -> ClothingIRSignature {
        ClothingIRSignature::from_u32(field(self.raw, 9, 2)).unwrap_or_default()
    }

    /// Sets bits 9–10. See [`clothing_ir_signature`](Self::clothing_ir_signature).
    pub const fn set_clothing_ir_signature(&mut self, value: ClothingIRSignature) {
        self.raw = with_field(self.raw, 9, 2, value as u32);
    }

    /// Whether signal smoke is in use (bit 11).
    #[must_use]
    pub const fn signal_smoke_in_use(self) -> bool {
        field(self.raw, 11, 1) == 1
    }

    /// Sets bit 11. See [`signal_smoke_in_use`](Self::signal_smoke_in_use).
    pub const fn set_signal_smoke_in_use(&mut self, value: bool) {
        self.raw = with_field(self.raw, 11, 1, value as u32);
    }

    /// Whether flash lights are on (bit 12).
    #[must_use]
    pub const fn flash_lights_on(self) -> bool {
        field(self.raw, 12, 1) == 1
    }

    /// Sets bit 12. See [`flash_lights_on`](Self::flash_lights_on).
    pub const fn set_flash_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 12, 1, value as u32);
    }

    /// Whether a signal mirror is in use (bit 13).
    #[must_use]
    pub const fn signal_mirror_in_use(self) -> bool {
        field(self.raw, 13, 1) == 1
    }

    /// Sets bit 13. See [`signal_mirror_in_use`](Self::signal_mirror_in_use).
    pub const fn set_signal_mirror_in_use(&mut self, value: bool) {
        self.raw = with_field(self.raw, 13, 1, value as u32);
    }

    /// Whether an IR strobe is on (bit 14).
    #[must_use]
    pub const fn ir_strobe_on(self) -> bool {
        field(self.raw, 14, 1) == 1
    }

    /// Sets bit 14. See [`ir_strobe_on`](Self::ir_strobe_on).
    pub const fn set_ir_strobe_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 14, 1, value as u32);
    }

    /// Whether an IR illuminator (flare) is on (bit 15).
    #[must_use]
    pub const fn ir_illuminator_on(self) -> bool {
        field(self.raw, 15, 1) == 1
    }

    /// Sets bit 15. See [`ir_illuminator_on`](Self::ir_illuminator_on).
    pub const fn set_ir_illuminator_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 15, 1, value as u32);
    }

    /// The life form's posture (bits 16–19) \[UID 392\].
    #[must_use]
    pub fn life_form_posture(self) -> AppearanceLifeFormPosture {
        AppearanceLifeFormPosture::from_u32(field(self.raw, 16, 4)).unwrap_or_default()
    }

    /// Sets bits 16–19. See [`life_form_posture`](Self::life_form_posture).
    pub const fn set_life_form_posture(&mut self, value: AppearanceLifeFormPosture) {
        self.raw = with_field(self.raw, 16, 4, value as u32);
    }

    /// Whether the life form has a lit cigarette, cigar or similar item, mainly for its IR signature (bit 20).
    #[must_use]
    pub const fn is_smoking_cigarette(self) -> bool {
        field(self.raw, 20, 1) == 1
    }

    /// Sets bit 20. See [`is_smoking_cigarette`](Self::is_smoking_cigarette).
    pub const fn set_is_smoking_cigarette(&mut self, value: bool) {
        self.raw = with_field(self.raw, 20, 1, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the life form is mounted on, or hoisted by, another platform, such as a troop transport or helicopter hoist (bit 22).
    #[must_use]
    pub const fn mounted_hoisted(self) -> bool {
        field(self.raw, 22, 1) == 1
    }

    /// Sets bit 22. See [`mounted_hoisted`](Self::mounted_hoisted).
    pub const fn set_mounted_hoisted(&mut self, value: bool) {
        self.raw = with_field(self.raw, 22, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }

    /// The position of the primary weapon or implement (bits 24–25) \[UID 393\].
    #[must_use]
    pub fn weapon_implement_1(self) -> AppearanceLifeFormWeaponImplement {
        AppearanceLifeFormWeaponImplement::from_u32(field(self.raw, 24, 2)).unwrap_or_default()
    }

    /// Sets bits 24–25. See [`weapon_implement_1`](Self::weapon_implement_1).
    pub const fn set_weapon_implement_1(&mut self, value: AppearanceLifeFormWeaponImplement) {
        self.raw = with_field(self.raw, 24, 2, value as u32);
    }

    /// The position of the secondary weapon or implement (bits 26–27) \[UID 393\].
    #[must_use]
    pub fn weapon_implement_2(self) -> Option<AppearanceLifeFormWeaponImplement> {
        Some(
            AppearanceLifeFormWeaponImplement::from_u32(field(self.raw, 26, 2)).unwrap_or_default(),
        )
    }

    /// Sets bits 26–27. See [`weapon_implement_2`](Self::weapon_implement_2).
    pub const fn set_weapon_implement_2(&mut self, value: AppearanceLifeFormWeaponImplement) {
        self.raw = with_field(self.raw, 26, 2, value as u32);
    }

    /// The camouflage color (bits 28–29) \[UID 384\].
    #[must_use]
    pub fn camouflage_type(self) -> AppearanceCamouflageType {
        AppearanceCamouflageType::from_u32(field(self.raw, 28, 2)).unwrap_or_default()
    }

    /// Sets bits 28–29. See [`camouflage_type`](Self::camouflage_type).
    pub const fn set_camouflage_type(&mut self, value: AppearanceCamouflageType) {
        self.raw = with_field(self.raw, 28, 2, value as u32);
    }

    /// Whether the life form is in a prepared concealed position (bit 30) \[UID 385\].
    #[must_use]
    pub fn concealed_stationary(self) -> AppearanceConcealedPosition {
        AppearanceConcealedPosition::from_u32(field(self.raw, 30, 1)).unwrap_or_default()
    }

    /// Sets bit 30. See [`concealed_stationary`](Self::concealed_stationary).
    pub const fn set_concealed_stationary(&mut self, value: AppearanceConcealedPosition) {
        self.raw = with_field(self.raw, 30, 1, value as u32);
    }

    /// Whether the life form uses concealment while moving (bit 31) \[UID 394\].
    #[must_use]
    pub fn concealed_movement(self) -> AppearanceConcealedMovement {
        AppearanceConcealedMovement::from_u32(field(self.raw, 31, 1)).unwrap_or_default()
    }

    /// Sets bit 31. See [`concealed_movement`](Self::concealed_movement).
    pub const fn set_concealed_movement(&mut self, value: AppearanceConcealedMovement) {
        self.raw = with_field(self.raw, 31, 1, value as u32);
    }
}

impl fmt::Debug for HumanLifeFormAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HumanLifeFormAppearance")
            .field("paint_scheme", &self.paint_scheme())
            .field("health", &self.health())
            .field("compliance_status", &self.compliance_status())
            .field("clothing_ir_signature", &self.clothing_ir_signature())
            .field("signal_smoke_in_use", &self.signal_smoke_in_use())
            .field("flash_lights_on", &self.flash_lights_on())
            .field("signal_mirror_in_use", &self.signal_mirror_in_use())
            .field("ir_strobe_on", &self.ir_strobe_on())
            .field("ir_illuminator_on", &self.ir_illuminator_on())
            .field("life_form_posture", &self.life_form_posture())
            .field("is_smoking_cigarette", &self.is_smoking_cigarette())
            .field("is_frozen", &self.is_frozen())
            .field("mounted_hoisted", &self.mounted_hoisted())
            .field("state", &self.state())
            .field("weapon_implement_1", &self.weapon_implement_1())
            .field("weapon_implement_2", &self.weapon_implement_2())
            .field("camouflage_type", &self.camouflage_type())
            .field("concealed_stationary", &self.concealed_stationary())
            .field("concealed_movement", &self.concealed_movement())
            .finish()
    }
}

impl From<EntityAppearance> for HumanLifeFormAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<HumanLifeFormAppearance> for EntityAppearance {
    fn from(view: HumanLifeFormAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of a non-human life form (SISO-REF-010 \[UID 480\]).
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct NonHumanLifeFormAppearance {
    raw: u32,
}

impl NonHumanLifeFormAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The visible severity of any injury (bits 3–4) \[UID 390\].
    #[must_use]
    pub fn health(self) -> AppearanceLifeformHealth {
        AppearanceLifeformHealth::from_u32(field(self.raw, 3, 2)).unwrap_or_default()
    }

    /// Sets bits 3–4. See [`health`](Self::health).
    pub const fn set_health(&mut self, value: AppearanceLifeformHealth) {
        self.raw = with_field(self.raw, 3, 2, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the life form is mounted on, or hoisted by, another platform, such as a troop transport or helicopter hoist (bit 22).
    #[must_use]
    pub const fn mounted_hoisted(self) -> bool {
        field(self.raw, 22, 1) == 1
    }

    /// Sets bit 22. See [`mounted_hoisted`](Self::mounted_hoisted).
    pub const fn set_mounted_hoisted(&mut self, value: bool) {
        self.raw = with_field(self.raw, 22, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }
}

impl fmt::Debug for NonHumanLifeFormAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NonHumanLifeFormAppearance")
            .field("health", &self.health())
            .field("is_frozen", &self.is_frozen())
            .field("mounted_hoisted", &self.mounted_hoisted())
            .field("state", &self.state())
            .finish()
    }
}

impl From<EntityAppearance> for NonHumanLifeFormAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<NonHumanLifeFormAppearance> for EntityAppearance {
    fn from(view: NonHumanLifeFormAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of an environmental entity (SISO-REF-010 \[UID 38\]).
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct EnvironmentalAppearance {
    raw: u32,
}

impl EnvironmentalAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The density (bits 16–19) \[UID 395\]. `None` if the bits hold a value that isn't defined.
    #[must_use]
    pub fn density(self) -> Option<AppearanceEnvironmentalDensity> {
        AppearanceEnvironmentalDensity::from_u32(field(self.raw, 16, 4))
    }

    /// Sets bits 16–19. See [`density`](Self::density).
    pub const fn set_density(&mut self, value: AppearanceEnvironmentalDensity) {
        self.raw = with_field(self.raw, 16, 4, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }

    /// Whether the entity is masked or cloaked (bit 31).
    #[must_use]
    pub const fn masked_cloaked(self) -> bool {
        field(self.raw, 31, 1) == 1
    }

    /// Sets bit 31. See [`masked_cloaked`](Self::masked_cloaked).
    pub const fn set_masked_cloaked(&mut self, value: bool) {
        self.raw = with_field(self.raw, 31, 1, value as u32);
    }
}

impl fmt::Debug for EnvironmentalAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnvironmentalAppearance")
            .field("density", &self.density())
            .field("is_frozen", &self.is_frozen())
            .field("state", &self.state())
            .field("masked_cloaked", &self.masked_cloaked())
            .finish()
    }
}

impl From<EntityAppearance> for EnvironmentalAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<EnvironmentalAppearance> for EntityAppearance {
    fn from(view: EnvironmentalAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of a cultural feature (SISO-REF-010 \[UID 39\]).
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct CulturalFeatureAppearance {
    raw: u32,
}

impl CulturalFeatureAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The damaged area, when Damage is Slight (1) or Moderate (2) (bits 0–2) \[UID 889\].
    #[must_use]
    pub fn damage_area(self) -> DamageArea {
        DamageArea::from_u32(field(self.raw, 0, 3)).unwrap_or_default()
    }

    /// Sets bits 0–2. See [`damage_area`](Self::damage_area).
    pub const fn set_damage_area(&mut self, value: DamageArea) {
        self.raw = with_field(self.raw, 0, 3, value as u32);
    }

    /// The damaged appearance (bits 3–4) \[UID 379\].
    #[must_use]
    pub fn damage(self) -> AppearanceDamage {
        AppearanceDamage::from_u32(field(self.raw, 3, 2)).unwrap_or_default()
    }

    /// Sets bits 3–4. See [`damage`](Self::damage).
    pub const fn set_damage(&mut self, value: AppearanceDamage) {
        self.raw = with_field(self.raw, 3, 2, value as u32);
    }

    /// Whether smoke is emanating from the entity (bit 5).
    #[must_use]
    pub const fn is_smoke_emanating(self) -> bool {
        field(self.raw, 5, 1) == 1
    }

    /// Sets bit 5. See [`is_smoke_emanating`](Self::is_smoke_emanating).
    pub const fn set_is_smoke_emanating(&mut self, value: bool) {
        self.raw = with_field(self.raw, 5, 1, value as u32);
    }

    /// Whether the entity is burning and flames are visible (bit 15).
    #[must_use]
    pub const fn is_flaming(self) -> bool {
        field(self.raw, 15, 1) == 1
    }

    /// Sets bit 15. See [`is_flaming`](Self::is_flaming).
    pub const fn set_is_flaming(&mut self, value: bool) {
        self.raw = with_field(self.raw, 15, 1, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the internal heat is on (bit 22).
    #[must_use]
    pub const fn internal_heat_on(self) -> bool {
        field(self.raw, 22, 1) == 1
    }

    /// Sets bit 22. See [`internal_heat_on`](Self::internal_heat_on).
    pub const fn set_internal_heat_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 22, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }

    /// Whether the exterior lights are on (bit 28).
    #[must_use]
    pub const fn exterior_lights_on(self) -> bool {
        field(self.raw, 28, 1) == 1
    }

    /// Sets bit 28. See [`exterior_lights_on`](Self::exterior_lights_on).
    pub const fn set_exterior_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 28, 1, value as u32);
    }

    /// Whether the interior lights are on (bit 29).
    #[must_use]
    pub const fn interior_lights_on(self) -> bool {
        field(self.raw, 29, 1) == 1
    }

    /// Sets bit 29. See [`interior_lights_on`](Self::interior_lights_on).
    pub const fn set_interior_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 29, 1, value as u32);
    }

    /// Whether the entity is masked or cloaked (bit 31).
    #[must_use]
    pub const fn masked_cloaked(self) -> bool {
        field(self.raw, 31, 1) == 1
    }

    /// Sets bit 31. See [`masked_cloaked`](Self::masked_cloaked).
    pub const fn set_masked_cloaked(&mut self, value: bool) {
        self.raw = with_field(self.raw, 31, 1, value as u32);
    }
}

impl fmt::Debug for CulturalFeatureAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CulturalFeatureAppearance")
            .field("damage_area", &self.damage_area())
            .field("damage", &self.damage())
            .field("is_smoke_emanating", &self.is_smoke_emanating())
            .field("is_flaming", &self.is_flaming())
            .field("is_frozen", &self.is_frozen())
            .field("internal_heat_on", &self.internal_heat_on())
            .field("state", &self.state())
            .field("exterior_lights_on", &self.exterior_lights_on())
            .field("interior_lights_on", &self.interior_lights_on())
            .field("masked_cloaked", &self.masked_cloaked())
            .finish()
    }
}

impl From<EntityAppearance> for CulturalFeatureAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<CulturalFeatureAppearance> for EntityAppearance {
    fn from(view: CulturalFeatureAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of a supply (SISO-REF-010 \[UID 40\]).
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct SupplyAppearance {
    raw: u32,
}

impl SupplyAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The visual paint design (bit 0) \[UID 378\].
    #[must_use]
    pub fn paint_scheme(self) -> AppearancePaintScheme {
        AppearancePaintScheme::from_u32(field(self.raw, 0, 1)).unwrap_or_default()
    }

    /// Sets bit 0. See [`paint_scheme`](Self::paint_scheme).
    pub const fn set_paint_scheme(&mut self, value: AppearancePaintScheme) {
        self.raw = with_field(self.raw, 0, 1, value as u32);
    }

    /// The damaged appearance (bits 3–4) \[UID 379\].
    #[must_use]
    pub fn damage(self) -> AppearanceDamage {
        AppearanceDamage::from_u32(field(self.raw, 3, 2)).unwrap_or_default()
    }

    /// Sets bits 3–4. See [`damage`](Self::damage).
    pub const fn set_damage(&mut self, value: AppearanceDamage) {
        self.raw = with_field(self.raw, 3, 2, value as u32);
    }

    /// The status of the supply's parachute (bits 7–8) \[UID 401\].
    #[must_use]
    pub fn parachute_status(self) -> Parachute {
        Parachute::from_u32(field(self.raw, 7, 2)).unwrap_or_default()
    }

    /// Sets bits 7–8. See [`parachute_status`](Self::parachute_status).
    pub const fn set_parachute_status(&mut self, value: Parachute) {
        self.raw = with_field(self.raw, 7, 2, value as u32);
    }

    /// Whether the entity is burning and flames are visible (bit 15).
    #[must_use]
    pub const fn is_flaming(self) -> bool {
        field(self.raw, 15, 1) == 1
    }

    /// Sets bit 15. See [`is_flaming`](Self::is_flaming).
    pub const fn set_is_flaming(&mut self, value: bool) {
        self.raw = with_field(self.raw, 15, 1, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }

    /// The deployed status (bits 24–25) \[UID 399\].
    #[must_use]
    pub fn deployed_status(self) -> AppearanceSupplyDeployed {
        AppearanceSupplyDeployed::from_u32(field(self.raw, 24, 2)).unwrap_or_default()
    }

    /// Sets bits 24–25. See [`deployed_status`](Self::deployed_status).
    pub const fn set_deployed_status(&mut self, value: AppearanceSupplyDeployed) {
        self.raw = with_field(self.raw, 24, 2, value as u32);
    }

    /// Whether the entity is masked or cloaked (bit 31).
    #[must_use]
    pub const fn masked_cloaked(self) -> bool {
        field(self.raw, 31, 1) == 1
    }

    /// Sets bit 31. See [`masked_cloaked`](Self::masked_cloaked).
    pub const fn set_masked_cloaked(&mut self, value: bool) {
        self.raw = with_field(self.raw, 31, 1, value as u32);
    }
}

impl fmt::Debug for SupplyAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SupplyAppearance")
            .field("paint_scheme", &self.paint_scheme())
            .field("damage", &self.damage())
            .field("parachute_status", &self.parachute_status())
            .field("is_flaming", &self.is_flaming())
            .field("is_frozen", &self.is_frozen())
            .field("state", &self.state())
            .field("deployed_status", &self.deployed_status())
            .field("masked_cloaked", &self.masked_cloaked())
            .finish()
    }
}

impl From<EntityAppearance> for SupplyAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<SupplyAppearance> for EntityAppearance {
    fn from(view: SupplyAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of a radio (SISO-REF-010 \[UID 41\]).
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct RadioAppearance {
    raw: u32,
}

impl RadioAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }
}

impl fmt::Debug for RadioAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RadioAppearance")
            .field("is_frozen", &self.is_frozen())
            .field("state", &self.state())
            .finish()
    }
}

impl From<EntityAppearance> for RadioAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<RadioAppearance> for EntityAppearance {
    fn from(view: RadioAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of an expendable (SISO-REF-010 \[UID 42\]).
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct ExpendableAppearance {
    raw: u32,
}

impl ExpendableAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The damaged appearance (bits 3–4) \[UID 379\].
    #[must_use]
    pub fn damage(self) -> AppearanceDamage {
        AppearanceDamage::from_u32(field(self.raw, 3, 2)).unwrap_or_default()
    }

    /// Sets bits 3–4. See [`damage`](Self::damage).
    pub const fn set_damage(&mut self, value: AppearanceDamage) {
        self.raw = with_field(self.raw, 3, 2, value as u32);
    }

    /// Whether smoke is emanating from the entity (bit 5).
    #[must_use]
    pub const fn is_smoke_emanating(self) -> bool {
        field(self.raw, 5, 1) == 1
    }

    /// Sets bit 5. See [`is_smoke_emanating`](Self::is_smoke_emanating).
    pub const fn set_is_smoke_emanating(&mut self, value: bool) {
        self.raw = with_field(self.raw, 5, 1, value as u32);
    }

    /// The status of a flare's parachute (bits 7–8) \[UID 401\].
    #[must_use]
    pub fn parachute_status(self) -> Parachute {
        Parachute::from_u32(field(self.raw, 7, 2)).unwrap_or_default()
    }

    /// Sets bits 7–8. See [`parachute_status`](Self::parachute_status).
    pub const fn set_parachute_status(&mut self, value: Parachute) {
        self.raw = with_field(self.raw, 7, 2, value as u32);
    }

    /// The color of a flare's light, or of the smoke from a smoke expendable (bits 9–11) \[UID 402\]. `None` if the bits hold a value that isn't defined.
    #[must_use]
    pub fn flare_smoke_color(self) -> Option<FlareSmokeColor> {
        FlareSmokeColor::from_u32(field(self.raw, 9, 3))
    }

    /// Sets bits 9–11. See [`flare_smoke_color`](Self::flare_smoke_color).
    pub const fn set_flare_smoke_color(&mut self, value: FlareSmokeColor) {
        self.raw = with_field(self.raw, 9, 3, value as u32);
    }

    /// Whether the entity is burning and flames are visible (bit 15).
    #[must_use]
    pub const fn is_flaming(self) -> bool {
        field(self.raw, 15, 1) == 1
    }

    /// Sets bit 15. See [`is_flaming`](Self::is_flaming).
    pub const fn set_is_flaming(&mut self, value: bool) {
        self.raw = with_field(self.raw, 15, 1, value as u32);
    }

    /// Whether launch flash is present (bit 16).
    #[must_use]
    pub const fn launch_flash_present(self) -> bool {
        field(self.raw, 16, 1) == 1
    }

    /// Sets bit 16. See [`launch_flash_present`](Self::launch_flash_present).
    pub const fn set_launch_flash_present(&mut self, value: bool) {
        self.raw = with_field(self.raw, 16, 1, value as u32);
    }

    /// The status of a flare or smoke expendable (bits 17–18) \[UID 403\]. `None` if the bits hold a value that isn't defined.
    #[must_use]
    pub fn flare_smoke_status(self) -> Option<FlareSmokeStatus> {
        FlareSmokeStatus::from_u32(field(self.raw, 17, 2))
    }

    /// Sets bits 17–18. See [`flare_smoke_status`](Self::flare_smoke_status).
    pub const fn set_flare_smoke_status(&mut self, value: FlareSmokeStatus) {
        self.raw = with_field(self.raw, 17, 2, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the power plant is on (bit 22).
    #[must_use]
    pub const fn power_plant_on(self) -> bool {
        field(self.raw, 22, 1) == 1
    }

    /// Sets bit 22. See [`power_plant_on`](Self::power_plant_on).
    pub const fn set_power_plant_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 22, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }

    /// The status of spot chaff (bits 24–25) \[UID 404\]. `None` if the bits hold a value that isn't defined.
    #[must_use]
    pub fn spot_chaff_status(self) -> Option<SpotChaffStatus> {
        SpotChaffStatus::from_u32(field(self.raw, 24, 2))
    }

    /// Sets bits 24–25. See [`spot_chaff_status`](Self::spot_chaff_status).
    pub const fn set_spot_chaff_status(&mut self, value: SpotChaffStatus) {
        self.raw = with_field(self.raw, 24, 2, value as u32);
    }

    /// Whether the entity is masked or cloaked (bit 31).
    #[must_use]
    pub const fn masked_cloaked(self) -> bool {
        field(self.raw, 31, 1) == 1
    }

    /// Sets bit 31. See [`masked_cloaked`](Self::masked_cloaked).
    pub const fn set_masked_cloaked(&mut self, value: bool) {
        self.raw = with_field(self.raw, 31, 1, value as u32);
    }
}

impl fmt::Debug for ExpendableAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExpendableAppearance")
            .field("damage", &self.damage())
            .field("is_smoke_emanating", &self.is_smoke_emanating())
            .field("parachute_status", &self.parachute_status())
            .field("flare_smoke_color", &self.flare_smoke_color())
            .field("is_flaming", &self.is_flaming())
            .field("launch_flash_present", &self.launch_flash_present())
            .field("flare_smoke_status", &self.flare_smoke_status())
            .field("is_frozen", &self.is_frozen())
            .field("power_plant_on", &self.power_plant_on())
            .field("state", &self.state())
            .field("spot_chaff_status", &self.spot_chaff_status())
            .field("masked_cloaked", &self.masked_cloaked())
            .finish()
    }
}

impl From<EntityAppearance> for ExpendableAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<ExpendableAppearance> for EntityAppearance {
    fn from(view: ExpendableAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// Appearance of a sensor or emitter (SISO-REF-010 \[UID 43\]).
///
/// Convert an [`EntityAppearance`] with `From`, or use
/// [`EntityAppearance::view`] to pick the layout from the entity's type.
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct SensorEmitterAppearance {
    raw: u32,
}

impl SensorEmitterAppearance {
    /// A view of the given 32 bits.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The 32 bits, including any this layout doesn't use.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// The visual paint design (bit 0) \[UID 378\].
    #[must_use]
    pub fn paint_scheme(self) -> AppearancePaintScheme {
        AppearancePaintScheme::from_u32(field(self.raw, 0, 1)).unwrap_or_default()
    }

    /// Sets bit 0. See [`paint_scheme`](Self::paint_scheme).
    pub const fn set_paint_scheme(&mut self, value: AppearancePaintScheme) {
        self.raw = with_field(self.raw, 0, 1, value as u32);
    }

    /// Whether the entity can no longer move under its own power (bit 1).
    #[must_use]
    pub const fn mobility_killed(self) -> bool {
        field(self.raw, 1, 1) == 1
    }

    /// Sets bit 1. See [`mobility_killed`](Self::mobility_killed).
    pub const fn set_mobility_killed(&mut self, value: bool) {
        self.raw = with_field(self.raw, 1, 1, value as u32);
    }

    /// Whether the entity can no longer carry out its mission, for example because of a damaged antenna (bit 2).
    #[must_use]
    pub const fn mission_killed(self) -> bool {
        field(self.raw, 2, 1) == 1
    }

    /// Sets bit 2. See [`mission_killed`](Self::mission_killed).
    pub const fn set_mission_killed(&mut self, value: bool) {
        self.raw = with_field(self.raw, 2, 1, value as u32);
    }

    /// The damaged appearance (bits 3–4) \[UID 379\].
    #[must_use]
    pub fn damage(self) -> AppearanceDamage {
        AppearanceDamage::from_u32(field(self.raw, 3, 2)).unwrap_or_default()
    }

    /// Sets bits 3–4. See [`damage`](Self::damage).
    pub const fn set_damage(&mut self, value: AppearanceDamage) {
        self.raw = with_field(self.raw, 3, 2, value as u32);
    }

    /// Whether smoke is emanating from the entity (bit 5).
    #[must_use]
    pub const fn is_smoke_emanating(self) -> bool {
        field(self.raw, 5, 1) == 1
    }

    /// Sets bit 5. See [`is_smoke_emanating`](Self::is_smoke_emanating).
    pub const fn set_is_smoke_emanating(&mut self, value: bool) {
        self.raw = with_field(self.raw, 5, 1, value as u32);
    }

    /// Whether the engine is emitting smoke (bit 6).
    #[must_use]
    pub const fn is_engine_emitting_smoke(self) -> bool {
        field(self.raw, 6, 1) == 1
    }

    /// Sets bit 6. See [`is_engine_emitting_smoke`](Self::is_engine_emitting_smoke).
    pub const fn set_is_engine_emitting_smoke(&mut self, value: bool) {
        self.raw = with_field(self.raw, 6, 1, value as u32);
    }

    /// The size of the trailing effects (bits 7–8) \[UID 381\].
    #[must_use]
    pub fn trailing_effects(self) -> AppearanceTrailingEffects {
        AppearanceTrailingEffects::from_u32(field(self.raw, 7, 2)).unwrap_or_default()
    }

    /// Sets bits 7–8. See [`trailing_effects`](Self::trailing_effects).
    pub const fn set_trailing_effects(&mut self, value: AppearanceTrailingEffects) {
        self.raw = with_field(self.raw, 7, 2, value as u32);
    }

    /// Whether the lights are on (bit 12).
    #[must_use]
    pub const fn lights_on(self) -> bool {
        field(self.raw, 12, 1) == 1
    }

    /// Sets bit 12. See [`lights_on`](Self::lights_on).
    pub const fn set_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 12, 1, value as u32);
    }

    /// Whether the entity is burning and flames are visible (bit 15).
    #[must_use]
    pub const fn is_flaming(self) -> bool {
        field(self.raw, 15, 1) == 1
    }

    /// Sets bit 15. See [`is_flaming`](Self::is_flaming).
    pub const fn set_is_flaming(&mut self, value: bool) {
        self.raw = with_field(self.raw, 15, 1, value as u32);
    }

    /// Whether the antenna is raised (bit 16).
    #[must_use]
    pub const fn antenna_raised(self) -> bool {
        field(self.raw, 16, 1) == 1
    }

    /// Sets bit 16. See [`antenna_raised`](Self::antenna_raised).
    pub const fn set_antenna_raised(&mut self, value: bool) {
        self.raw = with_field(self.raw, 16, 1, value as u32);
    }

    /// The camouflage color (bits 17–18) \[UID 384\].
    #[must_use]
    pub fn camouflage_type(self) -> AppearanceCamouflageType {
        AppearanceCamouflageType::from_u32(field(self.raw, 17, 2)).unwrap_or_default()
    }

    /// Sets bits 17–18. See [`camouflage_type`](Self::camouflage_type).
    pub const fn set_camouflage_type(&mut self, value: AppearanceCamouflageType) {
        self.raw = with_field(self.raw, 17, 2, value as u32);
    }

    /// The type of concealment (bit 19) \[UID 385\].
    #[must_use]
    pub fn concealed_position(self) -> AppearanceConcealedPosition {
        AppearanceConcealedPosition::from_u32(field(self.raw, 19, 1)).unwrap_or_default()
    }

    /// Sets bit 19. See [`concealed_position`](Self::concealed_position).
    pub const fn set_concealed_position(&mut self, value: AppearanceConcealedPosition) {
        self.raw = with_field(self.raw, 19, 1, value as u32);
    }

    /// Whether the entity is frozen and shouldn't be dead reckoned (bit 21).
    #[must_use]
    pub const fn is_frozen(self) -> bool {
        field(self.raw, 21, 1) == 1
    }

    /// Sets bit 21. See [`is_frozen`](Self::is_frozen).
    pub const fn set_is_frozen(&mut self, value: bool) {
        self.raw = with_field(self.raw, 21, 1, value as u32);
    }

    /// Whether the power plant is on (bit 22).
    #[must_use]
    pub const fn power_plant_on(self) -> bool {
        field(self.raw, 22, 1) == 1
    }

    /// Sets bit 22. See [`power_plant_on`](Self::power_plant_on).
    pub const fn set_power_plant_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 22, 1, value as u32);
    }

    /// Whether the entity is active or deactivated (bit 23) \[UID 386\].
    #[must_use]
    pub fn state(self) -> AppearanceEntityorObjectState {
        AppearanceEntityorObjectState::from_u32(field(self.raw, 23, 1)).unwrap_or_default()
    }

    /// Sets bit 23. See [`state`](Self::state).
    pub const fn set_state(&mut self, value: AppearanceEntityorObjectState) {
        self.raw = with_field(self.raw, 23, 1, value as u32);
    }

    /// Whether the tent is extended (bit 24).
    #[must_use]
    pub const fn tent_extended(self) -> bool {
        field(self.raw, 24, 1) == 1
    }

    /// Sets bit 24. See [`tent_extended`](Self::tent_extended).
    pub const fn set_tent_extended(&mut self, value: bool) {
        self.raw = with_field(self.raw, 24, 1, value as u32);
    }

    /// Whether the blackout lights are on (bit 26).
    #[must_use]
    pub const fn blackout_lights_on(self) -> bool {
        field(self.raw, 26, 1) == 1
    }

    /// Sets bit 26. See [`blackout_lights_on`](Self::blackout_lights_on).
    pub const fn set_blackout_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 26, 1, value as u32);
    }

    /// Whether the interior lights are on (bit 29).
    #[must_use]
    pub const fn interior_lights_on(self) -> bool {
        field(self.raw, 29, 1) == 1
    }

    /// Sets bit 29. See [`interior_lights_on`](Self::interior_lights_on).
    pub const fn set_interior_lights_on(&mut self, value: bool) {
        self.raw = with_field(self.raw, 29, 1, value as u32);
    }
}

impl fmt::Debug for SensorEmitterAppearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SensorEmitterAppearance")
            .field("paint_scheme", &self.paint_scheme())
            .field("mobility_killed", &self.mobility_killed())
            .field("mission_killed", &self.mission_killed())
            .field("damage", &self.damage())
            .field("is_smoke_emanating", &self.is_smoke_emanating())
            .field("is_engine_emitting_smoke", &self.is_engine_emitting_smoke())
            .field("trailing_effects", &self.trailing_effects())
            .field("lights_on", &self.lights_on())
            .field("is_flaming", &self.is_flaming())
            .field("antenna_raised", &self.antenna_raised())
            .field("camouflage_type", &self.camouflage_type())
            .field("concealed_position", &self.concealed_position())
            .field("is_frozen", &self.is_frozen())
            .field("power_plant_on", &self.power_plant_on())
            .field("state", &self.state())
            .field("tent_extended", &self.tent_extended())
            .field("blackout_lights_on", &self.blackout_lights_on())
            .field("interior_lights_on", &self.interior_lights_on())
            .finish()
    }
}

impl From<EntityAppearance> for SensorEmitterAppearance {
    fn from(appearance: EntityAppearance) -> Self {
        Self::from_raw(appearance.to_raw())
    }
}

impl From<SensorEmitterAppearance> for EntityAppearance {
    fn from(view: SensorEmitterAppearance) -> Self {
        Self::from_raw(view.raw)
    }
}

/// An [`EntityAppearance`] read with the layout for its entity's type. Returned by
/// [`EntityAppearance::view`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AppearanceView {
    /// [`LandPlatformAppearance`]
    LandPlatform(LandPlatformAppearance),
    /// [`AirPlatformAppearance`]
    AirPlatform(AirPlatformAppearance),
    /// [`SurfacePlatformAppearance`]
    SurfacePlatform(SurfacePlatformAppearance),
    /// [`SubsurfacePlatformAppearance`]
    SubsurfacePlatform(SubsurfacePlatformAppearance),
    /// [`SpacePlatformAppearance`]
    SpacePlatform(SpacePlatformAppearance),
    /// [`MunitionAppearance`]
    Munition(MunitionAppearance),
    /// [`HumanLifeFormAppearance`]
    HumanLifeForm(HumanLifeFormAppearance),
    /// [`NonHumanLifeFormAppearance`]
    NonHumanLifeForm(NonHumanLifeFormAppearance),
    /// [`EnvironmentalAppearance`]
    Environmental(EnvironmentalAppearance),
    /// [`CulturalFeatureAppearance`]
    CulturalFeature(CulturalFeatureAppearance),
    /// [`SupplyAppearance`]
    Supply(SupplyAppearance),
    /// [`RadioAppearance`]
    Radio(RadioAppearance),
    /// [`ExpendableAppearance`]
    Expendable(ExpendableAppearance),
    /// [`SensorEmitterAppearance`]
    SensorEmitter(SensorEmitterAppearance),
}

impl From<AppearanceView> for EntityAppearance {
    fn from(view: AppearanceView) -> Self {
        match view {
            AppearanceView::LandPlatform(view) => view.into(),
            AppearanceView::AirPlatform(view) => view.into(),
            AppearanceView::SurfacePlatform(view) => view.into(),
            AppearanceView::SubsurfacePlatform(view) => view.into(),
            AppearanceView::SpacePlatform(view) => view.into(),
            AppearanceView::Munition(view) => view.into(),
            AppearanceView::HumanLifeForm(view) => view.into(),
            AppearanceView::NonHumanLifeForm(view) => view.into(),
            AppearanceView::Environmental(view) => view.into(),
            AppearanceView::CulturalFeature(view) => view.into(),
            AppearanceView::Supply(view) => view.into(),
            AppearanceView::Radio(view) => view.into(),
            AppearanceView::Expendable(view) => view.into(),
            AppearanceView::SensorEmitter(view) => view.into(),
        }
    }
}

/// Category values from 200 up are non-human life forms (SISO-REF-010-2023 UIDs 472–479).
const FIRST_NON_HUMAN_LIFE_FORM_CATEGORY: u8 = 200;

impl EntityAppearance {
    /// The appearance read with the layout for `entity_type`: by kind, and for platforms
    /// by domain (Land 1, Air 2, Surface 3, Subsurface 4, Space 5). Life forms with a
    /// category of 200 or more are non-human.
    ///
    /// `None` for an entity of kind Other, or a platform in a domain with no layout.
    ///
    /// ```
    /// use open_dis_rust::common::{
    ///     enums::{AppearanceDamage, EntityKind},
    ///     records::{AppearanceView, EntityAppearance, EntityType},
    /// };
    ///
    /// let mut tank = EntityType::default();
    /// tank.kind = EntityKind::Platform;
    /// tank.domain = 1; // Land
    ///
    /// let appearance = EntityAppearance::from_raw(0b1_0000); // bits 3-4 hold 2
    /// let Some(AppearanceView::LandPlatform(land)) = appearance.view(&tank) else {
    ///     unreachable!()
    /// };
    /// assert_eq!(land.damage(), AppearanceDamage::ModerateDamage);
    /// ```
    #[must_use]
    pub const fn view(self, entity_type: &EntityType) -> Option<AppearanceView> {
        let raw = self.to_raw();
        Some(match entity_type.kind {
            EntityKind::Other => return None,
            EntityKind::Platform => match entity_type.domain {
                1 => AppearanceView::LandPlatform(LandPlatformAppearance::from_raw(raw)),
                2 => AppearanceView::AirPlatform(AirPlatformAppearance::from_raw(raw)),
                3 => AppearanceView::SurfacePlatform(SurfacePlatformAppearance::from_raw(raw)),
                4 => {
                    AppearanceView::SubsurfacePlatform(SubsurfacePlatformAppearance::from_raw(raw))
                }
                5 => AppearanceView::SpacePlatform(SpacePlatformAppearance::from_raw(raw)),
                _ => return None,
            },
            EntityKind::Munition => AppearanceView::Munition(MunitionAppearance::from_raw(raw)),
            EntityKind::LifeForm => {
                if entity_type.category >= FIRST_NON_HUMAN_LIFE_FORM_CATEGORY {
                    AppearanceView::NonHumanLifeForm(NonHumanLifeFormAppearance::from_raw(raw))
                } else {
                    AppearanceView::HumanLifeForm(HumanLifeFormAppearance::from_raw(raw))
                }
            }
            EntityKind::Environmental => {
                AppearanceView::Environmental(EnvironmentalAppearance::from_raw(raw))
            }
            EntityKind::CulturalFeature => {
                AppearanceView::CulturalFeature(CulturalFeatureAppearance::from_raw(raw))
            }
            EntityKind::Supply => AppearanceView::Supply(SupplyAppearance::from_raw(raw)),
            EntityKind::Radio => AppearanceView::Radio(RadioAppearance::from_raw(raw)),
            EntityKind::Expendable => {
                AppearanceView::Expendable(ExpendableAppearance::from_raw(raw))
            }
            EntityKind::SensorEmitter => {
                AppearanceView::SensorEmitter(SensorEmitterAppearance::from_raw(raw))
            }
        })
    }
}

#[cfg(test)]
mod tests;
