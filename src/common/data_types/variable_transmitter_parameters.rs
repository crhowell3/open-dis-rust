//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::enums::VariableRecordTypes;

/// Variable Transmitter Parameters record (IEEE 1278.1-2012 §6.2.95).
/// `record_length` is the whole record in octets, a multiple of 8.
#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct VariableTransmitterParameters {
    /// SISO-REF-010 UID 66.
    pub record_type: VariableRecordTypes,
    #[deku(
        update = "crate::common::wire::padded_record_length(self.record_specific_fields.len())?",
        assert = "crate::common::wire::valid_record_length(*record_length)"
    )]
    pub record_length: u16,
    /// Everything after the record length. When decoded, this includes the
    /// padding to the 64-bit boundary; when encoded, padding is added to
    /// reach it.
    #[deku(
        count = "crate::common::wire::record_fields_len(*record_length)",
        writer = "crate::common::wire::write_padded_fields(deku::writer, record_specific_fields)"
    )]
    pub record_specific_fields: Vec<u8>,
}

impl VariableTransmitterParameters {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
