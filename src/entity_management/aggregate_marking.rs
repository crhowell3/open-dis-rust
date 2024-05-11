//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug, Default)]
pub struct AggregateMarking {
    pub character_set: u8,
    pub characters: [i8; 31],
}

impl AggregateMarking {
    #[must_use]
    pub fn new(character_set: u8, characters: [i8; 31]) -> Self {
        AggregateMarking {
            character_set,
            characters,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.character_set);
        for i in self.characters {
            buf.put_i8(i);
        }
    }

    pub fn decode(buf: &mut BytesMut) -> AggregateMarking {
        let character_set = buf.get_u8();
        let mut characters: [i8; 31] = [0; 31];
        for char in &mut characters {
            *char = buf.get_i8();
        }
        AggregateMarking {
            character_set,
            characters,
        }
    }
}
