//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::enums::EntityMarkingCharacterSet;
use deku::{
    DekuError, DekuReader, DekuWriter,
    no_std_io::{Read, Seek, Write},
    reader::Reader,
    writer::Writer,
};

/// Number of character octets in an Entity Marking record.
const MARKING_CHARACTERS: usize = 11;

/// Entity Marking record: a character set followed by 11 character octets.
///
/// Always 12 octets on the wire. A longer `entity_marking_string` is truncated
/// to its first 11 octets when encoded, a shorter one is zero-padded, and the
/// padding is removed when decoded.
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EntityMarking {
    pub entity_marking_character_set: EntityMarkingCharacterSet,
    #[deku(
        reader = "read_characters(deku::reader)",
        writer = "write_characters(deku::writer, &self.entity_marking_string)"
    )]
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
}

/// Read the 11 character octets, dropping trailing zero padding. Octets that
/// aren't valid UTF-8 become U+FFFD.
fn read_characters<R: Read + Seek>(reader: &mut Reader<R>) -> Result<String, DekuError> {
    let characters = <[u8; MARKING_CHARACTERS]>::from_reader_with_ctx(reader, ())?;
    let text = characters
        .iter()
        .position(|&b| b == 0)
        .and_then(|end| characters.get(..end))
        .unwrap_or(&characters);
    Ok(String::from_utf8_lossy(text).into_owned())
}

/// Write `text` as exactly 11 octets: truncated or zero-padded.
fn write_characters<W: Write + Seek>(writer: &mut Writer<W>, text: &str) -> Result<(), DekuError> {
    let mut characters = [0u8; MARKING_CHARACTERS];
    for (slot, byte) in characters.iter_mut().zip(text.bytes()) {
        *slot = byte;
    }
    characters.to_writer(writer, ())
}
