//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! A macro system for generating PDUs with trait-based serialization/length/deserialize.
//! Place this file at crate root and `pub mod pdu_macro;` in lib.rs.
//! Use `use crate::define_pdu;` in PDU modules.

use bytes::{Buf, BufMut, BytesMut};

/// Serialize a single field into the buffer.
pub trait FieldSerialize {
    fn serialize_field(&self, buf: &mut BytesMut);
}

/// Deserialize a single field from the buffer.
pub trait FieldDeserialize: Sized {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self;
}

/// Return the serialized length of this field in bytes.
pub trait FieldLen {
    fn field_len(&self) -> usize;
}

/// Trait for types that can be deserialized given an externally-provided length.
/// Used by the macro when a field is annotated with `#[len = length_field_name]`.
pub trait FieldDeserializeWithLen: Sized {
    fn deserialize_with_len<B: Buf>(buf: &mut B, len: usize) -> Self;
}

// Blanket impl so `Option<T>` can be deserialized with an externally-provided length.
// This is the only path `#[len = ...]` Option fields take: a length of 0 means `None`.
impl<T> FieldDeserializeWithLen for Option<T>
where
    T: FieldDeserializeWithLen,
{
    fn deserialize_with_len<B: Buf>(buf: &mut B, len: usize) -> Self {
        if len == 0 {
            None
        } else {
            Some(<T as FieldDeserializeWithLen>::deserialize_with_len(
                buf, len,
            ))
        }
    }
}

// Helper macros for generated code. These are kept private to the macro expansion
// but exported so they can be used from the `define_pdu!` expansion.
//
// Note: `$t` arrives here as an already-parsed `ty` fragment, which can't be
// destructured (e.g. matched against `Option<$inner:ty>`). Option fields are
// handled through the trait impls instead (`FieldLen` returns 0 for `None`,
// and the blanket `FieldDeserializeWithLen` impl above maps a 0 length to `None`).
#[macro_export]
macro_rules! __pdu_prep_serialize_field {
    // Field has a length attribute: set the length field before length calculation.
    ( len = $len_field:ident ; $self:ident, $field:ident, $t:ty ) => {
        $self.$len_field = u8::try_from(<$t as $crate::pdu_macro::FieldLen>::field_len(
            &$self.$field,
        ))
        .unwrap_or_default();
    };

    // Default: no-op
    ( ; $self:ident, $field:ident, $t:ty ) => {};
}

#[macro_export]
macro_rules! __pdu_deserialize_field {
    // Field has a length attribute: read using FieldDeserializeWithLen.
    ( len = $len_field:ident ; $field:ident, $t:ty, $buf:ident ) => {
        let $field: $t = <$t as $crate::pdu_macro::FieldDeserializeWithLen>::deserialize_with_len(
            $buf,
            $len_field as usize,
        );
    };

    // Default: plain FieldDeserialize
    ( ; $field:ident, $t:ty, $buf:ident ) => {
        let $field: $t = <$t as $crate::pdu_macro::FieldDeserialize>::deserialize_field($buf);
    };
}

// ------ Implementations for primitive types ------

macro_rules! impl_primitive {
    ($ty:ty, $put:ident, $get:ident) => {
        impl FieldSerialize for $ty {
            fn serialize_field(&self, buf: &mut BytesMut) {
                buf.$put(*self);
            }
        }

        impl FieldDeserialize for $ty {
            fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
                buf.$get()
            }
        }

        impl FieldLen for $ty {
            fn field_len(&self) -> usize {
                std::mem::size_of::<$ty>()
            }
        }
    };
}

impl_primitive!(u8, put_u8, get_u8);
impl_primitive!(i8, put_i8, get_i8);
impl_primitive!(u16, put_u16, get_u16);
impl_primitive!(i16, put_i16, get_i16);
impl_primitive!(u32, put_u32, get_u32);
impl_primitive!(i32, put_i32, get_i32);
impl_primitive!(u64, put_u64, get_u64);
impl_primitive!(i64, put_i64, get_i64);
impl_primitive!(f32, put_f32, get_f32);
impl_primitive!(f64, put_f64, get_f64);

// String: serialized_len = bytes in UTF-8 (no extra length prefix; adapt if your PDU requires a length prefix)
impl FieldSerialize for String {
    fn serialize_field(&self, buf: &mut BytesMut) {
        buf.put_slice(self.as_bytes());
    }
}
impl FieldDeserialize for String {
    fn deserialize_field<B: Buf>(_buf: &mut B) -> Self {
        Self::new()
    }
}
impl FieldLen for String {
    fn field_len(&self) -> usize {
        self.len()
    }
}

// Vec<T>
impl<T> FieldSerialize for Vec<T>
where
    T: FieldSerialize,
{
    fn serialize_field(&self, buf: &mut BytesMut) {
        for item in self {
            item.serialize_field(buf);
        }
    }
}
impl<T> FieldDeserialize for Vec<T>
where
    T: FieldDeserialize,
{
    fn deserialize_field<B: Buf>(_buf: &mut B) -> Self {
        Self::new()
    }
}
impl<T> FieldLen for Vec<T>
where
    T: FieldLen,
{
    fn field_len(&self) -> usize {
        self.iter().map(FieldLen::field_len).sum()
    }
}

// Option<T>
impl<T> FieldSerialize for Option<T>
where
    T: FieldSerialize,
{
    fn serialize_field(&self, buf: &mut BytesMut) {
        if let Some(v) = self.as_ref() {
            v.serialize_field(buf);
        }
    }
}
impl<T> FieldDeserialize for Option<T>
where
    T: FieldDeserialize,
{
    fn deserialize_field<B: Buf>(_buf: &mut B) -> Self {
        None
    }
}
impl<T> FieldLen for Option<T>
where
    T: FieldLen,
{
    fn field_len(&self) -> usize {
        self.as_ref().map_or(0, FieldLen::field_len)
    }
}

/// Implements `FieldSerialize`, `FieldDeserialize` and `FieldLen` for a
/// fixed-layout record by serializing its fields in declaration order.
///
/// ```ignore
/// use crate::impl_record_fields;
///
/// pub struct LiveEventIdentifier { pub site_number: u8, pub application_number: u8, pub event_number: u16 }
/// impl_record_fields!(LiveEventIdentifier { site_number: u8, application_number: u8, event_number: u16 });
/// ```
#[macro_export]
macro_rules! impl_record_fields {
    ($name:ident { $($f:ident : $t:ty),* $(,)? }) => {
        impl $crate::pdu_macro::FieldSerialize for $name {
            fn serialize_field(&self, buf: &mut bytes::BytesMut) {
                $( <$t as $crate::pdu_macro::FieldSerialize>::serialize_field(&self.$f, buf); )*
            }
        }

        impl $crate::pdu_macro::FieldDeserialize for $name {
            fn deserialize_field<B: bytes::Buf>(buf: &mut B) -> Self {
                // Struct-expression fields are evaluated in source order,
                // so this reads the buffer in wire order.
                Self {
                    $( $f: <$t as $crate::pdu_macro::FieldDeserialize>::deserialize_field(buf), )*
                }
            }
        }

        impl $crate::pdu_macro::FieldLen for $name {
            fn field_len(&self) -> usize {
                0 $( + <$t as $crate::pdu_macro::FieldLen>::field_len(&self.$f) )*
            }
        }
    };
}

/// Implements `FieldSerialize`, `FieldDeserialize` and `FieldLen` for an
/// 8-bit `bitflags` type.
///
/// Unknown bits are retained on deserialize so that flags defined by newer
/// revisions of the standard round-trip unchanged.
#[macro_export]
macro_rules! impl_u8_flag_fields {
    ($name:ident) => {
        impl $crate::pdu_macro::FieldSerialize for $name {
            fn serialize_field(&self, buf: &mut bytes::BytesMut) {
                bytes::BufMut::put_u8(buf, self.bits());
            }
        }

        impl $crate::pdu_macro::FieldDeserialize for $name {
            fn deserialize_field<B: bytes::Buf>(buf: &mut B) -> Self {
                Self::from_bits_retain(bytes::Buf::get_u8(buf))
            }
        }

        impl $crate::pdu_macro::FieldLen for $name {
            fn field_len(&self) -> usize {
                1
            }
        }
    };
}

/// Generates the common test set for a PDU.
///
/// - `pdu_type`: the `PduType` the PDU must report.
/// - `default_bits`: length of a default PDU in bits, per the standard.
/// - `wrong_type`: any other PDU, used to check type mismatches are rejected.
///
/// The generated module is self-contained: it brings its own serialize helper
/// and constants, so the calling test module only needs the PDU types in scope.
#[macro_export]
macro_rules! pdu_tests {
    (
        $mod_name:ident, $pdu:ty,
        pdu_type: $pdu_type:expr,
        default_bits: $default_bits:expr,
        wrong_type: $other:ty $(,)?
    ) => {
        mod $mod_name {
            use super::*;

            fn to_bytes<P: $crate::common::pdu::Pdu>(pdu: &mut P) -> bytes::Bytes {
                let mut buf = bytes::BytesMut::new();
                pdu.serialize(&mut buf).expect("serialize failed");
                buf.freeze()
            }

            #[test]
            fn cast_to_any() {
                let pdu = <$pdu>::new();
                assert!(pdu.as_any().is::<$pdu>());
            }

            #[test]
            fn new_sets_pdu_type() {
                let pdu = <$pdu>::new();
                assert_eq!(pdu.header().pdu_type(), $pdu_type);
            }

            #[test]
            fn new_sets_default_length() {
                const DEFAULT_LENGTH: u16 =
                    $default_bits / $crate::common::constants::BITS_PER_BYTE;
                let pdu = <$pdu>::new();
                assert_eq!(pdu.header().length, DEFAULT_LENGTH);
            }

            #[test]
            fn serialized_length_matches_header() {
                let mut pdu = <$pdu>::new();
                let bytes = to_bytes(&mut pdu);
                assert_eq!(bytes.len(), usize::from(pdu.header().length));
            }

            #[test]
            fn round_trip_is_byte_identical() {
                let mut pdu = <$pdu>::new();
                let original = to_bytes(&mut pdu);

                let mut buf = original.clone();
                let mut decoded = <$pdu>::deserialize(&mut buf).expect("deserialize failed");

                assert_eq!(
                    bytes::Buf::remaining(&buf),
                    0,
                    "deserialize left bytes unread"
                );
                assert_eq!(decoded.header, pdu.header);
                assert_eq!(to_bytes(&mut decoded), original);
            }

            #[test]
            fn rejects_wrong_pdu_type() {
                let mut other = <$other>::new();
                let mut buf = to_bytes(&mut other);
                assert!(<$pdu>::deserialize(&mut buf).is_err());
            }
        }
    };
}

#[macro_export]
macro_rules! define_pdu {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            header: $header:ty,
            pdu_type: $pdu_type:expr,
            protocol_family: $protocol_family:expr,
            fields: {
                $(
                    $(#[len = $len_field:ident])? $fvis:vis $field:ident : $ftype:ty,
                )*
            }

        }
    ) => {
        $(#[$meta])*
        #[derive(Default)]
        $vis struct $name {
            header: $header,
            $(
                $fvis $field : $ftype,
            )*
        }

        impl $name {
            /// Deserialize the body (fields) that follows an already-read header.
            ///
            /// Fields are read into locals first so that a `#[len = foo]` field
            /// can refer to the already-read `foo`.
            fn deserialize_body<B: bytes::Buf>(buf: &mut B, header: $header) -> Self {
                $(
                    $crate::__pdu_deserialize_field!( $( len = $len_field )? ; $field, $ftype, buf );
                )*

                Self {
                    header,
                    $(
                        $field,
                    )*
                }
            }
        }

        impl $crate::common::pdu::Pdu for $name {
            type Header = $header;

            fn calculate_length(&self) -> Result<u16, $crate::common::dis_error::DISError> {
                // Requires a `LENGTH` const on the header type.
                let mut len: usize = <$header>::LENGTH;

                $(
                    len += <$ftype as $crate::pdu_macro::FieldLen>::field_len(&self.$field);
                )*

                u16::try_from(len).map_err(|_| $crate::common::dis_error::DISError::PduSizeExceeded {
                    size: len,
                    max_size: $crate::common::constants::MAX_PDU_SIZE_OCTETS,
                })
            }

            fn header(&self) -> &Self::Header {
                &self.header
            }

            fn header_mut(&mut self) -> &mut Self::Header {
                &mut self.header
            }

            fn serialize(&mut self, buf: &mut bytes::BytesMut) -> Result<(), $crate::common::dis_error::DISError> {
                self.header.set_pdu_type($pdu_type);
                self.header.set_protocol_family($protocol_family);

                // Fields annotated `#[len = foo]` update `self.foo` before the
                // overall PDU length is computed.
                $( $crate::__pdu_prep_serialize_field!( $( len = $len_field )? ; self, $field, $ftype ); )*

                let len = self.calculate_length()?;
                self.header.set_length(len);

                self.header.serialize(buf);

                $(
                    <$ftype as $crate::pdu_macro::FieldSerialize>::serialize_field(&self.$field, buf);
                )*

                Ok(())
            }

            fn deserialize<B: bytes::Buf>(buf: &mut B) -> Result<Self, $crate::common::dis_error::DISError>
            where Self: Sized {
                let header: Self::Header = <Self::Header as $crate::pdu_macro::FieldDeserialize>::deserialize_field(buf);

                if header.pdu_type() != $pdu_type {
                    return Err($crate::common::dis_error::DISError::invalid_header(
                        format!("Expected PDU type {:?}, got {:?}", $pdu_type, header.pdu_type()),
                        None,
                    ));
                }

                Ok(Self::deserialize_body(buf, header))
            }

            fn deserialize_without_header<B: bytes::Buf>(buf: &mut B, header: Self::Header) -> Result<Self, $crate::common::dis_error::DISError>
            where Self: Sized {
                Ok(Self::deserialize_body(buf, header))
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                let mut pdu = Self::default();
                pdu.header.set_pdu_type($pdu_type);
                pdu.header.set_protocol_family($protocol_family);
                pdu.finalize();
                pdu
            }
        }
    };
}
