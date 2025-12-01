//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::data_types::entity_type::EntityType,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Clone, Debug, Default)]
pub struct SupplyQuantity {
    pub supply_type: EntityType,
    pub quantity: f32,
}

impl SupplyQuantity {
    #[must_use]
    pub const fn new(supply_type: EntityType, quantity: f32) -> Self {
        Self {
            supply_type,
            quantity,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.supply_type.serialize(buf);
        buf.put_f32(self.quantity);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            supply_type: EntityType::deserialize(buf),
            quantity: buf.get_f32(),
        }
    }
}

impl FieldSerialize for SupplyQuantity {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for SupplyQuantity {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for SupplyQuantity {
    fn field_len(&self) -> usize {
        self.supply_type.field_len() + self.quantity.field_len()
    }
}
