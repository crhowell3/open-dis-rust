//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::{
        data_types::{EntityCoordinateVector, EventId},
        enums::{ComponentDamageStatus, ComponentIdentification, ComponentVisualSmokeColor},
    },
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
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

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u32(self.record_type);
        buf.put_u16(self.record_length);
        buf.put_u16(self.padding);
        self.damage_location.serialize(buf);
        buf.put_f32(self.damage_diameter);
        buf.put_f32(self.temperature);
        buf.put_u8(self.component_identification as u8);
        buf.put_u8(self.component_damage_status as u8);
        buf.put_u8(self.component_visual_damage_status);
        buf.put_u8(self.component_visual_smoke_color as u8);
        self.fire_event_id.serialize(buf);
        buf.put_u16(self.padding2);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            record_type: buf.get_u32(),
            record_length: buf.get_u16(),
            padding: buf.get_u16(),
            damage_location: EntityCoordinateVector::deserialize(buf),
            damage_diameter: buf.get_f32(),
            temperature: buf.get_f32(),
            component_identification: ComponentIdentification::deserialize(buf),
            component_damage_status: ComponentDamageStatus::deserialize(buf),
            component_visual_damage_status: buf.get_u8(),
            component_visual_smoke_color: ComponentVisualSmokeColor::deserialize(buf),
            fire_event_id: EventId::deserialize(buf),
            padding2: buf.get_u16(),
        }
    }
}

impl FieldSerialize for DirectedEnergyDamage {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for DirectedEnergyDamage {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for DirectedEnergyDamage {
    fn field_len(&self) -> usize {
        self.record_type.field_len()
            + self.record_length.field_len()
            + self.padding.field_len()
            + self.damage_location.field_len()
            + self.damage_diameter.field_len()
            + self.temperature.field_len()
            + self.component_identification.field_len()
            + self.component_damage_status.field_len()
            + self.component_visual_damage_status.field_len()
            + self.component_visual_smoke_color.field_len()
            + self.fire_event_id.field_len()
            + self.padding2.field_len()
    }
}
