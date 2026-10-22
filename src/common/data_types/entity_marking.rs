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
        let mut chars = [0u8; 11];
        let src = self.entity_marking_string.as_bytes();
        let n = src.len().min(chars.len());
        chars[..n].copy_from_slice(&src[..n]);
        buf.put_slice(&chars);
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        let entity_marking_character_set = EntityMarkingCharacterSet::deserialize(buf);

        let mut chars = [0u8; 11];
        buf.copy_to_slice(&mut chars);
        let len = chars.iter().position(|&b| b == 0).unwrap_or(chars.len());

        Self {
            entity_marking_character_set,
            entity_marking_string: String::from_utf8_lossy(&chars[..len]).into_owned(),
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
