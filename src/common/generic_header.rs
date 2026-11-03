//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

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

    /// Encodes the header and appends it to `buf`
    fn serialize(&self, buf: &mut Vec<u8>);

    /// Decodes a header from the front of `buf`, advancing `buf`
    /// past it.
    ///
    /// # Errors
    ///
    /// Will return a `DISError` if deserialization fails
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DISError>;
}
