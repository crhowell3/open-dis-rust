//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use super::{dis_error::DISError, pdu_header::PduHeader};
use bytes::BytesMut;
use std::any::Any;

pub trait Pdu {
    fn serialize(&mut self, buf: &mut BytesMut);
    /// # Errors
    ///
    /// Will return `DISError` if the PDU header provided is invalid
    fn deserialize(buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized;
    /// # Errors
    ///
    /// Will return `DISError` if the PDU header provided is invalid
    fn deserialize_without_header(
        buffer: BytesMut,
        pdu_header: PduHeader,
    ) -> Result<Self, DISError>
    where
        Self: Sized;
    fn as_any(&self) -> &dyn Any;
}
