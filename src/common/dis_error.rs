//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use thiserror::Error;

use crate::common::enums::ProtocolVersion;

/// Protocol version and header related states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PduState {
    Uninitialized,
    HeaderValid,
    BodyValid,
    Complete,
}

#[derive(Error, Debug)]
/// Enhanced error types for DIS protocol operations
pub enum DISError {
    #[error("Invalid PDU header: {reason}")]
    InvalidHeader {
        reason: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("PDU deserialization error: {msg}")]
    DeserializationError {
        msg: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("PDU serialization error: {msg}")]
    SerializationError {
        msg: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Invalid PDU state transition: {current_state:?} -> {target_state:?}")]
    InvalidStateTransition {
        current_state: PduState,
        target_state: PduState,
    },

    #[error("Protocol version mismatch: expected {expected:?}, got {got:?}")]
    ProtocolVersionMismatch {
        expected: ProtocolVersion,
        got: ProtocolVersion,
    },

    #[error("Invalid field value: {field} = {value}, {reason}")]
    InvalidFieldValue {
        field: String,
        value: String,
        reason: String,
    },

    #[error("Buffer underflow: tried to read {attempted} bytes, but only {available} bytes remain")]
    BufferUnderflow { attempted: usize, available: usize },

    #[error("Network error: {0}")]
    NetworkError(#[from] std::io::Error),

    #[error("PDU size exceeds maximum allowed: {size} > {max_size}")]
    PduSizeExceeded { size: usize, max_size: usize },
}

impl DISError {
    /// Create a new `InvalidHeader` error with optional source error
    pub fn invalid_header<S: Into<String>>(
        reason: S,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self::InvalidHeader {
            reason: reason.into(),
            source,
        }
    }

    /// Create a new `DeserializationError` with optional source error
    pub fn deserialization_error<S: Into<String>>(
        msg: S,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self::DeserializationError {
            msg: msg.into(),
            source,
        }
    }

    /// Create a new `SerializationError` with optional source error
    pub fn serialization_error<S: Into<String>>(
        msg: S,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self::SerializationError {
            msg: msg.into(),
            source,
        }
    }

    /// Create a new `InvalidFieldValue` error
    pub fn invalid_field<S: Into<String>>(field: S, value: S, reason: S) -> Self {
        Self::InvalidFieldValue {
            field: field.into(),
            value: value.into(),
            reason: reason.into(),
        }
    }

    #[must_use]
    /// Create a new `BufferUnderflow` error
    pub const fn buffer_underflow(attempted: usize, available: usize) -> Self {
        Self::BufferUnderflow {
            attempted,
            available,
        }
    }

    #[must_use]
    /// Create a new `PduSizeExceeded` error
    pub const fn pdu_size_exceeded(size: usize, max_size: usize) -> Self {
        Self::PduSizeExceeded { size, max_size }
    }
}

/// Result type for DIS operations
pub type DISResult<T> = Result<T, DISError>;
