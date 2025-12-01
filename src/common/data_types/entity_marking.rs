use crate::{
    common::{SerializedLength, enums::EntityMarkingCharacterSet},
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};
use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug, Default)]
pub struct EntityMarking {
    pub entity_marking_character_set: EntityMarkingCharacterSet,
    pub entity_marking_string: String,
}

impl EntityMarking {
    #[must_use]
    pub const fn new(
        entity_marking_character_set: EntityMarkingCharacterSet,
        entity_marking_string: String,
    ) -> Self {
        Self {
            entity_marking_character_set,
            entity_marking_string,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.entity_marking_character_set as u8);
        let marking = self.entity_marking_string.clone();
        buf.put_slice(&marking.into_bytes()[..]);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            entity_marking_character_set: EntityMarkingCharacterSet::deserialize(buf),
            entity_marking_string: buf.remaining().to_string(),
        }
    }
}

impl FieldSerialize for EntityMarking {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for EntityMarking {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for EntityMarking {
    fn field_len(&self) -> usize {
        Self::LENGTH
    }
}

impl SerializedLength for EntityMarking {
    const LENGTH: usize = 12;
}
