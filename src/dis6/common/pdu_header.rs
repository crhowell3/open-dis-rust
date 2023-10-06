//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation 
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{BytesMut, BufMut, Buf};
use chrono::{Utc, Timelike};
use num_derive::FromPrimitive;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PduHeader {
    // The version of the protocol
    pub protocol_version: u8,
    // Exercise ID
    pub exercise_id: u8,
    // Type of PDU, unique for each PDU class
    pub pdu_type: u8,
    // Value that refers to the protocol family
    pub protocol_family: u8,
    // Timestamp value
    pub timestamp: u32,
    // Length, in bytes, of the PDU
    pub length: u16,
    // Zero-filled array of padding
    pub padding: u16,
}

impl PduHeader {
    pub fn new(pdu_type: PduType, protocol_family: ProtocolFamily, exercise_id: u8, length: u16)
        -> Self {
        PduHeader {
            protocol_version,
            exercise_id,
            pdu_type,
            protocol_family,
            timestamp,
            length: length as u16,
            padding: 0 as u16
        }        
    }

    pub fn default(pdu_type: PduType, protocol_family: ProtocolFamily, length: u16) -> Self {
        PduHeader {
            protocol_version,
            exercise_id: 1,
            pdu_type,
            protocol_family,
            timestamp: PduHeader::calculate_dis_timestamp() as u32,
            length: length as u16,
            padding: 0 as u16
        }
    }

    /// Gets the current time in terms of IEEE-1278.1 DIS time units
    pub fn calculate_dis_timestamp() -> u32 {
        let minute_curr = ((Utc::now().minute() * 60) * 1e6 as u32) as u64;
        let second_curr = (Utc::now().second() * 1e6 as u32) as u64;
        let nanosecond_curr = (Utc::now().nanosecond() / 1000) as u64;
        let dis_time = (second_curr + minute_curr + nanosecond_curr) as f32 / 1.68;
        dis_time as u32
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u8(self.protocol_version as u8);
        buf.put_u8(self.exercise_id as u8);
        buf.put_u8(self.pdu_type as u8);
        buf.put_u8(self.protocol_family as u8);
        buf.put_u32(self.timestamp as u32);
        buf.put_u16(self.length as u16);
        buf.put_u16(self.padding as u16);
    }

    pub fn decode_pdu_type(data: u8) -> PduType {
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
            _ => PduType::Other
        }
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq)]
pub enum ProtocolFamily {
    Other = 0,
    EntityInformation = 1,
    Warfare = 2,
    Logistics = 3,
    RadioCommunications = 4,
    SimulationManagement = 5,
    DistributedEmissionRegeneration = 6
}

#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq)]
pub enum PduType {
    Other = 0,
    EntityState = 1,
    Fire = 2,
    Detonation = 3,
    Collision = 4,
    ServiceRequest = 5,
    ResupplyOffer = 6,
    ResupplyReceived = 7,
    ResupplyCancel = 8,
    RepairComplete = 9,
    RepairResponse = 10,
    CreateEntity = 11,
    RemoveEntity = 12,
    StartResume = 13,
    StopFreeze = 14,
    Acknowledge = 15,
    ActionRequest = 16,
    ActionResponse = 17,
    DataQuery = 18,
    SetData = 19,
    Data = 20,
    EventReport = 21,
    Comment = 22,
    ElectromagneticEmission = 23,
    Designator = 24,
    Transmitter = 25,
    Signal = 26,
    Receiver = 27,
    IFF = 28,
    UnderwaterAcoustic = 29,
    SupplementalEmission = 30,
    IntercomSignal = 31,
    IntercomControl = 32,
    AggregateState = 33,
    IsGroupOf = 34,
    TransferOwnership = 35,
    IsPartOf = 36,
    MinefieldState = 37,
    MinefieldQuery = 38,
    MinefieldData = 39,
    MinefieldResponseNack = 40,
    EnvironmentalProcess = 41,
    GriddedData = 42,
    PointObjectState = 43,
    LinearObjectState = 44,
    ArealObjectState = 45,
    TSPI = 46,
    Appearance = 47,
    ArticulatedParts = 48,
    LEFire = 49,
    LEDetonation = 50,
    CreateEntityReliable = 51,
    RemoveEntityReliable = 52,
    StartResumeReliable = 53,
    StopFreezeReliable = 54,
    AcknowledgeReliable = 55,
    ActionRequestReliable = 56,
    ActionResponseReliable = 57,
    DataQueryReliable = 58,
    SetDataReliable = 59,
    DataReliable = 60,
    EventReportReliable = 61,
    CommentReliable = 62,
    RecordReliable = 63,
    SetRecordReliable = 64,
    RecordQueryReliable = 65,
    CollisionElastic = 66,
    EntityStateUpdate = 67,
    DirectedEnergyFire = 68,
    EntityDamageStatus = 69,
    InformationOperationsAction = 70,
    InformationOperationsReport = 71,
    Attribute = 72
}
