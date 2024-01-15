//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use num_derive::FromPrimitive;

// SISO-REF-010-2023 Protocol Version [UID 3]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ProtocolVersion {
    #[default]
    Other = 0,
    DIS_PDUv1 = 1,
    IEEE1278_1993 = 2,
    DIS_PDUv2_Third_Draft = 3,
    DIS_PDUv2_Fourth_Draft_Revised = 4,
    IEEE1278_1_1995 = 5,
    IEEE1278_1A_1998 = 6,
    IEEE1278_1_2012 = 7,
}

// SISO-REF-010-2023 PDU Type [UID 4]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PduType {
    #[default]
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
    Attribute = 72,
}

// SISO-REF-010-2023 Protocol Family [UID 5]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ProtocolFamily {
    #[default]
    Other = 0,
    EntityInformation = 1,
    Warfare = 2,
    Logistics = 3,
    RadioCommunications = 4,
    SimulationManagement = 5,
    DistributedEmissionRegeneration = 6,
    EntityManagement = 7,
    Minefield = 8,
    SyntheticEnvironment = 9,
    SimulationManagementWithReliability = 10,
    LiveEntityInformationInteraction = 11,
    NonRealTime = 12,
    InformationOperations = 13,
}

// SISO-REF-010-2023 Force ID [UID 6]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ForceId {
    #[default]
    Other = 0,
    Friendly = 1,
    Opposing = 2,
    Neutral = 3,
    Friendly2 = 4,
    Opposing2 = 5,
    Neutral2 = 6,
    Friendly3 = 7,
    Opposing3 = 8,
    Neutral3 = 9,
    Friendly4 = 10,
    Opposing4 = 11,
    Neutral4 = 12,
    Friendly5 = 13,
    Opposing5 = 14,
    Neutral5 = 15,
    Friendly6 = 16,
    Opposing6 = 17,
    Neutral6 = 18,
    Friendly7 = 19,
    Opposing7 = 20,
    Neutral7 = 21,
    Friendly8 = 22,
    Opposing8 = 23,
    Neutral8 = 24,
    Friendly9 = 25,
    Opposing9 = 26,
    Neutral9 = 27,
    Friendly10 = 28,
    Opposing10 = 29,
    Neutral10 = 30,
}

// SISO-REF-010-2023 Entity Kind [UID 7]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EntityKind {
    #[default]
    Other = 0,
    Platform = 1,
    Munition = 2,
    LifeForm = 3,
    Environmental = 4,
    CulturalFeature = 5,
    Supply = 6,
    Radio = 7,
    Expendable = 8,
    SensorEmitter = 9,
}

// SISO-REF-010-2023 Other Kinds [UID 8]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum OtherKinds {
    #[default]
    Other = 0,
    Land = 1,
    Air = 2,
    Surface = 3,
    Subsurface = 4,
    Space = 5,
}

// SISO-REF-010-2023 Land Domain Categories [UID 9]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LandDomainCategories {
    #[default]
    Other = 0,
    Tank = 1,
    ArmoredFightingVehicle,
    ArmoredUtilityVehicle,
    SelfPropelledArtillery,
    TowedArtillery,
    SmallWheeledUtilityVehicle,
    LargeWheeledUtilityVehicle,
    SmallTrackedUtilityVehicle,
    LargeTrackedUtilityVehicle,
    Mortar,
    MinePlow,
    MineRake,
    MineRoller,
    CargoTrailer,
    FuelTrailer,
    GeneratorTrailer,
    WaterTrailer,
    EngineerEquipment,
    HeavyEquipmentTransportTrailer,
    MaintenanceEquipmentTrailer,
    Limber,
    ChemicalDecontaminationTrailer,
    WarningSystem,
    TrainEngine,
    TrainCar,
    TrainCaboose,
    #[deprecated]
    CivilianVehicle,
    AirDefenseMissileDefenseUnitEquipment,
    C3ISystem,
    OperationsFacility,
    IntelligenceFacility,
    SurveillanceFacility,
    CommunicationsFacility,
    CommandFacility,
    C4IFacility,
    ControlFacility,
    FireControlFacility,
    MissileDefenseFacility,
    FieldCommandPost,
    ObservationPost,
    MineFlail = 41,
    Unmanned = 50,
    Motorcycle = 80,
    Car = 81,
    Bus,
    SingleUnitCargoTruck,
    SingleUnitUtilityEmergencyTruck,
    MultipleUnitCargoTruck,
    MultipleUnitUtilityEmergencyTruck,
    ConstructionSpecialtyVehicle,
    FarmSpecialtyVehicle,
    Trailer,
    Recreational,
    NonMotorized,
    Trains,
    UtilityEmergencyCar,
}

// SISO-REF-010-2023 Air Domain Categories [UID 10]
