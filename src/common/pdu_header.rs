//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#![allow(clippy::must_use_candidate)]

use bytes::{Buf, BufMut, BytesMut};
use chrono::{Timelike, Utc};
use modular_bitfield::prelude::*;

use crate::common::{
    SerializedLength,
    enums::{
        ActiveInterrogationIndicator, CoupledExtensionIndicator, DetonationTypeIndicator,
        FireTypeIndicator, IntercomAttachedIndicator, LVCIndicator, PduStatusIFFSimulationMode,
        PduType, ProtocolFamily, ProtocolVersion, RadioAttachedIndicator,
        TransferredEntityIndicator,
    },
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
    pub fn new_zero() -> Self {
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
    pub fn to_u8(&self) -> u8 {
        self.into_bytes()[0]
    }

    #[must_use]
    pub fn from_u8(b: u8) -> Self {
        Self::from_bytes([b])
    }
}

impl Default for PduStatusRecord {
    fn default() -> Self {
        Self::new_zero()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
}

impl Default for PduHeader {
    fn default() -> Self {
        PduHeader {
            protocol_version: ProtocolVersion::IEEE1278_1_2012,
            exercise_id: 1,
            pdu_type: PduType::default(),
            protocol_family: ProtocolFamily::default(),
            timestamp: PduHeader::calculate_dis_timestamp(),
            length: 0_u16,
            status_record: PduStatusRecord::default(),
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
        PduHeader {
            protocol_version: ProtocolVersion::IEEE1278_1_2012,
            exercise_id,
            pdu_type,
            protocol_family,
            timestamp: PduHeader::calculate_dis_timestamp(),
            length,
            status_record: PduStatusRecord::default(),
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

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.protocol_version as u8);
        buf.put_u8(self.exercise_id);
        buf.put_u8(self.pdu_type as u8);
        buf.put_u8(self.protocol_family as u8);
        buf.put_u32(self.timestamp);
        buf.put_u16(self.length);
        buf.put_u8(self.status_record.to_u8());
    }

    fn deserialize_protocol_version(data: u8) -> ProtocolVersion {
        match data {
            1 => ProtocolVersion::DIS_PDUv1,
            2 => ProtocolVersion::IEEE1278_1993,
            3 => ProtocolVersion::DIS_PDUv2_Third_Draft,
            4 => ProtocolVersion::DIS_PDUv2_Fourth_Draft_Revised,
            5 => ProtocolVersion::IEEE1278_1_1995,
            6 => ProtocolVersion::IEEE1278_1A_1998,
            7 => ProtocolVersion::IEEE1278_1_2012,
            _ => ProtocolVersion::Other,
        }
    }

    pub fn deserialize<B: Buf>(buf: &mut B) -> PduHeader {
        PduHeader {
            protocol_version: PduHeader::deserialize_protocol_version(buf.get_u8()),
            exercise_id: buf.get_u8(),
            pdu_type: PduHeader::deserialize_pdu_type(buf.get_u8()),
            protocol_family: PduHeader::deserialize_protocol_family(buf.get_u8()),
            timestamp: buf.get_u32(),
            length: buf.get_u16(),
            status_record: PduStatusRecord::from_u8(buf.get_u8()),
        }
    }

    #[must_use]
    pub fn deserialize_pdu_type(data: u8) -> PduType {
        match data {
            1 => PduType::EntityState,
            2 => PduType::Fire,
            3 => PduType::Detonation,
            4 => PduType::Collision,
            5 => PduType::ServiceRequest,
            6 => PduType::ResupplyOffer,
            7 => PduType::ResupplyReceived,
            8 => PduType::ResupplyCancel,
            9 => PduType::RepairComplete,
            10 => PduType::RepairResponse,
            11 => PduType::CreateEntity,
            12 => PduType::RemoveEntity,
            13 => PduType::StartResume,
            14 => PduType::StopFreeze,
            15 => PduType::Acknowledge,
            16 => PduType::ActionRequest,
            17 => PduType::ActionResponse,
            18 => PduType::DataQuery,
            19 => PduType::SetData,
            20 => PduType::Data,
            21 => PduType::EventReport,
            22 => PduType::Comment,
            23 => PduType::ElectromagneticEmission,
            24 => PduType::Designator,
            25 => PduType::Transmitter,
            26 => PduType::Signal,
            27 => PduType::Receiver,
            28 => PduType::IFF,
            29 => PduType::UnderwaterAcoustic,
            30 => PduType::SupplementalEmission,
            31 => PduType::IntercomSignal,
            32 => PduType::IntercomControl,
            33 => PduType::AggregateState,
            34 => PduType::IsGroupOf,
            35 => PduType::TransferOwnership,
            36 => PduType::IsPartOf,
            37 => PduType::MinefieldState,
            38 => PduType::MinefieldQuery,
            39 => PduType::MinefieldData,
            40 => PduType::MinefieldResponseNack,
            41 => PduType::EnvironmentalProcess,
            42 => PduType::GriddedData,
            43 => PduType::PointObjectState,
            44 => PduType::LinearObjectState,
            45 => PduType::ArealObjectState,
            46 => PduType::TSPI,
            47 => PduType::Appearance,
            48 => PduType::ArticulatedParts,
            49 => PduType::LEFire,
            50 => PduType::LEDetonation,
            51 => PduType::CreateEntityReliable,
            52 => PduType::RemoveEntityReliable,
            53 => PduType::StartResumeReliable,
            54 => PduType::StopFreezeReliable,
            55 => PduType::AcknowledgeReliable,
            56 => PduType::ActionRequestReliable,
            57 => PduType::ActionResponseReliable,
            58 => PduType::DataQueryReliable,
            59 => PduType::SetDataReliable,
            60 => PduType::DataReliable,
            61 => PduType::EventReportReliable,
            62 => PduType::CommentReliable,
            63 => PduType::RecordReliable,
            64 => PduType::SetRecordReliable,
            65 => PduType::RecordQueryReliable,
            66 => PduType::CollisionElastic,
            67 => PduType::EntityStateUpdate,
            68 => PduType::DirectedEnergyFire,
            69 => PduType::EntityDamageStatus,
            70 => PduType::InformationOperationsAction,
            71 => PduType::InformationOperationsReport,
            72 => PduType::Attribute,
            _ => PduType::Other,
        }
    }

    #[must_use]
    fn deserialize_protocol_family(data: u8) -> ProtocolFamily {
        match data {
            1 => ProtocolFamily::EntityInformation,
            2 => ProtocolFamily::Warfare,
            3 => ProtocolFamily::Logistics,
            4 => ProtocolFamily::RadioCommunications,
            5 => ProtocolFamily::SimulationManagement,
            6 => ProtocolFamily::DistributedEmissionRegeneration,
            7 => ProtocolFamily::EntityManagement,
            8 => ProtocolFamily::Minefield,
            9 => ProtocolFamily::SyntheticEnvironment,
            10 => ProtocolFamily::SimulationManagementWithReliability,
            11 => ProtocolFamily::LiveEntityInformationInteraction,
            12 => ProtocolFamily::NonRealTime,
            13 => ProtocolFamily::InformationOperations,
            _ => ProtocolFamily::Other,
        }
    }
}

impl SerializedLength for PduHeader {
    const LENGTH: usize = 12;
}
