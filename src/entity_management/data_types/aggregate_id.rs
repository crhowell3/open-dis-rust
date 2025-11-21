//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug, Default)]
pub struct AggregateId {
    pub site: u16,
    pub application: u16,
    pub aggregate_id: u16,
}

impl AggregateId {
    #[must_use]
    pub fn new(site: u16, application: u16, aggregate_id: u16) -> Self {
        AggregateId {
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

    pub fn deserialize<B: Buf>(buf: &mut B) -> AggregateId {
        AggregateId {
            site: buf.get_u16(),
            application: buf.get_u16(),
            aggregate_id: buf.get_u16(),
        }
    }
}
