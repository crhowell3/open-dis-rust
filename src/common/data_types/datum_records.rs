//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use deku::{
    DekuError, DekuReader, DekuWriter,
    no_std_io::{Read, Seek, Write},
    reader::Reader,
    writer::Writer,
};

use crate::common::wire;

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct FixedDatumRecord {
    pub datum_id: u32,
    pub datum_value: u32,
}

/// Variable Datum record: an ID, a length in bits, then the value, padded with
/// zeros to a 64-bit boundary.
///
/// `value` holds the value's octets, `ceil(length_bits / 8)` of them; the
/// padding is added when encoding and skipped when decoding.
#[derive(Debug, Clone, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct VariableDatumRecord {
    pub datum_id: u32,
    #[deku(assert = "wire::fits_in_pdu(value_len(*length_bits))")]
    pub length_bits: u32,
    /// Encoding fails unless this holds exactly `ceil(length_bits / 8)` octets.
    #[deku(
        reader = "read_value(deku::reader, *length_bits)",
        writer = "write_value(deku::writer, value, *length_bits)"
    )]
    pub value: Vec<u8>,
}

/// Octets needed to hold `length_bits` bits.
fn value_len(length_bits: u32) -> usize {
    usize::try_from(length_bits.div_ceil(8)).unwrap_or(usize::MAX)
}

/// Zero octets after the value, to reach the next 64-bit boundary.
fn padding_len(length_bits: u32) -> usize {
    let padded = usize::try_from(length_bits.div_ceil(64))
        .unwrap_or(usize::MAX)
        .saturating_mul(8);
    padded.saturating_sub(value_len(length_bits))
}

/// Read the value's octets, then skip its padding.
fn read_value<R: Read + Seek>(
    reader: &mut Reader<R>,
    length_bits: u32,
) -> Result<Vec<u8>, DekuError> {
    let mut value = vec![0; value_len(length_bits)];
    for octet in &mut value {
        *octet = u8::from_reader_with_ctx(reader, ())?;
    }
    for _ in 0..padding_len(length_bits) {
        u8::from_reader_with_ctx(reader, ())?;
    }
    Ok(value)
}

/// Write the value's octets, then zero padding to the 64-bit boundary.
fn write_value<W: Write + Seek>(
    writer: &mut Writer<W>,
    value: &[u8],
    length_bits: u32,
) -> Result<(), DekuError> {
    if value.len() != value_len(length_bits) {
        return Err(DekuError::Assertion(
            format!(
                "VariableDatumRecord: {length_bits} bits needs {} octets, but value has {}",
                value_len(length_bits),
                value.len()
            )
            .into(),
        ));
    }
    value.to_writer(writer, ())?;
    vec![0u8; padding_len(length_bits)].to_writer(writer, ())
}
