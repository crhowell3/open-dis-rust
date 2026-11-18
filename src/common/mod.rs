//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! All commonly used non-PDU data types

pub mod constants;
/// Data types the standard defines or implies that are not §6.2 records.
pub mod data_types;
/// The crate's error type, [`DISError`].
pub(crate) mod dis_error;
/// The compact records used by the Live Entity Information/Interaction PDUs.
pub mod live_entity_records;
/// The [`Pdu`] trait every PDU implements, and the traits that name the two
/// kinds of PDU.
pub(crate) mod pdu;
/// The records of IEEE 1278.1-2012 §6.2 that PDUs are built from, in section order.
pub mod records;
pub(crate) mod wire;

pub use dis_error::{DISError, DISResult, WireFormatError};
pub use pdu::{LiveEntityPdu, Pdu, StandardPdu};
