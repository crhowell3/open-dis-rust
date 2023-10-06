//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation 
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{BytesMut, BufMut, Buf};
use chrono::{Utc, Timelike};
use num_derive::FromPrimitive;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PduHeader {
    // The version of the protocol
    protocol_version: u8,
    // Exercise ID
    exercise_id: u8,
    // Type of PDU, unique for each PDU class
    pdu_type: u8,
    // Value that refers to the protocol family
    protocol_family: u8,
    // Timestamp value
    timestamp: u32,
    // Length, in bytes, of the PDU
    length: u16,
    // Zero-filled array of padding
    padding: i16,
}

impl PduHeader {
    pub fn new(pdu_type: PduType, protocol_family: ProtocolFamily, exercise_id: u8, length: u16)
        -> Self {
        PduHeader {
            protocol_version,
            exercise_id,
            pdu_type,
            protocol_family,
            timestamp,
            length: length as u16,
            padding: 0 as u16
        }        
    }

    pub fn default(pdu_type: PduType, protocol_family: ProtocolFamily, length: u16) -> Self {
        PduHeader {
            protocol_version,
            exercise_id: 1,
            pdu_type,
            protocol_family,
            length: length as u16,
            padding: 0 as u16
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        
    }
}
