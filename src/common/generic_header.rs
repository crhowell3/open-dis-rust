//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BytesMut};

use crate::common::enums::{PduType, ProtocolFamily};

pub trait GenericHeader: Sized {
    fn pdu_type(&self) -> PduType;
    fn set_pdu_type(&mut self, value: PduType);

    fn protocol_family(&self) -> ProtocolFamily;
    fn set_protocol_family(&mut self, value: ProtocolFamily);

    fn length(&self) -> u16;
    fn set_length(&mut self, value: u16);

    fn serialize(&self, buf: &mut BytesMut);
    fn deserialize<B: Buf>(bug: &mut B) -> Self;
}
