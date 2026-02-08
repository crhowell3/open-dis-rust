//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use chrono::{Timelike, Utc};

use crate::{
    common::{
        GenericHeader, SerializedLength,
        enums::{DISLiveEntitySubprotocolNumber, PduType, ProtocolFamily, ProtocolVersion},
    },
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
        buf.put_u8(self.protocol_version as u8);
        buf.put_u8(self.exercise_id);
        buf.put_u8(self.pdu_type as u8);
        buf.put_u8(self.protocol_family as u8);
        buf.put_u32(self.timestamp);
        buf.put_u16(self.length);
        buf.put_u8(self.subprotocol_number as u8);
        buf.put_u8(self.padding);
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self {
            protocol_version: ProtocolVersion::deserialize(buf),
            exercise_id: buf.get_u8(),
            pdu_type: PduType::deserialize(buf),
            protocol_family: ProtocolFamily::deserialize(buf),
            timestamp: buf.get_u32(),
            length: buf.get_u16(),
            subprotocol_number: DISLiveEntitySubprotocolNumber::deserialize(buf),
            padding: buf.get_u8(),
        }
    }
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
        let minute_curr = u64::from((Utc::now().minute() * 60) * 1_000_000);
        let second_curr = u64::from(Utc::now().second() * 1_000_000);
        let nanosecond_curr = u64::from(Utc::now().nanosecond() / 1000);
        let dis_time = (second_curr + minute_curr + nanosecond_curr) as f32 / 1.68;
        dis_time as u32
    }
}

impl FieldSerialize for LiveEntityPduHeader {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for LiveEntityPduHeader {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for LiveEntityPduHeader {
    fn field_len(&self) -> usize {
        Self::LENGTH
    }
}

impl SerializedLength for LiveEntityPduHeader {
    const LENGTH: usize = 12;
}
