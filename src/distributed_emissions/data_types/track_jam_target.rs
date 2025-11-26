//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

use crate::{
    common::{SerializedLength, data_types::entity_id::EntityId},
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Default, Debug)]
pub struct TrackJamTarget {
    pub track_jam: EntityId,
    pub emitter_id: u8,
    pub beam_id: u8,
}

impl TrackJamTarget {
    pub fn serialize(&self, buf: &mut BytesMut) {
        self.track_jam.serialize(buf);
        buf.put_u8(self.emitter_id);
        buf.put_u8(self.beam_id);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            track_jam: EntityId::deserialize(buf),
            emitter_id: buf.get_u8(),
            beam_id: buf.get_u8(),
        }
    }
}

impl FieldSerialize for TrackJamTarget {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for TrackJamTarget {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for TrackJamTarget {
    fn field_len(&self) -> usize {
        EntityId::LENGTH + 1 + 1
    }
}
