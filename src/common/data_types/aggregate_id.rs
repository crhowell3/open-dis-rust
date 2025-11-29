//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize};

#[derive(Clone, Debug, Default)]
pub struct AggregateId {
    pub site: u16,
    pub application: u16,
    pub aggregate_id: u16,
}

impl AggregateId {
    #[must_use]
    pub const fn new(site: u16, application: u16, aggregate_id: u16) -> Self {
        Self {
            site,
            application,
            aggregate_id,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.site);
        buf.put_u16(self.application);
        buf.put_u16(self.aggregate_id);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            site: buf.get_u16(),
            application: buf.get_u16(),
            aggregate_id: buf.get_u16(),
        }
    }
}

impl FieldSerialize for AggregateId {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for AggregateId {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for AggregateId {
    fn field_len(&self) -> usize {
        6
    }
}
