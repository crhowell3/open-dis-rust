// src/pdu_macro.rs
// A macro system for generating PDUs with trait-based serialization/length/deserialize.
// Place this file at crate root and `pub mod pdu_macro;` in lib.rs.
// Use `use crate::define_pdu;` in PDU modules.
//
// NOTE: This file purposely avoids depending on your specific crate layout.
// You will need to implement/adapt trait impls for your custom types
// (EntityId, enums, headers, etc.) to interoperate with this system.

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

// ------ Implementations for primitive types ------

macro_rules! impl_primitive {
    ($ty:ty, $put:ident, $get:ident, $len:expr) => {
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
                $len
            }
        }
    };
}

impl_primitive!(u8, put_u8, get_u8, 1usize);
impl_primitive!(i8, put_i8, get_i8, 1usize);
impl_primitive!(u16, put_u16, get_u16, 2usize);
impl_primitive!(i16, put_i16, get_i16, 2usize);
impl_primitive!(u32, put_u32, get_u32, 4usize);
impl_primitive!(i32, put_i32, get_i32, 4usize);
impl_primitive!(u64, put_u64, get_u64, 8usize);
impl_primitive!(i64, put_i64, get_i64, 8usize);
impl_primitive!(f32, put_f32, get_f32, 4usize);
impl_primitive!(f64, put_f64, get_f64, 8usize);

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
                    $fvis:vis $field:ident : $ftype:ty,
                )*
            }

        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            header: $header,
            $(
                $fvis $field : $ftype,
            )*
        }

        // Default impl using Default::default for header/fields (requires those impls)
        impl Default for $name {
            fn default() -> Self {
                Self {
                    header: <$header>::default(),
                    $(
                        $field: <$ftype>::default(),
                    )*
                }
            }
        }

        // Body deserializer generated in terms of FieldDeserialize
        impl $name {
            /// Deserialize only the body (fields), leaving header defaulted.
            /// Note: for variable-length arrays/strings the generated code will call
            /// `FieldDeserialize::deserialize_field()`, but for real variable-length fields
            /// you should write custom code in the manual body impl below or adapt the macro.
            fn deserialize_body<B: bytes::Buf>(buf: &mut B) -> Self {
                $(
                    // Each field is constructed via the FieldDeserialize trait.
                    let $field: $ftype = <$ftype as $crate::pdu_macro::FieldDeserialize>::deserialize_field(buf);
                )*

                Self {
                    header: <$header>::default(),
                    $(
                        $field,
                    )*
                }
            }
        }

        // Implement the Pdu trait (your crate's Pdu trait path may differ; adapt the path)
        impl $crate::common::pdu::Pdu for $name {
            type Header = $header;

            fn calculate_length(&self) -> Result<u16, $crate::common::dis_error::DISError> {
                // Start with header length const; requires header::LENGTH const
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
                // set header fields
                self.header.set_pdu_type($pdu_type);
                self.header.set_protocol_family($protocol_family);

                // compute length the correct way and set it
                let len = self.calculate_length()?;
                self.header.set_length(len);

                // write header
                self.header.serialize(buf);

                // serialize each field
                $(
                    <$ftype as $crate::pdu_macro::FieldSerialize>::serialize_field(&self.$field, buf);
                )*

                Ok(())
            }

            fn deserialize<B: bytes::Buf>(buf: &mut B) -> Result<Self, $crate::common::dis_error::DISError>
            where Self: Sized {
                // deserialize header using its associated function
                let header: Self::Header = <Self::Header as $crate::pdu_macro::FieldDeserialize>::deserialize_field(buf);

                // check PDU type (assumes header exposes pdu_type() method; adapt if different)
                if header.pdu_type() != $pdu_type {
                    return Err($crate::common::dis_error::DISError::invalid_header(
                        format!("Expected PDU type {:?}, got {:?}", $pdu_type, header.pdu_type()),
                        None,
                    ));
                }

                // read body fields with the generated deserializer
                let mut body = Self::deserialize_body(buf);
                body.header = header;
                Ok(body)
            }

            fn deserialize_without_header<B: bytes::Buf>(buf: &mut B, header: Self::Header) -> Result<Self, $crate::common::dis_error::DISError>
            where Self: Sized {
                let mut body = Self::deserialize_body(buf);
                body.header = header;
                Ok(body)
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        // Provide a convenience constructor (matching your existing API)
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
