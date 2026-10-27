//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BytesMut};
use chrono::{DateTime, Timelike, Utc};

use crate::common::{
    GenericHeader, SerializedLength,
    enums::{DISLiveEntitySubprotocolNumber, PduType, ProtocolFamily, ProtocolVersion},
};

const MICROS_PER_HOUR: u64 = 3_600_000_000;
const DIS_UNITS_PER_HOUR: u64 = 2_147_483_648;

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
    pub exercise_id: u8,
    /// Type of PDU, unique for each PDU class
    pub pdu_type: PduType,
    /// Value that refers to the protocol family
    pub protocol_family: ProtocolFamily,
    /// Timestamp value
    pub timestamp: u32,
    /// Length, in bytes, of the PDU
    pub length: u16,
    /// The subprotocol to be used to decode the PDU
    pub subprotocol_number: DISLiveEntitySubprotocolNumber,
    /// Padded so that this header maintains the same length as the PDU Header
    padding: u8,
}

impl Default for LiveEntityPduHeader {
    fn default() -> Self {
        Self {
            protocol_version: ProtocolVersion::IEEE1278_1_2012,
            exercise_id: 1,
            pdu_type: PduType::default(),
            protocol_family: ProtocolFamily::default(),
            timestamp: Self::calculate_dis_timestamp(),
            length: 0,
            subprotocol_number: DISLiveEntitySubprotocolNumber::default(),
            padding: 0,
        }
    }
}

impl GenericHeader for LiveEntityPduHeader {
    fn pdu_type(&self) -> PduType {
        self.pdu_type
    }

    fn set_pdu_type(&mut self, value: PduType) {
        self.pdu_type = value;
    }

    fn protocol_family(&self) -> ProtocolFamily {
        self.protocol_family
    }

    fn set_protocol_family(&mut self, value: ProtocolFamily) {
        self.protocol_family = value;
    }

    fn length(&self) -> u16 {
        self.length
    }

    fn set_length(&mut self, value: u16) {
        self.length = value;
    }

    fn serialize(&self, buf: &mut BytesMut) {
        let _ = crate::common::wire::write(self, buf);
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, crate::common::DISError> {
        Ok(crate::common::wire::read(buf)?)
    }
}

fn micros_past_hour(now: &DateTime<Utc>) -> Option<u64> {
    u64::from(now.minute())
        .checked_mul(60_000_000)?
        .checked_add(u64::from(now.second()).checked_mul(1_000_000)?)?
        .checked_add(u64::from(now.timestamp_subsec_micros()))
}

impl LiveEntityPduHeader {
    #[must_use]
    pub fn new(
        pdu_type: PduType,
        protocol_family: ProtocolFamily,
        exercise_id: u8,
        length: u16,
    ) -> Self {
        Self {
            protocol_version: ProtocolVersion::IEEE1278_1_2012,
            exercise_id,
            pdu_type,
            protocol_family,
            timestamp: Self::calculate_dis_timestamp(),
            length,
            ..Default::default()
        }
    }

    /// Gets the current time in terms of IEEE-1278.1 DIS time units
    #[must_use]
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn calculate_dis_timestamp() -> u32 {
        let now = Utc::now();

        micros_past_hour(&now)
            .and_then(|us| us.checked_mul(DIS_UNITS_PER_HOUR))
            .and_then(|v| v.checked_div(MICROS_PER_HOUR))
            .and_then(|v| u32::try_from(v).ok())
            .and_then(|v| v.checked_mul(2))
            .map_or(u32::MAX, |v| v | 1)
    }
}

impl SerializedLength for LiveEntityPduHeader {
    const LENGTH: usize = 12;
}
