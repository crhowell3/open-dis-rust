//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    enums::{DISLiveEntitySubprotocolNumber, PduType, ProtocolFamily, ProtocolVersion},
    records::{ExerciseIdentifier, Timestamp},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct LiveEntityPduHeader {
    /// The version of the protocol
    pub protocol_version: ProtocolVersion,
    /// Exercise ID
    pub exercise_id: ExerciseIdentifier,
    /// Type of PDU set from the PDU when it is encoded. Read it with
    /// [`pdu_type`](Sef::pdu_type).
    pub(crate) pdu_type: PduType,
    /// Protocol family set from the PDU when it is encoded. Read it with
    /// [`protocol_family`](Sef::protocol_family).
    pub(crate) protocol_family: ProtocolFamily,
    /// When the data in the PDU was valid. New headers use [`Timestamp::now`].
    pub timestamp: Timestamp,
    /// Length of the PDU in octets, including the header. This value is set
    /// upon encoding. Read it with [`length`](Sef::length).
    pub(crate) length: u16,
    /// The subprotocol to be used to decode the PDU
    pub subprotocol_number: DISLiveEntitySubprotocolNumber,
    /// Padded so that this header maintains the same length as the PDU Header
    padding: u8,
}

impl Default for LiveEntityPduHeader {
    fn default() -> Self {
        Self {
            protocol_version: ProtocolVersion::IEEE1278_1_2012,
            exercise_id: ExerciseIdentifier::new(1),
            pdu_type: PduType::default(),
            protocol_family: ProtocolFamily::default(),
            timestamp: Timestamp::now(),
            length: 0,
            subprotocol_number: DISLiveEntitySubprotocolNumber::default(),
            padding: 0,
        }
    }
}

impl LiveEntityPduHeader {
    #[must_use]
    pub const fn pdu_type(&self) -> PduType {
        self.pdu_type
    }

    /// The protocol family, set from the PDU when it is encoded.
    #[must_use]
    pub const fn protocol_family(&self) -> ProtocolFamily {
        self.protocol_family
    }

    /// The PDU's length in octets, header included, set when it is encoded.
    #[must_use]
    pub const fn length(&self) -> u16 {
        self.length
    }

    /// Encodes the header alone and appends it to `buf`
    pub fn serialize(&self, buf: &mut Vec<u8>) {
        let _ = crate::common::wire::write(self, buf);
    }

    /// Decodes a header from the front of `buf`, advancing `buf` past it. Pass it to
    /// [`Pdu::deserialize_without_header`](crate::common::Pdu::deserialize_without_header)
    /// to decode the rest of the PDU.
    ///
    /// # Errors
    ///
    /// Will return a `DekuError` if `buf` does not start with a valid header.
    pub fn deserialize(buf: &mut &[u8]) -> Result<Self, crate::common::DISError> {
        crate::common::wire::read(buf).map_err(crate::common::DISError::from_wire)
    }
}

impl crate::common::pdu::DerivedFields for LiveEntityPduHeader {
    #[cfg(test)]
    fn length(&self) -> u16 {
        self.length
    }

    fn set_pdu_type(&mut self, value: PduType) {
        self.pdu_type = value;
    }

    fn set_protocol_family(&mut self, value: ProtocolFamily) {
        self.protocol_family = value;
    }

    fn set_length(&mut self, value: u16) {
        self.length = value;
    }
}
