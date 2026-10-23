//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AggregateMarking {
    pub character_set: u8,
    pub characters: [i8; 31],
}

impl AggregateMarking {
    #[must_use]
    pub const fn new(character_set: u8, characters: [i8; 31]) -> Self {
        Self {
            character_set,
            characters,
        }
    }
}
