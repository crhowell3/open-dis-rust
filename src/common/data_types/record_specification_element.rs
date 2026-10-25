//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct RecordSpecificationElement {
    pub record_id: u32,
    pub record_set_serial_number: u32,
    pub record_length: u16,
    #[deku(assert = "crate::common::wire::fits_in_pdu(values_len(*record_length, *record_count))")]
    pub record_count: u16,
    /// Exactly `record_length * record_count` octets; encoding fails otherwise.
    #[deku(
        count = "values_len(*record_length, *record_count)",
        assert = "record_values.len() == values_len(*record_length, *record_count)"
    )]
    pub record_values: Vec<u8>,
}

impl RecordSpecificationElement {
    #[must_use]
    pub const fn new(
        record_id: u32,
        record_set_serial_number: u32,
        record_length: u16,
        record_count: u16,
        record_values: Vec<u8>,
    ) -> Self {
        Self {
            record_id,
            record_set_serial_number,
            record_length,
            record_count,
            record_values,
        }
    }
}

/// The number of octets in `record_values`: `record_length * record_count`.
/// Can't overflow: two 16-bit values multiply to at most 32 bits.
fn values_len(record_length: u16, record_count: u16) -> usize {
    usize::from(record_length).saturating_mul(usize::from(record_count))
}
