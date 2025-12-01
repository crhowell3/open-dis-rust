//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
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
//! ## Example usage
//!
//! ```rust
//! use open_dis_rust::common::Pdu;
//! use open_dis_rust::simulation_management::AcknowledgePdu;
//! use bytes::BytesMut;
//!
//! // Create new mutable byte array
//! let mut bytes = BytesMut::new();
//! // Create a pre-populated AcknowledgePdu
//! let mut pdu = AcknowledgePdu::default();
//! // Serialize the PDU into the byte array, which can then be sent over UDP
//! pdu.serialize(&mut bytes);
//! ```

pub mod common;
pub mod distributed_emissions;
pub mod entity_information;
pub mod entity_management;
pub mod information_operations;
pub mod pdu_macro;
//pub mod live_entity_information;
pub mod logistics;
pub mod minefield;
pub mod radio_communications;
pub mod simulation_management;
pub mod simulation_management_with_reliability;
pub mod synthetic_environment;
pub mod warfare;
