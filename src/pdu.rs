//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use super::{
    constants::PDU_HEADER_SIZE, dis_error::DISError, live_entity_records::LiveEntityPduHeader,
    records::PduHeader,
};
use crate::siso::enums::{PduType, ProtocolFamily};
use std::{any::Any, fmt::Debug};

/// A DIS Protocol Data Unit
///
/// Something that can be encoded to and decoded from the bytes of the
/// DIS wire format
pub trait Pdu {
    /// The PDU's header: [`PduHeader`] for most PDUs, or [`LiveEntityPduHeader`] for the
    /// Live Entity Information/Interaction family. [`StandardPdu`] and [`LiveEntityPdu`]
    /// represent the two variants.
    type Header: Copy + Debug + Default + PartialEq + Eq;

    /// The PDU's type. It is fixed for each PDU struct, and `serialize` writes it into
    /// the header.
    fn pdu_type(&self) -> PduType;

    /// The PDU's protocol family. It is fixed for each PDU struct, and `serialize` writes
    /// it into the header.
    fn protocol_family(&self) -> ProtocolFamily;

    /// The PDU's total encoded length in octets, header included
    /// # Errors
    ///
    /// Will return `DISError` if the calculated PDU length is greater than the maximum allowed size
    fn calculate_length(&self) -> Result<u16, DISError>;

    /// The PDU's header.
    fn header(&self) -> &Self::Header;
    /// A mutable reference to the PDU's header to change its exercise ID, timestamp, or status.
    fn header_mut(&mut self) -> &mut Self::Header;

    /// Runs the PDU's update rules and sets the header's length without encoding it.
    /// Errors are ignored here; [`serialize`](Pdu::serialize) reports them.
    fn finalize(&mut self);

    /// Encodes the PDU and appends it to `buf`
    ///
    /// Before encoding, this sets the header's PDU type, protocol family, and length
    /// along with every count, length, and flag field that depends on the PDU's contents.
    /// Anything already in `buf` is kept, so several PDUs can be written into one buffer.
    ///
    /// # Errors
    ///
    /// Will return `DISError` if serialization fails, especially if the dynamically calculated PDU
    /// length is greater than the maximum allowed size
    fn serialize(&mut self, buf: &mut Vec<u8>) -> Result<(), DISError>;

    /// Encodes the PDU into a new buffer
    ///
    /// # Errors
    ///
    /// Will return `DISError` in the same case as [`serialize`](Pdu::serialize).
    fn to_bytes(&mut self) -> Result<Vec<u8>, DISError> {
        let mut buf = Vec::new();
        self.serialize(&mut buf)?;
        Ok(buf)
    }

    /// Decodes one PDU from the front of `buf`, advancing `buf` past it
    ///
    /// Exactly the number of octets given by the header's length field
    /// are consumed, so anything after them, such as another bundled PDU,
    /// is left in `buf`.
    ///
    /// # Errors
    ///
    /// Will return `DISError` if `buf` does not start with a valid PDU
    fn deserialize(buf: &mut &[u8]) -> Result<Self, DISError>
    where
        Self: Sized;

    /// Decodes a PDU body from the front of `buf` using a header that has
    /// already been deserialized.
    ///
    /// # Errors
    ///
    /// Will return `DISError` if the PDU header provided is invalid
    fn deserialize_without_header(
        buf: &mut &[u8],
        pdu_header: Self::Header,
    ) -> Result<Self, DISError>
    where
        Self: Sized;

    /// The PDU as [`Any`] to downcast a `dyn Pdu` to its concrete type.
    fn as_any(&self) -> &dyn Any;
}

/// A PDU with the standard 12-octet [`PduHeader`], which carries a [`PduStatus`](crate::records::PduStatus) record.
/// Every PDU outside the Live Entity Information/Interaction family is one.
///
/// It is implemented for every `Pdu<Header = PduHeader>`, so it is a shorter way to write
/// that bound:
///
/// ```
/// use open_dis_rust::prelude::*;
///
/// fn set_exercise(pdu: &mut impl StandardPdu, exercise_id: ExerciseIdentifier) {
///     pdu.header_mut().exercise_id = exercise_id;
/// }
/// ```
pub trait StandardPdu: Pdu<Header = PduHeader> {}

impl<T: Pdu<Header = PduHeader> + ?Sized> StandardPdu for T {}

/// A PDU of the Live Entity Information/Interaction family, whose
/// [`LiveEntityPduHeader`] has a subprotocol number where other PDUs have
/// their PDU Status record.
///
/// It is implemented for every `Pdu<Header = LiveEntityPduHeader>`. A function that takes
/// one does not accept standard PDUs:
///
/// ```compile_fail
/// use open_dis_rust::{entity_information::EntityStatePdu, prelude::*};
///
/// fn subprotocol(pdu: &impl LiveEntityPdu) {}
/// subprotocol(&EntityStatePdu::new());
/// ```
pub trait LiveEntityPdu: Pdu<Header = LiveEntityPduHeader> {}

impl<T: Pdu<Header = LiveEntityPduHeader> + ?Sized> LiveEntityPdu for T {}

/// The header fields that encoding sets from the PDU. Private to the crate so that
/// those fields stay read-only for users.
pub trait DerivedFields {
    #[cfg(test)]
    fn length(&self) -> u16;
    fn set_pdu_type(&mut self, value: PduType);
    fn set_protocol_family(&mut self, value: ProtocolFamily);
    fn set_length(&mut self, value: u16);
}

pub fn total_length(header_len: usize, body_len: usize) -> Result<u16, DISError> {
    use crate::constants::MAX_PDU_SIZE_OCTETS;
    let size = header_len.saturating_add(body_len);
    u16::try_from(size)
        .ok()
        .filter(|_| size <= MAX_PDU_SIZE_OCTETS)
        .ok_or(DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })
}

/// Encodes `header` followed by `body` onto the end of `buf`, then rewrites the
/// header with the PDU's real length. `buf` is restored if anything fails.
pub fn write_pdu<H, B>(header: &mut H, body: &B, buf: &mut Vec<u8>) -> Result<(), DISError>
where
    H: DerivedFields + deku::DekuWriter<()>,
    B: deku::DekuWriter<()>,
{
    let start = buf.len();
    let result = (|| {
        crate::wire::write(&*header, buf).map_err(DISError::from_wire)?;
        crate::wire::write(body, buf).map_err(DISError::from_wire)?;
        let body_len = buf
            .len()
            .saturating_sub(start)
            .saturating_sub(PDU_HEADER_SIZE);
        let length = total_length(PDU_HEADER_SIZE, body_len)?;
        header.set_length(length);

        let mut encoded = Vec::with_capacity(PDU_HEADER_SIZE);
        crate::wire::write(&*header, &mut encoded).map_err(DISError::from_wire)?;
        buf.get_mut(start..start.saturating_add(encoded.len()))
            .ok_or_else(|| DISError::serialization_error("PDU header was not written", None))?
            .copy_from_slice(&encoded);
        Ok(())
    })();
    if result.is_err() {
        buf.truncate(start);
    }
    result
}

pub fn take_body<'a>(
    buf: &mut &'a [u8],
    declared_len: u16,
    header_len: usize,
) -> Result<&'a [u8], DISError> {
    let body_len = usize::from(declared_len)
        .checked_sub(header_len)
        .ok_or_else(|| {
            DISError::invalid_header(
                format!("PDU length {declared_len} is shorter than its {header_len}-octet header"),
                None,
            )
        })?;
    let (body, rest) = buf
        .split_at_checked(body_len)
        .ok_or_else(|| DISError::buffer_underflow(body_len, buf.len()))?;
    *buf = rest;
    Ok(body)
}

pub fn check_body_used(body: &[u8], declared_len: u16) -> Result<(), DISError> {
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

macro_rules! impl_pdu {
    ($name:ty, $header:ty, $pdu_type:expr, $family:expr $(,)?) => {
        impl $crate::pdu::Pdu for $name {
            type Header = $header;

            fn pdu_type(&self) -> $crate::siso::enums::PduType {
                $pdu_type
            }

            fn protocol_family(&self) -> $crate::siso::enums::ProtocolFamily {
                $family
            }

            fn calculate_length(&self) -> Result<u16, $crate::DISError> {
                let body = $crate::wire::encoded_len(self).map_err($crate::DISError::from_wire)?;
                $crate::pdu::total_length($crate::constants::PDU_HEADER_SIZE, body)
            }

            fn header(&self) -> &Self::Header {
                &self.header
            }

            fn header_mut(&mut self) -> &mut Self::Header {
                &mut self.header
            }

            /// Run the update rules and set the header length. Errors are
            /// ignored here; `serialize` reports them.
            fn finalize(&mut self) {
                let _ = deku::DekuUpdate::update(self);
                let length = self.calculate_length().unwrap_or_default();
                $crate::pdu::DerivedFields::set_length(&mut self.header, length);
            }

            fn serialize(&mut self, buf: &mut Vec<u8>) -> Result<(), $crate::DISError> {
                $crate::pdu::DerivedFields::set_pdu_type(&mut self.header, $pdu_type);
                $crate::pdu::DerivedFields::set_protocol_family(&mut self.header, $family);
                deku::DekuUpdate::update(self).map_err($crate::DISError::from_wire)?;
                let mut header = self.header;
                $crate::pdu::write_pdu(&mut header, &*self, buf)?;
                self.header = header;
                Ok(())
            }

            fn deserialize(buf: &mut &[u8]) -> Result<Self, $crate::DISError>
            where
                Self: Sized,
            {
                let header: $header =
                    $crate::wire::read(buf).map_err($crate::DISError::from_wire)?;
                let found = header.pdu_type();
                if found != $pdu_type {
                    return Err($crate::DISError::invalid_header(
                        format!("expected PDU type {:?}, got {found:?}", $pdu_type),
                        None,
                    ));
                }
                Self::deserialize_without_header(buf, header)
            }

            fn deserialize_without_header(
                buf: &mut &[u8],
                header: Self::Header,
            ) -> Result<Self, $crate::DISError>
            where
                Self: Sized,
            {
                let declared = header.length();
                let mut body =
                    $crate::pdu::take_body(buf, declared, $crate::constants::PDU_HEADER_SIZE)?;
                let mut pdu: Self =
                    $crate::wire::read(&mut body).map_err($crate::DISError::from_wire)?;
                $crate::pdu::check_body_used(body, declared)?;
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
                $crate::pdu::DerivedFields::set_pdu_type(&mut pdu.header, $pdu_type);
                $crate::pdu::DerivedFields::set_protocol_family(&mut pdu.header, $family);
                $crate::pdu::Pdu::finalize(&mut pdu);
                pdu
            }
        }
    };
}

#[cfg(test)]
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
                assert_eq!(pdu.header().pdu_type(), $pdu_type);
                assert_eq!(pdu.pdu_type(), $pdu_type);
            }

            #[test]
            fn new_sets_default_length() {
                const DEFAULT_LENGTH: u16 = $default_bits / $crate::wire::BITS_PER_BYTE;
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

                let mut buf: &[u8] = &original;
                let mut decoded = <$pdu>::deserialize(&mut buf).expect("deserialize failed");

                assert!(buf.is_empty(), "deserialize left bytes unread");
                assert_eq!(decoded.header, pdu.header);
                assert_eq!(to_bytes(&mut decoded), original);
            }

            #[test]
            fn rejects_wrong_pdu_type() {
                let mut other = <$other>::new();
                let bytes = to_bytes(&mut other);
                assert!(<$pdu>::deserialize(&mut bytes.as_slice()).is_err());
            }
        }
    };
}

pub(crate) use impl_pdu;
#[cfg(test)]
pub(crate) use pdu_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        LiveEntityPdu, StandardPdu,
        entity_information::EntityStatePdu,
        live_entity_information::TimeSpacePositionInformationPdu,
        records::{ExerciseIdentifier, VariableDatumRecord},
        simulation_management::{AcknowledgePdu, CommentPdu},
        siso::enums::PduType,
    };

    fn exercise_of(pdu: &impl StandardPdu) -> ExerciseIdentifier {
        pdu.header().exercise_id
    }

    fn subprotocol_of(
        pdu: &impl LiveEntityPdu,
    ) -> crate::siso::enums::DISLiveEntitySubprotocolNumber {
        pdu.header().subprotocol_number
    }

    #[test]
    fn pdus_are_told_apart_by_their_header() {
        assert_eq!(
            exercise_of(&EntityStatePdu::new()),
            ExerciseIdentifier::new(1)
        );
        let tspi = TimeSpacePositionInformationPdu::new();
        assert_eq!(subprotocol_of(&tspi), tspi.header().subprotocol_number);
    }

    #[test]
    fn standard_pdus_can_be_trait_objects() {
        let mut pdus: Vec<Box<dyn StandardPdu>> = vec![
            Box::new(EntityStatePdu::new()),
            Box::new(AcknowledgePdu::new()),
        ];
        let types: Vec<PduType> = pdus.iter().map(|pdu| pdu.pdu_type()).collect();
        assert_eq!(types, [PduType::EntityState, PduType::Acknowledge]);
        let mut buf = Vec::new();
        for pdu in &mut pdus {
            pdu.serialize(&mut buf).unwrap();
        }
        assert_eq!(buf.len(), 144 + 32);
    }

    #[test]
    fn pdu_type_and_family_are_known_before_encoding() {
        let pdu = TimeSpacePositionInformationPdu::default();
        assert_eq!(pdu.pdu_type(), PduType::TimeSpacePositionInformation);
        assert_eq!(
            pdu.protocol_family(),
            crate::siso::enums::ProtocolFamily::LiveEntityInformationInteraction
        );
    }

    #[test]
    fn serialize_appends_to_existing_contents() {
        let mut buf = vec![0xAA, 0xBB];
        let mut pdu = AcknowledgePdu::new();
        pdu.serialize(&mut buf).unwrap();
        assert_eq!(buf.get(..2), Some(&[0xAA, 0xBB][..]));
        assert_eq!(buf.len(), 2 + usize::from(pdu.header().length));
        assert_eq!(buf.get(2..), Some(pdu.to_bytes().unwrap().as_slice()));
    }

    #[test]
    fn serialize_leaves_buffer_unchanged_on_error() {
        let mut pdu = CommentPdu::new();
        for i in 0..9 {
            pdu.variable_datum_records.push(VariableDatumRecord {
                datum_id: i,
                length_bits: 8000,
                value: vec![0; 1000],
            });
        }
        let mut buf = vec![1, 2, 3];
        assert!(matches!(
            pdu.serialize(&mut buf),
            Err(DISError::PduSizeExceeded { .. })
        ));
        assert_eq!(buf, [1, 2, 3]);
    }

    #[test]
    fn deserialize_reads_one_pdu_and_leaves_the_rest() {
        let mut buf = Vec::new();
        EntityStatePdu::new().serialize(&mut buf).unwrap();
        AcknowledgePdu::new().serialize(&mut buf).unwrap();

        let mut rest = buf.as_slice();
        EntityStatePdu::deserialize(&mut rest).unwrap();
        assert_eq!(rest.len(), 32);
        AcknowledgePdu::deserialize(&mut rest).unwrap();
        assert!(rest.is_empty());
    }

    #[test]
    fn wire_format_error_describes_the_problem() {
        let mut bytes = AcknowledgePdu::new().to_bytes().unwrap();
        if let Some(version) = bytes.first_mut() {
            *version = 99;
        }
        let err = AcknowledgePdu::deserialize(&mut bytes.as_slice()).unwrap_err();
        let DISError::WireFormat(inner) = &err else {
            panic!("expected a wire format error, got {err:?}");
        };
        assert!(
            err.to_string()
                .contains("99 is not a defined ProtocolVersion value")
        );
        assert!(
            inner
                .to_string()
                .contains("99 is not a defined ProtocolVersion value")
        );
        // The message is complete on its own; nothing is repeated as a source.
        assert!(std::error::Error::source(inner).is_none());
    }
}
