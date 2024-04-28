//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug)]
pub struct AggregateMarking {
    pub character_set: u8,
    pub characters: Vec<i8>,
}

impl Default for AggregateMarking {
    fn default() -> Self {
        AggregateMarking {
            character_set: 0,
            characters: vec![0; 31],
        }
    }
}

impl AggregateMarking {
    pub fn new(character_set: u8, characters: Vec<i8>) -> Self {
        AggregateMarking {
            character_set,
            characters,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.character_set);
        for i in 0..self.characters.len() {
            buf.put_i8(self.characters[i]);
        }
    }

    pub fn decode(buf: &mut BytesMut) -> AggregateMarking {
        let character_set = buf.get_u8();
        let mut characters: Vec<i8> = vec![0; 31];
        for _i in 0..31 {
            characters.push(buf.get_i8());
        }
        AggregateMarking {
            character_set,
            characters,
        }
    }
}
