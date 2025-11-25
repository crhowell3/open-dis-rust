//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use super::{dis_error::DISError, pdu_header::PduHeader};
use bytes::{Buf, BytesMut};
use std::any::Any;

pub trait Pdu {
    /// # Errors
    ///
    /// Will return `DISError` if the calculated PDU length is greater than the maximum allowed size
    fn length(&self) -> Result<u16, DISError>;

    fn header(&self) -> &PduHeader;

    fn header_mut(&mut self) -> &mut PduHeader;

    fn finalize(&mut self) {
        let len = self.length();
        self.header_mut().length = len.unwrap_or_default();
    }

    /// # Errors
    ///
    /// Will return `DISError` if serialization fails, especially if the dynamically calculated PDU
    /// length is greater than the maximum allowed size
    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError>;

    /// # Errors
    ///
    /// Will return `DISError` if the PDU header provided is invalid
    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized;
    /// # Errors
    ///
    /// Will return `DISError` if the PDU header provided is invalid
    fn deserialize_without_header<B: Buf>(
        buffer: &mut B,
        pdu_header: PduHeader,
    ) -> Result<Self, DISError>
    where
        Self: Sized;

    fn as_any(&self) -> &dyn Any;
}
