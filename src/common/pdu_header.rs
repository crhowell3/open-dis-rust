//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#![allow(clippy::must_use_candidate)]

use bytes::{Buf, BufMut, BytesMut};
use chrono::{Timelike, Utc};
use modular_bitfield::prelude::*;

use crate::{
    common::{
        GenericHeader, SerializedLength,
        enums::{
            ActiveInterrogationIndicator, CoupledExtensionIndicator, DetonationTypeIndicator,
            FireTypeIndicator, IntercomAttachedIndicator, LVCIndicator, PduStatusIFFSimulationMode,
            PduType, ProtocolFamily, ProtocolVersion, RadioAttachedIndicator,
            TransferredEntityIndicator,
        },
    },
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

#[bitfield(bits = 8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PduStatusRecord {
    pub tei: TransferredEntityIndicator,
    pub lvc: LVCIndicator,
    pub cei: CoupledExtensionIndicator,
    pub bit4_5: B2,
    #[skip]
    __reserved: B2,
}

impl PduStatusRecord {
    #[must_use]
    pub const fn new_zero() -> Self {
        Self::new()
    }

    #[must_use]
    pub fn get_dti(&self) -> DetonationTypeIndicator {
        DetonationTypeIndicator::from(self.bit4_5())
    }

    pub fn set_dti(&mut self, dti: DetonationTypeIndicator) {
        self.set_bit4_5(dti.into());
    }

    #[must_use]
    pub fn get_rai(&self) -> RadioAttachedIndicator {
        RadioAttachedIndicator::from(self.bit4_5())
    }

    pub fn set_rai(&mut self, rai: RadioAttachedIndicator) {
        self.set_bit4_5(rai.into());
    }

    #[must_use]
    pub fn get_iai(&self) -> IntercomAttachedIndicator {
        IntercomAttachedIndicator::from(self.bit4_5())
    }

    pub fn set_iai(&mut self, iai: IntercomAttachedIndicator) {
        self.set_bit4_5(iai.into());
    }

    #[must_use]
    pub fn get_fti(&self) -> FireTypeIndicator {
        let bit4 = self.bit4_5() & 0b01;
        if bit4 == 0 {
            FireTypeIndicator::Munition
        } else {
            FireTypeIndicator::Expendable
        }
    }

    pub fn set_fti(&mut self, fti: FireTypeIndicator) {
        let mut v = self.bit4_5();
        v = (v & 0b10) | (fti as u8 & 0x01);
        self.set_bit4_5(v);
    }

    #[must_use]
    pub fn get_ism(&self) -> PduStatusIFFSimulationMode {
        let bit4 = self.bit4_5() & 0b01;
        if bit4 == 0 {
            PduStatusIFFSimulationMode::Regeneration
        } else {
            PduStatusIFFSimulationMode::Interactive
        }
    }

    pub fn set_ism(&mut self, ism: PduStatusIFFSimulationMode) {
        let mut v = self.bit4_5();
        v = (v & 0b10) | (ism as u8 & 0x01);
        self.set_bit4_5(v);
    }

    #[must_use]
    pub fn get_aii(&self) -> ActiveInterrogationIndicator {
        let bit5 = (self.bit4_5() >> 1) & 0b01;
        if bit5 == 0 {
            ActiveInterrogationIndicator::NotActive
        } else {
            ActiveInterrogationIndicator::Active
        }
    }

    pub fn set_aii(&mut self, aii: ActiveInterrogationIndicator) {
        let mut v = self.bit4_5();
        v = (v & 0b01) | ((aii as u8) << 1);
        self.set_bit4_5(v);
    }

    #[must_use]
    pub const fn to_u8(&self) -> u8 {
        self.into_bytes()[0]
    }

    #[must_use]
    pub const fn from_u8(b: u8) -> Self {
        Self::from_bytes([b])
    }
}

impl Default for PduStatusRecord {
    fn default() -> Self {
        Self::new_zero()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PduHeader {
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
    /// PDU status record
    pub status_record: PduStatusRecord,
    /// Padding
    padding: u8,
}

impl Default for PduHeader {
    fn default() -> Self {
        Self {
            protocol_version: ProtocolVersion::IEEE1278_1_2012,
            exercise_id: 1,
            pdu_type: PduType::default(),
            protocol_family: ProtocolFamily::default(),
            timestamp: Self::calculate_dis_timestamp(),
            length: 0,
            status_record: PduStatusRecord::default(),
            padding: 0,
        }
    }
}

impl GenericHeader for PduHeader {
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
        buf.put_u8(self.status_record.to_u8());
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
            status_record: PduStatusRecord::from_u8(buf.get_u8()),
            padding: buf.get_u8(),
        }
    }
}

impl PduHeader {
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
            status_record: PduStatusRecord::default(),
            padding: 0,
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

impl FieldSerialize for PduHeader {
    fn serialize_field(&self, buf: &mut BytesMut) {
        self.serialize(buf);
    }
}

impl FieldDeserialize for PduHeader {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize(buf)
    }
}

impl FieldLen for PduHeader {
    fn field_len(&self) -> usize {
        Self::LENGTH
    }
}

impl SerializedLength for PduHeader {
    const LENGTH: usize = 12;
}
