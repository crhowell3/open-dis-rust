//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::GenericHeader;

use super::dis_error::DISError;
use bytes::{Buf, BytesMut};
use std::any::Any;

pub trait Pdu {
    type Header: GenericHeader + Default;

    /// # Errors
    ///
    /// Will return `DISError` if the calculated PDU length is greater than the maximum allowed size
    fn calculate_length(&self) -> Result<u16, DISError>;

    fn header(&self) -> &Self::Header;
    fn header_mut(&mut self) -> &mut Self::Header;

    fn finalize(&mut self) {
        let len = self.calculate_length();
        self.header_mut().set_length(len.unwrap_or_default());
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
        pdu_header: Self::Header,
    ) -> Result<Self, DISError>
    where
        Self: Sized;

    fn as_any(&self) -> &dyn Any;
}
