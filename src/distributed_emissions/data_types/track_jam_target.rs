//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::common::entity_id::EntityId;

#[derive(Copy, Clone, Debug)]
pub struct TrackJamTarget {
    pub track_jam: EntityId,
    pub emitter_id: u8,
    pub beam_id: u8,
}

impl Default for TrackJamTarget {
    fn default() -> Self {
        TrackJamTarget {
            track_jam: EntityId::default(1),
            emitter_id: 0,
            beam_id: 0,
        }
    }
}

impl TrackJamTarget {
    pub fn serialize(&self, buf: &mut BytesMut) {
        self.track_jam.serialize(buf);
        buf.put_u8(self.emitter_id);
        buf.put_u8(self.beam_id);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> TrackJamTarget {
        TrackJamTarget {
            track_jam: EntityId::deserialize(buf),
            emitter_id: buf.get_u8(),
            beam_id: buf.get_u8(),
        }
    }
}
