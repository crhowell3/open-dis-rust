//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::GenericHeader;

use super::dis_error::DISError;
use bytes::{Buf, BytesMut};
use std::any::Any;

pub trait Pdu {
    type Header: GenericHeader + Default;

    /// # Errors
    ///
    /// Will return `DISError` if the calculated PDU length is greater than the maximum allowed size
    fn calculate_length(&self) -> Result<u16, DISError>;

    fn header(&self) -> &Self::Header;
    fn header_mut(&mut self) -> &mut Self::Header;

    fn finalize(&mut self) {
        let len = self.calculate_length();
        self.header_mut().set_length(len.unwrap_or_default());
    }

    /// # Errors
    ///
    /// Will return `DISError` if serialization fails, especially if the dynamically calculated PDU
    /// length is greater than the maximum allowed size
    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError>;

    /// # Errors
    ///
    /// Will return `DISError` if the PDU header provided is invalid
    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized;
    /// # Errors
    ///
    /// Will return `DISError` if the PDU header provided is invalid
    fn deserialize_without_header<B: Buf>(
        buffer: &mut B,
        pdu_header: Self::Header,
    ) -> Result<Self, DISError>
    where
        Self: Sized;

    fn as_any(&self) -> &dyn Any;
}

#[doc(hidden)]
pub fn total_length(header_len: usize, body_len: usize) -> Result<u16, DISError> {
    use crate::common::constants::MAX_PDU_SIZE_OCTETS;
    let size = header_len.saturating_add(body_len);
    u16::try_from(size)
        .ok()
        .filter(|_| size <= MAX_PDU_SIZE_OCTETS)
        .ok_or(DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })
}

#[doc(hidden)]
pub fn take_body<B: Buf>(
    buf: &mut B,
    declared_len: u16,
    header_len: usize,
) -> Result<bytes::Bytes, DISError> {
    let body_len = usize::from(declared_len)
        .checked_sub(header_len)
        .ok_or_else(|| {
            DISError::invalid_header(
                format!("PDU length {declared_len} is shorter than its {header_len}-octet header"),
                None,
            )
        })?;
    if buf.remaining() < body_len {
        return Err(DISError::buffer_underflow(body_len, buf.remaining()));
    }
    Ok(buf.copy_to_bytes(body_len))
}

#[doc(hidden)]
pub fn check_body_used(body: &bytes::Bytes, declared_len: u16) -> Result<(), DISError> {
    if body.is_empty() {
        Ok(())
    } else {
        Err(DISError::deserialization_error(
            format!(
                "PDU length {declared_len} is {} octets longer than its fields",
                body.len()
            ),
            None,
        ))
    }
}

#[macro_export]
macro_rules! impl_pdu {
    ($name:ty, $header:ty, $pdu_type:expr, $family:expr $(,)?) => {
        impl $crate::common::pdu::Pdu for $name {
            type Header = $header;

            fn calculate_length(&self) -> Result<u16, $crate::common::DISError> {
                let body = $crate::common::wire::encoded_len(self)?;
                $crate::common::pdu::total_length(
                    <$header as $crate::common::SerializedLength>::LENGTH,
                    body,
                )
            }

            fn header(&self) -> &Self::Header {
                &self.header
            }

            fn header_mut(&mut self) -> &mut Self::Header {
                &mut self.header
            }

            /// Run the update rules and set the header length. Errors are
            /// ignored here; [`serialize`](Self::serialize) reports them.
            fn finalize(&mut self) {
                let _ = deku::DekuUpdate::update(self);
                let length = self.calculate_length().unwrap_or_default();
                $crate::common::GenericHeader::set_length(&mut self.header, length);
            }

            fn serialize(
                &mut self,
                buf: &mut bytes::BytesMut,
            ) -> Result<(), $crate::common::DISError> {
                $crate::common::GenericHeader::set_pdu_type(&mut self.header, $pdu_type);
                $crate::common::GenericHeader::set_protocol_family(&mut self.header, $family);
                deku::DekuUpdate::update(self)?;

                let mut body = bytes::BytesMut::new();
                $crate::common::wire::write(self, &mut body)?;
                let length = $crate::common::pdu::total_length(
                    <$header as $crate::common::SerializedLength>::LENGTH,
                    body.len(),
                )?;
                $crate::common::GenericHeader::set_length(&mut self.header, length);

                $crate::common::wire::write(&self.header, buf)?;
                buf.extend_from_slice(&body);
                Ok(())
            }

            fn deserialize<B: bytes::Buf>(buf: &mut B) -> Result<Self, $crate::common::DISError>
            where
                Self: Sized,
            {
                let header: $header = $crate::common::wire::read(buf)?;
                let found = $crate::common::GenericHeader::pdu_type(&header);
                if found != $pdu_type {
                    return Err($crate::common::DISError::invalid_header(
                        format!("expected PDU type {:?}, got {found:?}", $pdu_type),
                        None,
                    ));
                }
                Self::deserialize_without_header(buf, header)
            }

            fn deserialize_without_header<B: bytes::Buf>(
                buf: &mut B,
                header: Self::Header,
            ) -> Result<Self, $crate::common::DISError>
            where
                Self: Sized,
            {
                let declared = $crate::common::GenericHeader::length(&header);
                let mut body = $crate::common::pdu::take_body(
                    buf,
                    declared,
                    <$header as $crate::common::SerializedLength>::LENGTH,
                )?;
                let mut pdu: Self = $crate::common::wire::read(&mut body)?;
                $crate::common::pdu::check_body_used(&body, declared)?;
                pdu.header = header;
                Ok(pdu)
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl $name {
            /// A default PDU with its header's type, family and length set.
            #[must_use]
            pub fn new() -> Self {
                let mut pdu = Self::default();
                $crate::common::GenericHeader::set_pdu_type(&mut pdu.header, $pdu_type);
                $crate::common::GenericHeader::set_protocol_family(&mut pdu.header, $family);
                $crate::common::pdu::Pdu::finalize(&mut pdu);
                pdu
            }
        }
    };
}

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

            #[test]
            fn cast_to_any() {
                let pdu = <$pdu>::new();
                assert!(pdu.as_any().is::<$pdu>());
            }

            #[test]
            fn new_sets_pdu_type() {
                let pdu = <$pdu>::new();
                assert_eq!(
                    $crate::common::GenericHeader::pdu_type(pdu.header()),
                    $pdu_type
                );
            }

            #[test]
            fn new_sets_default_length() {
                const DEFAULT_LENGTH: u16 = $default_bits / BITS_PER_BYTE;
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

                assert_eq!(buf.remaining(), 0, "deserialize left bytes unread");
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
