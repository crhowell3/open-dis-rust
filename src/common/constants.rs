//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! Constants and compile-time computations for DIS protocol

/// Entity ID constants
pub const ALL_ENTITIES: u16 = 0xFFFF;
pub const ALL_APPLIC: u16 = 0xFFFF;
pub const ALL_SITES: u16 = 0xFFFF;

pub const NO_ENTITY: u16 = 0x0000;
pub const NO_APPLIC: u16 = 0x0000;
pub const NO_SITE: u16 = 0x0000;

/// Maximum PDU size in bytes as defined by the DIS standard
pub const MAX_PDU_SIZE_OCTETS: usize = 8192;
pub const MAX_PDU_SIZE_BITS: usize = 65_536;
pub const BITS_PER_BYTE: u16 = 8;

/// PDU header size in bytes
pub const PDU_HEADER_SIZE: usize = 12;

/// Maximum number of articulation parameters
pub const MAX_ARTICULATION_PARAMS: usize = 64;

/// Maximum size of entity marking string
pub const MAX_ENTITY_MARKING_LENGTH: usize = 32;

/// Miscellaneous
pub const ALL_AGGS: u16 = 0xFFFF;
pub const ALL_EMITTERS: u16 = 0xFF;
pub const ALL_BEAMS: u16 = 0xFF;
pub const ALL_OBJECTS: u16 = 0xFFFF;

pub const EP_NO_SEQUENCE: u16 = 0xFFFF;

pub const NO_AGG: u16 = 0x0000;
pub const NO_BEAM: u16 = 0x0000;
pub const NO_CATEGORY: u16 = 0x0000;
pub const NO_EMITTER: u16 = 0x0000;
pub const NO_FIRE_MISSION: u16 = 0x0000;
pub const NO_KIND: u16 = 0x0000;
pub const NO_OBJECT: u16 = 0x0000;
pub const NO_PATTERN: f32 = 0.0;
pub const NO_REF_NUMBER: u16 = 0x0000;
pub const NO_SPECIFIC: u16 = 0x0000;
pub const NO_SUBCAT: u16 = 0x0000;
pub const NO_VALUE: u16 = 0x0000;

pub const POWER_ENGINE_OFF: f32 = -100.0;
pub const POWER_IDLE: f32 = 0.0;
pub const POWER_MAX_AFTERBURNER: f32 = 100.0;
pub const POWER_MILITARY: f32 = 50.0;
pub const POWER_MIN_AFTERBURNER: f32 = 51.0;

pub const RQST_ASSIGN_ID: u16 = 0xFFFE;

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
}
