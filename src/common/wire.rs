//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! Interface between byte slices and vectors and `deku`. Internal to this crate.
//!
//! Wire types derive `deku::DekuRead` and `deku::DekuWrite` with the crate's standard
//! attribute line, which makes them big endian by default and lets a parent pass its
//! byte order down:
//!
//! ```ignore
//! #[deku(endian = "endian", ctx = "endian: deku::ctx::Endian", ctx_default = "deku::ctx::Endian::Big")]
//! ```
//!
//! [`read`], [`write`], and [`encoded_len`] run those types against a byte slice
//! or a `Vec<u8>`.

use deku::{
    DekuError, DekuReader, DekuWriter,
    no_std_io::{self, ErrorKind, Read, Seek, SeekFrom, Write},
    reader::Reader,
    writer::Writer,
};

use crate::common::constants::MAX_PDU_SIZE_OCTETS;

fn unsupported_seek() -> no_std_io::Error {
    no_std_io::Error::new(ErrorKind::Unsupported, "only forward seeks are supported")
}

/// Reads from the front of a slice, advancing it as octets are consumed
struct SliceSource<'a, 'b> {
    buf: &'a mut &'b [u8],
    position: u64,
}

impl SliceSource<'_, '_> {
    fn advance(&mut self, n: usize) {
        let rest: &[u8] = self.buf;
        *self.buf = rest.get(n..).unwrap_or_default();
        self.position = self
            .position
            .saturating_add(u64::try_from(n).unwrap_or(u64::MAX));
    }
}

impl Read for SliceSource<'_, '_> {
    fn read(&mut self, out: &mut [u8]) -> no_std_io::Result<usize> {
        let n = out.len().min(self.buf.len());
        if let (Some(dst), Some(src)) = (out.get_mut(..n), self.buf.get(..n)) {
            dst.copy_from_slice(src);
        }
        self.advance(n);
        Ok(n)
    }
}

impl Seek for SliceSource<'_, '_> {
    fn seek(&mut self, pos: SeekFrom) -> no_std_io::Result<u64> {
        let SeekFrom::Current(offset) = pos else {
            return Err(unsupported_seek());
        };
        let skip = usize::try_from(offset).map_err(|_| unsupported_seek())?;
        if skip > self.buf.len() {
            return Err(ErrorKind::UnexpectedEof.into());
        }
        self.advance(skip);
        Ok(self.position)
    }
}

/// Appends to the end of a vector
struct VecSink<'a>(&'a mut Vec<u8>);

impl Write for VecSink<'_> {
    fn write(&mut self, data: &[u8]) -> no_std_io::Result<usize> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> no_std_io::Result<()> {
        Ok(())
    }
}

impl Seek for VecSink<'_> {
    fn seek(&mut self, pos: SeekFrom) -> no_std_io::Result<u64> {
        match pos {
            SeekFrom::Current(0) => Ok(u64::try_from(self.0.len()).unwrap_or(u64::MAX)),
            _ => Err(unsupported_seek()),
        }
    }
}

#[derive(Default)]
struct CountingSink(usize);

impl Write for CountingSink {
    fn write(&mut self, data: &[u8]) -> no_std_io::Result<usize> {
        self.0 = self.0.saturating_add(data.len());
        Ok(data.len())
    }

    fn flush(&mut self) -> no_std_io::Result<()> {
        Ok(())
    }
}

impl Seek for CountingSink {
    fn seek(&mut self, pos: SeekFrom) -> no_std_io::Result<u64> {
        match pos {
            SeekFrom::Current(0) => Ok(u64::try_from(self.0).unwrap_or(u64::MAX)),
            _ => Err(unsupported_seek()),
        }
    }
}

/// Decode one `T` from the front of `buf`, consuming the bytes used
///
/// # Errors
///
/// Returns a [`DekuError`] if `buf` runs out or a field holds a value the standard
/// doesn't define
pub fn read<T>(buf: &mut &[u8]) -> Result<T, DekuError>
where
    T: for<'a> DekuReader<'a, ()>,
{
    let mut source = SliceSource { buf, position: 0 };
    let mut reader = Reader::new(&mut source);
    T::from_reader_with_ctx(&mut reader, ())
}

/// Encode `value` onto the end of `buf`
///
/// # Errors
///
/// Returns a [`DekuError`] if a field can't be encoded, i.e., a count that
/// doesn't fit its field. `buf` may hold a partial encoding afterwards.
pub fn write<T: DekuWriter<()>>(value: &T, buf: &mut Vec<u8>) -> Result<(), DekuError> {
    let mut sink = VecSink(buf);
    let mut writer = Writer::new(&mut sink);
    value.to_writer(&mut writer, ())?;
    writer.finalize()
}

/// The number of bytes `value` encodes to
///
/// # Errors
///
/// See [`write()`].
pub fn encoded_len<T: DekuWriter<()>>(value: &T) -> Result<usize, DekuError> {
    let mut sink = CountingSink::default();
    let mut writer = Writer::new(&mut sink);
    value.to_writer(&mut writer, ())?;
    writer.finalize()?;
    Ok(sink.0)
}

pub fn fits_in_pdu<N: TryInto<usize>>(count: N) -> bool {
    count
        .try_into()
        .is_ok_and(|count: usize| count <= MAX_PDU_SIZE_OCTETS)
}

/// Updates all items in a list
///
/// # Errors
///
/// Will return a `DekuError` if an update operation fails
pub fn update_each<T: deku::DekuUpdate>(items: &mut [T]) -> Result<usize, DekuError> {
    for item in items.iter_mut() {
        item.update()?;
    }
    Ok(items.len())
}

const VARIABLE_RECORD_HEADER: usize = 6;

#[must_use]
pub fn valid_record_length(record_length: u16) -> bool {
    usize::from(record_length) >= VARIABLE_RECORD_HEADER && fits_in_pdu(record_length)
}

#[must_use]
pub fn record_fields_len(record_length: u16) -> usize {
    usize::from(record_length).saturating_sub(VARIABLE_RECORD_HEADER)
}

/// Get the length of a record that has been padded
///
/// # Errors
///
/// Will return a `DekuError` if the record is too long
pub fn padded_record_length(fields_len: usize) -> Result<usize, DekuError> {
    fields_len
        .checked_add(VARIABLE_RECORD_HEADER)
        .and_then(|len| len.checked_next_multiple_of(8))
        .ok_or_else(|| DekuError::Assertion("record is too long".into()))
}

/// Write field to buffer that are padded for alignment
///
/// # Errors
///
/// Will return a `DekuError` if the writer fails
pub fn write_padded_fields<W: no_std_io::Write + no_std_io::Seek>(
    writer: &mut Writer<W>,
    fields: &[u8],
) -> Result<(), DekuError> {
    let padding = padded_record_length(fields.len())?
        .saturating_sub(VARIABLE_RECORD_HEADER)
        .saturating_sub(fields.len());
    fields.to_writer(writer, ())?;
    vec![0u8; padding].to_writer(writer, ())
}

/// Write n zeroes to the buffer
///
/// # Errors
///
/// Will return a `DekuError` if the writer fails
pub fn write_zeros<W: no_std_io::Write + no_std_io::Seek>(
    writer: &mut Writer<W>,
    count: usize,
) -> Result<(), DekuError> {
    vec![0u8; count].to_writer(writer, ())
}

#[must_use]
pub const fn padding_to(len: usize, align: usize) -> usize {
    len.next_multiple_of(align).saturating_sub(len)
}

/// Skip n bytes in the buffer
///
/// # Errors
///
/// Will return a `DekuError` if `from_reader_with_ctx` fails
pub fn skip<R: no_std_io::Read + no_std_io::Seek>(
    reader: &mut Reader<R>,
    count: usize,
) -> Result<(), DekuError> {
    for _ in 0..count {
        u8::from_reader_with_ctx(reader, ())?;
    }
    Ok(())
}

/// Read n bytes from buffer
///
/// # Errors
///
/// Will returnu a `DekuError` if the number of bytes provided exceeds the max size of
/// a PDU
pub fn read_n<'a, T, R>(
    reader: &mut Reader<R>,
    count: usize,
    endian: deku::ctx::Endian,
) -> Result<Vec<T>, DekuError>
where
    T: DekuReader<'a, deku::ctx::Endian>,
    R: no_std_io::Read + no_std_io::Seek,
{
    if !fits_in_pdu(count) {
        return Err(DekuError::Assertion(
            format!("{count} values can't fit in a PDU").into(),
        ));
    }
    (0..count)
        .map(|_| T::from_reader_with_ctx(reader, endian))
        .collect()
}

macro_rules! impl_deku_default_endian {
    ($name:ty) => {
        impl<'a> deku::DekuReader<'a, ()> for $name {
            fn from_reader_with_ctx<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
                reader: &mut deku::reader::Reader<R>,
                (): (),
            ) -> Result<Self, deku::DekuError> {
                <Self as deku::DekuReader<'a, deku::ctx::Endian>>::from_reader_with_ctx(
                    reader,
                    deku::ctx::Endian::Big,
                )
            }
        }

        impl deku::DekuWriter<()> for $name {
            fn to_writer<W: deku::no_std_io::Write + deku::no_std_io::Seek>(
                &self,
                writer: &mut deku::writer::Writer<W>,
                (): (),
            ) -> Result<(), deku::DekuError> {
                <Self as deku::DekuWriter<deku::ctx::Endian>>::to_writer(
                    self,
                    writer,
                    deku::ctx::Endian::Big,
                )
            }
        }
    };
}

pub(crate) use impl_deku_default_endian;
