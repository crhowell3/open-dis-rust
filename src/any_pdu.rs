//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use std::ops::Range;

use num_traits::FromPrimitive;

use crate::{
    DISError, DISResult,
    constants::PDU_HEADER_SIZE,
    pdu::Pdu,
    records::ExerciseIdentifier,
    siso::enums::{PduType, ProtocolFamily},
};

/// The header octet that holds the PDU type, in both kinds of PDU header
const PDU_TYPE_OCTET: usize = 2;
/// The header octets that hold the PDU's length, in both kinds of PDU header
const LENGTH_OCTETS: Range<usize> = 8..10;

/// Defines `AnyPdu` with one variant per PDU, named after its `PduType` variant.
macro_rules! any_pdu {
    ($($variant:ident => $module:ident::$pdu:ident,)+) => {
        /// Any PDU, for decoding datagrams whose PDU type isn't known in advance
        ///
        /// Each variant holds one PDU and is named after its [`PduType`]. Decode with
        /// [`AnyPdu::deserialize`], or [`AnyPdu::deserialize_all`] for a datagram that
        /// may hold several PDUs, then `match` on the variants you handle:
        ///
        /// ```
        /// use open_dis_rust::{prelude::*, simulation_management::AcknowledgePdu};
        ///
        /// # fn main() -> Result<(), DISError> {
        /// let datagram = AcknowledgePdu::new().to_bytes()?;
        ///
        /// match AnyPdu::deserialize(&mut datagram.as_slice())? {
        ///     AnyPdu::Acknowledge(ack) => println!("acknowledged request {}", ack.request_id),
        ///     AnyPdu::EntityState(pdu) => println!("entity {:?}", pdu.entity_id),
        ///     other => println!("ignoring {:?} PDU", other.pdu_type()),
        /// }
        /// # Ok(())
        /// # }
        /// ```
        ///
        /// Every PDU converts into an `AnyPdu` with `From`, so PDUs of different
        /// types can be kept in one collection and sent with [`AnyPdu::serialize`].
        ///
        /// New variants may be added in minor releases, for example for a later
        /// version of the standard, so a `match` needs a wildcard arm.
        #[derive(Debug)]
        #[non_exhaustive]
        pub enum AnyPdu {
            $(
                #[doc = concat!(
                    "[`", stringify!($pdu), "`](crate::", stringify!($module), "::",
                    stringify!($pdu), ")"
                )]
                $variant(crate::$module::$pdu),
            )+
        }

        impl AnyPdu {
            fn deserialize_as(pdu_type: PduType, buf: &mut &[u8]) -> DISResult<Self> {
                match pdu_type {
                    $(
                        PduType::$variant => {
                            <crate::$module::$pdu as Pdu>::deserialize(buf).map(Self::$variant)
                        }
                    )+
                    PduType::Other => Err(DISError::invalid_header(
                        "PDU type 0 (Other) has no defined layout",
                        None,
                    )),
                }
            }

            /// The PDU's type
            #[must_use]
            pub fn pdu_type(&self) -> PduType {
                match self {
                    $(Self::$variant(pdu) => pdu.pdu_type(),)+
                }
            }

            /// The PDU's protocol family
            #[must_use]
            pub fn protocol_family(&self) -> ProtocolFamily {
                match self {
                    $(Self::$variant(pdu) => pdu.protocol_family(),)+
                }
            }

            /// The exercise the PDU belongs to, from its header
            #[must_use]
            pub fn exercise_id(&self) -> ExerciseIdentifier {
                match self {
                    $(Self::$variant(pdu) => pdu.header().exercise_id,)+
                }
            }

            /// Encodes the PDU and appends it to `buf`, as [`Pdu::serialize`] does.
            ///
            /// # Errors
            ///
            /// Will return `DISError` in the same cases as [`Pdu::serialize`].
            pub fn serialize(&mut self, buf: &mut Vec<u8>) -> DISResult<()> {
                match self {
                    $(Self::$variant(pdu) => pdu.serialize(buf),)+
                }
            }
        }

        $(
            impl From<crate::$module::$pdu> for AnyPdu {
                fn from(pdu: crate::$module::$pdu) -> Self {
                    Self::$variant(pdu)
                }
            }
        )+
    };
}

any_pdu! {
    EntityState => entity_information::EntityStatePdu,
    Fire => warfare::FirePdu,
    Detonation => warfare::DetonationPdu,
    Collision => entity_information::CollisionPdu,
    ServiceRequest => logistics::ServiceRequestPdu,
    ResupplyOffer => logistics::ResupplyOfferPdu,
    ResupplyReceived => logistics::ResupplyReceivedPdu,
    ResupplyCancel => logistics::ResupplyCancelPdu,
    RepairComplete => logistics::RepairCompletePdu,
    RepairResponse => logistics::RepairResponsePdu,
    CreateEntity => simulation_management::CreateEntityPdu,
    RemoveEntity => simulation_management::RemoveEntityPdu,
    StartResume => simulation_management::StartResumePdu,
    StopFreeze => simulation_management::StopFreezePdu,
    Acknowledge => simulation_management::AcknowledgePdu,
    ActionRequest => simulation_management::ActionRequestPdu,
    ActionResponse => simulation_management::ActionResponsePdu,
    DataQuery => simulation_management::DataQueryPdu,
    SetData => simulation_management::SetDataPdu,
    Data => simulation_management::DataPdu,
    EventReport => simulation_management::EventReportPdu,
    Comment => simulation_management::CommentPdu,
    ElectromagneticEmissions => distributed_emissions::ElectromagneticEmissionsPdu,
    Designator => distributed_emissions::DesignatorPdu,
    Transmitter => radio_communications::TransmitterPdu,
    Signal => radio_communications::SignalPdu,
    Receiver => radio_communications::ReceiverPdu,
    IFF => distributed_emissions::IFFPdu,
    UnderwaterAcoustic => distributed_emissions::UnderwaterAcousticPdu,
    SupplementalEmission => distributed_emissions::SupplementalEmissionPdu,
    IntercomSignal => radio_communications::IntercomSignalPdu,
    IntercomControl => radio_communications::IntercomControlPdu,
    AggregateState => entity_management::AggregateStatePdu,
    IsGroupOf => entity_management::IsGroupOfPdu,
    TransferOwnership => entity_management::TransferOwnershipPdu,
    IsPartOf => entity_management::IsPartOfPdu,
    MinefieldState => minefield::MinefieldStatePdu,
    MinefieldQuery => minefield::MinefieldQueryPdu,
    MinefieldData => minefield::MinefieldDataPdu,
    MinefieldResponseNack => minefield::MinefieldResponseNackPdu,
    EnvironmentalProcess => synthetic_environment::EnvironmentalProcessPdu,
    GriddedData => synthetic_environment::GriddedDataPdu,
    PointObjectState => synthetic_environment::PointObjectStatePdu,
    LinearObjectState => synthetic_environment::LinearObjectStatePdu,
    ArealObjectState => synthetic_environment::ArealObjectStatePdu,
    TimeSpacePositionInformation => live_entity_information::TimeSpacePositionInformationPdu,
    Appearance => live_entity_information::AppearancePdu,
    ArticulatedParts => live_entity_information::ArticulatedPartsPdu,
    LiveEntityFire => live_entity_information::LiveEntityFirePdu,
    LiveEntityDetonation => live_entity_information::LiveEntityDetonationPdu,
    CreateEntityReliable => simulation_management_with_reliability::CreateEntityReliablePdu,
    RemoveEntityReliable => simulation_management_with_reliability::RemoveEntityReliablePdu,
    StartResumeReliable => simulation_management_with_reliability::StartResumeReliablePdu,
    StopFreezeReliable => simulation_management_with_reliability::StopFreezeReliablePdu,
    AcknowledgeReliable => simulation_management_with_reliability::AcknowledgeReliablePdu,
    ActionRequestReliable => simulation_management_with_reliability::ActionRequestReliablePdu,
    ActionResponseReliable => simulation_management_with_reliability::ActionResponseReliablePdu,
    DataQueryReliable => simulation_management_with_reliability::DataQueryReliablePdu,
    SetDataReliable => simulation_management_with_reliability::SetDataReliablePdu,
    DataReliable => simulation_management_with_reliability::DataReliablePdu,
    EventReportReliable => simulation_management_with_reliability::EventReportReliablePdu,
    CommentReliable => simulation_management_with_reliability::CommentReliablePdu,
    RecordReliable => simulation_management_with_reliability::RecordReliablePdu,
    SetRecordReliable => simulation_management_with_reliability::SetRecordReliablePdu,
    RecordQueryReliable => simulation_management_with_reliability::RecordQueryReliablePdu,
    CollisionElastic => entity_information::CollisionElasticPdu,
    EntityStateUpdate => entity_information::EntityStateUpdatePdu,
    DirectedEnergyFire => warfare::DirectedEnergyFirePdu,
    EntityDamageStatus => warfare::EntityDamageStatusPdu,
    InformationOperationsAction => information_operations::InformationOperationsActionPdu,
    InformationOperationsReport => information_operations::InformationOperationsReportPdu,
    Attribute => entity_information::AttributePdu,
}

impl AnyPdu {
    /// Decodes one PDU of any type from the front of `buf`, advancing `buf` past it
    ///
    /// The PDU type in the header picks the PDU to decode. As with
    /// [`Pdu::deserialize`], exactly the number of octets given by the header's length
    /// field are consumed, so another bundled PDU after it is left in `buf`. On error,
    /// `buf` is left unchanged.
    ///
    /// # Errors
    ///
    /// Will return `DISError` if `buf` is shorter than a header, its PDU type is not a
    /// defined PDU type, or the rest is not a valid PDU of that type.
    pub fn deserialize(buf: &mut &[u8]) -> DISResult<Self> {
        let Some(&octet) = buf.get(PDU_TYPE_OCTET) else {
            return Err(DISError::buffer_underflow(PDU_HEADER_SIZE, buf.len()));
        };
        let pdu_type = PduType::from_u8(octet).ok_or_else(|| {
            DISError::invalid_header(format!("{octet} is not a defined PDU type"), None)
        })?;

        let mut rest = *buf;
        let pdu = Self::deserialize_as(pdu_type, &mut rest)?;
        *buf = rest;
        Ok(pdu)
    }

    /// Decodes every PDU in a datagram, which the standard allows to hold several
    ///
    /// Each item is the result of [`AnyPdu::deserialize`] on what is left of the datagram.
    /// When a PDU cannot be decoded, such as one that is malformed or of a PDU type the
    /// standard does not define, its error is yielded and decoding carries on after it if
    /// its header's length field fits in the datagram. Otherwise, the error is the last item.
    ///
    /// ```
    /// use open_dis_rust::{
    ///     prelude::*,
    ///     simulation_management::{AcknowledgePdu, CommentPdu},
    /// };
    ///
    /// # fn main() -> Result<(), DISError> {
    /// let mut datagram = Vec::new();
    /// AcknowledgePdu::new().serialize(&mut datagram)?;
    /// CommentPdu::new().serialize(&mut datagram)?;
    ///
    /// for pdu in AnyPdu::deserialize_all(&datagram) {
    ///     println!("{:?}", pdu?.pdu_type());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn deserialize_all(datagram: &[u8]) -> impl Iterator<Item = DISResult<Self>> + '_ {
        let mut rest = datagram;
        std::iter::from_fn(move || {
            if rest.is_empty() {
                return None;
            }
            let result = Self::deserialize(&mut rest);
            if result.is_err() {
                rest = skip_pdu(rest).unwrap_or_default();
            }
            Some(result)
        })
    }

    /// Encodes the PDU into a new buffer, as [`Pdu::to_bytes`] does.
    ///
    /// # Errors
    ///
    /// Will return `DISError` in the same cases as [`Pdu::serialize`].
    pub fn to_bytes(&mut self) -> DISResult<Vec<u8>> {
        let mut buf = Vec::new();
        self.serialize(&mut buf)?;
        Ok(buf)
    }
}

/// What follows the PDU at the front of `buf`, going by its header's length field, or
/// `None` if that length is shorter than a header or longer than `buf`.
fn skip_pdu(buf: &[u8]) -> Option<&[u8]> {
    let length = <[u8; 2]>::try_from(buf.get(LENGTH_OCTETS)?).ok()?;
    let length = usize::from(u16::from_be_bytes(length));
    if length < PDU_HEADER_SIZE {
        return None;
    }
    buf.get(length..)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::indexing_slicing)]

    use super::*;
    use crate::{
        entity_information::EntityStatePdu,
        live_entity_information::TimeSpacePositionInformationPdu,
        simulation_management::{AcknowledgePdu, CommentPdu},
    };

    fn bytes(pdu: impl Into<AnyPdu>) -> Vec<u8> {
        pdu.into().to_bytes().unwrap()
    }

    #[test]
    fn decodes_standard_and_live_entity_pdus() {
        let es = bytes(EntityStatePdu::new());
        let tspi = bytes(TimeSpacePositionInformationPdu::new());

        assert!(matches!(
            AnyPdu::deserialize(&mut es.as_slice()).unwrap(),
            AnyPdu::EntityState(_)
        ));
        assert!(matches!(
            AnyPdu::deserialize(&mut tspi.as_slice()).unwrap(),
            AnyPdu::TimeSpacePositionInformation(_)
        ));
    }

    #[test]
    fn leaves_the_next_pdu_in_the_buffer() {
        let mut datagram = bytes(AcknowledgePdu::new());
        let first = datagram.len();
        datagram.extend(bytes(CommentPdu::new()));

        let mut buf = datagram.as_slice();
        AnyPdu::deserialize(&mut buf).unwrap();
        assert_eq!(buf, &datagram[first..]);
    }

    #[test]
    fn errors_leave_the_buffer_unchanged() {
        let mut datagram = bytes(AcknowledgePdu::new());
        datagram.truncate(datagram.len() - 1);

        let mut buf = datagram.as_slice();
        assert!(AnyPdu::deserialize(&mut buf).is_err());
        assert_eq!(buf, datagram.as_slice());
    }

    #[test]
    fn rejects_undefined_and_other_pdu_types() {
        for pdu_type in [0, 73, 255] {
            let mut datagram = bytes(AcknowledgePdu::new());
            datagram[PDU_TYPE_OCTET] = pdu_type;
            assert!(matches!(
                AnyPdu::deserialize(&mut datagram.as_slice()),
                Err(DISError::InvalidHeader { .. })
            ));
        }
    }

    #[test]
    fn rejects_buffers_shorter_than_the_pdu_type() {
        assert!(matches!(
            AnyPdu::deserialize(&mut [7, 1].as_slice()),
            Err(DISError::BufferUnderflow { .. })
        ));
    }

    #[test]
    fn deserialize_all_reads_a_bundle() {
        let mut datagram = bytes(AcknowledgePdu::new());
        datagram.extend(bytes(EntityStatePdu::new()));
        datagram.extend(bytes(CommentPdu::new()));

        let types: Vec<PduType> = AnyPdu::deserialize_all(&datagram)
            .map(|pdu| pdu.unwrap().pdu_type())
            .collect();
        assert_eq!(
            types,
            [PduType::Acknowledge, PduType::EntityState, PduType::Comment]
        );
    }

    #[test]
    fn deserialize_all_skips_a_pdu_it_cannot_decode() {
        let mut bad = bytes(AcknowledgePdu::new());
        bad[PDU_TYPE_OCTET] = 200;
        let mut datagram = bad;
        datagram.extend(bytes(CommentPdu::new()));

        let results: Vec<_> = AnyPdu::deserialize_all(&datagram).collect();
        assert_eq!(results.len(), 2);
        assert!(results[0].is_err());
        assert!(matches!(results[1], Ok(AnyPdu::Comment(_))));
    }

    #[test]
    fn deserialize_all_stops_when_the_length_does_not_fit() {
        let mut datagram = bytes(AcknowledgePdu::new());
        datagram.extend(bytes(CommentPdu::new()));
        datagram.truncate(datagram.len() - 1);

        let results: Vec<_> = AnyPdu::deserialize_all(&datagram).collect();
        assert_eq!(results.len(), 2);
        assert!(results[0].is_ok());
        assert!(results[1].is_err());
    }

    #[test]
    fn deserialize_all_of_nothing_is_empty() {
        assert_eq!(AnyPdu::deserialize_all(&[]).count(), 0);
    }

    #[test]
    fn reports_the_header_fields() {
        let mut pdu = AcknowledgePdu::new();
        pdu.header_mut().exercise_id = ExerciseIdentifier::new(9);
        let any = AnyPdu::from(pdu);

        assert_eq!(any.pdu_type(), PduType::Acknowledge);
        assert_eq!(any.protocol_family(), ProtocolFamily::SimulationManagement);
        assert_eq!(any.exercise_id(), ExerciseIdentifier::new(9));
    }
}
