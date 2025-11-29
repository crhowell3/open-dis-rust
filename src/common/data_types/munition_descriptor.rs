//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::data_types::entity_type::EntityType,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
pub struct MunitionDescriptor {
    pub munition_type: EntityType,
    pub warhead: u16,
    pub fuse: u16,
    pub quantity: u16,
    pub rate: u16,
}

impl MunitionDescriptor {
    #[must_use]
    pub const fn new(
        munition_type: EntityType,
        warhead: u16,
        fuse: u16,
        quantity: u16,
        rate: u16,
    ) -> Self {
        Self {
            munition_type,
            warhead,
            fuse,
            quantity,
            rate,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        self.munition_type.serialize(buf);
        buf.put_u16(self.warhead);
        buf.put_u16(self.fuse);
        buf.put_u16(self.quantity);
        buf.put_u16(self.rate);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            munition_type: EntityType::deserialize(buf),
            warhead: buf.get_u16(),
            fuse: buf.get_u16(),
            quantity: buf.get_u16(),
            rate: buf.get_u16(),
        }
    }
}

impl FieldSerialize for MunitionDescriptor {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for MunitionDescriptor {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for MunitionDescriptor {
    fn field_len(&self) -> usize {
        self.munition_type.field_len()
            + self.warhead.field_len()
            + self.fuse.field_len()
            + self.quantity.field_len()
            + self.rate.field_len()
    }
}
