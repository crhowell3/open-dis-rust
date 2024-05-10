//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::{event_id::EventId, vector3_float::Vector3Float};

#[derive(Copy, Clone, Debug)]
pub struct DirectedEnergyDamage {
    pub record_type: u32,
    pub record_length: u16,
    pub padding: u16,
    pub damage_location: Vector3Float,
    pub damage_diameter: f32,
    pub temperature: f32,
    pub component_identification: u8,
    pub component_damage_status: u8,
    pub component_visual_damage_status: u8,
    pub component_visual_smoke_color: u8,
    pub fire_event_id: EventId,
    pub padding2: u16,
}

impl Default for DirectedEnergyDamage {
    fn default() -> Self {
        DirectedEnergyDamage {
            record_type: 0,
            record_length: 0,
            padding: 0,
            damage_location: Vector3Float::default(),
            damage_diameter: 0.0,
            temperature: 0.0,
            component_identification: 0,
            component_damage_status: 0,
            component_visual_damage_status: 0,
            component_visual_smoke_color: 0,
            fire_event_id: EventId::default(1),
            padding2: 0,
        }
    }
}

impl DirectedEnergyDamage {
    pub fn new(
        record_type: u32,
        record_length: u16,
        padding: u16,
        damage_location: Vector3Float,
        damage_diameter: f32,
        temperature: f32,
        component_identification: u8,
        component_damage_status: u8,
        component_visual_damage_status: u8,
        component_visual_smoke_color: u8,
        fire_event_id: EventId,
        padding2: u16,
    ) -> Self {
        DirectedEnergyDamage {
            record_type,
            record_length,
            padding,
            damage_location,
            damage_diameter,
            temperature,
            component_identification,
            component_damage_status,
            component_visual_damage_status,
            component_visual_smoke_color,
            fire_event_id,
            padding2,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(self.record_type);
        buf.put_u16(self.record_length);
        buf.put_u16(self.padding);
        self.damage_location.serialize(buf);
        buf.put_f32(self.damage_diameter);
        buf.put_f32(self.temperature);
        buf.put_u8(self.component_identification);
        buf.put_u8(self.component_damage_status);
        buf.put_u8(self.component_visual_damage_status);
        buf.put_u8(self.component_visual_smoke_color);
        self.fire_event_id.serialize(buf);
        buf.put_u16(self.padding2);
    }

    pub fn decode(buf: &mut BytesMut) -> DirectedEnergyDamage {
        DirectedEnergyDamage {
            record_type: buf.get_u32(),
            record_length: buf.get_u16(),
            padding: buf.get_u16(),
            damage_location: Vector3Float::decode(buf),
            damage_diameter: buf.get_f32(),
            temperature: buf.get_f32(),
            component_identification: buf.get_u8(),
            component_damage_status: buf.get_u8(),
            component_visual_damage_status: buf.get_u8(),
            component_visual_smoke_color: buf.get_u8(),
            fire_event_id: EventId::decode(buf),
            padding2: buf.get_u16(),
        }
    }
}
