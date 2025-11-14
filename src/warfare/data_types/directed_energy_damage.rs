//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::{
    EntityCoordinateVector,
    enums::{ComponentDamageStatus, ComponentIdentification, ComponentVisualSmokeColor},
    event_id::EventId,
};

#[derive(Copy, Clone, Debug)]
pub struct DirectedEnergyDamage {
    pub record_type: u32,
    pub record_length: u16,
    _padding: u16,
    pub damage_location: EntityCoordinateVector,
    pub damage_diameter: f32,
    pub temperature: f32,
    pub component_identification: ComponentIdentification,
    pub component_damage_status: ComponentDamageStatus,
    pub component_visual_damage_status: u8, // TODO(cameron): implement ComponentVisualDamageStatus record
    pub component_visual_smoke_color: ComponentVisualSmokeColor,
    pub fire_event_id: EventId,
    _padding2: u16,
}

impl Default for DirectedEnergyDamage {
    fn default() -> Self {
        DirectedEnergyDamage {
            record_type: 0,
            record_length: 0,
            _padding: 0_u16,
            damage_location: EntityCoordinateVector::default(),
            damage_diameter: 0.0,
            temperature: 0.0,
            component_identification: ComponentIdentification::default(),
            component_damage_status: ComponentDamageStatus::default(),
            component_visual_damage_status: 0,
            component_visual_smoke_color: ComponentVisualSmokeColor::default(),
            fire_event_id: EventId::default(1),
            _padding2: 0_u16,
        }
    }
}

impl DirectedEnergyDamage {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
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
        DirectedEnergyDamage {
            record_type,
            record_length,
            _padding: 0_u16,
            damage_location,
            damage_diameter,
            temperature,
            component_identification,
            component_damage_status,
            component_visual_damage_status,
            component_visual_smoke_color,
            fire_event_id,
            _padding2: 0_u16,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(self.record_type);
        buf.put_u16(self.record_length);
        buf.put_u16(self._padding);
        self.damage_location.serialize(buf);
        buf.put_f32(self.damage_diameter);
        buf.put_f32(self.temperature);
        buf.put_u8(self.component_identification as u8);
        buf.put_u8(self.component_damage_status as u8);
        buf.put_u8(self.component_visual_damage_status);
        buf.put_u8(self.component_visual_smoke_color as u8);
        self.fire_event_id.serialize(buf);
        buf.put_u16(self._padding2);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> DirectedEnergyDamage {
        DirectedEnergyDamage {
            record_type: buf.get_u32(),
            record_length: buf.get_u16(),
            _padding: buf.get_u16(),
            damage_location: EntityCoordinateVector::deserialize(buf),
            damage_diameter: buf.get_f32(),
            temperature: buf.get_f32(),
            component_identification: ComponentIdentification::deserialize(buf),
            component_damage_status: ComponentDamageStatus::deserialize(buf),
            component_visual_damage_status: buf.get_u8(),
            component_visual_smoke_color: ComponentVisualSmokeColor::deserialize(buf),
            fire_event_id: EventId::deserialize(buf),
            _padding2: buf.get_u16(),
        }
    }
}
