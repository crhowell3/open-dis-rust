//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::{
    data_types::{EntityCoordinateVector, EventId},
    enums::{ComponentDamageStatus, ComponentIdentification, ComponentVisualSmokeColor},
};

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
    pub damage_location: EntityCoordinateVector,
    pub damage_diameter: f32,
    pub temperature: f32,
    pub component_identification: ComponentIdentification,
    pub component_damage_status: ComponentDamageStatus,
    pub component_visual_damage_status: u8, // TODO(cameron): implement ComponentVisualDamageStatus record
    pub component_visual_smoke_color: ComponentVisualSmokeColor,
    pub fire_event_id: EventId,
    padding2: u16,
}

impl DirectedEnergyDamage {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        record_type: u32,
        record_length: u16,
        damage_location: EntityCoordinateVector,
        damage_diameter: f32,
        temperature: f32,
        component_identification: ComponentIdentification,
        component_damage_status: ComponentDamageStatus,
        component_visual_damage_status: u8,
        component_visual_smoke_color: ComponentVisualSmokeColor,
        fire_event_id: EventId,
    ) -> Self {
        Self {
            record_type,
            record_length,
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
