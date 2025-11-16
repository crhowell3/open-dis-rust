//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! Constants and compile-time computations for DIS protocol

/// Maximum PDU size in bytes as defined by the DIS standard
pub const MAX_PDU_SIZE_OCTETS: usize = 8192;
pub const MAX_PDU_SIZE_BITS: usize = 65_536;

/// PDU header size in bytes
pub const PDU_HEADER_SIZE: usize = 12;

/// Maximum number of articulation parameters
pub const MAX_ARTICULATION_PARAMS: usize = 64;

/// Maximum size of entity marking string
pub const MAX_ENTITY_MARKING_LENGTH: usize = 32;

/// Protocol version constants
pub const PROTOCOL_VERSION_1995: u8 = 3;
pub const PROTOCOL_VERSION_1998: u8 = 4;
pub const PROTOCOL_VERSION_2012: u8 = 7;

#[must_use]
/// Compile-time PDU size validation
pub const fn validate_pdu_size(size: usize) -> bool {
    size <= MAX_PDU_SIZE_OCTETS
}

#[must_use]
/// Compile-time calculation of PDU size including header
pub const fn total_pdu_size(payload_size: usize) -> usize {
    PDU_HEADER_SIZE + payload_size
}

#[must_use]
/// Compile-time string length validation for entity marking
pub const fn validate_marking_length(len: usize) -> bool {
    len <= MAX_ENTITY_MARKING_LENGTH
}

#[must_use]
/// Compile-time protocol version validation
pub const fn is_valid_protocol_version(version: u8) -> bool {
    matches!(
        version,
        PROTOCOL_VERSION_1995 | PROTOCOL_VERSION_1998 | PROTOCOL_VERSION_2012
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pdu_size_validation() {
        assert!(validate_pdu_size(1000));
        assert!(validate_pdu_size(MAX_PDU_SIZE_OCTETS));
        assert!(!validate_pdu_size(MAX_PDU_SIZE_OCTETS + 1));
    }

    #[test]
    fn test_total_pdu_size() {
        assert_eq!(total_pdu_size(100), PDU_HEADER_SIZE + 100);
        assert_eq!(total_pdu_size(0), PDU_HEADER_SIZE);
    }

    #[test]
    fn test_marking_length_validation() {
        assert!(validate_marking_length(31));
        assert!(validate_marking_length(MAX_ENTITY_MARKING_LENGTH));
        assert!(!validate_marking_length(MAX_ENTITY_MARKING_LENGTH + 1));
    }

    #[test]
    fn test_protocol_version_validation() {
        assert!(is_valid_protocol_version(PROTOCOL_VERSION_1995));
        assert!(is_valid_protocol_version(PROTOCOL_VERSION_1998));
        assert!(is_valid_protocol_version(PROTOCOL_VERSION_2012));
        assert!(!is_valid_protocol_version(0));
        assert!(!is_valid_protocol_version(255));
    }
}
