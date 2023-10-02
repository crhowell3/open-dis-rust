//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation 
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

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
        
    }
}
