//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::Buf;

pub trait PduBody: Sized {
    fn deserialize_body<B: Buf>(buf: &mut B) -> Self;
}
