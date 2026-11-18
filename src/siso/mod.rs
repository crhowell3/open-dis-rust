//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! SISO-REF-010-2023, the enumerations and bit layouts IEEE 1278.1 refers to by UID.

pub mod appearance;
pub mod bit_records;
pub mod enums;

/// Implements deku's reader and writer for a fieldless enum that derives
/// `FromPrimitive`, encoding it as its discriminant in `$repr`.
///
/// Decoding a value the enum doesn't define is an error. This is written by
/// hand rather than derived because a derived reader for the largest enums
/// (thousands of variants) needs megabytes of stack in debug builds.
macro_rules! impl_deku_enum {
    ($name:ident, $repr:ty, $from:ident) => {
        impl<'a> deku::DekuReader<'a, deku::ctx::Endian> for $name {
            fn from_reader_with_ctx<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
                reader: &mut deku::reader::Reader<R>,
                endian: deku::ctx::Endian,
            ) -> Result<Self, deku::DekuError> {
                let value =
                    <$repr as deku::DekuReader<'a, deku::ctx::Endian>>::from_reader_with_ctx(
                        reader, endian,
                    )?;
                <Self as FromPrimitive>::$from(value).ok_or_else(|| {
                    deku::DekuError::Parse(
                        format!("{value} is not a defined {} value", stringify!($name)).into(),
                    )
                })
            }
        }

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

        impl deku::DekuWriter<deku::ctx::Endian> for $name {
            fn to_writer<W: deku::no_std_io::Write + deku::no_std_io::Seek>(
                &self,
                writer: &mut deku::writer::Writer<W>,
                endian: deku::ctx::Endian,
            ) -> Result<(), deku::DekuError> {
                // Lossless: a fieldless enum cast to the type its values fit in.
                #[allow(clippy::as_conversions)]
                let value = *self as $repr;
                value.to_writer(writer, endian)
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

pub(crate) use impl_deku_enum;
