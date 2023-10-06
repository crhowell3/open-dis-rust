//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation 
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use std::any::Any;
use bytes::BytesMut;

pub trait Pdu {
    fn serialize(&self);
    fn deserialize(buffer: BytesMut);
    fn as_any(&self) -> &dyn Any;
}
