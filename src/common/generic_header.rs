//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BytesMut};

use crate::common::{
    DISError,
    enums::{PduType, ProtocolFamily},
};

pub trait GenericHeader: Sized {
    fn pdu_type(&self) -> PduType;
    fn set_pdu_type(&mut self, value: PduType);

    fn protocol_family(&self) -> ProtocolFamily;
    fn set_protocol_family(&mut self, value: ProtocolFamily);

    fn length(&self) -> u16;
    fn set_length(&mut self, value: u16);

    fn serialize(&self, buf: &mut BytesMut);

    /// Attempts to deserialize a PDU from the buffer
    ///
    /// # Errors
    ///
    /// Will return a `DISError` if deserialization fails
    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>;
}
