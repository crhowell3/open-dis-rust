//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::entity_type::EntityType;

#[derive(Clone, Debug, Default)]
pub struct SupplyQuantity {
    pub supply_type: EntityType,
    pub quantity: f32,
}

impl SupplyQuantity {
    #[must_use]
    pub fn new(supply_type: EntityType, quantity: f32) -> Self {
        SupplyQuantity {
            supply_type,
            quantity,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.supply_type.serialize(buf);
        buf.put_f32(self.quantity);
    }

    pub fn deserialize(buf: &mut BytesMut) -> SupplyQuantity {
        SupplyQuantity {
            supply_type: EntityType::deserialize(buf),
            quantity: buf.get_f32(),
        }
    }
}
