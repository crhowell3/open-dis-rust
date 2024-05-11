use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug)]
pub struct EntityMarking {
    pub entity_marking_character_set: EntityMarkingCharacterSet,
    pub entity_marking_string: String,
}

impl EntityMarking {
    #[must_use]
    pub fn new(
        entity_marking_character_set: EntityMarkingCharacterSet,
        entity_marking_string: String,
    ) -> Self {
        EntityMarking {
            entity_marking_character_set,
            entity_marking_string,
        }
    }

    #[must_use]
    pub fn default(marking: String) -> Self {
        EntityMarking {
            entity_marking_character_set: EntityMarkingCharacterSet::ASCII,
            entity_marking_string: marking,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.entity_marking_character_set as u8);
        let marking = self.entity_marking_string.clone();
        buf.put_slice(&marking.into_bytes()[..]);
    }

    pub fn decode(buf: &mut BytesMut) -> EntityMarking {
        EntityMarking {
            entity_marking_character_set: EntityMarkingCharacterSet::from_u8(buf.get_u8()),
            entity_marking_string: buf.remaining().to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[allow(clippy::module_name_repetitions)]
pub enum EntityMarkingCharacterSet {
    #[default]
    Unused = 0,
    ASCII = 1,
    ArmyMarking = 2,
    DigitChevron = 3,
}

impl EntityMarkingCharacterSet {
    #[must_use]
    pub fn from_u8(bit: u8) -> EntityMarkingCharacterSet {
        match bit {
            1 => EntityMarkingCharacterSet::ASCII,
            2 => EntityMarkingCharacterSet::ArmyMarking,
            3 => EntityMarkingCharacterSet::DigitChevron,
            _ => EntityMarkingCharacterSet::Unused,
        }
    }
}
