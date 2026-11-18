//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! # `open_dis_rust` crate
//!
//! This is a library that implements the IEEE 1278.1-2012 Distributed Interactive Simulation
//! application protocol.
//!
//! For more information on this standard, you may view the publication from the
//! [IEEE website](https://ieee.org/).
//!
//! ## How the crate is organized
//!
//! - **IEEE 1278.1-2012:** one module per protocol family holds its PDUs, such as
//!   [`entity_information`]. [`records`], [`live_entity_records`], and [`constants`] hold
//!   what the PDUs are built from.
//! - **SISO-REF-010-2023:** [`siso`] holds the enumerations and bit layouts the standard
//!   refers to by UID.
//! - **The rest:** the [`Pdu`] trait every PDU implements, [`StandardPdu`] and
//!   [`LiveEntityPdu`] for the two kinds of header, [`DISError`], and the [`prelude`].
//!
//! ## Example usage
//!
//! ```rust
//! use open_dis_rust::{prelude::*, simulation_management::AcknowledgePdu};
//!
//! # fn main() -> Result<(), DISError> {
//! // Create a pre-populated Acknowledge PDU and encode it, ready to send over UDP
//! let mut pdu = AcknowledgePdu::new();
//! let bytes: Vec<u8> = pdu.to_bytes()?;
//!
//! // Decode it again from a byte slice
//! let decoded = AcknowledgePdu::deserialize(&mut bytes.as_slice())?;
//! assert_eq!(decoded.request_id, pdu.request_id);
//! # Ok(())
//! # }
//! ```

const VERSION: &str = env!("CARGO_PKG_VERSION");

// IEEE 1278.1-2012: the PDUs, one module per protocol family
pub mod distributed_emissions;
pub mod entity_information;
pub mod entity_management;
pub mod information_operations;
pub mod live_entity_information;
pub mod logistics;
pub mod minefield;
pub mod radio_communications;
pub mod simulation_management;
pub mod simulation_management_with_reliability;
pub mod synthetic_environment;
pub mod warfare;

/// IEEE 1278.1-2012: what the PDUs are built from
pub mod constants;
/// The compact records used by the Live Entity Information/Interaction PDUs (IEEE
/// 1278.1-2012 §9) and their header
pub mod live_entity_records;
/// The records of IEEE 1278.1-2012 §6.2 and Annex B that PDUs are built from, in section
/// order, including the PDU header
pub mod records;

/// SISO-REF-010-2023
pub mod siso;

/// The crate's own API: the traits every PDU implements and the error type
pub(crate) mod dis_error;
pub(crate) mod pdu;
pub(crate) mod wire;

pub use dis_error::{DISError, DISResult, WireFormatError};
pub use pdu::{LiveEntityPdu, Pdu, StandardPdu};

/// The crate's version, such as `"1.0.0"`.
#[must_use]
pub fn get_version() -> String {
    VERSION.to_string()
}

/// Common traits and types for working with DIS PDUs.
///
/// ```
/// use open_dis_rust::prelude::*;
/// ```
pub mod prelude {
    pub use crate::siso::enums::{EntityKind, PduType, ProtocolFamily, ProtocolVersion};
    pub use crate::{
        DISError, LiveEntityPdu, Pdu, StandardPdu,
        constants::{MAX_PDU_SIZE_OCTETS, PDU_HEADER_SIZE},
        live_entity_records::LiveEntityPduHeader,
        records::{
            ClockTime, EntityIdentifier, EntityType, ExerciseIdentifier, PduHeader,
            SimulationAddress, Timestamp,
        },
    };
}

#[cfg(test)]
mod pdu_round_trips;
