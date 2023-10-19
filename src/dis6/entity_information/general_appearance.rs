//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]

pub struct GeneralAppearance {
    pub entity_paint_scheme: EntityPaintScheme,
    pub entity_mobility_kill: EntityMobilityKill,
    pub entity_fire_power: EntityFirePower,
    pub entity_damage: EntityDamage,
    pub entity_smoke: EntitySmoke,
    pub entity_trailing_effect: EntityTrailingEffect,
    pub entity_hatch_state: EntityHatchState,
    pub entity_lights: EntityLights,
    pub entity_flaming_effect: EntityFlamingEffect,
}

impl GeneralAppearance {
    pub fn new(
        entity_paint_scheme: EntityPaintScheme,
        entity_mobility_kill: EntityMobilityKill,
        entity_fire_power: EntityFirePower,
        entity_damage: EntityDamage,
        entity_smoke: EntitySmoke,
        entity_trailing_effect: EntityTrailingEffect,
        entity_hatch_state: EntityHatchState,
        entity_lights: EntityLights,
        entity_flaming_effect: EntityFlamingEffect,
    ) -> Self {
        GeneralAppearance {
            entity_paint_scheme,
            entity_mobility_kill,
            entity_fire_power,
            entity_damage,
            entity_smoke,
            entity_trailing_effect,
            entity_hatch_state,
            entity_lights,
            entity_flaming_effect,
        }
    }

    pub fn default() -> Self {
        GeneralAppearance {
            entity_paint_scheme: EntityPaintScheme::UniformColor,
            entity_mobility_kill: EntityMobilityKill::NoMobilityKill,
            entity_fire_power: EntityFirePower::NoFirePowerKill,
            entity_damage: EntityDamage::NoDamage,
            entity_smoke: EntitySmoke::NotSmoking,
            entity_trailing_effect: EntityTrailingEffect::None,
            entity_hatch_state: EntityHatchState::NotApplicable,
            entity_lights: EntityLights::None,
            entity_flaming_effect: EntityFlamingEffect::None,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        let entity_paint_scheme: u16 = self.entity_paint_scheme as u16;
        let entity_paint_scheme = entity_paint_scheme << 15;
        let entity_mobility_kill: u16 = self.entity_mobility_kill as u16;
        let entity_mobility_kill = entity_mobility_kill << 14;
        let entity_fire_power: u16 = self.entity_fire_power as u16;
        let entity_fire_power = entity_fire_power << 13;
        let entity_damage: u16 = self.entity_damage as u16;
        let entity_damage = entity_damage << 11;
        let entity_smoke: u16 = self.entity_smoke as u16;
        let entity_smoke = entity_smoke << 9;
        let entity_trailing_effect: u16 = self.entity_trailing_effect as u16;
        let entity_trailing_effect = entity_trailing_effect << 7;
        let entity_hatch_state: u16 = self.entity_hatch_state as u16;
        let entity_hatch_state = entity_hatch_state << 4;
        let entity_lights: u16 = self.entity_lights as u16;
        let entity_lights = entity_lights << 1;
        let entity_flaming_effect: u16 = self.entity_flaming_effect as u16;

        let general_appearance: u16 = 0u16
            | entity_paint_scheme
            | entity_mobility_kill
            | entity_fire_power
            | entity_damage
            | entity_smoke
            | entity_trailing_effect
            | entity_hatch_state
            | entity_lights
            | entity_flaming_effect;
        buf.put_u16(general_appearance);
    }

    pub fn decode(buf: &mut BytesMut) -> GeneralAppearance {
        let bytes = buf.get_u16();
        GeneralAppearance {
            entity_paint_scheme: EntityPaintScheme::from_u8((bytes >> 1) as u8),
            entity_mobility_kill: EntityMobilityKill::from_u8((bytes >> 1) as u8),
            entity_fire_power: EntityFirePower::from_u8((bytes >> 1) as u8),
            entity_damage: EntityDamage::from_u8((bytes >> 2) as u8),
            entity_smoke: EntitySmoke::from_u8((bytes >> 2) as u8),
            entity_trailing_effect: EntityTrailingEffect::from_u8((bytes >> 2) as u8),
            entity_hatch_state: EntityHatchState::from_u8((bytes >> 3) as u8),
            entity_lights: EntityLights::from_u8((bytes >> 3) as u8),
            entity_flaming_effect: EntityFlamingEffect::from_u8((bytes >> 1) as u8),
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Serialize, Deserialize)]
pub enum EntityPaintScheme {
    UniformColor = 0,
    Camouflage = 1,
}

impl EntityPaintScheme {
    pub fn from_u8(bit: u8) -> EntityPaintScheme {
        match bit {
            0 => EntityPaintScheme::UniformColor,
            1 => EntityPaintScheme::Camouflage,
            2_u8..=u8::MAX => EntityPaintScheme::UniformColor,
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Serialize, Deserialize)]
pub enum EntityMobilityKill {
    NoMobilityKill = 0,
    MobilityKill = 1,
}

impl EntityMobilityKill {
    pub fn from_u8(bit: u8) -> EntityMobilityKill {
        match bit {
            0 => EntityMobilityKill::NoMobilityKill,
            1 => EntityMobilityKill::MobilityKill,
            2_u8..=u8::MAX => EntityMobilityKill::NoMobilityKill,
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Serialize, Deserialize)]
pub enum EntityFirePower {
    NoFirePowerKill = 0,
    FirePowerKill = 1,
}

impl EntityFirePower {
    pub fn from_u8(bit: u8) -> EntityFirePower {
        match bit {
            0 => EntityFirePower::NoFirePowerKill,
            1 => EntityFirePower::FirePowerKill,
            2_u8..=u8::MAX => EntityFirePower::NoFirePowerKill,
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Serialize, Deserialize)]
pub enum EntityDamage {
    NoDamage = 0,
    SlightDamage = 1,
    ModerateDamage = 2,
    Destroyed = 3,
}

impl EntityDamage {
    pub fn from_u8(bit: u8) -> EntityDamage {
        match bit {
            0 => EntityDamage::NoDamage,
            1 => EntityDamage::SlightDamage,
            2 => EntityDamage::ModerateDamage,
            3_u8..=u8::MAX => EntityDamage::Destroyed,
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Serialize, Deserialize)]
pub enum EntitySmoke {
    NotSmoking = 0,
    SmokePlumeRising = 1,
    EngineSmoke = 2,
    EngineSmokeAndSmokePlumeRising = 3,
}

impl EntitySmoke {
    pub fn from_u8(bit: u8) -> EntitySmoke {
        match bit {
            0 => EntitySmoke::NotSmoking,
            1 => EntitySmoke::SmokePlumeRising,
            2 => EntitySmoke::EngineSmoke,
            3_u8..=u8::MAX => EntitySmoke::EngineSmokeAndSmokePlumeRising,
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Serialize, Deserialize)]
pub enum EntityTrailingEffect {
    None = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
}

impl EntityTrailingEffect {
    pub fn from_u8(bit: u8) -> EntityTrailingEffect {
        match bit {
            0 => EntityTrailingEffect::None,
            1 => EntityTrailingEffect::Small,
            2 => EntityTrailingEffect::Medium,
            3_u8..=u8::MAX => EntityTrailingEffect::Large,
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Serialize, Deserialize)]
pub enum EntityHatchState {
    NotApplicable = 0,
    PrimaryHatchClosed = 1,
    PrimaryHatchPopped = 2,
    PrimaryHatchPoppedAndPersonVisibleUnderHatch = 3,
    PrimaryHatchOpen = 4,
    PrimaryHatchOpenAndPersonVisible = 5,
}

impl EntityHatchState {
    pub fn from_u8(bit: u8) -> EntityHatchState {
        match bit {
            0 => EntityHatchState::NotApplicable,
            1 => EntityHatchState::PrimaryHatchClosed,
            2 => EntityHatchState::PrimaryHatchPopped,
            3 => EntityHatchState::PrimaryHatchPoppedAndPersonVisibleUnderHatch,
            4 => EntityHatchState::PrimaryHatchOpen,
            5_u8..=u8::MAX => EntityHatchState::PrimaryHatchOpenAndPersonVisible,
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Serialize, Deserialize)]
pub enum EntityLights {
    None = 0,
    RunningLightsOn = 1,
    NavigationLightsOn = 2,
    FormationLightsOn = 3,
}

impl EntityLights {
    pub fn from_u8(bit: u8) -> EntityLights {
        match bit {
            0 => EntityLights::None,
            1 => EntityLights::RunningLightsOn,
            2 => EntityLights::NavigationLightsOn,
            3_u8..=u8::MAX => EntityLights::FormationLightsOn,
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq, Serialize, Deserialize)]
pub enum EntityFlamingEffect {
    None = 0,
    FlamesPresent = 1,
}

impl EntityFlamingEffect {
    pub fn from_u8(bit: u8) -> EntityFlamingEffect {
        match bit {
            0 => EntityFlamingEffect::None,
            1 => EntityFlamingEffect::FlamesPresent,
            2_u8..=u8::MAX => EntityFlamingEffect::None,
        }
    }
}
