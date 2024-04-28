//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Clone, Debug)]
pub struct GridAxisRecord {}

impl Default for GridAxisRecord {
    fn default() -> Self {
        GridAxisRecord {}
    }
}

impl GridAxisRecord {
    pub fn new() -> Self {
        GridAxisRecord {}
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        unimplemented!();
    }

    pub fn decode(buf: &mut BytesMut) -> GridAxisRecord {
        GridAxisRecord {}
    }
}
