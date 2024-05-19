//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use num_derive::FromPrimitive;

// SISO-REF-010-2023 Protocol Version [UID 3]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ProtocolVersion {
    Other = 0,
    DIS_PDUv1 = 1,
    IEEE1278_1993 = 2,
    DIS_PDUv2_Third_Draft = 3,
    DIS_PDUv2_Fourth_Draft_Revised = 4,
    IEEE1278_1_1995 = 5,
    IEEE1278_1A_1998 = 6,
    #[default]
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
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AirDomainCategories {
    #[default]
    Other = 0,
    FighterAirDefense = 1,
    AttackStrike = 2,
    Bomber = 3,
    CargoTanker = 4,
    ASWPatrolObservation = 5,
    ElectronicWarfare = 6,
    Reconnaissance = 7,
    SurveillanceC2 = 8,
    AirSeaRescue = 9,
    AttackHelicopter = 20,
    UtilityHelicopter = 21,
    AntiSubmarineWarfarePatrolHelicopter = 22,
    CargoHelicopter = 23,
    ObservationHelicopter = 24,
    SpecialOperationsHelicopter = 25,
    TrainingHelicopter = 26,
    Trainer = 40,
    Unmanned = 50,
    NonCombatantCommercialAircraft = 57,
    CivilianUltralightAircraftNonrigidWing = 80,
    CivilianUltralightAircraftRigidWing = 81,
    CivilianFixedWingAircraftGlider = 83,
    CivilianFixedWingAircraftLightSport = 84,
    CivilianFixedWingAircraftSmall = 85,
    CivilianFixedWingAircraftMedium = 86,
    CivilianFixedWingAircraftLarge = 87,
    CivilianFixedWingAircraftHeavy = 88,
    CivilianHelicopterSmall = 90,
    CivilianHelicopterMedium = 91,
    CivilianHelicopterLarge = 92,
    CivilianAutogyro = 93,
    CivilianLighterthanAirBalloon = 100,
    CivilianLighterthanAirAirship = 101,
}

// SISO-REF-010-2023 Surface Domain Categories [UID 11]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SurfaceDomainCategories {
    #[default]
    Other = 0,
    Carrier = 1,
    CommandShipCruiser = 2,
    GuidedMissileCruiser = 3,
    GuidedMissileDestroyer = 4,
    Destroyer = 5,
    GuidedMissileFrigate = 6,
    LightPatrolCraft = 7,
    MineCountermeasureShipCraft = 8,
    DockLandingShip = 9,
    TankLandingShip = 10,
    LandingCraft = 11,
    LightCarrier = 12,
    CruiserHelicopterCarrier = 13,
    Hydrofoil = 14,
    AirCushionSurfaceEffect = 15,
    Auxiliary = 16,
    AuxiliaryMerchantMarine = 17,
    Utility = 18,
    UnmannedSurfaceVehicle = 19,
    LittoralCombatShips = 20,
    SurveillanceShip = 21,
    Frigate = 50,
    Battleship = 51,
    HeavyCruiser = 52,
    DestroyerTender = 53,
    AmphibiousAssaultShip = 54,
    AmphibiousCargoShip = 55,
    AmphibiousTransportDock = 56,
    AmmunitionShip = 57,
    CombatStoresShip = 58,
    SurveillanceTowedArraySonarSystem = 59,
    FastCombatSupportShip = 60,
    NonCombatantShip = 61,
    CoastGuardCutters = 62,
    CoastGuardBoats = 63,
    FastAttackCraft = 64,
    InflatableBoat = 65,
    PassengerVessel = 80,
    DryCargoShip = 81,
    Tanker = 82,
    OffshoreSupportVessel = 83,
    PrivateMotorboat = 84,
    PrivateSailboat = 85,
    FishingVessel = 86,
    OtherVessels = 87,
    SearchandRescueVessels = 100,
    LifeSavingEquipment = 101,
}

// SISO-REF-010-2023 Subsurface Domain Categories [UID 12]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubsurfaceDomainCategories {
    #[default]
    Other = 0,
    SSBN = 1,
    SSGN = 2,
    SSN = 3,
    SSG = 4,
    SS = 5,
    SSAN = 6,
    SSA = 7,
    UnmannedUnderwaterVehicle = 8,
    SSB = 9,
    SSC = 10,
    SSP = 11,
    SSM = 12,
    SSNR = 13,
    SST = 14,
    AGSS = 15,
    SemiSubmersibleBoats = 16,
    CivilianSubmarines = 80,
    CivilianSubmersibles = 81,
    CivilianSemiSubmersibleBoats = 82,
}

// SISO-REF-010-2023 SpaceDomainCategories [UID 13]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SpaceDomainCategories {
    #[default]
    Other = 0,
    MannedSpacecraft = 1,
    Unmanned = 2,
    Booster = 3,
    Debris = 10,
    SatelliteUnknownUnspecifiedMission = 11,
    SatelliteCommunication = 12,
    SatelliteNavigation = 13,
    SatelliteScienceExperimentalDemonstration = 14,
    SatelliteInert = 15,
    SatelliteEarthObservation = 16,
    SatelliteSpaceSurveillance = 17,
    SatelliteAstronomy = 18,
}

// SISO-REF-010-2023 MunitionKind [UID 14]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MunitionKind {
    #[default]
    Other = 0,
    AntiAir = 1,
    AntiArmor = 2,
    AntiGuidedWeapon = 3,
    AntiRadar = 4,
    AntiSatellite = 5,
    AntiShip = 6,
    AntiSubmarine = 7,
    AntiPersonnel = 8,
    BattlefieldSupport = 9,
    Strategic = 10,
    Tactical = 11,
    DirectedEnergyWeapon = 12,
}

// SISO-REF-010-2023 MunitionCategory [UID 15]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MunitionCategory {
    #[default]
    Other = 0,
    Guided = 1,
    Ballistic = 2,
    Fixed = 3,
}

// SISO-REF-010-2023 USWeaponSubcategories [UID 16]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
#[allow(non_camel_case_types)]
pub enum USWeaponSubcategories {
    #[default]
    AssaultmachinepistolKFAMP = 1,
    Automaticmodel1911A1_45 = 2,
    CombatMasterMarkVI_45Detronics = 3,
    DecockerKP90DC_45 = 4,
    DecockerKP91DC_40 = 5,
    GeneralofficersModel15_45 = 6,
    Nova9mmLaFrance = 7,
    PersonalDefenseWeaponMP5KPDW9mm = 8,
    SilencedColt_45LaFrance = 9,
    _5900series9mmSmithWessonSW = 10,
    M9 = 11,
    Model1911A1SpringfieldArmory = 12,
    Model20009mm = 13,
    P99mmSpringfieldArmory = 14,
    P129mm = 15,
    P85MarkII9mmRuger = 16,
    AdvancedCombatRifle5_56mmAAI = 17,
    CommandoassaultrifleModel7335_56mmColt = 18,
    InfantryrifleMini1420GB5_56mmRuger = 19,
    Mini145_56mmRuger = 20,
    MiniThirty7_62mmRuger = 21,
    Semiautomaticmodel82A2_50Barrett = 22,
    SniperWeaponSystemM247_62mm = 23,
    SnipingrifleM21SpringfieldArmory = 24,
    SnipingrifleM40A17_62mm = 25,
    SnipingrifleM6007_62mm = 26,
    AR15M165_56mm = 27,
    M1_30_1 = 28,
    M147_62mmNATO = 29,
    M14M1AM1A1A1SpringfieldArmory = 30,
    M14KassaultrifleLaFrance = 31,
    M16A2assaultrifle5_56mmColt = 32,
    M217_62mmU_S_ = 33,
    M77MarkII5_56mmRuger = 34,
    M77V7_62mmRuger = 35,
    S167_62x36mmGrendel = 36,
    SAR87_62mm = 37,
    SAR48007_62mm = 38,
    AssaultcarbineM16KLaFrance = 39,
    M1_30_2 = 40,
    M4Model7205_56mmColt = 41,
    M9009mmCalico = 42,
    AC556F5_56mmRuger = 43,
    M3_45 = 44,
    M11Cobray = 45,
    M9519mmCalico = 46,
    MP51010mm = 47,
    _9mmColt = 48,
    Ingram = 49,
    ExternallypoweredEPG7_62mmAres = 50,
    GECAL50 = 51,
    GeneralpurposeM607_62mm = 52,
    HeavyM2HBQCB_50RAMO = 53,
    LightassaultM60E3Enhanced7_62mm = 54,
    LightM16A25_56mmColt = 55,
    Light5_56mmAres = 56,
    LightweightM2_50RAMO = 57,
    LightweightassaultM60E37_62mm = 58,
    MinigunM1347_62mmGeneralElectric = 59,
    MGsystemMK19Mod340mm = 60,
    MGsystemorkitM2HBQCB_50SacoDefense = 61,
    M1919A4_30calBrowning = 62,
    _50calBrowning = 63,
    ColoredsmokehandgrenadeM18 = 64,
    ColoredsmokegrenadesFederalLaboratories = 65,
    InfraredsmokegrenadeM76 = 66,
    SmokehandgrenadeANM8HC = 67,
    DelayfragmentationhandgrenadeM61 = 68,
    DelayfragmentationhandgrenadeM67 = 69,
    ImpactfragmentationhandgrenadeM57 = 70,
    ImpactfragmentationhandgrenadeM68 = 71,
    IncendiaryhandgrenadeANM14TH3 = 72,
    LauncherIM20340mm = 73,
    LauncherM7940mm = 74,
    MultiplegrenadelauncherMM140mm = 75,
    MultishotportableflameweaponM202A266mm = 76,
    PortableABCM97 = 77,
    PortableM2A17 = 78,
    PortableM9E17 = 79,
    DragonmediumAntiArmormissileM47FGM77A = 80,
    JavelinAAWSM = 81,
    LightAntiTankWeaponM72LAWII = 82,
    RedeyeFIM43GeneralDynamics = 83,
    Saberdualpurposemissilesystem = 84,
    StingerFIM92GeneralDynamics = 85,
    TOWheavyAntiTankweapon = 86,
    BearTrapAPdevicePancor = 87,
    ChainGunautomaticweaponEX347_62mm = 88,
    CloseAssaultWeaponSystemCAWSAAI = 89,
    CAWSOlinHecklerandKoch = 90,
    CrossfireSAMModel88 = 91,
    DragonandM16 = 92,
    FiringportweaponM2315_56mmColt = 93,
    FoxholeDiggerExplosiveKitEXFODA = 94,
    InfantrySupportWeaponASP30RM30mm = 95,
    JackhammerMk3A2Pancor = 96,
    LightAntiArmorweaponM136AT4 = 97,
    M26A2 = 98,
    MasterKeyS = 99,
    Minigun5_56mm = 100,
    MultipurposeIndividualMunitionMPIMMarquardt = 101,
    MultipurposeweaponAT8 = 102,
    RecoillessrifleM40M40A2andM40A4106mm = 103,
    RecoillessrifleM6790mm = 104,
    RevolverSP101 = 105,
    RevolverSuperRedhawk_44magnumRuger = 106,
    RAWrocket140mmBrunswick = 107,
    RiflelauncherAntiArmorMunitionRAAMOlin = 108,
    RocketlauncherM203_5in = 109,
    RocketlauncherEnhancedM72EseriesHEAT66mm = 110,
    SelectivefireweaponAC5565_56mmRuger = 111,
    SelectivefireweaponAC556F5_56mmRuger = 112,
    ShotgunM870Mk1U_S_MarineCorpsRemington = 113,
    SMAWMk19383mmMcDonnellDouglas = 114,
    SMAWDDisposableSMAW = 115,
    SquadAutomaticWeaponSAWM2495_56mm = 116,
    TacticalSupportWeapon5012_50calPeregrine = 117,
    TelescopedAmmunitionRevolverGunTARG_50calAres = 118,
    UltimateoverundercombinationCiener = 119,
    M18A1Claymoremine = 120,
    Mortar81mm = 121,
    MachinegunM2407_62mm = 134,
}

// SISO-REF-010-2023 RussiaWeaponSubcategories [UID 17]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum RussiaWeaponSubcategories {
    #[default]
    Automatic9mmStechkin = 201,
    PSM5_45mm = 202,
    Selfloading9mmMakarov = 203,
    TT337_62mmTokarev = 204,
    AssaultrifleAKandAKM7_62mm = 205,
    AssaultrifleAK74andAKS745_45mm = 206,
    Selfloadingrifle7_62mmSimonov = 207,
    SniperrifleSVD7_62mmDragunov = 208,
    AKSU745_45mm = 209,
    PPS437_62mm = 210,
    PPSh417_62mm = 211,
    GeneralpurposePK7_62mm = 212,
    HeavyDShK38andModel384612_7mmDegtyarev = 213,
    HeavyNSV12_7mm = 214,
    LightRPD7_62mm = 215,
    LightRPK7_62mm = 216,
    LightRPK745_45mm = 217,
    HandgrenadeM75 = 218,
    HandgrenadeRGD5 = 219,
    APhandgrenadeF1 = 220,
    AThandgrenadeRKG3 = 221,
    AThandgrenadeRKG3M = 222,
    AThandgrenadeRKG3T = 223,
    FragmentationhandgrenadeRGN = 224,
    FragmentationhandgrenadeRGO = 225,
    SmokehandgrenadeRDG1 = 226,
    Plamyalauncher30mmAGS17 = 227,
    RiflemountedlauncherBG1540mm = 228,
    LPO50 = 229,
    ROKS3 = 230,
    CartmountedTPO50 = 231,
    GimletSA16 = 232,
    GrailSA7 = 233,
    GremlinSA14 = 234,
    SaggerAT3 = 235,
    SaxhornAT7 = 236,
    SpigotABAT14 = 237,
    SA18 = 238,
    SA19 = 239,
    Grad1Pmanportabletripodrocketlauncher122mm = 240,
    LightAntiArmorweaponRPG18 = 241,
    LightAntiTankweaponRPG22 = 242,
    MGRPG = 243,
    PortablerocketlauncherRPG16 = 244,
    Recoillessgun73mmSPG9 = 245,
    VATrocketlauncherRPG7 = 246,
    Mon50AntiPersonnelmine = 248,
    RPG29Vampir = 249,
    LaserDesignator = 250,
    AT4Spigot = 251,
    SA24IglaS = 252,
    Type69RPG = 253,
}

// SISO-REF-010-2023 UKWeaponSubcategories [UID 18]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum UKWeaponSubcategories {
    #[default]
    LAW80 = 1,
    Blowpipe = 2,
    Javelin = 3,
    _51mmmortar = 4,
    SLR7_62mmrifle = 5,
    Sterling9mmsubmachinegun = 6,
    L7A2generalpurposeMG = 7,
    L6WombatRecoillessrifle = 8,
    CarlGustav89mmrecoillessrifle = 9,
    SA80Individuallightsupportweapon = 10,
    Trigat = 11,
    MilanATmissile = 12,
}

// SISO-REF-010-2023 FrenchWeaponSubcategories [UID 19]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum FrenchWeaponSubcategories {
    #[default]
    ACLSTRIM = 1,
    Mistralmissile = 2,
    MilanATmissile = 3,
    LRACF189mmATrocketlauncher = 4,
    FAMASrifle = 5,
    AA52machinegun = 6,
    _58mmriflegrenade = 7,
    FRF1sniperrifle = 8,
}

// SISO-REF-010-2023 LifeFormsSubcategoryGermanWeapons [UID 20]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormsSubcategoryGermanWeapons {
    #[default]
    G3rifle = 1,
    G11rifle = 2,
    P1pistol = 3,
    MG3machinegun = 4,
    Milanmissile = 5,
    MP1Uzisubmachinegun = 6,
    Panzerfaust3LightAntiTankWeapon = 7,
    DM19handgrenade = 8,
    DM29handgrenade = 9,
}

// SISO-REF-010-2023 EnvironmentalSubcategory [UID 21]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EnvironmentalSubcategory {
    #[default]
    Other = 0,
    VerySmall = 20,
    Small = 40,
    Medium = 60,
    Large = 80,
    VeryLarge = 100,
}

// SISO-REF-010-2023 RadioCategory [UID 22]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum RadioCategory {
    #[default]
    Other = 0,
    VoiceTransmissionReception = 1,
    DataLinkTransmissionReception = 2,
    VoiceandDataLinkTransmissionReception = 3,
    InstrumentedLandingSystemGlideslopeTransmitter = 4,
    InstrumentedLandingSystemLocalizerTransmitter = 5,
    InstrumentedLandingSystemOuterMarkerBeacon = 6,
    InstrumentedLandingSystemMiddleMarkerBeacon = 7,
    InstrumentedLandingSystemInnerMarkerBeacon = 8,
    InstrumentedLandingSystem = 9,
    TacticalAirNavigation = 10,
    TacticalAirNavigation = 11,
    TacticalAirNavigation = 12,
    VariableOmniRanging = 13,
    VariableOmniRanging = 14,
    CombinedVORILSReceiver = 15,
    CombinedVORTACANTransmitter = 16,
    NonDirectionalBeaconTransmitter = 17,
    NonDirectionalBeaconReceiver = 18,
    NonDirectionalBeaconTransmitter = 19,
    DistanceMeasuringEquipment = 20,
    Link16Terminal = 21,
    Link11Terminal = 22,
    Link11BTerminal = 23,
    EPLRSSADLTerminal = 24,
    F22IntraFlightDataLink = 25,
    F35MultifunctionAdvancedDataLink = 26,
    SINCGARSTerminal = 27,
    LBandSATCOMTerminal = 28,
    IBSTerminal = 29,
    GPS = 30,
    TacticalVideo = 31,
    AirtoAirMissileDatalink = 32,
    Link16SurrogateforNonNATOTDLTerminal = 33,
    MQ19CBandLOSDatalink = 34,
    MQ19KuBandSATCOMDatalink = 35,
    AirtoGroundWeaponDatalink = 36,
    AutomaticIdentificationSystem = 37,
    JPALSDataLink = 38,
    CombatSearchandRescueRadio = 40,
    CounterUnmannedAircraftSystemRadio = 41,
    EmergencyPositionIndicatingRadioBeacons = 42,
    ElectronicAttackSystems = 50,
    TacticalTargetingNetworkTechnology = 51,
}

// SISO-REF-010-2023 RadioSubcategory [UID 23]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum RadioSubcategory {
    #[default]
    Other = 0,
    JointElectronicsTypeDesignationSystemNonspecificSeries = 1,
    ManufacturerDesignation = 2,
    NationalDesignation = 3,
    JETDSARCSet1 = 11,
    JETDSARCSet2 = 12,
    JETDSARCSet3 = 13,
    JETDSARCSet4 = 14,
    JETDSBRCSet1 = 15,
    JETDSBRCSet2 = 16,
    JETDSBRCSet3 = 17,
    JETDSBRCSet4 = 18,
    JETDSCRCSet1 = 19,
    JETDSCRCSet2 = 20,
    JETDSCRCSet3 = 21,
    JETDSCRCSet4 = 22,
    JETDSDRCSet1 = 23,
    JETDSDRCSet2 = 24,
    JETDSDRCSet3 = 25,
    JETDSDRCSet4 = 26,
    JETDSFRCSet1 = 27,
    JETDSFRCSet2 = 28,
    JETDSFRCSet3 = 29,
    JETDSFRCSet4 = 30,
    JETDSGRCSet1 = 31,
    JETDSGRCSet2 = 32,
    JETDSGRCSet3 = 33,
    JETDSGRCSet4 = 34,
    JETDSKRCSet1 = 35,
    JETDSKRCSet2 = 36,
    JETDSKRCSet3 = 37,
    JETDSKRCSet4 = 38,
    JETDSMRCSet1 = 39,
    JETDSMRCSet2 = 40,
    JETDSMRCSet3 = 41,
    JETDSMRCSet4 = 42,
    JETDSPRCSet1 = 43,
    JETDSPRCSet2 = 44,
    JETDSPRCSet3 = 45,
    JETDSPRCSet4 = 46,
    JETDSSRCSet1 = 47,
    JETDSSRCSet2 = 48,
    JETDSSRCSet3 = 49,
    JETDSSRCSet4 = 50,
    JETDSTRCSet1 = 51,
    JETDSTRCSet2 = 52,
    JETDSTRCSet3 = 53,
    JETDSTRCSet4 = 54,
    JETDSVRCSet1 = 55,
    JETDSVRCSet2 = 56,
    JETDSVRCSet3 = 57,
    JETDSVRCSet4 = 58,
    JETDSWRCSet1 = 59,
    JETDSWRCSet2 = 60,
    JETDSWRCSet3 = 61,
    JETDSWRCSet4 = 62,
    JETDSZRCSet1 = 63,
    JETDSZRCSet2 = 64,
    JETDSZRCSet3 = 65,
    JETDSZRCSet4 = 66,
}

// SISO-REF-010-2023 ExpendableAirCategory [UID 25]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ExpendableAirCategory {
    #[default]
    Other = 0,
    Chaff = 1,
    Flare = 2,
    CombinedChaffandFlare = 3,
    ActiveEmitter = 4,
    PassiveDecoy = 5,
    WingedDecoy = 6,
    SignalIlluminationFlare = 7,
    SmokeGenerator = 8,
    CombinedFlareandSmokeGenerator = 12,
    SARNightLight = 13,
    SARBuoy = 14,
}

// SISO-REF-010-2023 ExpendableSurfaceCategory [UID 26]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ExpendableSurfaceCategory {
    #[default]
    Other = 0,
    Flare = 2,
    ActiveEmitter = 4,
    PassiveDecoy = 5,
    SmokeGenerator = 8,
    CombinedFlareandSmokeGenerator = 12,
    SARBuoy = 14,
}

// SISO-REF-010-2023 ExpendableSubsurfaceCategory [UID 27]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ExpendableSubsurfaceCategory {
    #[default]
    Other = 0,
    Activeemitter = 4,
    Passivedecoy = 5,
    Signal = 7,
    NoiseMakerDecoy = 9,
    BubbleMakerDecoy = 10,
    MultiModeDecoy = 11,
}

// SISO-REF-010-2023 SensorEmitterCategory [UID 28]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SensorEmitterCategory {
    #[default]
    Other = 0,
    Multispectral = 1,
    RFActive = 2,
    RFPassive = 3,
    Optical = 4,
    ElectroOptical = 5,
    Seismic = 6,
    Chemicalpointdetector = 7,
    Chemicalstandoff = 8,
    Thermal = 9,
    AcousticActive = 10,
    AcousticPassive = 11,
    ContactPressure = 12,
    ElectroMagneticRadiation = 13,
    ParticleRadiation = 14,
    Magnetic = 15,
    Gravitational = 16,
}

// SISO-REF-010-2023 Country [UID 29]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Country {
    #[default]
    Other = 0,
    Afghanistan = 1,
    Albania = 2,
    Algeria = 3,
    AmericanSamoa = 4,
    Andorra = 5,
    Angola = 6,
    Anguilla = 7,
    Antarctica = 8,
    AntiguaandBarbuda = 9,
    Argentina = 10,
    Aruba = 11,
    AshmoreandCartierIslands = 12,
    Australia = 13,
    Austria = 14,
    Bahamas = 15,
    Bahrain = 16,
    BakerIsland = 17,
    Bangladesh = 18,
    Barbados = 19,
    BassasdaIndia = 20,
    Belgium = 21,
    Belize = 22,
    Benin = 23,
    Bermuda = 24,
    Bhutan = 25,
    Bolivia = 26,
    Botswana = 27,
    BouvetIsland = 28,
    Brazil = 29,
    BritishIndianOceanTerritory = 30,
    VirginIslands = 31,
    BruneiDarussalam = 32,
    Bulgaria = 33,
    BurkinaFaso = 34,
    Myanmar = 35,
    Burundi = 36,
    Cambodia = 37,
    Cameroon = 38,
    Canada = 39,
    CaboVerde = 40,
    CaymanIslands = 41,
    CentralAfricanRepublic = 42,
    Chad = 43,
    Chile = 44,
    ChinaPeoplesRepublicof = 45,
    ChristmasIsland = 46,
    Cocos = 47,
    Colombia = 48,
    Comoros = 49,
    Congo = 50,
    CookIslands = 51,
    CoralSeaIslands = 52,
    CostaRica = 53,
    Cuba = 54,
    Cyprus = 55,
    Czechoslovakia = 56,
    Denmark = 57,
    Djibouti = 58,
    Dominica = 59,
    DominicanRepublic = 60,
    Ecuador = 61,
    Egypt = 62,
    ElSalvador = 63,
    EquatorialGuinea = 64,
    Ethiopia = 65,
    EuropaIsland = 66,
    FalklandIslands = 67,
    FaroeIslands = 68,
    Fiji = 69,
    Finland = 70,
    France = 71,
    FrenchGuiana = 72,
    FrenchPolynesia = 73,
    FrenchSouthernTerritories = 74,
    Gabon = 75,
    GambiaThe = 76,
    GazaStrip = 77,
    Germany = 78,
    Ghana = 79,
    Gibraltar = 80,
    GloriosoIslands = 81,
    Greece = 82,
    Greenland = 83,
    Grenada = 84,
    Guadeloupe = 85,
    Guam = 86,
    Guatemala = 87,
    Guernsey = 88,
    Guinea = 89,
    GuineaBissau = 90,
    Guyana = 91,
    Haiti = 92,
    HeardIslandandMcDonaldIslands = 93,
    Honduras = 94,
    HongKong = 95,
    HowlandIsland = 96,
    Hungary = 97,
    Iceland = 98,
    India = 99,
    Indonesia = 100,
    Iran = 101,
    Iraq = 102,
    Ireland = 104,
    Israel = 105,
    Italy = 106,
    CotedIvoire = 107,
    Jamaica = 108,
    JanMayen = 109,
    Japan = 110,
    JarvisIsland = 111,
    Jersey = 112,
    JohnstonAtoll = 113,
    Jordan = 114,
    JuandeNovaIsland = 115,
    Kenya = 116,
    KingmanReef = 117,
    Kiribati = 118,
    Korea = 119,
    Korea = 120,
    Kuwait = 121,
    LaoPeoplesDemocraticRepublic = 122,
    Lebanon = 123,
    Lesotho = 124,
    Liberia = 125,
    Libya = 126,
    Liechtenstein = 127,
    Luxembourg = 128,
    Madagascar = 129,
    Macao = 130,
    Malawi = 131,
    Malaysia = 132,
    Maldives = 133,
    Mali = 134,
    Malta = 135,
    IsleofMan = 136,
    MarshallIslands = 137,
    Martinique = 138,
    Mauritania = 139,
    Mauritius = 140,
    Mayotte = 141,
    Mexico = 142,
    Micronesia = 143,
    Monaco = 144,
    Mongolia = 145,
    Montserrat = 146,
    Morocco = 147,
    Mozambique = 148,
    Namibia = 149,
    Nauru = 150,
    NavassaIsland = 151,
    Nepal = 152,
    Netherlands = 153,
    NetherlandsAntilles = 154,
    NewCaledonia = 155,
    NewZealand = 156,
    Nicaragua = 157,
    Niger = 158,
    Nigeria = 159,
    Niue = 160,
    NorfolkIsland = 161,
    NorthernMarianaIslands = 162,
    Norway = 163,
    Oman = 164,
    Pakistan = 165,
    PalmyraAtoll = 166,
    Panama = 168,
    PapuaNewGuinea = 169,
    ParacelIslands = 170,
    Paraguay = 171,
    Peru = 172,
    Philippines = 173,
    Pitcairn = 174,
    Poland = 175,
    Portugal = 176,
    PuertoRico = 177,
    Qatar = 178,
    Reunion = 179,
    Romania = 180,
    Rwanda = 181,
    SaintKittsandNevis = 182,
    SaintHelenaAscensionandTristandaCunha = 183,
    SaintLucia = 184,
    SaintPierreandMiquelon = 185,
    SaintVincentandtheGrenadines = 186,
    SanMarino = 187,
    SaoTomeandPrincipe = 188,
    SaudiArabia = 189,
    Senegal = 190,
    Seychelles = 191,
    SierraLeone = 192,
    Singapore = 193,
    SolomonIslands = 194,
    Somalia = 195,
    SouthGeorgiaandtheSouthSandwichIslands = 196,
    SouthAfrica = 197,
    Spain = 198,
    SpratlyIslands = 199,
    SriLanka = 200,
    Sudan = 201,
    Suriname = 202,
    Svalbard = 203,
    Eswatini = 204,
    Sweden = 205,
    Switzerland = 206,
    SyrianArabRepublic = 207,
    TaiwanProvinceofChina = 208,
    TanzaniaUnitedRepublicof = 209,
    Thailand = 210,
    Togo = 211,
    Tokelau = 212,
    Tonga = 213,
    TrinidadandTobago = 214,
    TromelinIsland = 215,
    Palau = 216,
    Tunisia = 217,
    Turkey = 218,
    TurksandCaicosIslands = 219,
    Tuvalu = 220,
    Uganda = 221,
    Russia = 222,
    UnitedArabEmirates = 223,
    UnitedKingdomofGreatBritainandNorthernIreland = 224,
    UnitedStatesofAmerica = 225,
    Uruguay = 226,
    Vanuatu = 227,
    HolySee = 228,
    Venezuela = 229,
    VietNam = 230,
    VirginIslands = 231,
    WakeIsland = 232,
    WallisandFutuna = 233,
    WesternSahara = 234,
    WestBank = 235,
    Samoa = 236,
    Yemen = 237,
    SerbiaandMontenegro = 240,
    Zaire = 241,
    Zambia = 242,
    Zimbabwe = 243,
    Armenia = 244,
    Azerbaijan = 245,
    Belarus = 246,
    BosniaandHerzegovina = 247,
    ClippertonIsland = 248,
    Croatia = 249,
    Estonia = 250,
    Georgia = 251,
    Kazakhstan = 252,
    Kyrgyzstan = 253,
    Latvia = 254,
    Lithuania = 255,
    NorthMacedonia = 256,
    MidwayIslands = 257,
    Moldova = 258,
    Montenegro = 259,
    Russia = 260,
    SerbiaandMontenegro = 261,
    Slovenia = 262,
    Tajikistan = 263,
    Turkmenistan = 264,
    Ukraine = 265,
    Uzbekistan = 266,
    CzechRepublic = 267,
    Slovakia = 268,
    AalandIslands = 269,
    BonaireSintEustatiusandSaba = 270,
    Congo = 271,
    Curacao = 272,
    Eritrea = 273,
    SaintBarthelemy = 274,
    SaintMartin = 275,
    Serbia = 276,
    SintMaarten = 277,
    SouthSudan = 278,
    SvalbardandJanMayen = 279,
    TimorLeste = 280,
    UnitedStatesMinorOutlyingIslands = 281,
    PalestineStateof = 282,
}

// SISO-REF-010-2023 DeadReckoningAlgorithm [UID 44]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DeadReckoningAlgorithm {
    #[default]
    Other = 0,
    StaticNonmovingEntity = 1,
    DRMConstantVelocityLowAccelerationLinearMotionEntity = 2,
    DRMConstantVelocityLowAccelerationLinearMotionEntitywithExtrapolationofOrientation = 3,
    DRMHighSpeedorManeuveringEntitywithExtrapolationofOrientation = 4,
    DRMHighSpeedorManeuveringEntity = 5,
    DRMSimilartoFPWexceptinBodyCoordinates = 6,
    DRMSimilartoRPWexceptinBodyCoordinates = 7,
    DRMSimilartoRVWexceptinBodyCoordinates = 8,
    DRMSimilartoFVWexceptinBodyCoordinates = 9,
}

// SISO-REF-010-2023 EntityMarkingCharacterSet [UID 45]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EntityMarkingCharacterSet {
    #[default]
    Unused = 0,
    ASCII = 1,
    USArmyMarking = 2,
    DigitChevron = 3,
}

// SISO-REF-010-2023 EntityCapabilities [UID 55]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EntityCapabilities {
    #[default]
    LandPlatformEntityCapabilities = 0,
    AirPlatformEntityCapabilities = 1,
    SurfacePlatformEntityCapabilities = 2,
    SubsurfacePlatformEntityCapabilities = 3,
    SpacePlatformEntityCapabilities = 4,
    MunitionEntityCapabilities = 5,
    LifeFormsEntityCapabilities = 6,
    EnvironmentalEntityCapabilities = 7,
    CulturalFeatureEntityCapabilities = 8,
    SupplyEntityCapabilities = 9,
    RadioEntityCapabilities = 10,
    ExpendableEntityCapabilities = 11,
    SensorEmitterEntityCapabilities = 12,
}

// SISO-REF-010-2023 VariableParameterRecordType [UID 56]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum VariableParameterRecordType {
    #[default]
    ArticulatedPart = 0,
    AttachedPart = 1,
    Separation = 2,
    EntityType = 3,
    EntityAssociation = 4,
}

// SISO-REF-010-2023 AttachedParts [UID 57]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AttachedParts {
    #[default]
    NothingEmpty = 0,
    SequentialIDsformodelspecificstations = 1,
    FuselageStations = 512,
    LeftwingStations = 640,
    RightwingStations = 768,
    M16A42rifle = 896,
    M249SAW = 897,
    M60Machinegun = 898,
    M203GrenadeLauncher = 899,
    M136AT4 = 900,
    M47Dragon = 901,
    AAWSMJavelin = 902,
    M18A1ClaymoreMine = 903,
    MK19GrenadeLauncher = 904,
    M2MachineGun = 905,
}

// SISO-REF-010-2023 ArticulatedPartsTypeMetric [UID 58]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ArticulatedPartsTypeMetric {
    #[default]
    NotSpecified = 0,
    Position = 1,
    PositionRate = 2,
    Extension = 3,
    ExtensionRate = 4,
    X = 5,
    XRate = 6,
    Y = 7,
    YRate = 8,
    Z = 9,
    ZRate = 10,
    Azimuth = 11,
    AzimuthRate = 12,
    Elevation = 13,
    ElevationRate = 14,
    Rotation = 15,
    RotationRate = 16,
}

// SISO-REF-010-2023 ArticulatedPartsTypeClass [UID 59]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ArticulatedPartsTypeClass {
    #[default]
    NotSpecified = 0,
    Rudder = 1024,
    LeftFlap = 1056,
    RightFlap = 1088,
    LeftAileron = 1120,
    RightAileron = 1152,
    HelicopterMainRotor = 1184,
    HelicopterTailRotor = 1216,
    OtherAircraftControlSurfacesDefinedasNeeded = 1248,
    PropellerNumber1 = 1280,
    PropellerNumber2 = 1312,
    PropellerNumber3 = 1344,
    PropellerNumber4 = 1376,
    LeftStabilator = 1408,
    RightStabilator = 1440,
    LeftRuddervator = 1472,
    RightRuddervator = 1504,
    LeftLeadingEdgeFlapSlat = 1536,
    RightLeadingEdgeFlapSlat = 1568,
    LeftElevator = 1600,
    RightElevator = 1632,
    Canard = 1664,
    Canard = 1696,
    ElevonInner = 1728,
    ElevonInner = 1760,
    ElevonMiddle = 1792,
    ElevonMiddle = 1824,
    ElevonOuter = 1856,
    ElevonOuter = 1888,
    Canopy = 1920,
    Spoiler = 1952,
    Spoiler = 1984,
    Periscope = 2048,
    GenericAntenna = 2080,
    Snorkel = 2112,
    OtherExtendiblePartsDefinedasNeeded = 2144,
    DivePlaneLeft = 2176,
    DivePlaneRight = 2208,
    DivePlaneLeft = 2240,
    DivePlaneRight = 2272,
    DivePlaneLeft = 2304,
    DivePlaneRight = 2336,
    LandingGear = 3072,
    TailHook = 3104,
    SpeedBrake = 3136,
    LeftDoorofPrimaryWeaponBay = 3168,
    RightDoorofPrimaryWeaponBay = 3200,
    TankorAPCHatch = 3232,
    Wingsweep = 3264,
    BridgeLauncher = 3296,
    BridgeSection1 = 3328,
    BridgeSection2 = 3360,
    BridgeSection3 = 3392,
    PrimaryBlade1 = 3424,
    PrimaryBlade2 = 3456,
    PrimaryBoom = 3488,
    PrimaryLauncherArm = 3520,
    OtherFixedPositionPartsDefinedasNeeded = 3552,
    LandingGearNose = 3584,
    LandingGearLeftMain = 3616,
    LandingGearRightMain = 3648,
    DoorsofLeftSideWeaponBay = 3680,
    DoorsofRightSideWeaponBay = 3712,
    SpotSearchLight1 = 3744,
    SpotSearchLight2 = 3776,
    SpotSearchLight3 = 3808,
    SpotSearchLight4 = 3840,
    LandingLight = 3872,
    PrimaryTurretNumber1 = 4096,
    PrimaryTurretNumber2 = 4128,
    PrimaryTurretNumber3 = 4160,
    PrimaryTurretNumber4 = 4192,
    PrimaryTurretNumber5 = 4224,
    PrimaryTurretNumber6 = 4256,
    PrimaryTurretNumber7 = 4288,
    PrimaryTurretNumber8 = 4320,
    PrimaryTurretNumber9 = 4352,
    PrimaryTurretNumber10 = 4384,
    PrimaryGunNumber1 = 4416,
    PrimaryGunNumber2 = 4448,
    PrimaryGunNumber3 = 4480,
    PrimaryGunNumber4 = 4512,
    PrimaryGunNumber5 = 4544,
    PrimaryGunNumber6 = 4576,
    PrimaryGunNumber7 = 4608,
    PrimaryGunNumber8 = 4640,
    PrimaryGunNumber9 = 4672,
    PrimaryGunNumber10 = 4704,
    PrimaryLauncher1 = 4736,
    PrimaryLauncher2 = 4768,
    PrimaryLauncher3 = 4800,
    PrimaryLauncher4 = 4832,
    PrimaryLauncher5 = 4864,
    PrimaryLauncher6 = 4896,
    PrimaryLauncher7 = 4928,
    PrimaryLauncher8 = 4960,
    PrimaryLauncher9 = 4992,
    PrimaryLauncher10 = 5024,
    PrimaryDefenseSystems1 = 5056,
    PrimaryDefenseSystems2 = 5088,
    PrimaryDefenseSystems3 = 5120,
    PrimaryDefenseSystems4 = 5152,
    PrimaryDefenseSystems5 = 5184,
    PrimaryDefenseSystems6 = 5216,
    PrimaryDefenseSystems7 = 5248,
    PrimaryDefenseSystems8 = 5280,
    PrimaryDefenseSystems9 = 5312,
    PrimaryDefenseSystems10 = 5344,
    PrimaryRadar1 = 5376,
    PrimaryRadar2 = 5408,
    PrimaryRadar3 = 5440,
    PrimaryRadar4 = 5472,
    PrimaryRadar5 = 5504,
    PrimaryRadar6 = 5536,
    PrimaryRadar7 = 5568,
    PrimaryRadar8 = 5600,
    PrimaryRadar9 = 5632,
    PrimaryRadar10 = 5664,
    SecondaryTurretNumber1 = 5696,
    SecondaryTurretNumber2 = 5728,
    SecondaryTurretNumber3 = 5760,
    SecondaryTurretNumber4 = 5792,
    SecondaryTurretNumber5 = 5824,
    SecondaryTurretNumber6 = 5856,
    SecondaryTurretNumber7 = 5888,
    SecondaryTurretNumber8 = 5920,
    SecondaryTurretNumber9 = 5952,
    SecondaryTurretNumber10 = 5984,
    SecondaryGunNumber1 = 6016,
    SecondaryGunNumber2 = 6048,
    SecondaryGunNumber3 = 6080,
    SecondaryGunNumber4 = 6112,
    SecondaryGunNumber5 = 6144,
    SecondaryGunNumber6 = 6176,
    SecondaryGunNumber7 = 6208,
    SecondaryGunNumber8 = 6240,
    SecondaryGunNumber9 = 6272,
    SecondaryGunNumber10 = 6304,
    SecondaryLauncher1 = 6336,
    SecondaryLauncher2 = 6368,
    SecondaryLauncher3 = 6400,
    SecondaryLauncher4 = 6432,
    SecondaryLauncher5 = 6464,
    SecondaryLauncher6 = 6496,
    SecondaryLauncher7 = 6528,
    SecondaryLauncher8 = 6560,
    SecondaryLauncher9 = 6592,
    SecondaryLauncher10 = 6624,
    SecondaryDefenseSystems1 = 6656,
    SecondaryDefenseSystems2 = 6688,
    SecondaryDefenseSystems3 = 6720,
    SecondaryDefenseSystems4 = 6752,
    SecondaryDefenseSystems5 = 6784,
    SecondaryDefenseSystems6 = 6816,
    SecondaryDefenseSystems7 = 6848,
    SecondaryDefenseSystems8 = 6880,
    SecondaryDefenseSystems9 = 6912,
    SecondaryDefenseSystems10 = 6944,
    SecondaryRadar1 = 6976,
    SecondaryRadar2 = 7008,
    SecondaryRadar3 = 7040,
    SecondaryRadar4 = 7072,
    SecondaryRadar5 = 7104,
    SecondaryRadar6 = 7136,
    SecondaryRadar7 = 7168,
    SecondaryRadar8 = 7200,
    SecondaryRadar9 = 7232,
    SecondaryRadar10 = 7264,
    DeckElevator1 = 7296,
    DeckElevator2 = 7328,
    Catapult1 = 7360,
    Catapult2 = 7392,
    JetBlastDeflector1 = 7424,
    JetBlastDeflector2 = 7456,
    ArrestorWires1 = 7488,
    ArrestorWires2 = 7520,
    ArrestorWires3 = 7552,
    WingFold = 7584,
    FuselageFold = 7616,
    MainCargoDoor = 7648,
    CargoRamp = 7680,
    AirtoAirRefuelingBoom = 7712,
    PrimaryAerialRefuelingReceptacleDoor = 7744,
    SecondaryAerialRefuelingReceptacleDoor = 7776,
    AerialRefuelingReceptacleLatch = 7808,
    CargoDoor1 = 7840,
    CargoDoor2 = 7872,
    CargoDoor3 = 7904,
    CargoDoor4 = 7936,
    CargoDoor5 = 7968,
    CargoDoor6 = 8000,
    CargoDoor7 = 8032,
    CargoDoor8 = 8064,
    CargoDoor9 = 8096,
    CargoDoor10 = 8128,
    CentreRefuellingDrogue = 8160,
    PortRefuellingDrogue = 8192,
    StarboardRefuellingDrogue = 8224,
    SubmarineEngineExhaustMast = 8256,
    SubmarineMast1 = 8288,
    SubmarineMast2 = 8320,
    SubmarineMast3 = 8352,
    SubmarineMast4 = 8384,
    SubmarineMast5 = 8416,
    SubmarineMast6 = 8448,
    SubmarineMast7 = 8480,
    SubmarineMast8 = 8512,
    SubmarineMast9 = 8544,
    SubmarineMast10 = 8576,
    VectoredThrustNozzle = 8608,
    LeftDooroftheLeftWeaponBay = 8640,
    RightDooroftheLeftWeaponBay = 8672,
    LeftDooroftheRightWeaponBay = 8704,
    RightDooroftheRightWeaponBay = 8736,
    GunDoor = 8768,
    CountermeasureDoorLeft = 8800,
    CountermeasureDoorRight = 8832,
    HookDoorForward = 8864,
    HookDoorAft = 8896,
    LiftFanUpperDoor = 8928,
    LiftFanLowerDoorLeft = 8960,
    LiftFanLowerDoorRight = 8992,
    RefuelProbeDoor = 9024,
    LeftEngineNacelle = 9056,
    RightEngineNacelle = 9088,
    _1stLeftWheel = 9120,
    _1stRightWheel = 9152,
    _2ndLeftWheel = 9184,
    _2ndRightWheel = 9216,
    _3rdLeftWheel = 9248,
    _3rdRightWheel = 9280,
    _4thLeftWheel = 9312,
    _4thRightWheel = 9344,
    _5thLeftWheel = 9376,
    _5thRightWheel = 9408,
    _6thLeftWheel = 9440,
    _6thRightWheel = 9472,
    _7thLeftWheel = 9504,
    _7thRightWheel = 9536,
    _8thLeftWheel = 9568,
    _8thRightWheel = 9600,
    _9thLeftWheel = 9632,
    _9thRightWheel = 9664,
    _10thLeftWheel = 9696,
    _10thRightWheel = 9728,
    RefuelingProbe = 9760,
    SteeringWheel = 9792,
    CraneBody = 9824,
    CraneArm1 = 9856,
    CraneArm2 = 9888,
    CraneArm3 = 9920,
    CraneBoom = 9952,
    CraneHook = 9984,
    Trailer = 10016,
    RollerLeft = 10048,
    RollerRight = 10080,
    PrimaryGunRecoil = 10112,
    SecondaryGunRecoil = 10144,
}

// SISO-REF-010-2023 MunitionDescriptorWarhead [UID 60]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MunitionDescriptorWarhead {
    #[default]
    Other = 0000,
    Cargo = 0010,
    FuelAirExplosive = 0020,
    GlassBeads = 0030,
    _1um = 0031,
    _5um = 0032,
    _10um = 0033,
    HighExplosive = 1000,
    HEPlastic = 1100,
    HEIncendiary = 1200,
    HEFragmentation = 1300,
    HEAntiTank = 1400,
    HEBomblets = 1500,
    HEShapedCharge = 1600,
    HEContinuousRod = 1610,
    HETungstenBall = 1615,
    HEBlastFragmentation = 1620,
    HESteerableDartswithHE = 1625,
    HEDarts = 1630,
    HEFlechettes = 1635,
    HEDirectedFragmentation = 1640,
    HESemiArmorPiercing = 1645,
    HEShapedChargeFragmentation = 1650,
    HESemiArmorPiercingFragmentation = 1655,
    HEHollowCharge = 1660,
    HEDoubleHollowCharge = 1665,
    HEGeneralPurpose = 1670,
    HEBlastPenetrator = 1675,
    HERodPenetrator = 1680,
    HEAntiPersonnel = 1685,
    HEShapedChargeFragmentationIncendiary = 1690,
    HEPenetratorBlastFragmentation = 1695,
    Smoke = 2000,
    WP = 2005,
    FOGO = 2010,
    HC = 2015,
    Illumination = 3000,
    Practice = 4000,
    Blank = 4001,
    Dummy = 4002,
    Kinetic = 5000,
    Mines = 6000,
    Nuclear = 7000,
    NuclearIMT = 7010,
    NuclearVariousYields = 7011,
    ChemicalGeneral = 8000,
    ChemicalBlisterAgent = 8100,
    HD = 8110,
    ThickenedHD = 8115,
    DustyHD = 8120,
    L = 8125,
    HN3 = 8130,
    HL = 8135,
    CX = 8140,
    DMMP = 8145,
    DMHP = 8150,
    DMA = 8155,
    DEM = 8160,
    PX = 8165,
    ChemicalBloodAgent = 8200,
    AC = 8210,
    CK = 8215,
    CG = 8220,
    ChemicalNerveAgent = 8300,
    VX = 8310,
    ThickenedVX = 8315,
    DustyVX = 8320,
    GA = 8325,
    ThickenedGA = 8330,
    DustyGA = 8335,
    GB = 8340,
    ThickenedGB = 8345,
    DustyGB = 8350,
    GD = 8355,
    ThickenedGD = 8360,
    DustyGD = 8365,
    GF = 8370,
    ThickenedGF = 8375,
    DustyGF = 8380,
    SVX = 8385,
    BIS = 8410,
    TCP = 8415,
    MS = 8425,
    TEP = 8430,
    H2O = 8445,
    TO1 = 8450,
    TO2 = 8455,
    TO3 = 8460,
    SulfurHexafluoride = 8465,
    AA = 8470,
    HF = 8475,
    Biological = 9000,
    BiologicalVirus = 9100,
    BiologicalBacteria = 9200,
    BiologicalRickettsia = 9300,
    BiologicalGeneticallyModifiedMicroorganisms = 9400,
    BiologicalToxin = 9500,
}

// SISO-REF-010-2023 MunitionDescriptorFuse [UID 61]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MunitionDescriptorFuse {
    #[default]
    Other = 0000,
    IntelligentInfluence = 0010,
    Sensor = 0020,
    Selfdestruct = 0030,
    UltraQuick = 0040,
    Body = 0050,
    DeepIntrusion = 0060,
    Multifunction = 0100,
    PointDetonation = 0200,
    BaseDetonation = 0300,
    Contact = 1000,
    ContactInstant = 1100,
    ContactDelayed = 1200,
    _10msDelay = 1201,
    _20msDelay = 1202,
    _50msDelay = 1205,
    _60msDelay = 1206,
    _100msDelay = 1210,
    _125msDelay = 1212,
    _250msDelay = 1225,
    _5msDelay = 1250,
    _15msDelay = 1251,
    _25msDelay = 1252,
    _30msDelay = 1253,
    _35msDelay = 1254,
    _40msDelay = 1255,
    _45msDelay = 1256,
    _90msDelay = 1257,
    _120msDelay = 1258,
    _180msDelay = 1259,
    _240msDelay = 1260,
    ContactElectronic = 1300,
    ContactGraze = 1400,
    ContactCrush = 1500,
    ContactHydrostatic = 1600,
    ContactMechanical = 1700,
    ContactChemical = 1800,
    ContactPiezoelectric = 1900,
    ContactPointInitiating = 1910,
    ContactPointInitiatingBaseDetonating = 1920,
    ContactBaseDetonating = 1930,
    ContactBallisticCapandBase = 1940,
    ContactBase = 1950,
    ContactNose = 1960,
    ContactFittedinStandoffProbe = 1970,
    ContactNonaligned = 1980,
    Timed = 2000,
    TimedProgrammable = 2100,
    TimedBurnout = 2200,
    TimedPyrotechnic = 2300,
    TimedElectronic = 2400,
    TimedBaseDelay = 2500,
    TimedReinforcedNoseImpactDelay = 2600,
    TimedShortDelayImpact = 2700,
    _10msDelay = 2701,
    _20msDelay = 2702,
    _50msDelay = 2705,
    _60msDelay = 2706,
    _100msDelay = 2710,
    _125msDelay = 2712,
    _250msDelay = 2725,
    TimedNoseMountedVariableDelay = 2800,
    TimedLongDelaySide = 2900,
    TimedSelectableDelay = 2910,
    TimedImpact = 2920,
    TimedSequence = 2930,
    Proximity = 3000,
    ProximityActiveLaser = 3100,
    ProximityMagnetic = 3200,
    ProximityActiveRadar = 3300,
    ProximityRadioFrequency = 3400,
    ProximityProgrammable = 3500,
    ProximityProgrammablePrefragmented = 3600,
    ProximityInfrared = 3700,
    Command = 4000,
    CommandElectronicRemotelySet = 4100,
    Altitude = 5000,
    AltitudeRadioAltimeter = 5100,
    AltitudeAirBurst = 5200,
    Depth = 6000,
    Acoustic = 7000,
    Pressure = 8000,
    PressureDelay = 8010,
    Inert = 8100,
    Dummy = 8110,
    Practice = 8120,
    PlugRepresenting = 8130,
    Training = 8150,
    Pyrotechnic = 9000,
    PyrotechnicDelay = 9010,
    Electrooptical = 9100,
    Electromechanical = 9110,
    ElectromechanicalNose = 9120,
    Strikerless = 9200,
    StrikerlessNoseImpact = 9210,
    StrikerlessCompressionIgnition = 9220,
    CompressionIgnition = 9300,
    CompressionIgnitionStrikerlessNoseImpact = 9310,
    Percussion = 9400,
    PercussionInstantaneous = 9410,
    Electronic = 9500,
    ElectronicInternallyMounted = 9510,
    ElectronicRangeSetting = 9520,
    ElectronicProgrammed = 9530,
    Mechanical = 9600,
    MechanicalNose = 9610,
    MechanicalTail = 9620,
}

// SISO-REF-010-2023 DetonationResult [UID 62]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DetonationResult {
    #[default]
    Other = 0,
    EntityImpact = 1,
    EntityProximateDetonation = 2,
    GroundImpact = 3,
    GroundProximateDetonation = 4,
    Detonation = 5,
    NoneorNoDetonation = 6,
    HEhitsmall = 7,
    HEhitmedium = 8,
    HEhitlarge = 9,
    Armorpiercinghit = 10,
    Dirtblastsmall = 11,
    Dirtblastmedium = 12,
    Dirtblastlarge = 13,
    Waterblastsmall = 14,
    Waterblastmedium = 15,
    Waterblastlarge = 16,
    Airhit = 17,
    Buildinghitsmall = 18,
    Buildinghitmedium = 19,
    Buildinghitlarge = 20,
    Mineclearinglinecharge = 21,
    Environmentobjectimpact = 22,
    Environmentobjectproximatedetonation = 23,
    WaterImpact = 24,
    AirBurst = 25,
    Killwithfragmenttype1 = 26,
    Killwithfragmenttype2 = 27,
    Killwithfragmenttype3 = 28,
    Killwithfragmenttype1afterflyoutfailure = 29,
    Killwithfragmenttype2afterflyoutfailure = 30,
    Missduetoflyoutfailure = 31,
    Missduetoendgamefailure = 32,
    Missduetoflyoutandendgamefailure = 33,
}

// SISO-REF-010-2023 ServiceRequestServiceTypeRequested [UID 63]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ServiceRequestServiceTypeRequested {
    #[default]
    Other = 0,
    Resupply = 1,
    Repair = 2,
    AerialRefuelingHighFidelity = 3,
    AerialRefuelingLowFidelity = 4,
}

// SISO-REF-010-2023 RepairCompleteRepair [UID 64]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum RepairCompleteRepair {
    #[default]
    NoRepairsPerformed = 0,
    AllRequestedrepairSperformed = 1,
    MotorEngine = 10,
    Starter = 20,
    Alternator = 30,
    Generator = 40,
    Battery = 50,
    EngineCoolantLeak = 60,
    FuelFilter = 70,
    TransmissionOilLeak = 80,
    EngineOilLeak = 90,
    Pumps = 100,
    Filters = 110,
    Transmission = 120,
    Brakes = 130,
    SuspensionSystem = 140,
    OilFilter = 150,
    Hull = 1000,
    Airframe = 1010,
    TruckBody = 1020,
    TankBody = 1030,
    TrailerBody = 1040,
    Turret = 1050,
    Propeller = 1500,
    Filters = 1520,
    Wheels = 1540,
    Tire = 1550,
    Track = 1560,
    GunElevationDrive = 2000,
    GunStabilizationSystem = 2010,
    GunnersPrimarySight = 2020,
    CommandersExtensionToTheGPS = 2030,
    LoadingMechanism = 2040,
    GunnersAuxiliarySight = 2050,
    GunnersControlPanel = 2060,
    GunnersControlAssemblyHandle = 2070,
    CommandersControlHandlesAssembly = 2090,
    CommandersWeaponStation = 2100,
    CommandersIndependentThermalViewer = 2110,
    GeneralWeapons = 2120,
    FuelTransferPump = 4000,
    FuelLines = 4010,
    Gauges = 4020,
    GeneralFuelSystem = 4030,
    ElectronicWarfareSystems = 4500,
    DetectionSystems = 4600,
    DetectionSystemsRadioFrequency = 4610,
    DetectionSystemsMicrowave = 4620,
    DetectionSystemsInfrared = 4630,
    DetectionSystemsLaser = 4640,
    RangeFinders = 4700,
    RangeOnlyrAdar = 4710,
    LaserRangeFinder = 4720,
    ElectronicSystems = 4800,
    ElectronicsSystemsRadioFrequency = 4810,
    ElectronicsSystemsMicrowave = 4820,
    ElectronicsSystemsInfrared = 4830,
    ElectronicsSystemsLaser = 4840,
    Radios = 5000,
    CommunicationSystems = 5010,
    Intercoms = 5100,
    Encoders = 5200,
    EncryptionDevices = 5250,
    Decoders = 5300,
    DecryptionDevices = 5350,
    Computers = 5500,
    NavigationAndControlSystems = 6000,
    FireControlSystems = 6500,
    AirSupply = 8000,
    Filters = 8010,
    WaterSupply = 8020,
    RefrigerationSystem = 8030,
    ChemicalBiologicalAndRadiologicalProtection = 8040,
    WaterWashdownSystems = 8050,
    DecontaminationSystems = 8060,
    Watersupply = 9000,
    Coolingsystem = 9010,
    Winches = 9020,
    Catapults = 9030,
    Cranes = 9040,
    Launchers = 9050,
    Lifeboats = 10000,
    Landingcraft = 10010,
    Ejectionseats = 10020,
}

// SISO-REF-010-2023 RepairResponseRepairResult [UID 65]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum RepairResponseRepairResult {
    #[default]
    Other = 0,
    RepairEnded = 1,
    InvalidRepair = 2,
    RepairInterrupted = 3,
    ServiceCanceledByTheSupplier = 4,
}

// SISO-REF-010-2023 VariableRecordTypes [UID 66]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum VariableRecordTypes {
    #[default]
    EntityIDList = 1,
    DDCPJoinTransactionJoinRequestMessage = 1001,
    DDCPSetPlaybackWindowTransactionSetPlaybackWindowRequestMessage = 1002,
    DDCPLoadMissionRecordingTransactionLoadMissionRecordingRequestMessage = 1003,
    DDCPCueTransactionCueRequestMessage = 1004,
    DDCPPlayTransactionPlayRequestMessage = 1005,
    DDCPStopTransactionStopRequestMessage = 1006,
    DDCPPauseTransactionPauseRequestMessage = 1007,
    DDCPEndTransactionEndRequestMessage = 1009,
    DDCPJoinResponseMessage = 1051,
    DDCPRequestReceiptMessage = 1052,
    DDCPPlaybackWindowConfirmedMessage = 1053,
    DDCPMissionRecordingLoadedMessage = 1054,
    DDCPCueConfirmedMessage = 1055,
    DDCPTimetoCompleteMessage = 1056,
    DDCPPlayCommencedMessage = 1057,
    DDCPStopConfirmedMessage = 1058,
    DDCPPauseConfirmedMessage = 1059,
    DDCPEndResponseMessage = 1061,
    DDCPMasterAnnounceMessage = 1111,
    DDCPDeviceAnnounceMessage = 1112,
    DDCPDeviceExitMessage = 1114,
    DDCPDeviceHeartbeatMessage = 1115,
    DDCPMasterTimeSyncMessage = 1116,
    DDCPErrorMessage = 1118,
    DDCPMasterStopSyncMessage = 1119,
    DDCPMasterTransitionMessage = 1120,
    MissionTime = 1200,
    HighFidelityHAVEQUICKSATURNRadio = 3000,
    BlankingSectorattributerecord = 3500,
    AngleDeceptionattributerecord = 3501,
    FalseTargetsattributerecord = 3502,
    DEPrecisionAimpointrecord = 4000,
    DEAreaAimpointrecord = 4001,
    DirectedEnergyDamageDescriptionrecord = 4500,
    CryptoControl = 5000,
    Mode5STransponderLocation = 5001,
    Mode5STransponderLocationError = 5002,
    SquitterAirbornePositionReport = 5003,
    SquitterAirborneVelocityReport = 5004,
    SquitterSurfacePositionReport = 5005,
    SquitterIdentificationReport = 5006,
    GICB = 5007,
    SquitterEventDrivenReport = 5008,
    AntennaLocation = 5009,
    BasicInteractive = 5010,
    InteractiveMode4Reply = 5011,
    InteractiveMode5Reply = 5012,
    InteractiveBasicMode5 = 5013,
    InteractiveBasicModeS = 5014,
    IOEffect = 5500,
    IOCommunicationsNode = 5501,
    Identification = 10000,
    TrainerInitialConditionsFilename = 10010,
    Increment3_1MissionDataLoadName = 10020,
    Increment2MissionDataLoadName = 10030,
    SetMarkpointCommand = 10110,
    MarkpointID = 10115,
    ReactionLevel = 10140,
    WeaponReload = 10150,
    CESEntitySetClearStatus = 10157,
    ActivateEntity = 10160,
    DisengageReengage = 10170,
    FuelFreeze = 10190,
    FireLaunchDispense = 10250,
    TargetAssignment = 10254,
    CICEnable = 10256,
    ShootInhibit = 10258,
    Posture = 10259,
    JammerState = 10262,
    JammerType = 10263,
    DynamicTargeting = 10264,
    ManualJammingOnOverride = 10267,
    SOJAxis = 10268,
    EmitterOverride = 10280,
    Shields = 10290,
    CrashOverride = 10300,
    StopBuzzer = 10306,
    TargetLasingOnOff = 10307,
    TargetLasingLaserCode = 10308,
    PowerPlant = 10310,
    TacticalLightingOnOffControlLightControl = 10311,
    TacticalLightingBlinkerControlBlinkerValue = 10312,
    TacticalLightingOnOffControlLightControlType = 10313,
    ParkVehicle = 10314,
    SignalingOnOff = 10315,
    SignalingDevice = 10316,
    OwnshipID = 10400,
    StateChange = 10600,
    EntityType = 11000,
    Concatenated = 11100,
    Kind = 11110,
    Domain = 11120,
    Country = 11130,
    Category = 11140,
    Subcategory = 11150,
    Specific = 11160,
    Extra = 11170,
    ForceID = 11180,
    ForceID = 11200,
    Description = 11300,
    TankerBoomControl = 11500,
    AirportLights = 11501,
    WeatherPost = 11502,
    LocalizerandGlideSlope = 11503,
    TACANNavAids = 11504,
    AlternativeEntityType = 12000,
    Kind = 12110,
    Domain = 12120,
    Country = 12130,
    Category = 12140,
    Subcategory = 12150,
    Specific = 12160,
    Extra = 12170,
    Description = 12300,
    EntityMarking = 13000,
    EntityMarkingCharacters = 13100,
    CrewID = 13200,
    TaskOrganization = 14000,
    RegimentName = 14200,
    BattalionName = 14300,
    CompanyName = 14400,
    PlatoonName = 14500,
    SquadName = 14520,
    TeamName = 14540,
    BumperNumber = 14600,
    VehicleNumber = 14700,
    UnitNumber = 14800,
    DISIdentity = 15000,
    DISSiteID = 15100,
    DISHostID = 15200,
    DISEntityID = 15300,
    MountIntent = 15400,
    TetherUnthetherCommandID = 15500,
    TeleportEntityDataRecord = 15510,
    DISAggregateID = 15600,
    OwnershipStatus = 15800,
    Reconstitute = 19177,
    Loads = 20000,
    CrewMembers = 21000,
    CrewMemberID = 21100,
    Health = 21200,
    JobAssignment = 21300,
    Fuel = 23000,
    Quantity = 23100,
    Quantity = 23105,
    Ammunition = 24000,
    _120mmHEATquantity = 24001,
    _120mmSABOTquantity = 24002,
    _12_7mmM8quantity = 24003,
    _12_7mmM20quantity = 24004,
    _7_62mmM62quantity = 24005,
    M250UKL8A1quantity = 24006,
    M250UKL8A3quantity = 24007,
    _7_62mmM80quantity = 24008,
    _12_7mmquantity = 24009,
    _7_62mmquantity = 24010,
    Minesquantity = 24060,
    Type = 24100,
    Kind = 24110,
    Domain = 24120,
    Country = 24130,
    Category = 24140,
    Subcategory = 24150,
    Extra = 24160,
    Description = 24300,
    Cargo = 25000,
    VehicleMass = 26000,
    SupplyQuantity = 27000,
    Armament = 28000,
    Status = 30000,
    Activateentity = 30010,
    SubscriptionState = 30100,
    Roundtriptimedelay = 30300,
    TADILJmessagecount = 30400,
    TADILJmessagecount = 30401,
    TADILJmessagecount = 30402,
    TADILJmessagecount = 30403,
    TADILJmessagecount = 30404,
    TADILJmessagecount = 30405,
    TADILJmessagecount = 30406,
    TADILJmessagecount = 30407,
    TADILJmessagecount = 30408,
    TADILJmessagecount = 30409,
    TADILJmessagecount = 30410,
    TADILJmessagecount = 30411,
    TADILJmessagecount = 30412,
    TADILJmessagecount = 30413,
    TADILJmessagecount = 30414,
    TADILJmessagecount = 30415,
    TADILJmessagecount = 30416,
    TADILJmessagecount = 30417,
    TADILJmessagecount = 30418,
    TADILJmessagecount = 30419,
    TADILJmessagecount = 30420,
    TADILJmessagecount = 30421,
    TADILJmessagecount = 30422,
    TADILJmessagecount = 30423,
    TADILJmessagecount = 30424,
    TADILJmessagecount = 30425,
    TADILJmessagecount = 30426,
    TADILJmessagecount = 30427,
    TADILJmessagecount = 30428,
    TADILJmessagecount = 30429,
    TADILJmessagecount = 30430,
    TADILJmessagecount = 30431,
    Position = 31000,
    Routetype = 31010,
    MilGrid10 = 31100,
    GeocentricCoordinates = 31200,
    X = 31210,
    Y = 31220,
    Z = 31230,
    Latitude = 31300,
    Longitude = 31400,
    LineofSight = 31500,
    X = 31510,
    Y = 31520,
    Z = 31530,
    Altitude = 31600,
    DestinationLatitude = 31700,
    DestinationLongitude = 31800,
    DestinationAltitude = 31900,
    Orientation = 32000,
    HullHeadingAngle = 32100,
    HullPitchAngle = 32200,
    RollAngle = 32300,
    X = 32500,
    Y = 32600,
    Z = 32700,
    Appearance = 33000,
    AmbientLighting = 33100,
    Lights = 33101,
    PaintScheme = 33200,
    Smoke = 33300,
    TrailingEffects = 33400,
    Flaming = 33500,
    Marking = 33600,
    MinePlowsAttached = 33710,
    MineRollersAttached = 33720,
    TankTurretAzimuth = 33730,
    FailuresandMalfunctions = 34000,
    Age = 34100,
    Kilometers = 34110,
    Damage = 35000,
    Cause = 35050,
    MobilityKill = 35100,
    FirePowerKill = 35200,
    PersonnelCasualties = 35300,
    Velocity = 36000,
    Xvelocity = 36100,
    Yvelocity = 36200,
    Zvelocity = 36300,
    Speed = 36400,
    Acceleration = 37000,
    Xacceleration = 37100,
    Yacceleration = 37200,
    Zacceleration = 37300,
    EngineStatus = 38100,
    PrimaryTargetLine = 39000,
    Exercise = 40000,
    ExerciseState = 40010,
    RestartRefresh = 40015,
    AFATDSFileName = 40020,
    TerrainDatabase = 41000,
    Missions = 42000,
    MissionID = 42100,
    MissionType = 42200,
    MissionRequestTimeStamp = 42300,
    ExerciseDescription = 43000,
    Name = 43100,
    Entities = 43200,
    Version = 43300,
    GuiseMode = 43410,
    SimulationApplicationActiveStatus = 43420,
    SimulationApplicationRoleRecord = 43430,
    SimulationApplicationState = 43440,
    VisualOutputMode = 44000,
    SimulationManagerRole = 44100,
    SimulationManagerSiteID = 44110,
    SimulationManagerApplicID = 44120,
    SimulationManagerEntityID = 44130,
    SimulationManagerActiveStatus = 44140,
    AfterActiveReviewRole = 44200,
    AfterActiveReviewSiteID = 44210,
    AfterActiveApplicID = 44220,
    AfterActiveReviewEntityID = 44230,
    AfterActiveReviewActiveStatus = 44240,
    ExerciseLoggerRole = 44300,
    ExerciseLoggerSiteID = 44310,
    ExerciseLoggerApplicID = 44320,
    ExerciseEntityID = 44330,
    ExerciseLoggerActiveStatus = 44340,
    SyntheticEnvironmentManagerRole = 44400,
    SyntheticEnvironmentManagerSiteID = 44410,
    SyntheticEnvironmentManagerApplicID = 44420,
    SyntheticEnvironmentManagerEntityID = 44430,
    SyntheticEnvironmentManagerActiveStatus = 44440,
    SIMNETDISTranslatorRole = 44500,
    SIMNETDISTranslatorSiteID = 44510,
    SIMNETDISTranslatorApplicID = 44520,
    SIMNETDISTranslatorEntityID = 44530,
    SIMNETDISTranslatorActiveStatus = 44540,
    ApplicationRate = 45000,
    ApplicationTime = 45005,
    ApplicationTimestep = 45010,
    FeedbackTime = 45020,
    SimulationRate = 45030,
    SimulationTime = 45040,
    SimulationTimestep = 45050,
    TimeInterval = 45060,
    TimeLatency = 45070,
    TimeScheme = 45080,
    ExerciseElapsedTime = 46000,
    ElapsedTime = 46010,
    Environment = 50000,
    ScenarioDate = 50103,
    TimeDateValid = 50106,
    ScenarioTime = 50118,
    SnowEnableDisable = 50120,
    WeatherAttributesRequest = 50124,
    METHeartbeatMessage = 50126,
    ContrailsEnable = 50600,
    ContrailAltitudes = 50700,
    Weather = 51000,
    WeatherCondition = 51010,
    ThermalCondition = 51100,
    ThermalVisibility = 51110,
    ThermalVisibility = 51111,
    Time = 52000,
    Time = 52001,
    TimeofDayDiscrete = 52100,
    TimeofDayContinuous = 52200,
    TimeMode = 52300,
    TimeScene = 52305,
    CurrentHour = 52310,
    CurrentMinute = 52320,
    CurrentSecond = 52330,
    Azimuth = 52340,
    MaximumElevation = 52350,
    TimeZone = 52360,
    TimeRate = 52370,
    TheNumberOfSimulationSecondsSinceTheStartOfTheExercise = 52380,
    TimeSunriseEnabled = 52400,
    SunriseHour = 52410,
    SunriseMinute = 52420,
    SunriseSecond = 52430,
    SunriseAzimuth = 52440,
    TimeSunsetEnabled = 52500,
    SunsetHour = 52510,
    SunsetHour = 52511,
    SunsetMinute = 52520,
    SunsetSecond = 52530,
    Date = 52600,
    Date = 52601,
    Date = 52602,
    Month = 52610,
    Day = 52620,
    Year = 52630,
    Clouds = 53000,
    CloudLayerEnable = 53050,
    CloudLayerSelection = 53060,
    Visibility = 53100,
    BaseAltitude = 53200,
    BaseAltitude = 53250,
    Ceiling = 53300,
    Ceiling = 53350,
    Characteristics = 53400,
    ConcentrationLength = 53410,
    Transmittance = 53420,
    Radiance = 53430,
    Precipitation = 54000,
    Rain = 54100,
    Fog = 55000,
    Visibility = 55100,
    Visibility = 55101,
    Visibility = 55105,
    Density = 55200,
    Base = 55300,
    ViewLayerfromabove_ = 55401,
    TransitionRange = 55410,
    Bottom = 55420,
    Bottom = 55425,
    Ceiling = 55430,
    Ceiling = 55435,
    HeavenlyBodies = 56000,
    Sun = 56100,
    SunVisible = 56105,
    Position = 56110,
    SunPositionElevationDegrees = 56111,
    PositionAzimuth = 56120,
    SunPositionAzimuthDegrees = 56121,
    PositionElevation = 56130,
    PositionIntensity = 56140,
    Moon = 56200,
    MoonVisible = 56205,
    Position = 56210,
    PositionAzimuth = 56220,
    MoonPositionAzimuthDegrees = 56221,
    PositionElevation = 56230,
    MoonPositionElevationDegrees = 56231,
    PositionIntensity = 56240,
    Horizon = 56310,
    HorizonAzimuth = 56320,
    HorizonElevation = 56330,
    HorizonHeading = 56340,
    HorizonIntensity = 56350,
    Humidity = 57200,
    Visibility = 57300,
    Winds = 57400,
    Speed = 57410,
    WindSpeedKnots = 57411,
    WindDirection = 57420,
    WindDirectionDegrees = 57421,
    Rainsoak = 57500,
    TideSpeed = 57610,
    TideSpeedKnots = 57611,
    TideDirection = 57620,
    TideDirectionDegrees = 57621,
    Haze = 58000,
    Visibility = 58100,
    Visibility = 58105,
    Density = 58200,
    Ceiling = 58430,
    Ceiling = 58435,
    ContaminantsandObscurants = 59000,
    ContaminantObscurantType = 59100,
    Persistence = 59110,
    ChemicalDosage = 59115,
    ChemicalAirConcentration = 59120,
    ChemicalGroundDeposition = 59125,
    ChemicalMaximumGroundDeposition = 59130,
    ChemicalDosageThreshold = 59135,
    BiologicalDosage = 59140,
    BiologicalAirConcentration = 59145,
    BiologicalDosageThreshold = 59150,
    BiologicalBinnedParticleCount = 59155,
    RadiologicalDosage = 59160,
    Communications = 60000,
    FireBottleReload = 61005,
    ChannelType = 61100,
    ChannelType = 61101,
    ChannelIdentification = 61200,
    AlphaIdentification = 61300,
    RadioIdentification = 61400,
    LandLineIdentification = 61500,
    IntercomIdentification = 61600,
    GroupNetworkChannelNumber = 61700,
    RadioCommunicationsStatus = 62100,
    BoomInterphone = 62101,
    StationaryRadioTransmittersDefaultTime = 62200,
    MovingRadioTransmittersDefaultTime = 62300,
    StationaryRadioSignalsDefaultTime = 62400,
    MovingRadioSignalDefaultTime = 62500,
    RadioInitializationTransecSecurityKey = 63101,
    RadioInitializationInternalNoiseLevel = 63102,
    RadioInitializationSquelchThreshold = 63103,
    RadioInitializationAntennaLocation = 63104,
    RadioInitializationAntennaPatternType = 63105,
    RadioInitializationAntennaPatternLength = 63106,
    RadioInitializationBeamDefinition = 63107,
    RadioInitializationTransmitHeartbeatTime = 63108,
    RadioInitializationTransmitDistanceThresholdVariableRecord = 63109,
    RadioChannelInitializationLockoutID = 63110,
    RadioChannelInitializationHopsetID = 63111,
    RadioChannelInitializationPresetFrequency = 63112,
    RadioChannelInitializationFrequencySyncTime = 63113,
    RadioChannelInitializationComsecKey = 63114,
    RadioChannelInitializationAlpha = 63115,
    AlgorithmParameters = 70000,
    DeadReckoningAlgorithm = 71000,
    DRALocationThreshold = 71100,
    DRAOrientationThreshold = 71200,
    DRATimeThreshold = 71300,
    SimulationManagementParameters = 72000,
    CheckpointInterval = 72100,
    TransmitterTimeThreshold = 72600,
    ReceiverTimeThreshold = 72700,
    InteroperabilityMode = 73000,
    SIMNETDataCollection = 74000,
    EventID = 75000,
    SourceSiteID = 75100,
    SourceHostID = 75200,
    ArticulatedParts = 90000,
    PartID = 90050,
    Index = 90070,
    Position = 90100,
    PositionRate = 90200,
    Extension = 90300,
    ExtensionRate = 90400,
    X = 90500,
    Xrate = 90600,
    Y = 90700,
    Yrate = 90800,
    Z = 90900,
    Zrate = 91000,
    Azimuth = 91100,
    AzimuthRate = 91200,
    Elevation = 91300,
    ElevationRate = 91400,
    Rotation = 91500,
    RotationRate = 91600,
    DRAAngularXVelocity = 100001,
    DRAAngularYVelocity = 100002,
    DRAAngularZVelocity = 100003,
    AppearanceTrailingEffects = 100004,
    AppearanceHatch = 100005,
    AppearanceCharacterSet = 100008,
    CapabilityAmmunitionSupplier = 100010,
    CapabilityMiscellaneousSupplier = 100011,
    CapabilityRepairProvider = 100012,
    ArticulationParameter = 100014,
    ArticulationParameterType = 100047,
    ArticulationParameterValue = 100048,
    TimeofDayScene = 100058,
    LatitudeNorth = 100061,
    LongitudeEast = 100063,
    TacticalDriverStatus = 100068,
    SonarSystemStatus = 100100,
    Accomplishedaccept = 100160,
    Upperlatitude = 100161,
    LatitudeSouth = 100162,
    Westernlongitude = 100163,
    LongitudeWest = 100164,
    CDROMNumber = 100165,
    DTEDdiskID = 100166,
    Altitude = 100167,
    TacticalSystemStatus = 100169,
    JTIDSStatus = 100170,
    TADILJStatus = 100171,
    DSDDStatus = 100172,
    WeaponSystemStatus = 100200,
    Subsystemstatus = 100205,
    Numberofinterceptorsfired = 100206,
    Numberofinterceptordetonations = 100207,
    Numberofmessagebuffersdropped = 100208,
    Satellitesensorbackground = 100213,
    Satellitesensorbackground = 100214,
    ScriptNumber = 100218,
    EntityTrackUpdateData = 100300,
    LocalForceTraining = 100400,
    EntityTrackIdentityData = 100500,
    EntityforTrackEvent = 100510,
    IFFstatus = 100520,
    EngagementData = 100600,
    TargetLatitude = 100610,
    TargetLongitude = 100620,
    AreaofInterestCenterLatitude = 100631,
    AreaofInterestCenterLongitude = 100632,
    AreaofInterestRadius = 100633,
    AreaofInterestType = 100634,
    TargetAggregateID = 100640,
    GICIdentificationNumber = 100650,
    EstimatedTimeofFlighttoTBMImpact = 100660,
    EstimatedInterceptTime = 100661,
    EstimatedTimeofFlighttoNextWaypoint = 100662,
    EntityTrackEquipmentData = 100700,
    EmissionEWData = 100800,
    AppearanceData = 100900,
    CommandOrderData = 101000,
    EnvironmentalData = 101100,
    SignificantEventData = 101200,
    OperatorActionData = 101300,
    ADAEngagementMode = 101310,
    ADAShootingStatus = 101320,
    ADAMode = 101321,
    ADARadarStatus = 101330,
    ShootCommand = 101340,
    ADAWeaponStatus = 101350,
    ADAFiringDisciple = 101360,
    OrderStatus = 101370,
    TimeSynchronization = 101400,
    TomahawkData = 101500,
    NumberofDetonations = 102100,
    NumberofIntercepts = 102200,
    OBTControlMT201 = 200201,
    SensorDataMT202 = 200202,
    EnvironmentalDataMT203 = 200203,
    OwnshipDataMT204 = 200204,
    AcousticContactDataMT205 = 200205,
    SonobuoyDataMT207 = 200207,
    SonobuoyContactDataMT210 = 200210,
    HeloControlMT211 = 200211,
    ESMControlData = 200213,
    ESMContactDataMT214 = 200214,
    ESMEmitterDataMT215 = 200215,
    WeaponDefinitionDataMT217 = 200216,
    WeaponPresetDataMT217 = 200217,
    OBTControlMT301 = 200301,
    SensorDataMT302 = 200302,
    EnvironmentalDataMT303m = 200303,
    OwnshipDataMT304 = 200304,
    AcousticContactDataMT305 = 200305,
    SonobuoyDataMT307 = 200307,
    SonobuoyContactDataMT310 = 200310,
    HeloScenarioEquipmentStatus = 200311,
    ESMControlDataMT313 = 200313,
    ESMContactDataMT314 = 200314,
    ESMEmitterDataMT315 = 200315,
    WeaponDefinitionDataMT316 = 200316,
    WeaponPresetDataMT317 = 200317,
    PairingAssociation = 200400,
    Pointer = 200401,
    ReportingResponsibility = 200402,
    TrackNumber = 200403,
    IDforLink11Reporting = 200404,
    RemoteTrack = 200405,
    Link11ErrorRate = 200406,
    TrackQuality = 200407,
    Gridlock = 200408,
    Kill = 200409,
    TrackIDChangeResolution = 200410,
    WeaponsStatus = 200411,
    Link11Operator = 200412,
    ForceTrainingTransmit = 200413,
    ForceTrainingReceive = 200414,
    InterceptorAmplification = 200415,
    Consumables = 200416,
    Link11LocalTrackQuality = 200417,
    DLRP = 200418,
    ForceOrder = 200419,
    WilcoCantco = 200420,
    EMCBearing = 200421,
    ChangeTrackEligibility = 200422,
    LandMassReferencePoint = 200423,
    SystemReferencePoint = 200424,
    PUAmplification = 200425,
    SetDrift = 200426,
    BeginInitialization = 200427,
    StatusandControl = 200428,
    ScintillationChange = 200429,
    Link11IDControl = 200430,
    PUGuardList = 200431,
    WindsAloft = 200432,
    SurfaceWinds = 200433,
    SeaState = 200434,
    MagneticVariation = 200435,
    TrackEligibility = 200436,
    TrainingTrackNotification = 200437,
    TacanData = 200501,
    InterceptorAmplification = 200502,
    TacanAssignment = 200503,
    AutopilotStatus = 200504,
    Consumables = 200505,
    Downlink = 200506,
    TINReport = 200507,
    SpecialPointControl = 200508,
    ControlDiscretes = 200509,
    RequestTargetDiscretes = 200510,
    TargetDiscretes = 200511,
    ReplyDiscretes = 200512,
    CommandManeuvers = 200513,
    TargetData = 200514,
    TargetPointer = 200515,
    InterceptData = 200516,
    DecrementMissileInventory = 200517,
    Link4AAlert = 200518,
    StrikeControl = 200519,
    SpeedChange = 200521,
    CourseChange = 200522,
    AltitudeChange = 200523,
    ACLSANSPN46Status = 200524,
    ACLSAircraftReport = 200525,
    SPS67RadarOperatorFunctions = 200600,
    SPS55RadarOperatorFunctions = 200601,
    SPQ9ARadarOperatorFunctions = 200602,
    SPS49RadarOperatorFunctions = 200603,
    MK23RadarOperatorFunctions = 200604,
    SPS48RadarOperatorFunctions = 200605,
    SPS40RadarOperatorFunctions = 200606,
    MK95RadarOperatorFunctions = 200607,
    KillNoKill = 200608,
    CMTpc = 200609,
    CMC4AirGlobalData = 200610,
    CMC4GlobalData = 200611,
    LINKSIMCOMMENTPDU = 200612,
    NSSTOwnshipControl = 200613,
    Other = 240000,
    MassOfTheVehicle = 240001,
    ForceID = 240002,
    EntityTypeKind = 240003,
    EntityTypeDomain = 240004,
    EntityTypeCountry = 240005,
    EntityTypeCategory = 240006,
    EntityTypeSubCategory = 240007,
    EntityTypeSpecific = 240008,
    EntityTypeExtra = 240009,
    AlternativeEntityTypeKind = 240010,
    AlternativeEntityTypeDomain = 240011,
    AlternativeEntityTypeCountry = 240012,
    AlternativeEntityTypeCategory = 240013,
    AlternativeEntityTypeSubCategory = 240014,
    AlternativeEntityTypeSpecific = 240015,
    AlternativeEntityTypeExtra = 240016,
    EntityLocationX = 240017,
    EntityLocationY = 240018,
    EntityLocationZ = 240019,
    EntityLinearVelocityX = 240020,
    EntityLinearVelocityY = 240021,
    EntityLinearVelocityZ = 240022,
    EntityOrientationPsi = 240023,
    EntityOrientationTheta = 240024,
    EntityOrientationPhi = 240025,
    DeadReckoningAlgorithm = 240026,
    DeadReckoningLinearAccelerationX = 240027,
    DeadReckoningLinearAccelerationY = 240028,
    DeadReckoningLinearAccelerationZ = 240029,
    DeadReckoningAngularVelocityX = 240030,
    DeadReckoningAngularVelocityY = 240031,
    DeadReckoningAngularVelocityZ = 240032,
    EntityAppearance = 240033,
    EntityMarkingCharacterSet = 240034,
    EntityMarking11Bytes = 240035,
    Capability = 240036,
    NumberArticulationParameters = 240037,
    ArticulationParameterID = 240038,
    ArticulationParameterType = 240039,
    ArticulationParameterValue = 240040,
    TypeOfStores = 240041,
    QuantityOfStores = 240042,
    FuelQuantity = 240043,
    RadarSystemStatus = 240044,
    RadioCommunicationSystemStatus = 240045,
    DefaultTimeForRadioTransmissionForStationaryTransmitters = 240046,
    DefaultTimeForRadioTransmissionForMovingTransmitters = 240047,
    BodyPartDamagedRatio = 240048,
    NameOfTheTerrainDatabaseFile = 240049,
    NameOfLocalFile = 240050,
    AimpointBearing = 240051,
    AimpointElevation = 240052,
    AimpointRange = 240053,
    AirSpeed = 240054,
    Altitude = 240055,
    ApplicationStatus = 240056,
    AutoIff = 240057,
    BeaconDelay = 240058,
    BingoFuelSetting = 240059,
    CloudBottom = 240060,
    CloudTop = 240061,
    Direction = 240062,
    EndAction = 240063,
    Frequency = 240064,
    Freeze = 240065,
    Heading = 240066,
    Identification = 240067,
    InitialPointData = 240068,
    Latitude = 240069,
    Lights = 240070,
    Linear = 240071,
    Longitude = 240072,
    LowAltitude = 240073,
    MfdFormats = 240074,
    Nctr = 240075,
    NumberProjectiles = 240076,
    OperationCode = 240077,
    Pitch = 240078,
    Profiles = 240079,
    Quantity = 240080,
    RadarModes = 240081,
    RadarSearchVolume = 240082,
    Roll = 240083,
    Rotation = 240084,
    ScaleFactorX = 240085,
    ScaleFactorY = 240086,
    Shields = 240087,
    Steerpoint = 240088,
    Spare1 = 240089,
    Spare2 = 240090,
    Team = 240091,
    Text = 240092,
    TimeOfDay = 240093,
    TrailFlag = 240094,
    TrailSize = 240095,
    TypeOfProjectile = 240096,
    TypeOfTarget = 240097,
    TypeOfThreat = 240098,
    UhfFrequency = 240099,
    UtmAltitude = 240100,
    UtmLatitude = 240101,
    UtmLongitude = 240102,
    VhfFrequency = 240103,
    VisibilityRange = 240104,
    VoidAaaHit = 240105,
    VoidCollision = 240106,
    VoidEarthHit = 240107,
    VoidFriendly = 240108,
    VoidGunHit = 240109,
    VoidRocketHit = 240110,
    VoidSamHit = 240111,
    WeaponData = 240112,
    WeaponType = 240113,
    Weather = 240114,
    WindDirection = 240115,
    WindSpeed = 240116,
    WingStation = 240117,
    Yaw = 240118,
    MemoryOffset = 240119,
    MemoryData = 240120,
    VASI = 240121,
    Beacon = 240122,
    Strobe = 240123,
    Culture = 240124,
    Approach = 240125,
    RunwayEnd = 240126,
    Obstruction = 240127,
    RunwayEdge = 240128,
    RampTaxiway = 240129,
    LaserBombCode = 240130,
    RackType = 240131,
    HUD = 240132,
    RoleFileName = 240133,
    PilotName = 240134,
    PilotDesignation = 240135,
    ModelType = 240136,
    DISType = 240137,
    Class = 240138,
    Channel = 240139,
    EntityType = 240140,
    AlternativeEntityType = 240141,
    EntityLocation = 240142,
    EntityLinearVelocity = 240143,
    EntityOrientation = 240144,
    DeadReckoning = 240145,
    FailureSymptom = 240146,
    MaxFuel = 240147,
    RefuelingBoomConnect = 240148,
    AltitudeAGL = 240149,
    CalibratedAirspeed = 240150,
    TACANChannel = 240151,
    TACANBand = 240152,
    TACANMode = 240153,
    FuelFlowRate = 270115,
    FuelTemperature = 270116,
    FuelPressure = 270117,
    SKESlot = 270150,
    SKELead = 270151,
    SKEFrequency = 270152,
    FCICmd = 270153,
    FCINum = 270154,
    SKEBitField = 270155,
    FormationPosition = 270156,
    FormationNumber = 270157,
    FFSModeActive = 270158,
    FFSRole = 270159,
    FFSVCAS = 270160,
    FFSBitField = 270161,
    FFSCallSign = 270162,
    FFSGuidanceData = 270163,
    FFSTextData = 270164,
    FFSAirdropRequestData = 270165,
    FFSAirdropData = 270166,
    HorizontalCircularErrorProbable = 300000,
    HorizontalPositionError = 300001,
    VerticalPositionError = 300002,
    HorizontalVelocityError = 300003,
    VerticalVelocityError = 300004,
    _4thLowestJammertoSignalRatioforP = 300005,
    _4thLowestJammertoSignalRatioforP = 300006,
    GPSFigureofMerit = 300007,
    WeaponTransferGPSState = 300008,
    WeaponTransferHorizontalPositionError = 300009,
    WeaponTransferVerticalPositionError = 300010,
    WeaponTransferVerticalPositionError = 300011,
    WeaponTransferHorizontalVelocityError = 300012,
    TimeTransferError = 300013,
    AgeofEphemeris = 300014,
    NonFlyoutMunitionEntityRequestDISTypeEnumeration = 300016,
    NonFlyoutMunitionEntityRequestLaunchPointX = 300017,
    NonFlyoutMunitionEntityRequestLaunchPointY = 300018,
    NonFlyoutMunitionEntityRequestLaunchPointZ = 300019,
    NonFlyoutMunitionEntityRequestMaximumAltitude = 300020,
    NonFlyoutMunitionEntityRequestFlightPath = 300021,
    NonFlyoutMunitionEntityRequestImpactPointX = 300022,
    NonFlyoutMunitionEntityRequestImpactPointY = 300023,
    NonFlyoutMunitionEntityRequestImpactPointZ = 300024,
    NonFlyoutMunitionEntityRequestElapsedFlightTime = 300025,
    NonFlyoutMunitionEntityRequestLaunchTime = 300026,
    TimeError = 300027,
    Link16CommandVariety1 = 301100,
    Push = 301130,
    Rolex = 301140,
    TerminateIntercept = 301150,
    HealDamage = 301151,
    Destroy = 301152,
    TransferControlManagement = 301160,
    Link16ControlsPPLIEnable = 301170,
    Link16ControlsCommandControlEnable = 301171,
    Link16ReferencePointMessageInitiation = 301174,
    AssignExternalEntityLink16TrackNumber = 301175,
    Link16IntelligenceInfo = 301176,
    Link16TrackManagement = 301177,
    Link16ControlsCESGlobalPPLIPublish = 301178,
    Link16ControlsCESGlobalSurveillancePublish = 301179,
    RequestGlobalLink16Configuration = 301180,
    Link16ControlsSurveillanceEnable = 301181,
    Link16Pointer = 301182,
    Link16Vector = 301183,
    Link16ControlUnitChange = 301184,
    Link16Text = 301185,
    RequestLink16Objects = 301186,
    Link16RefObjectNameList = 301187,
    TotalNumberofPDUsinLink16RefObjectsResponse = 301189,
    PDUNumberinLink16RefObjectsResponse = 301190,
    TotalNumberofLink16RefObjects = 301191,
    Link16ControlsF2FAEnable = 301197,
    Link16ControlsF2FBEnable = 301198,
    STNofFormationLeader = 301199,
    FormationName = 301200,
    FormationRole = 301201,
    SurveillanceContributorSensorBasedDetection = 301202,
    F2FANPG = 301220,
    Link16ControlsF2FANet = 301221,
    F2FBNPG = 301222,
    Link16ControlsF2FBNet = 301223,
    SurveillanceEnabledNPB = 301224,
    SurveillanceEnabledNet = 301225,
    ControlUnitEnabled = 301226,
    ControlUnitEnabledNPG = 301227,
    ControlUnitEnabledNet = 301228,
    VoiceFrequency = 301229,
    Link16JTIDSVoiceCallsign = 301234,
    EntityIDofControlUnit = 301237,
    STNofControlUnit = 301238,
    NTRParticipationLevel = 301239,
    Link16ControlsCESGlobalPPLISubscribe = 301240,
    Link16ControlsCESGlobalSurveillanceSubscribe = 301241,
    NTRinMission = 301242,
    NTRMarking = 301243,
    NTRReceiptCompliance = 301244,
    FormationF2FNPG = 301255,
    FormationF2FChannel = 301256,
    JLVCLogReport = 400008,
    JLVCSupplyAdjust = 400009,
    JLVCEntityControl = 400010,
    JLVCHealthUpdate = 400011,
    JLVCRepairComplete = 400012,
    JLVCUnitActivation = 400013,
    JLVCBattleDamageRepair = 400014,
    JLVCMinefield = 400015,
    JLVCWire = 400016,
    JLVCAbatis = 400017,
    JLVCCrater = 400018,
    JLVCDitch = 400019,
    JLVCLanes = 400020,
    JLVCIED = 400021,
    JLVCRubble = 400022,
    JLVCSubmergedBarrier = 400023,
    JLVCFloatingBarrier = 400024,
    JLVCFoxhole = 400025,
    JLVCVehicleHole = 400026,
    JLVCVehicleFortification = 400027,
    JLVCSandbag = 400028,
    JLVCCheckpoint = 400029,
    JLVCContamCloud2D = 400030,
    JLVCPopulationEffect = 400031,
    JLVCMine = 400032,
    JLVCSeaMinefield = 400033,
    Munition = 500001,
    EngineFuel = 500002,
    StorageFuel = 500003,
    NotUsed = 500004,
    Expendable = 500005,
    TotalRecordSets = 500006,
    LaunchedMunition = 500007,
    Association = 500008,
    Sensor = 500009,
    MunitionReload = 500010,
    EngineFuelReload = 500011,
    StorageFuelReload = 500012,
    ExpendableReload = 500013,
    IFFChangeControlMode1Code = 500014,
    IFFChangeControlMode2Code = 500015,
    IFFChangeControlMode3Code = 500016,
    IFFChangeControlMode4Code = 500017,
    IFFChangeControlMode5Code = 500018,
    IFFChangeControlMode6Code = 500019,
    Link16Data = 500021,
    ARMAlert = 500022,
    IFFChangeControlModeOnOff = 500023,
    WeaponStatusData = 500024,
    ExpendableStatusData = 500025,
    TacticStatusData = 500026,
    EmitterSensorData = 500027,
    IOSControlData = 500028,
    StaticStatusData = 500029,
    RequestInactiveEntities = 500200,
    InactiveEntityQuantity = 500201,
    InactiveEntityID = 500202,
    InactiveEntityType = 500203,
    ActivationTriggerType = 500204,
    ActivationTriggerValue = 500205,
    AirtoAirMissileQty = 551001,
    AIM7MissileQty = 551002,
    AIM9MissileQty = 551003,
    AIM120MissileQty = 551004,
    AirtoGroundMissileQty = 551005,
    SurfacetoAirMissileQty = 551006,
    BulletQty = 551007,
    ChaffQty = 552001,
    FlareQty = 552002,
    FuelLevel = 553001,
    RouteType = 553002,
    ThreatMode = 553003,
    TargetOccluded = 553004,
    TerrainHeight = 553005,
    EntityStatus = 553006,
    MarshalStatus = 553007,
    PowerPlantStatus = 553008,
    NavLightStatus = 553009,
    InteriorLightStatus = 553010,
    LandingLightStatus = 553011,
    FormationLightStatus = 553012,
    AntiCollisionLightStatus = 553013,
    NavFormationFlashRate = 553014,
    AntiColOnDuration = 553015,
    AntiColOffDuration = 553016,
    InterceptStatus = 553017,
    LifeFormSignalingDeviceType = 553018,
    LifeFormMovementType = 553019,
    LifeFormInVehicle = 553020,
    MobilityKill = 553021,
    FirepowerKill = 553022,
    TankerEnabledDisabled = 553028,
    ThreatStatusTacticOKtoShootDownWeapons = 553029,
    TACANChannel = 554001,
    TACANBand = 554002,
    TACANMode = 554003,
    RWRStatus = 554004,
    UHFRadioFrequency = 554005,
    EmitJammingStatus = 554006,
    EmitJammingType = 554007,
    ReceiveJammingStatus = 554008,
    RADARMode = 554009,
    AvailableRADARModes = 554010,
    JammerPodEnumeration = 554100,
    JammerPodBehavior = 554101,
    JammerPodPrograms = 554102,
    JammerPodReceiverSensitivity = 554103,
    JammerPodReceiverFrequencyMinimum = 554104,
    JammerPodReceiverFrequencyMaximum = 554105,
    JammerPodPower = 554106,
    JammerPodVariability = 554107,
    JammerPodNumberofFalseTargets = 554108,
    JammerPodJammerKnob = 554109,
    JammerPodMissileJamming = 554110,
    EmitterOverride = 555001,
    JammerOverride = 555002,
    DisengageReengage = 555003,
    HeadingOverride = 555004,
    AltitudeOverride = 555005,
    SpeedOverride = 555006,
    VerboseOverride = 555007,
    OcclusionOverride = 555008,
    CommitRange = 556001,
    CurrentScenarioIFFMode4ACodeforThisThreatsAffiliation = 556007,
    CurrentScenarioIFFMode4BCodeforThisThreatsAffiliation = 556008,
    OktoEngageWaypointNumber = 556016,
    MaxSpeedatSeaLevel = 556017,
    MaxSpeed = 556018,
    CurrentWaypointNumber = 556019,
    RouteInformation = 556020,
    ThreatStatusStaticMultiTargetTrack = 556029,
    AirAirIRMissileQty = 557001,
    AirAirRadarMissileQty = 557002,
    AirGroundIRMissileQty = 557003,
    AirGroundRadarMissileQty = 557004,
    AirGroundAntiRadiationMissileQty = 557005,
    AirGroundBombQty = 557006,
    AirGroundRocketQty = 557007,
    SurfaceAirIRMissileQty = 557008,
    SurfaceAirRadarMissileQty = 557009,
    BulletQty = 557010,
    PPLIPublishEnabled = 559001,
    SurveillancePublishEnabled = 559002,
    NPG = 559003,
    NPGChannel = 559004,
    JTIDSTrackNumber = 559005,
    Link16ControlsSurveillanceReportable = 559006,
    Link16ControlsSurveillanceTrackQuality = 559007,
    Link16ControlsTargetPositionQuality = 559008,
    Link16ControlsQualityErrorType = 559009,
    Link16ControlsAffiliationDeterminationRule = 559010,
    Link16ControlsResetEntityAffiliation = 559011,
    Link16ControlsResetAllAffiliation = 559012,
    EndofMessages = 559999,
    MalfunctionActivateDeactivateSet = 600001,
    MalfunctionStatus = 600002,
    RequestJTIDSTrackNumbers = 600210,
    TrackNumbersvsEID = 600212,
    TotalNumberofJTIDSTrackNumbers = 600214,
    PDUNumberinJTIDSTrackNumberResponse = 600215,
    TotalNumberofPDUsinJTIDSTrackNumberResponse = 600216,
    AirtoAirRefuelerEntitiesRequest = 600218,
    AirtoAirRefuelingCount = 600219,
    AirToAirRefuelerEntity = 600220,
    FormationLibraryRequest = 600300,
    TotalNumberFormationLibraryPDUs = 600301,
    PDUNumberinFormationLibraryResponse = 600302,
    TotalNumberFormationLibraryItemsinPDU = 600303,
    FormationLibraryVariable = 600304,
    CreateRuntimeFormation = 600305,
    FormationRequestHeader = 600306,
    FormationPositionAbsolute = 600307,
    FormationPositionRelative = 600308,
    ExpendablesReload = 610006,
    PositionFreeze = 610007,
    ActivateOwnship = 610008,
    Chocks = 610009,
    WarmupCooldownOverride = 610010,
    GroundPower = 610011,
    ScrambleStart = 610012,
    OwnshipasaThreat = 610013,
    FuelExternal = 610015,
    FuelInternal = 610016,
    FuelTankTemp = 610017,
    GrossWeight = 610025,
    AngleOfAttack = 610026,
    GLoad = 610027,
    WeightOnWheels = 610029,
    StoredEnergySystemReload = 610032,
    KillOverride = 610035,
    ExpendablesFreeze = 610036,
    GPSSatellitesEnableDisable = 610037,
    OwnshipMessageDisplay = 610040,
    WeaponQuantityFreeze = 610042,
    GlobalControlFreezeWeaponsQuantityOnAllOwnships = 610043,
    GlobalControlFreezeFuelQuantityOnAllOwnships = 610044,
    GlobalControlFreezeKillOverrideOnAllOwnships = 610045,
    GlobalControlFreezeCrashOverrideOnAllOwnships = 610046,
    OwnshipOFPBlockNumber = 610047,
    WaypointInformationQuery = 610048,
    WaypointInformation = 610049,
    OwnshipSubsystemStatusData = 610050,
    CockpitSwitchStatus = 613002,
    IntegratedControlPanelMessages = 613003,
    ThrottlePositions = 613004,
    CurrentCriticalSwitchPosition = 613005,
    CorrectCriticalSwitchPosition = 613006,
    CurrentCriticalSwitchData = 613007,
    CorrectCriticalSwitchData = 613008,
    MissionInitialConditionsSet = 613013,
    GlobalControlMalfunctionActiveonAllOwnships = 613016,
    GlobalControlMalfunctionClearOnAllOwnships = 613017,
    ValidatedCriticalSwitchReport = 613020,
    SARMapPathname = 613021,
    ValidatedCriticalSwitchOwnshipID = 613022,
    LowerBoomEventReport = 613027,
    RaiseBoomEventReport = 613028,
    BreakawayEventReport = 613029,
    CompleteEventReport = 613030,
    AuxCommPanelFrequencyDisplay = 613031,
    NetworkStationInformation = 615000,
    GlobalControlSelectNetworkStation = 615001,
    NetworkStationUnderGlobalControl = 615002,
    GlobalControlStillControlling = 615003,
    GlobalControlReleaseControlofNetworkStation = 615004,
    GlobalControlFreezeWeaponQuantity = 615005,
    GlobalControlFreezeFuelQuantity = 615006,
    GlobalControlFreezeKillOverride = 615007,
    GlobalControlFreezeCrashOverride = 615008,
    GlobalControlMalfunctionActive = 615009,
    GlobalControlMalfunctionClear = 615010,
    GlobalControlStartDevices = 615011,
    GlobalControlFreezeDevices = 615012,
    GlobalControlJTIDSCommand = 615013,
    NetworkStationICSetInformation = 615015,
    GlobalControlResetICSet = 615017,
    NumberofControllingUnits = 615018,
    NetworkStationJTIDSControllingUnits = 615019,
    NetworkStationJTIDSObjectiveTracks = 615020,
    NumberofReferenceObjects = 615021,
    NetworkStationJTIDSReferenceObjects = 615022,
    NetworkedStationStillUnderControl = 615023,
    GlobalControlDeleteThreatEntities = 615024,
    NetworkStationOwnshipCallsigns = 615025,
    GlobalControlRequestFormationLibraryData = 615026,
    TotalNumberFormationLibraryPDUs = 615027,
    PDUNumberinFormationLibraryResponse = 615028,
    TotalNumberFormationLibraryItemsinPDUs = 615029,
    NetworkStationFormationLibraryItem = 615030,
    GlobalControlAddRelativeFormation = 615031,
    NetworkStationTICFilename = 615032,
    GlobalControlFreezeWarmupOverride = 615033,
    GlobalControlReloadSES = 615034,
    GlobalControlReloadWeapons = 615035,
    GlobalControlReloadExpendables = 615036,
    GlobalControlReloadFuel = 615037,
    GlobalControlReloadFirebottle = 615038,
    TestPattern = 700000,
    AudioTest = 700001,
    AudioTone = 700002,
    CalibrateThrottles = 700003,
    OperationalLimitsEventReport = 700004,
    OperationalLimits = 700005,
    EventMarkerMessage = 1000620,
    ReceiverAircraftAeroModelData = 2000000,
    TankerAircraftAeroModelData = 2000010,
    BoomAircraftAeroModelData = 2000020,
    AccesstoImageGeneratorData = 2000030,
    AccesstoImageGeneratorData = 2000031,
    AccesstoImageGeneratorData = 2000032,
    AccesstoImageGeneratorData = 2000033,
    AccesstoImageGeneratorData = 2000034,
    AccesstoImageGeneratorData = 2000035,
    AccesstoImageGeneratorData = 2000036,
    AccesstoImageGeneratorData = 2000037,
    AccesstoImageGeneratorData = 2000038,
    AccesstoImageGeneratorData = 2000039,
    AccesstoImageGeneratorData = 2000040,
    AccesstoImageGeneratorData = 2000041,
    AccesstoImageGeneratorData = 2000042,
    AccesstoImageGeneratorData = 2000043,
    AccesstoImageGeneratorData = 2000044,
    AccesstoImageGeneratorData = 2000045,
    HostLoadNumber = 2000050,
    ExtendedFireEventReports = 5005001,
    BattleDamageAssessmentEventReport = 5005002,
    ExtendedFireEventLauncher = 5005003,
    ExtendedFireEventMissile = 5005006,
    ExtendedFireEventMRMWeapon = 5005008,
    ExtendedFireEventGunFireControl = 5005009,
    ExtendedFireEventBomb = 5005010,
    ExtendedFireEventExpendable = 5005011,
    BattleDamageAssessment = 5005012,
    ExtendedFirePickleEvent = 5005014,
    RadarTrackReport = 5005055,
    JammerReport = 5005060,
    JammerFalseTargetsReport = 5005061,
    DetectEventReport = 5005063,
    MALDBeamReport = 5005070,
    TransmitterRadiationVolume = 5005080,
    TransmitterRadiationVolumev2 = 5005081,
    PhysicalNetworkDefinition = 5007010,
    NetworkChannelDefinition = 5007020,
    LogicalNetworkDefinition = 5007030,
    LogicalNetworkEntityDefinition = 5007040,
    PhysicalNetworkEntityDefinition = 5007050,
    C2Message = 5008010,
    CandidateObject = 5008020,
    SetofCandidateObjects = 5008030,
    BoundedRegion = 5008040,
    AngularRegion = 5008050,
    RoEObject = 5008060,
    TrackObject = 5008070,
    SetofTrackObjects = 5008080,
    LogicalEntityDefinition = 5009010,
    LogicalEntityRelationshipDefinition = 5009020,
    IntentBasedEWMessage = 5507010,
}

// SISO-REF-010-2023 StopFreezeReason [UID 67]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum StopFreezeReason {
    #[default]
    Other = 0,
    Recess = 1,
    Termination = 2,
    SystemFailure = 3,
    SecurityViolation = 4,
    EntityReconstitution = 5,
    Stopforreset = 6,
    Stopforrestart = 7,
    AbortTrainingReturntoTacticalOperations = 8,
}

// SISO-REF-010-2023 AcknowledgeFlag [UID 69]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AcknowledgeFlag {
    #[default]
    CreateEntity = 1,
    RemoveEntity = 2,
    StartResume = 3,
    StopFreeze = 4,
    TransferOwnership = 5,
}

// SISO-REF-010-2023 AcknowledgeResponseFlag [UID 70]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AcknowledgeResponseFlag {
    #[default]
    Other = 0,
    Abletocomply = 1,
    Unabletocomply = 2,
    PendingOperatorAction = 3,
}

// SISO-REF-010-2023 ActionRequestActionID [UID 71]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ActionRequestActionID {
    #[default]
    Other = 0,
    Localstorageoftherequestedinformation = 1,
    InformSMofeventranoutofammunition = 2,
    InformSMofeventkilledinaction = 3,
    InformSMofeventdamage = 4,
    InformSMofeventmobilitydisabled = 5,
    InformSMofeventfiredisabled = 6,
    InformSMofeventranoutoffuel = 7,
    Recallcheckpointdata = 8,
    Recallinitialparameters = 9,
    Initiatetetherlead = 10,
    Initiatetetherfollow = 11,
    Unthether = 12,
    Initiateservicestationresupply = 13,
    Initiatetailgateresupply = 14,
    Initiatehitchlead = 15,
    Initiatehitchfollow = 16,
    Unhitch = 17,
    Mount = 18,
    Dismount = 19,
    StartDRC = 20,
    StopDRC = 21,
    DataQuery = 22,
    StatusRequest = 23,
    SendObjectStateData = 24,
    Reconstitute = 25,
    LockSiteConfiguration = 26,
    UnlockSiteConfiguration = 27,
    UpdateSiteConfiguration = 28,
    QuerySiteConfiguration = 29,
    TetheringInformation = 30,
    MountIntent = 31,
    AcceptSubscription = 33,
    Unsubscribe = 34,
    Teleportentity = 35,
    Changeaggregatestate = 36,
    RequestStartPDU = 37,
    Wakeupgetreadyforinitialization = 38,
    Initializeinternalparameters = 39,
    Sendplandata = 40,
    Synchronizeinternalclocks = 41,
    Run = 42,
    Saveinternalparameters = 43,
    Simulatemalfunction = 44,
    Joinexercise = 45,
    Resignexercise = 46,
    Timeadvance = 47,
    TACCSFLOSRequestType1 = 100,
    TACCSFLOSRequestType2 = 101,
    AirmountMountRequest = 4303,
    AirmountDismountRequest = 4304,
    AirmountInformationRequest = 4305,
}

// SISO-REF-010-2023 ActionResponseRequestStatus [UID 72]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ActionResponseRequestStatus {
    #[default]
    Other = 0,
    Pending = 1,
    Executing = 2,
    PartiallyComplete = 3,
    Complete = 4,
    Requestrejected = 5,
    Retransmitrequestnow = 6,
    Retransmitrequestlater = 7,
    Invalidtimeparameters = 8,
    Simulationtimeexceeded = 9,
    Requestdone = 10,
    TACCSFLOSReplyType1 = 100,
    TACCSFLOSReplyType2 = 101,
    JoinExerciseRequestRejected = 201,
}

// SISO-REF-010-2023 EventReportEventType [UID 73]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EventReportEventType {
    #[default]
    Other = 0,
    RanOutofAmmunition = 2,
    KilledinAction = 3,
    Damage = 4,
    MobilityDisabled = 5,
    FireDisabled = 6,
    RanOutofFuel = 7,
    EntityInitialization = 8,
    RequestforIndirectFireorCASMission = 9,
    IndirectFireorCASFire = 10,
    MinefieldEntry = 11,
    MinefieldDetonation = 12,
    VehicleMasterPowerOn = 13,
    VehicleMasterPowerOff = 14,
    AggregateStateChangeRequested = 15,
    PreventCollisionDetonation = 16,
    OwnershipReport = 17,
    RadarPerception = 18,
    Detect = 19,
}

// SISO-REF-010-2023 RequiredReliabilityService [UID 74]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum RequiredReliabilityService {
    #[default]
    Acknowledged = 0,
    Unacknowledged = 1,
}

// SISO-REF-010-2023 EmitterName [UID 75]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
#[allow(deprecated, non_camel_case_types)]
pub enum EmitterName {
    #[default]
    _12456X = 2,
    _1L117 = 3,
    _1L121E = 4,
    _1L250 = 5,
    _1L220U = 6,
    _1L1221E = 7,
    _1RL257 = 9,
    _1RL138 = 10,
    _1RL257Jammer = 11,
    _5N20 = 12,
    _5H62B = 13,
    _5P10 = 14,
    _5P10E = 15,
    _5P1001 = 16,
    _5P1001E = 17,
    _5P1002 = 18,
    _5P1002E = 19,
    _5P1003 = 20,
    _5P1003E = 21,
    _5P10EMOD = 22,
    _621A3 = 25,
    _860F1AL101 = 40,
    _9B1103M2 = 42,
    _1226DECCAMIL = 45,
    _9B1348 = 46,
    _3KM6 = 47,
    _9KR400 = 48,
    _50N6A = 49,
    _55G61 = 50,
    _59N6 = 55,
    _5N69 = 57,
    _67N6 = 60,
    _76T6 = 63,
    _77T6ABM = 64,
    _80K6 = 65,
    _91N6A = 66,
    _96L6E = 70,
    _96L6TsP = 75,
    _9C18M3 = 76,
    _9C36M = 77,
    _9GR400 = 80,
    _9GR400A = 81,
    _9GR600 = 90,
    _9GR606 = 91,
    _9LV100 = 125,
    _9LV200TA = 135,
    _9LV200TV = 180,
    _9LV200TT = 181,
    _9LV200MKIII = 183,
    _9LV326 = 185,
    _9M96E2Seeker = 190,
    _9S15M2 = 195,
    _9S19M2 = 196,
    _9S19ME = 197,
    _9S32M = 198,
    _9S32ME = 199,
    _9S36E = 200,
    _9S112 = 215,
    A310Z = 225,
    A325A = 270,
    A346Z = 315,
    A353B = 360,
    A372A = 405,
    A372B = 450,
    A372C = 495,
    A377A = 540,
    A377B = 585,
    A380Z = 630,
    A381Z = 675,
    A398Z = 720,
    A403Z = 765,
    A409A = 810,
    A418A = 855,
    A419Z = 900,
    A429Z = 945,
    A432Z = 990,
    A434Z = 1035,
    AA6CAcrid = 1070,
    AA7CApex = 1073,
    A401A = 1080,
    AA10A = 1081,
    AA10CAlamoC = 1082,
    AA13ArrowSeeker = 1085,
    AAM4BMH = 1090,
    AA300 = 1094,
    AA12Seeker = 1095,
    AD4A = 1096,
    ADES = 1097,
    ADS4LRSR = 1098,
    ACR430 = 1099,
    Agave = 1100,
    ACSOPRIE = 1101,
    ABD2000 = 1102,
    ADACMK1 = 1110,
    ADACMK2 = 1111,
    ADAR = 1113,
    ADOUR = 1115,
    AGAT9B1348 = 1117,
    AdrosKT01AV = 1118,
    Agat9E420 = 1120,
    AGM158JASSMSAR = 1122,
    AGM88HARMMMW = 1123,
    AGRION15 = 1125,
    AHV7 = 1130,
    AHV17 = 1150,
    AIMK23 = 1170,
    AIDAII = 1215,
    AIM120A = 1216,
    AIM7MSparrow = 1218,
    _1L271 = 1230,
    ALA51 = 1240,
    AlbatrosMK2 = 1260,
    ALT50 = 1263,
    ALTAIR = 1264,
    AMAPS717 = 1265,
    AMES13MK1 = 1268,
    WGU16B = 1270,
    _1L133 = 1280,
    _1L133 = 1282,
    AMDR3D = 1288,
    ANASPS502 = 1305,
    Anemone = 1306,
    ANRITSUElectricAR30A = 1350,
    AntilopeV = 1395,
    ANAAQ24 = 1397,
    ANADM160 = 1398,
    AN_ALE50 = 1400,
    AN_ALQ76 = 1410,
    AN_ALQ99 = 1440,
    AN_ALQ99Band4 = 1441,
    AN_ALQ99LBT = 1442,
    AN_ALQ100 = 1485,
    AN_ALQ101 = 1530,
    AN_ALQ119 = 1575,
    AN_ALQ122 = 1585,
    AN_ALQ126A = 1620,
    AN_ALQ128 = 1621,
    AN_ALQ126B = 1622,
    AN_ALQ131 = 1626,
    AN_ALQ131BlkII = 1627,
    AN_ALQ135CD = 1628,
    AN_ALQ144A3 = 1630,
    AN_ALQ153 = 1632,
    AN_ALQ157Jammer = 1633,
    AN_ALQ155 = 1634,
    AN_ALQ156 = 1635,
    AN_ALQ161A = 1636,
    AN_ALQ161 = 1637,
    AN_ALQ162 = 1638,
    AN_ALQ164 = 1639,
    AN_ALQ165 = 1640,
    AN_ALQ187Jammer = 1641,
    AN_ALQ167 = 1642,
    AN_ALQ1721 = 1643,
    AN_ALQ1722 = 1644,
    AN_ALQ1723 = 1645,
    AN_ALQ176 = 1646,
    AN_ALQ178 = 1647,
    AN_ALQ184 = 1648,
    AN_ALQ1849 = 1649,
    AN_ALQ188 = 1650,
    AN_ALQ214 = 1651,
    AN_ALR56 = 1652,
    AN_ALQ221 = 1653,
    AN_ALR69 = 1654,
    AN_ALQ211 = 1655,
    AN_ALT16A = 1656,
    AN_ALQ173 = 1657,
    AN_ALT28 = 1658,
    AN_ALR66BJammer = 1659,
    AN_ALT32A = 1660,
    AN_ALQ196 = 1661,
    ALQ249NextGenerationJammer = 1662,
    AN_ALQ2401Jammer = 1663,
    AN_ALR66BJammer = 1664,
    AN_APD10 = 1665,
    AN_ALQ213 = 1670,
    ALQ214A45Jammer = 1672,
    AN_ALQ218 = 1680,
    ANAPG50 = 1700,
    ANAPG53 = 1710,
    ANAPG59 = 1755,
    ANAPG63 = 1800,
    ANAPG631 = 1805,
    ANAPG632 = 1807,
    ANAPG633 = 1809,
    ANAPG65 = 1845,
    ANAPG66 = 1870,
    ANAPG66 = 1871,
    ANAPG662 = 1872,
    ANAPG67 = 1880,
    ANAPG68 = 1890,
    ANAPG689 = 1895,
    ANAPG70 = 1935,
    ANAPG71 = 1940,
    ANAPG73 = 1945,
    ANAPG77 = 1960,
    ANAPG78 = 1970,
    ANAPG79 = 1971,
    ANAPG80 = 1972,
    ANAPG81 = 1974,
    ANAPG821 = 1975,
    ANAPG83 = 1976,
    ANAPG502 = 1980,
    ANAPN1 = 2025,
    ANAPN22 = 2070,
    ANAPN59 = 2115,
    ANAPN69 = 2160,
    ANAPN81 = 2205,
    ANAPN102 = 2220,
    ANAPN117 = 2250,
    ANAPN118 = 2295,
    ANAPN122 = 2320,
    ANAPN130 = 2340,
    ANAPN131 = 2385,
    ANAPN133 = 2430,
    ANAPN134 = 2475,
    ANAPN141 = 2476,
    ANAPN147 = 2520,
    ANAPN150 = 2565,
    ANAPN153 = 2610,
    ANAPN154 = 2655,
    ANAPN155 = 2700,
    ANAPN159 = 2745,
    ANAPN177 = 2746,
    ANAPN179 = 2747,
    ANAPN169 = 2748,
    ANAPN182 = 2790,
    ANAPN187 = 2835,
    ANAPN190 = 2880,
    ANAPN194 = 2925,
    ANAPN195 = 2970,
    ANAPN198 = 3015,
    ANAPN200 = 3060,
    ANAPN202 = 3105,
    ANAPN215 = 3106,
    ANAPN209 = 3120,
    ANAPN209D = 3121,
    ANAPN209A = 3122,
    ANAPN215 = 3148,
    ANAPN217 = 3150,
    ANAPN218 = 3152,
    ANAPN224 = 3153,
    ANAPN227 = 3154,
    ANAPN230 = 3155,
    ANAPN232 = 3156,
    ANAPN237A = 3157,
    ANAPN234 = 3158,
    ANAPN235 = 3159,
    ANAPN238 = 3160,
    ANAPN222 = 3161,
    ANAPN239 = 3162,
    ANAPN241 = 3164,
    ANAPN242 = 3166,
    ANAPN243 = 3170,
    ANAPN506 = 3195,
    ANAPQ72 = 3240,
    ANAPQ99 = 3285,
    ANAPQ100 = 3330,
    ANAPQ102 = 3375,
    ANAPQ107 = 3376,
    ANAPQ109 = 3420,
    ANAPQ113 = 3465,
    ANAPQ120 = 3510,
    ANAPQ122 = 3512,
    ANAPQ126 = 3555,
    ANAPQ128 = 3600,
    ANAPQ129 = 3645,
    ANAPQ148 = 3690,
    ANAPQ150A = 3700,
    ANAPQ153 = 3735,
    ANAPQ155 = 3770,
    ANAPQ159 = 3780,
    ANAPQ164 = 3785,
    ANAPQ166 = 3788,
    ANAPQ170 = 3790,
    ANAPQ174 = 3791,
    ANAPQ180 = 3794,
    ANAPQ181 = 3795,
    ANAPQ186 = 3800,
    ANAPS15J = 3810,
    ANAPS162 = 3813,
    ANAPS31 = 3820,
    ANAPS42 = 3825,
    ANAPS80 = 3870,
    ANAPS88 = 3915,
    ANAPS88A = 3916,
    ANAPS94 = 3920,
    ANAPS96 = 3922,
    ANAPS113 = 3958,
    ANAPS115 = 3960,
    ANAPS116 = 4005,
    ANAPS120 = 4050,
    ANAPS121 = 4095,
    ANAPS124 = 4140,
    ANAPS125 = 4185,
    ANAPS127 = 4190,
    ANAPS128 = 4230,
    ANAPS130 = 4275,
    ANAPS133 = 4320,
    ANAPS134 = 4365,
    ANAPS137 = 4410,
    ANAPS1375 = 4413,
    ANAPS137B = 4415,
    ANAPS137B5 = 4420,
    ANAPS137D5Elta = 4425,
    ANAPS138 = 4455,
    ANAPS139 = 4460,
    ANAPS143 = 4464,
    ANAPS1431 = 4465,
    ANAPS143B = 4466,
    ANAPS1433 = 4467,
    ANAPS143B3 = 4468,
    ANAPS153 = 4475,
    ANAPS154 = 4476,
    ANAPS150 = 4480,
    ANAPS145 = 4482,
    ANAPS147 = 4485,
    ANAPS149 = 4486,
    ANAPS503 = 4489,
    ANAPS504 = 4490,
    ANAPS705 = 4491,
    ANAPW22 = 4500,
    ANAPW23 = 4545,
    ANAPX6 = 4590,
    ANAPX7 = 4635,
    ANAPX39 = 4680,
    ANAPX64 = 4681,
    ANAPX72 = 4725,
    ANAPX76 = 4770,
    ANAPX78 = 4815,
    ANAPX100 = 4816,
    ANAPX101 = 4860,
    ANAPX113AIFF = 4870,
    ANAPY1 = 4900,
    ANAPY2 = 4905,
    ANAPY3 = 4950,
    ANAPY7 = 4952,
    ANAPY8 = 4953,
    ANAPY9 = 4954,
    ANAPY10 = 4955,
    ANARN21 = 4995,
    ANARN52 = 5040,
    ANARN84 = 5085,
    ANARN118 = 5130,
    ANARN153 = 5131,
    ANARN153 = 5165,
    ANARW73 = 5175,
    ANASB1 = 5220,
    ANASG21 = 5265,
    ANASN137 = 5266,
    ANASN128 = 5270,
    ANASQ108 = 5280,
    ANASQ239 = 5285,
    ANAST502 = 5290,
    ANAVQ55 = 5300,
    ANAWG9 = 5310,
    ANBRN1 = 5320,
    ANBPS5 = 5325,
    ANBPS9 = 5355,
    ANBPS15 = 5400,
    ANBPS15H = 5401,
    ANBPS15J = 5402,
    ANBPS16 = 5405,
    ANBPS162 = 5406,
    ANCPN4 = 5410,
    ANCPN18 = 5415,
    ANCRM30 = 5420,
    ANDPW23 = 5430,
    ANDSQ26PhoenixMH = 5445,
    ANDSQ28HarpoonMH = 5490,
    ANFPN1 = 5491,
    ANFPN28 = 5493,
    ANFPN33 = 5494,
    ANFPN40 = 5495,
    ANFPN62 = 5500,
    ANFPN66 = 5502,
    ANFPS8 = 5503,
    ANFPN67 = 5504,
    ANFPS16 = 5505,
    ANFPS5 = 5506,
    ANFPS18 = 5507,
    ANFPS89 = 5508,
    ANFPS49 = 5509,
    ANFPS117 = 5510,
    ANFPS85 = 5511,
    ANFPS88 = 5512,
    ANFPS113 = 5513,
    ANFPS115 = 5514,
    ANFPS20R = 5515,
    ANFPS132 = 5516,
    ANFPS77 = 5520,
    ANFPS41 = 5521,
    ANFPS100A = 5522,
    ANFPS103 = 5525,
    ANFPS108 = 5526,
    ANGPN12 = 5527,
    ANFPS124 = 5528,
    ANFPS129 = 5529,
    ANGPX6 = 5530,
    ANGPX8 = 5535,
    ANGRN12 = 5537,
    ANMPN14K = 5538,
    ANMPN14 = 5539,
    ANMPQ10 = 5540,
    ANMPN17 = 5541,
    ANMPQ3339465761ILL = 5545,
    ANMPQ34485562TA = 5550,
    ANMPQ49 = 5551,
    ANMPQ3550TA = 5555,
    ANMPQ50C = 5556,
    ANMPQ3751TT = 5560,
    ANMPQ43 = 5565,
    ANMPQ50 = 5567,
    ANMPQ53 = 5570,
    ANMPQ63 = 5571,
    ANMPQ64 = 5575,
    ANSLQ32 = 5576,
    ANMPQ65 = 5577,
    ANSLQ324 = 5578,
    ANSLQ32A = 5579,
    ANSPG34 = 5580,
    ANMSQ104 = 5582,
    ANMPS36 = 5583,
    ANSLQ503 = 5584,
    ANSPG48MK25MOD3 = 5620,
    ANSPG50 = 5625,
    ANSPG51 = 5670,
    ANPPQ2 = 5690,
    ANPPS15 = 5700,
    ANPPS5 = 5705,
    ANPPS5D = 5710,
    ANSPG51CWITI = 5715,
    ANSPG51FC = 5760,
    ANSPG51CD = 5761,
    ANSPG52 = 5805,
    ANSPG53 = 5850,
    ANSPG55B = 5895,
    ANSPG60 = 5940,
    ANSPG62 = 5985,
    ANSPG503 = 5995,
    ANSPN4 = 6015,
    ANSPN11 = 6025,
    ANSPN35 = 6030,
    ANSPN41 = 6050,
    ANSPN43 = 6075,
    ANSPN43A = 6076,
    ANSPN43C = 6078,
    ANSPN46 = 6085,
    ANSPQ2 = 6120,
    ANSPQ5A = 6155,
    ANSPQ9A = 6165,
    ANSPQ9B = 6166,
    ANSPQ34 = 6190,
    ANSPS4 = 6210,
    ANSPS5 = 6255,
    ANSPS5C = 6300,
    ANSPS6 = 6345,
    ANSPS10 = 6390,
    ANSPS21 = 6435,
    ANSPS28 = 6480,
    ANSPS37 = 6525,
    ANSPS39A = 6570,
    ANSPS40 = 6615,
    ANSPS41 = 6660,
    ANSPS48 = 6705,
    ANSPS48C = 6750,
    ANSPS48E = 6752,
    ANSPS49 = 6795,
    ANSPS491 = 6796,
    ANSPS492 = 6797,
    ANSPS493 = 6798,
    ANSPS494 = 6799,
    ANSPS495 = 6800,
    ANSPS496 = 6801,
    ANSPS497 = 6802,
    ANSPS498 = 6803,
    ANSPS49A1 = 6804,
    ANSPS52 = 6840,
    ANSPS53 = 6885,
    ANSPS55 = 6930,
    ANSPS52C = 6945,
    ANSPS55CS = 6970,
    ANSPS55SS = 6975,
    ANSPS58 = 7020,
    ANSPS58C = 7025,
    ANSPS59 = 7065,
    ANSPS64 = 7110,
    ANSPS649 = 7119,
    SPS6412 = 7120,
    ANSPS65 = 7155,
    ANSPS66 = 7175,
    ANSPS67 = 7200,
    ANSPS73 = 7201,
    ANSPS69 = 7210,
    ANSPS73 = 7215,
    ANSPS74 = 7216,
    ANSPS88 = 7225,
    ANSPS501 = 7226,
    ANSPS505 = 7230,
    ANSPY1 = 7245,
    ANSPY1A = 7250,
    ANSPY1B = 7252,
    ANSPY1B = 7253,
    ANSPY1D = 7260,
    ANSPY1D = 7261,
    ANSPY1F = 7265,
    ANSPY3 = 7266,
    ANTPN12 = 7267,
    ANSPY4 = 7268,
    ANTLQ32ARMDecoy = 7269,
    ANTPN17 = 7270,
    ANTPN8 = 7271,
    ANTPN22 = 7272,
    ANTLQ17A = 7273,
    ANTMS1 = 7274,
    ANTPN24 = 7275,
    ANTPN25 = 7276,
    ANTMS2 = 7277,
    ANTPN19 = 7278,
    ANTPN31 = 7279,
    ANTPQ18 = 7280,
    ANSPY6 = 7281,
    ANTPQ36 = 7295,
    ANTPQ37 = 7300,
    ANTPQ38 = 7301,
    ANTPQ39 = 7302,
    ANTPQ47 = 7303,
    ANTPS43 = 7305,
    ANTPS43E = 7310,
    ANTPQ48 = 7311,
    ANTPQ49 = 7312,
    ANTPQ46A = 7313,
    ANTPS34 = 7314,
    ANTPS59 = 7315,
    ANTPS44 = 7316,
    ANTPQ50 = 7317,
    ANTPS63 = 7320,
    ANTPS65 = 7321,
    ANTPS701 = 7322,
    ANTPS63SS = 7323,
    ANTPS73 = 7324,
    ANTPS75 = 7325,
    ANTPS77 = 7326,
    ANTPS78 = 7327,
    ANTPS79 = 7328,
    ANTPS703 = 7329,
    ANTPX467 = 7330,
    ANTPS80 = 7331,
    ANTPY2 = 7333,
    ANTSQ288 = 7334,
    ANULQ6A = 7335,
    ANULQ19 = 7340,
    ANULQ21 = 7345,
    ANUPN25 = 7380,
    ANUPS1 = 7425,
    ANUPS2 = 7426,
    ANUPS3 = 7427,
    ANUPX1 = 7470,
    ANUPX5 = 7515,
    ANUPX11 = 7560,
    ANUPX12 = 7605,
    ANUPX17 = 7650,
    ANUPX23 = 7695,
    ANUSQ1133 = 7700,
    ANVPS2 = 7740,
    ANPLM3 = 7750,
    ANPLM3A = 7751,
    ANPLM4 = 7752,
    ANZPY1 = 7753,
    ANZPY2MPRTIP = 7754,
    ANZPY3 = 7755,
    ANZPY8 = 7760,
    AORL1AS = 7761,
    AORL85KTKMTA = 7762,
    APAR = 7765,
    Aparna = 7770,
    APECSII = 7780,
    ApelcoAD77 = 7785,
    APG71 = 7830,
    APN148 = 7875,
    APN227 = 7920,
    #[deprecated]
    APQ113 = 7965,
    #[deprecated]
    APQ120 = 8010,
    #[deprecated]
    APQ148 = 8055,
    APS504V3 = 8100,
    AQUITAINEII = 8102,
    AR1 = 8103,
    AR3D = 8105,
    PlesseyAR5 = 8112,
    AR15 = 8113,
    AR152 = 8114,
    AR320 = 8115,
    AR325 = 8118,
    AR327 = 8120,
    Arbalet52 = 8121,
    ARBB31 = 8122,
    ARBB33 = 8123,
    ARIES = 8126,
    AriesNav = 8127,
    AriesCS = 8128,
    ARGS14E = 8134,
    ARGS31 = 8135,
    ARGUS = 8140,
    ARM31 = 8145,
    ARECIBO = 8150,
    ARED = 8160,
    ARI5954 = 8190,
    ARI5955 = 8235,
    ARI5979 = 8280,
    ARGSN31 = 8281,
    ARGOS10 = 8282,
    ARGOS800 = 8283,
    ARI5983 = 8284,
    ARI5991 = 8285,
    ARI5995 = 8290,
    ARINC564BNDXKINGRDR1E = 8325,
    ARINC700BNDXKINGRDR1E = 8370,
    ARK1 = 8375,
    ARLMMTI = 8378,
    ARMOR = 8379,
    ARSR3 = 8380,
    ARS400 = 8381,
    ARSR1 = 8382,
    ARSR4 = 8384,
    ARSR18 = 8390,
    ARTHUR = 8395,
    ARTHURMODB = 8400,
    ARTHURMODC = 8405,
    ARTISAN3D = 8410,
    AS2Kipper = 8415,
    AS2KipperMH = 8460,
    AS3YJ83KmmWMH = 8470,
    AS_34KormoranSeeker = 8480,
    AS4Kitchen = 8505,
    AS4KitchenMH = 8550,
    AS5KeltMH = 8595,
    AS6KingfishMH = 8640,
    AS7Kerry = 8685,
    AS7KerryMG = 8730,
    AS15KENTaltimeter = 8735,
    AS17AKryptonSeeker = 8736,
    AS17BKryptonSeeker = 8737,
    AS901 = 8750,
    AS901A = 8751,
    ASARS2 = 8755,
    ASDEKDD = 8756,
    ASLESHA = 8757,
    ASMGCS = 8758,
    ASMI18X = 8759,
    AspideAAMSAMILL = 8760,
    ASMI3 = 8761,
    AselsanMAR = 8762,
    ASR2000 = 8771,
    ASR4 = 8772,
    ASR4D = 8773,
    ASRO = 8775,
    ASR12 = 8776,
    ASR22AL = 8778,
    ASR3 = 8779,
    ASR5 = 8780,
    ASR7 = 8782,
    ASR8 = 8785,
    ASR9 = 8790,
    ASR9000 = 8791,
    ASTI = 8792,
    ASR11DASR = 8793,
    ASR12 = 8795,
    RaytheonASR10SS = 8812,
    ASR23SS = 8816,
    Arabel = 8818,
    ASTRE = 8819,
    AT2SwatterMG = 8820,
    _9K114ShturmMG = 8824,
    ASTOR = 8825,
    ASTRARCI = 8826,
    ATCR22 = 8830,
    ATCR22M = 8831,
    ATCR2T = 8832,
    ATCR33 = 8840,
    ATCR33KM = 8845,
    ATCR33S = 8846,
    ATCR3T = 8847,
    ATCR44 = 8848,
    ATCR44K = 8849,
    Argos73 = 8850,
    ATCR44MS = 8851,
    ATCR4T = 8852,
    AtlasElektronkTRSN = 8865,
    ATLAS8600X = 8866,
    Atlas9600M = 8867,
    ATLAS9600X = 8868,
    ATLAS9600S = 8869,
    ATLAS9740VTS = 8870,
    ATLASS = 8871,
    ATR500C = 8880,
    AVG65 = 8910,
    AVH7 = 8955,
    AVIACM = 8980,
    AVIAD = 8985,
    Aviaconversia = 8990,
    AviaconversiaII = 8993,
    AviaconversiaIII = 8995,
    AVQ20 = 9000,
    AVQ21 = 9005,
    AVQ30X = 9045,
    AVQ50 = 9075,
    AVQ70 = 9090,
    AWS5 = 9135,
    AWS6 = 9180,
    AWS6B300 = 9185,
    B597Z = 9200,
    B636Z = 9205,
    BackBoard = 9215,
    BackNetAB = 9225,
    BackTrap = 9270,
    BAESystemsRT1805APN = 9280,
    BAESDASS2000Jammer = 9281,
    BalanceBeam = 9285,
    BALTIKAB = 9300,
    BALTYK = 9310,
    BallEnd = 9315,
    BallGun = 9360,
    BALLPOINT = 9370,
    BandStand = 9405,
    BandStand3 = 9406,
    P3537 = 9450,
    BARAX = 9475,
    BASIR110D = 9485,
    BassTilt = 9495,
    Badger = 9505,
    BarracudaJammer = 9510,
    BaykalCountermeasuresSuite = 9530,
    Beacon = 9540,
    BeanSticks = 9585,
    BeeHind = 9630,
    BellNipJammer = 9638,
    BellPushJammer = 9639,
    BellCrownA = 9640,
    BellCrownB = 9642,
    BellSquat = 9643,
    BIGBACK = 9645,
    BigBirdABC = 9659,
    BigBirdD = 9660,
    BigBirdDMod = 9661,
    BigBirdE91N6E = 9662,
    BigBulge = 9675,
    BigBulgeA = 9720,
    BigBulgeB = 9765,
    BIGEYE = 9775,
    SNAR10 = 9780,
    BIGHEADB = 9781,
    BigMesh = 9810,
    BigNet = 9855,
    _9S15MT = 9885,
    BillFold = 9900,
    BLIGHTER400 = 9903,
    BlowpipeMG = 9905,
    BLR = 9920,
    BlueFox = 9930,
    BlueKestrel = 9933,
    BlueVixen = 9935,
    BlueSilk = 9945,
    BlueParrot = 9990,
    BlueOrchid = 10035,
    BMDJG8715 = 10057,
    BoatSail = 10080,
    BORA550 = 10090,
    BoforsElectronic9LV331 = 10125,
    BoforsEricssonSeaGiraffe50HC = 10170,
    BowlMesh = 10215,
    BoxBrick = 10260,
    BoxTrail = 10305,
    BMKG300GJammingPod = 10308,
    BMKG600JammingPod = 10310,
    BMKG800JammingPod = 10312,
    BMKG860186058606 = 10315,
    BPS11A = 10350,
    BPS14 = 10395,
    BPS15A = 10440,
    BR3440CAX57 = 10450,
    BR15TokyoKEIKI = 10485,
    BrahMos = 10500,
    BridgeMaster = 10510,
    BridgeMasterEEPA = 10511,
    BridgeMasterEATAandARPA = 10512,
    BridgeMasterEnaval = 10513,
    BrimstonemmWMH = 10520,
    BreadBin = 10530,
    Asr = 10540,
    BT271 = 10575,
    BU304 = 10595,
    BX732 = 10620,
    BUKMB = 10630,
    BuranD = 10642,
    BUREVISNYK1 = 10650,
    BuzzStand = 10665,
    C5AMultiModeRadar = 10710,
    C802AL = 10711,
    CAESAR = 10740,
    Caiman = 10755,
    CakeStand = 10800,
    CalypsoC61 = 10845,
    CalypsoC63 = 10846,
    CalypsoIi = 10890,
    CalypsoIII = 10891,
    CalypsoIV = 10892,
    CardionCoastal = 10895,
    CastorIi = 10935,
    Castor2JTT = 10940,
    CatHouse = 10980,
    CDR431 = 10985,
    CEAFAR = 10987,
    CEAMOUNT = 10988,
    CEAFAR2L = 10989,
    CEROS200 = 10990,
    CEROS200CWI = 10991,
    CEATAC = 10992,
    CEAOPS = 10993,
    CerberusIII = 10994,
    CHSSN6 = 10995,
    CerberusIV = 10996,
    ChairBlackTT = 11000,
    ChairBlackILL = 11010,
    LEMZ96L6 = 11020,
    CheeseBrick = 11025,
    CheeseCake = 11030,
    LeninetzObzorMS = 11070,
    Clamshell = 11115,
    CLC1 = 11117,
    CLC2 = 11118,
    CLC3 = 11119,
    CLR155 = 11120,
    COASTWATCHER100 = 11123,
    CoastalGiraffe = 11125,
    COBRA = 11130,
    CobraShoe = 11133,
    Colibri = 11137,
    CollinsWXR300 = 11155,
    CollinsWXR700X = 11160,
    CollinsTWR850 = 11165,
    CollinsDN101 = 11205,
    COMET1 = 11230,
    CONDORMK2 = 11235,
    ConsiliumSelesmarRTM25XIM = 11240,
    ContravesSeaHunterMK4 = 11250,
    CornCan = 11260,
    COSMOSKYMED1 = 11265,
    CR105RMCA = 11270,
    CREWDuke2 = 11280,
    CREWDuke3 = 11290,
    CrossBird = 11295,
    CrossDome = 11340,
    CrossLegs = 11385,
    CrossOut = 11430,
    CrossSlot = 11475,
    CrossSword = 11520,
    CrossUp = 11565,
    CrossSwordFC = 11610,
    CrotaleAcquisitionTA = 11655,
    CrotaleNGTA = 11660,
    CrotaleTT = 11665,
    CrotaleMGMissileSystem = 11700,
    CS10TA = 11715,
    CSFVaran = 11725,
    CSSN4MH = 11735,
    CSSC3CCAS1M1M2MH = 11745,
    HY2BMH = 11748,
    CSSC2BHY1AMH = 11790,
    CSSN4Sardine = 11800,
    CSSN8Saccade = 11810,
    CurlStoneB = 11825,
    CWS1 = 11830,
    CWS2 = 11835,
    CWS3 = 11840,
    Cygnus = 11860,
    CylinderHead = 11880,
    Cymbeline = 11902,
    CyranoII = 11925,
    CyranoIV = 11970,
    CyranoIVM = 11975,
    DA0100 = 12010,
    DA0500 = 12015,
    DA052 = 12016,
    DA_08 = 12018,
    Dawn = 12060,
    DCR = 12090,
    DeadDuck = 12105,
    DECCA20V909 = 12110,
    DECCA20V90S = 12111,
    DECCA45 = 12150,
    DECCA50 = 12195,
    DECCA71 = 12196,
    Decca72 = 12197,
    DECCA110 = 12240,
    DECCA170 = 12285,
    DECCAHF2 = 12292,
    DECCA202 = 12330,
    DECCAD202 = 12375,
    DECCA303 = 12420,
    DECCA535 = 12430,
    DECCA626 = 12465,
    DECCA629 = 12510,
    DECCA914 = 12555,
    DECCA916 = 12600,
    DECCA926 = 12610,
    DECCA1070A = 12615,
    Decca1008 = 12616,
    DECCA1226Commercial = 12645,
    DECCA1290 = 12655,
    DECCA1626 = 12690,
    DECCA2070 = 12691,
    Decca1630 = 12694,
    DECCA2459 = 12735,
    DECCAAWS1 = 12780,
    DECCAAWS2 = 12782,
    DECCAAWS4 = 12785,
    DECCAAWS42 = 12787,
    DECCAMAR = 12800,
    DECCARM326 = 12805,
    DECCARM416 = 12825,
    DECCARM970BT = 12850,
    DECCARM914 = 12870,
    DF21DSeeker = 12875,
    DECCARM1690 = 12915,
    DECCA1690 = 12916,
    DECCASuper101MK3 = 12960,
    DISS1 = 13005,
    DISS7 = 13006,
    DISS013 = 13007,
    DISS15D = 13015,
    DLD100A = 13020,
    RapierTTDN181 = 13050,
    Rapier2000TT = 13055,
    DogEar = 13095,
    DogHouse = 13140,
    DM3 = 13141,
    DM3B = 13142,
    DM5 = 13143,
    Don2 = 13185,
    DonAB2Kay = 13230,
    Donets = 13275,
    Doppler90Series = 13280,
    DownBeat = 13320,
    DR582 = 13360,
    DRAA2A = 13365,
    DRAA2B = 13410,
    DRAA9A = 13415,
    DRAA11A = 13420,
    DRAC37B = 13450,
    DRAC38 = 13452,
    DRAC39 = 13455,
    DRAC39A = 13456,
    DRAC43A = 13460,
    DRAC44A = 13465,
    DragonEye = 13477,
    DragonEye2 = 13480,
    DragonEye3 = 13481,
    DragonEye4 = 13485,
    DRBC30B = 13500,
    DRBC31A = 13545,
    DRBC31D = 13546,
    DRBC32 = 13585,
    DRBC32A = 13590,
    DRBC32D = 13635,
    DRBC33A = 13680,
    DRBI10 = 13725,
    DRBI23 = 13770,
    DRBJ11B = 13815,
    DRBN30 = 13860,
    DRBN32 = 13905,
    DRBN34 = 13915,
    DRBR51 = 13950,
    DRBV20A = 13994,
    DRBV20B = 13995,
    DRBV21Mars05 = 14020,
    DRBV22 = 14040,
    DRBV23 = 14041,
    DRBV26C = 14085,
    DRBV26D = 14086,
    DRBV30 = 14130,
    DRBV31 = 14131,
    DRBV50 = 14175,
    DRBV51 = 14220,
    DRBV51A = 14265,
    DRBV51B = 14310,
    DRBV51C = 14355,
    DropKick = 14400,
    DRUA31 = 14445,
    DrumTilt = 14490,
    DrumTiltA = 14535,
    DrumTiltB = 14545,
    DRUN30A = 14560,
    Dumbo = 14580,
    DWSR92 = 14583,
    DWSR93S = 14585,
    EAGLE = 14586,
    EAGLEMk1 = 14587,
    EAJPJammingPod = 14588,
    EKCOE390 = 14590,
    ECR90 = 14600,
    ECR90Jammer = 14601,
    EggCupAB = 14625,
    EISCAT = 14640,
    EKCOE120 = 14660,
    EKCO190 = 14670,
    Ekran1 = 14677,
    ELL8222 = 14710,
    ELL8240 = 14713,
    ELM2001B = 14715,
    ELM2022 = 14725,
    ELM2032 = 14726,
    ELM2052 = 14727,
    ELM2055 = 14728,
    ELM2060 = 14730,
    ELM2075 = 14735,
    ELM2022U3 = 14736,
    ELM2080 = 14737,
    ELM2080S = 14738,
    ELM2085 = 14739,
    ELM2106 = 14740,
    ELM2106NG = 14741,
    ELM2125 = 14742,
    ELM2129 = 14743,
    ELM2150 = 14744,
    ELM2083 = 14745,
    ELM2084 = 14746,
    ELM2160V1 = 14747,
    ELM2084MMR = 14748,
    ELM2112 = 14749,
    ELM2200 = 14750,
    ELM2133 = 14751,
    ELM2205 = 14755,
    ELM2207 = 14760,
    ELM2215 = 14765,
    ELM2216V = 14770,
    ELM2216XH = 14772,
    ELM2218S = 14775,
    ELT361 = 14776,
    ELM2258 = 14777,
    ELT553 = 14779,
    ELT558 = 14780,
    ELT572 = 14785,
    ELT715 = 14790,
    EltaELM2022A = 14800,
    ELTAELM2221GMSTGR = 14805,
    ELM2228S3D = 14806,
    ELM2705 = 14807,
    ELM2226 = 14808,
    ELM2228X = 14809,
    ELTASIS = 14810,
    ELM2238 = 14811,
    ELM2248 = 14815,
    ELM2288 = 14820,
    ELM2311 = 14821,
    ELM2026 = 14822,
    ELNA4007 = 14830,
    ELT318 = 14831,
    ELW2085 = 14832,
    ELT521 = 14833,
    ELW2090 = 14835,
    EnhancedMeteorDetectionRadarEMDR = 14845,
    EMD2900 = 14850,
    EMPAR = 14851,
    EndTray = 14895,
    EQ36 = 14896,
    EricssonSLAR = 14897,
    Erieye = 14898,
    ESR1 = 14900,
    ESR220 = 14901,
    ESR380 = 14902,
    ESTEREL = 14903,
    ET316 = 14905,
    ExocetType = 14935,
    ExocetAL = 14936,
    Exocet1 = 14940,
    Exocet1MH = 14985,
    Exocet2 = 15030,
    EyeBowl = 15075,
    EyeShield = 15120,
    F332Z = 15140,
    FalconClawTI = 15155,
    FalconClawTT = 15156,
    FALCON = 15160,
    FALCONG = 15161,
    FalconEye = 15163,
    FanSongA = 15165,
    FanSongBFTA = 15200,
    FanSongBFTT = 15210,
    FanSongCETA = 15220,
    FanSongCETT = 15230,
    FanSongCEMG = 15240,
    FanSongBFFMG = 15255,
    FanTail = 15300,
    FAR2117 = 15301,
    FAR2827 = 15302,
    FAR2837S = 15303,
    FAR3000 = 15304,
    FB7Radar = 15305,
    FCR1401 = 15310,
    FCS212E = 15312,
    FCS212G = 15313,
    FCS221A = 15315,
    FCS221C = 15317,
    FCS222 = 15318,
    FCS231 = 15319,
    FCS3 = 15320,
    FinCurve = 15345,
    FireCan = 15390,
    FireDish = 15435,
    FireDomeTA = 15470,
    FireDomeTT = 15475,
    FireDomeTI = 15480,
    FireIron = 15525,
    FireWheel = 15570,
    FishBowl = 15615,
    FK3 = 15620,
    FLAIR = 15650,
    FlapLid = 15660,
    _30N6E = 15661,
    FlapTruck = 15705,
    FlapWheel = 15750,
    FlashDance = 15795,
    FlashDanceM = 15800,
    P15 = 15840,
    _35N6 = 15842,
    FlatScreen = 15885,
    FlatSpin = 15930,
    FlatTrackJammer = 15970,
    FlatTwin = 15975,
    FL400 = 15980,
    FL1800 = 15985,
    FL1800U = 15990,
    FL1800S = 16000,
    Fledermaus = 16020,
    FLYCATCHER = 16030,
    FLYCATCHERMK2 = 16035,
    FlyScreen = 16065,
    FlyScreenAB = 16110,
    FlyTrapB = 16155,
    FM90 = 16160,
    FogLampMG = 16200,
    FogLampTT = 16245,
    FoilTwo = 16290,
    FootBall = 16300,
    FoxHunter = 16335,
    FoxFireAL = 16380,
    FoxFireILL = 16390,
    FR151A = 16400,
    FurunoFR1500FR1600 = 16405,
    FR1505DA = 16410,
    FR1510DS = 16412,
    FR2000 = 16420,
    Furuno2855W = 16421,
    FregatMAE = 16422,
    FregatN1 = 16423,
    FregatN2 = 16424,
    FrontDome = 16425,
    FregatMAE5 = 16426,
    FrontDoor = 16470,
    FrontPiece = 16515,
    FurbymmWMH = 16520,
    Furke = 16550,
    Furke2 = 16552,
    Furke4 = 16554,
    Furuno = 16560,
    Furuno1721 = 16561,
    Furuno1934C = 16564,
    Furuno1715 = 16565,
    Furuno1730 = 16580,
    Furuno1731Mark3 = 16581,
    Furuno1832 = 16585,
    Furuno1835 = 16587,
    Furuno1932 = 16590,
    Furuno1935 = 16596,
    Furuno701 = 16605,
    Furuno1940 = 16606,
    Furuno7112 = 16650,
    FurunoFR2130S = 16652,
    FurunoFAR2137S = 16654,
    FurunoFAR28X7 = 16655,
    FurunoFAR3230S = 16658,
    FR2110 = 16660,
    FR2115 = 16662,
    FR8062 = 16663,
    Furuno2125 = 16670,
    Furuno240 = 16690,
    Furuno2400 = 16695,
    FR801D = 16725,
    Furuno8051 = 16730,
    FurunoDRS2D = 16732,
    FurunoDRS4D = 16733,
    FurunoDRS4A = 16734,
    G030A = 16735,
    FurunoDRS6AXClass = 16736,
    DRS4W = 16737,
    GA0100 = 16740,
    Gabbiano = 16750,
    Gage = 16785,
    Gaofen3 = 16787,
    GAOFEN12 = 16789,
    GAPGATE = 16790,
    Gardenia = 16800,
    GarminGMR1224 = 16815,
    GarminFantom24 = 16820,
    GarminGWX68WeatherRadar = 16825,
    Garpin = 16830,
    GateGuard = 16833,
    GarpunBalE = 16835,
    GBS1 = 16840,
    GCA2000 = 16850,
    Furuno18321921Series = 16858,
    GEMINIDB = 16870,
    GEMOlympus = 16871,
    GEMSentinel = 16872,
    GEMBX132 = 16875,
    GEMSC2050X = 16876,
    GEMSeaEagle200N = 16877,
    GenericInternalJammer = 16879,
    MPDR12 = 16880,
    GENX = 16881,
    GepardTT = 16884,
    GERANF = 16888,
    GERFAUT = 16890,
    GFE1 = 16895,
    GIRAFFE = 16900,
    GIRAFFE1X = 16903,
    Giraffe40 = 16905,
    Giraffe50AT = 16908,
    Giraffe75 = 16912,
    GinSlingTA = 16915,
    GinSling = 16920,
    GinSlingMG = 16925,
    GoalKeeper = 16930,
    GoldenDome = 16935,
    GoldenHeart = 16940,
    GoldenRock = 16942,
    GPN22 = 16945,
    GPSJ10 = 16946,
    GPSJ25 = 16947,
    GPSJ40 = 16948,
    GPSJ50 = 16949,
    GRN9 = 16950,
    GRANK = 16951,
    GraveStone = 16960,
    GRAVES = 16963,
    GreenStain = 16965,
    GridBow = 17010,
    GrifoF = 17016,
    _9S32 = 17025,
    GRILLSCREEN = 17027,
    Grom2 = 17029,
    GROUNDMASTER400 = 17030,
    GT4 = 17031,
    GRS440 = 17032,
    GUARDIAN = 17050,
    Guardsman = 17055,
    RPK2 = 17070,
    HRJZ7264AJammer = 17075,
    H025 = 17079,
    HADR = 17080,
    HairNet = 17100,
    HalfPlateA = 17145,
    HalfPlateB = 17190,
    HARD = 17220,
    Harpoon = 17225,
    HatBox = 17230,
    HawkScreech = 17235,
    HayPole = 17250,
    HayRick = 17255,
    HeadLightA = 17280,
    HeadLights = 17325,
    HeadLightsC = 17370,
    HeadLightsMGA = 17415,
    HeadLightsMGB = 17460,
    HeadLightsTT = 17505,
    HeadNet = 17550,
    HeartAcheB = 17572,
    HellfiremmWMH = 17590,
    HenEgg = 17595,
    HenHouse = 17640,
    HenNest = 17685,
    HenRoost = 17730,
    Herakles = 17732,
    HF2MG = 17735,
    HGR105 = 17745,
    HighBrick = 17775,
    HighFix = 17820,
    HighGuard = 17842,
    HighLarkTI = 17865,
    HighLark1 = 17910,
    HighLark2 = 17955,
    HighLark4 = 18000,
    HighLune = 18045,
    HighPoleAB = 18090,
    HighScoop = 18135,
    _9S19MT = 18150,
    HighSieve = 18180,
    HillBlock = 18185,
    HimalayasCountermeasuresSuite = 18189,
    HG9550 = 18190,
    HJ6374 = 18193,
    HLDRADAR900 = 18194,
    HLJQ520 = 18195,
    HN503 = 18200,
    HNC03M = 18201,
    HomeTalk = 18225,
    HornSpoon = 18270,
    HotBrick = 18280,
    HotFlash = 18315,
    HotFlash2 = 18316,
    IHS6 = 18318,
    IRL144M = 18320,
    IRL144M = 18325,
    IRL144M = 18330,
    HPS106 = 18331,
    HPS104 = 18332,
    HQ9MH = 18339,
    HSR1128 = 18340,
    HT233 = 18348,
    HQ61 = 18350,
    HRJS = 18351,
    IDerbyER = 18352,
    IBIS80 = 18353,
    IBIS150 = 18355,
    IBIS200 = 18357,
    IFFMKXIIAIMSUPX29 = 18360,
    Janet = 18400,
    IFFMKXV = 18405,
    IFFINT = 18406,
    JackKnife = 18407,
    IFFTRSP = 18408,
    JMUSICElbitSystemsJammer = 18409,
    JavelinMG = 18410,
    JALQ8 = 18445,
    JFPS7 = 18449,
    JayBird = 18450,
    JFPS3 = 18451,
    JH10 = 18452,
    JMPQP7 = 18453,
    JL7 = 18454,
    JL10B = 18455,
    JMA1576 = 18456,
    JRCJMA92526CA = 18457,
    JLP40 = 18458,
    JRCJMR9200SeriesX = 18459,
    JRCNMD401 = 18460,
    JRCJRM310MK2 = 18461,
    JMA1596 = 18462,
    JMA7000 = 18464,
    JRCJMA7700 = 18465,
    JMA5320 = 18466,
    JRCJMR92106XC = 18467,
    JERS1 = 18468,
    JINDALEE = 18469,
    JRCJMA9900series = 18470,
    JLP40D = 18471,
    JRCJMA5300series = 18475,
    Jupiter = 18495,
    JupiterII = 18540,
    JY8 = 18550,
    JY8A = 18551,
    JY9 = 18555,
    JY9Modified = 18556,
    JY11EW = 18557,
    JY14 = 18560,
    JY14A = 18561,
    JY16 = 18565,
    JY24 = 18570,
    JAPG1 = 18571,
    JAPG2 = 18572,
    JY29 = 18575,
    JYL1 = 18578,
    JYL6 = 18580,
    JYL6A = 18582,
    JZQF612 = 18583,
    K376Z = 18585,
    K77M = 18586,
    Kaige = 18600,
    KALKAN = 18610,
    KBPAfganit = 18611,
    KALKANII = 18615,
    KelvinHughes2A = 18630,
    KelvinHughes149 = 18675,
    Karpaty = 18700,
    Kashtan3JammingSystem = 18710,
    KelvinHughestype1006 = 18720,
    KelvinHughestype1007 = 18765,
    KelvinHughesType1007FBand = 18766,
    KelvinHughes2007FBand = 18767,
    KelvinHughes2007IBand = 18768,
    KHNucleus5000 = 18770,
    KHMANTA = 18774,
    KHNUCLEUS26000 = 18775,
    KHNUCLEUS35000 = 18776,
    KHNUCLEUS36000A = 18777,
    KHFamily = 18780,
    KelvinHughes6000A = 18781,
    Kh38MAEMH = 18782,
    KG8605A = 18784,
    KH902M = 18785,
    KHOROMK = 18786,
    KHIBINY = 18787,
    KG300E = 18789,
    KHSharpEye = 18790,
    KHSharpEyeB = 18791,
    KHSharpEyeC = 18792,
    KH1700 = 18795,
    KingPin = 18797,
    KG300 = 18805,
    KiteScreech = 18810,
    KiteScreechA = 18855,
    KiteScreechB = 18900,
    KLC3B = 18930,
    KJ500NanjingRadar = 18944,
    Kivach = 18945,
    KJ500Jammer = 18946,
    KLC1 = 18947,
    KLJ1 = 18948,
    KLJ3 = 18950,
    KLJ4 = 18951,
    KLJ4B = 18952,
    KLJ5 = 18955,
    KLJ7 = 18960,
    KLJ7B = 18961,
    KLJ7A = 18962,
    KnifeRest = 18990,
    P10 = 19035,
    KNIFERESTC = 19037,
    KodenMD3730 = 19039,
    KJ2000 = 19040,
    KODENMDC900 = 19041,
    Koopol = 19042,
    KOPYOI = 19045,
    KR75 = 19050,
    KRONOS = 19051,
    KREDO1E = 19052,
    Krasukha2 = 19053,
    KRONOSGRANDNAVAL = 19054,
    KRM66E = 19060,
    KSASRN = 19080,
    KSATSR = 19125,
    KS1APHASEDARRAY = 19127,
    KS418 = 19129,
    KS418E = 19130,
    KZ100 = 19131,
    KZ900 = 19132,
    L175V = 19140,
    L3705PresidentSJammer = 19142,
    L415 = 19143,
    L88 = 19145,
    LAADS = 19150,
    LandFall = 19170,
    LandRollMG = 19215,
    LandRollTA = 19260,
    LandRollTT = 19305,
    LAZUR = 19306,
    Model791A = 19307,
    LAP3000 = 19309,
    LC150 = 19310,
    LEER3 = 19320,
    LegDrive = 19330,
    LeninetzV004 = 19340,
    Leningraf = 19350,
    LIANA = 19370,
    LightBulb = 19395,
    LIRAA10 = 19396,
    LIROD8 = 19397,
    LIRODMKII = 19398,
    LLX05K = 19399,
    LMTNRAI6A = 19400,
    LN55 = 19440,
    Ln66 = 19485,
    Liman = 19500,
    Liman2 = 19505,
    LockheedVigilance = 19520,
    LongBow = 19530,
    LongBrick = 19575,
    LongBull = 19620,
    LongEye = 19665,
    LongHead = 19710,
    LongTalk = 19755,
    LongTrack = 19800,
    LongTrough = 19845,
    LookTwo = 19890,
    LOPAR = 19920,
    LORAN = 19935,
    LowBlowTA = 19950,
    LowBlowTT = 19955,
    LowBlowMG = 19960,
    LowJackTT = 19970,
    LowJackMG = 19971,
    LowSieve = 19980,
    LowTrough = 20025,
    LR66 = 20029,
    LRA900 = 20030,
    TRS2050 = 20040,
    LW01 = 20060,
    #[deprecated]
    LW08 = 20070,
    M1983FCR = 20090,
    M2240 = 20115,
    M44 = 20160,
    M401Z = 20205,
    M585Z = 20250,
    M588Z = 20295,
    MA1IFFPortion = 20340,
    MADHACK = 20350,
    MARELD = 20360,
    MAType909 = 20385,
    MARCS152 = 20420,
    Marconi1810 = 20430,
    MarconiCanadaHC75 = 20475,
    MarconiS713 = 20495,
    MarconiS1802 = 20520,
    MarconiS247 = 20530,
    MarconiS810 = 20565,
    MarconiSA10 = 20585,
    MARCONIST801 = 20589,
    MarconiST805 = 20590,
    Marconitype967 = 20610,
    Marconitype968 = 20655,
    Marconitype992 = 20700,
    Marconisignaaltype1022 = 20745,
    Marconisignaaltype910 = 20790,
    Marconisignaaltype911 = 20835,
    Marconisignaaltype992R = 20880,
    MARTELLO743D = 20890,
    MARTELLOS723A = 20895,
    MASTERA = 20897,
    MBDAFLAADSMJammer = 20898,
    MELCO3 = 20915,
    MELODI = 20917,
    MERLIN = 20918,
    Meraj4 = 20919,
    NorthropGrummanMESA = 20920,
    MeshBrick = 20925,
    METEOR1500S = 20927,
    METEOR200 = 20929,
    METEOR50DX = 20930,
    METEOR300 = 20931,
    MeteorBVRAAM = 20933,
    MFR = 20935,
    MFSR210045 = 20940,
    MICAMH = 20942,
    MICARF = 20943,
    MineralME = 20945,
    MirageILL = 20950,
    MiysisJammer = 20955,
    MK15 = 20969,
    MK15CIWS = 20970,
    MK23 = 21015,
    MK23TAS = 21060,
    MK25 = 21105,
    Mk25Mod3 = 21110,
    Mk25Mod7 = 21130,
    MK35M2 = 21150,
    MK92 = 21195,
    MK92CAS = 21240,
    MK92STIR = 21285,
    MK95 = 21330,
    MKS818 = 21332,
    MLA1 = 21340,
    MMAPQ706 = 21359,
    MM950 = 21360,
    MMAPS705 = 21375,
    MMAPS784 = 21390,
    MMSPG73 = 21419,
    MMSPG74 = 21420,
    MMSPG75 = 21465,
    MMSPN703 = 21490,
    MMSPN730 = 21492,
    MMSPN753B = 21495,
    MMSPQ3 = 21500,
    MMSPS702 = 21510,
    MMSPS768 = 21555,
    MMSPS774 = 21600,
    MMSPS791 = 21610,
    MMSPS794 = 21615,
    MMSPS798 = 21620,
    MMSR = 21623,
    Model17C = 21625,
    Moon4 = 21645,
    MoonPie = 21646,
    MOONCONE = 21647,
    MoonRack = 21648,
    MOONFACE = 21649,
    MMRS = 21650,
    Model360 = 21655,
    Model378 = 21660,
    Model970 = 21661,
    Model974 = 21665,
    MONOLITB = 21672,
    Monument = 21675,
    Mouse = 21680,
    MP411ESM = 21682,
    MPDR18S = 21685,
    MPDR18X = 21690,
    MPDR45E = 21692,
    MR2311 = 21693,
    MPR = 21695,
    MR2314 = 21696,
    MPS1 = 21697,
    MR36B = 21698,
    MR231MOD = 21699,
    MR1600 = 21700,
    MRR = 21701,
    MR35 = 21702,
    MR36 = 21703,
    MRL1 = 21704,
    MRL4 = 21705,
    MRL5 = 21706,
    MSAM = 21707,
    MR36A = 21708,
    MSTAR = 21709,
    MT305X = 21710,
    MR10M1E = 21711,
    MR90 = 21712,
    MRK411 = 21715,
    MR320MTopazV = 21716,
    MSP418K = 21720,
    MuffCob = 21735,
    Mushroom = 21780,
    Mushroom1 = 21825,
    Mushroom2 = 21870,
    Mushroom3 = 21871,
    N23 = 21872,
    N011MBars = 21873,
    N011MBarsB = 21874,
    N011MBarsC = 21875,
    N011MBarsR = 21876,
    N035IrbisE = 21877,
    N036Byelka = 21878,
    N25 = 21879,
    N920Z = 21880,
    N001V = 21881,
    NACOSRADARPILOTPlatinum = 21884,
    NampoB = 21885,
    NAGIRA = 21886,
    NanjingB = 21890,
    NanjingC = 21895,
    Nayada = 21915,
    NAYADA5M = 21917,
    NAYADA5PV = 21918,
    NEBOM = 21919,
    NeboSVU = 21920,
    Neptun = 21960,
    Nettuno4100 = 21965,
    NIKEHERCULESMTR = 21970,
    NIKETT = 21980,
    NorthropGrummanMFEWJammer = 21981,
    NORINCO3D = 21982,
    NJ81E = 21983,
    Normandie = 21984,
    NRJ6A = 21985,
    NOSTRADAMUS = 21986,
    NPG1240 = 21987,
    NPG1460 = 21988,
    NPG434 = 21989,
    NPG630 = 21990,
    NPM510 = 21991,
    NutCan = 21992,
    NPVegaLiana = 21995,
    NovellaNV1_70 = 22000,
    NRBA50 = 22005,
    NRBA51 = 22050,
    NRBF20A = 22095,
    NRJ5 = 22110,
    NS9005 = 22115,
    NS100Series = 22125,
    NUR31 = 22127,
    NWS3 = 22130,
    NysaB = 22140,
    O524A = 22185,
    O580B = 22230,
    O625Z = 22275,
    O626Z = 22320,
    OceanMaster = 22335,
    OceanMaster400 = 22340,
    OddGroup = 22345,
    OddLot = 22365,
    OddPair = 22410,
    OddRods = 22411,
    Oka = 22455,
    OFOGH = 22460,
    OFOGH3 = 22463,
    OKEAN = 22500,
    OKEANA = 22505,
    OKINXE12C = 22545,
    OKO = 22560,
    OMEGA = 22590,
    OmeraORB32 = 22635,
    OMUL = 22640,
    OneEye = 22680,
    OP28 = 22690,
    OPRL4 = 22695,
    OPRM71 = 22696,
    OPS9 = 22697,
    OPS11BC = 22700,
    OPS12 = 22701,
    OPS14B = 22705,
    OPS14C = 22706,
    OPS16B = 22725,
    OPS18 = 22730,
    OPS19 = 22732,
    OPS20 = 22735,
    OPS22 = 22736,
    OPS24 = 22737,
    OPS28 = 22740,
    OPS28C = 22745,
    OPS39 = 22750,
    OPTIMA3_2 = 22760,
    OR2 = 22770,
    ORB31D = 22800,
    ORB31S = 22810,
    ORB32 = 22815,
    ORB42 = 22830,
    OrionRtn10X = 22860,
    SurfaceWave = 22890,
    OtomatMK1 = 22900,
    OtomatMKIITeseo = 22905,
    OtomatSeriesAL = 22906,
    OwlScreech = 22950,
    P360Z = 22955,
    P14 = 22956,
    P180U = 22957,
    P182 = 22959,
    PA1660 = 22960,
    P18M = 22961,
    P190U = 22962,
    P30 = 22963,
    P18MOD = 22964,
    P35M = 22965,
    PAGE = 22970,
    PaintBox = 22977,
    PalmFrond = 22995,
    ModifiedPaintBox = 22998,
    PalmFrondAB = 23040,
    Pandora = 23041,
    PALSAR2 = 23042,
    PantsirSMTAR = 23043,
    PAR2 = 23045,
    PAR2000 = 23050,
    PAR2090C = 23053,
    PAR80 = 23055,
    PatHandTT = 23085,
    PatHandMG = 23095,
    PATRIOT = 23100,
    PattyCake = 23130,
    PawnCake = 23175,
    PBR4Rubin = 23220,
    PCS514 = 23240,
    PeaSticks = 23265,
    PechoraSC = 23295,
    PeelCone = 23310,
    PeelGroup = 23355,
    PeelGroupA = 23400,
    PeelGroupB = 23445,
    PeelGroupMG = 23450,
    PeelPair = 23490,
    Pelena = 23500,
    PGZ07 = 23515,
    Phalanx = 23525,
    PhazotronGukol4 = 23529,
    PhazotronZhukAAE = 23530,
    Philips9LV200 = 23535,
    Philips9LV331 = 23580,
    PhilipsLV223 = 23625,
    PhilipsSeaGiraffe50HC = 23670,
    PhimatJammer = 23675,
    PICOSAR = 23680,
    PILOTMK2 = 23685,
    PinJib = 23690,
    PinTip = 23695,
    PL11 = 23700,
    PL12 = 23701,
    PL15 = 23704,
    PlankShad = 23710,
    PlankShave = 23715,
    PlankShaveA = 23760,
    PlankShaveB = 23805,
    PlateSteer = 23850,
    PlesseyAWS1 = 23895,
    PlesseyAWS2 = 23925,
    PlesseyAWS4 = 23940,
    PlesseyAWS6 = 23985,
    PlesseyRJ = 23990,
    PlesseyType904 = 24020,
    Plesseytype996 = 24030,
    PlesseyAWS9 = 24035,
    PlinthNet = 24075,
    Pluto = 24095,
    PNABRubinDownBeat = 24098,
    POHJANPALO = 24100,
    PolimentK = 24110,
    POLLUX = 24120,
    PotGroup = 24165,
    PotGroupMG = 24210,
    PotGroupTA = 24255,
    PotGroupTT = 24300,
    PorkFist = 24320,
    PorkTrough = 24345,
    PozitivME15P26 = 24385,
    PositiveME1_2 = 24386,
    PostBow = 24390,
    PostLamp = 24435,
    PotDrum = 24480,
    PotHead = 24525,
    PotShot = 24535,
    PraetorianCountermeasuresSuite = 24540,
    PRIMUS30A = 24569,
    PRIMUS40WXD = 24570,
    Primus400 = 24614,
    PRIMUS300SL = 24615,
    Primus500 = 24616,
    Primus650 = 24617,
    Primus700 = 24618,
    PRIMUS800 = 24619,
    Primus3000 = 24620,
    Primus870 = 24622,
    PRORA = 24630,
    PRS2 = 24631,
    PRS3Argon2 = 24633,
    PRORAPA1660 = 24635,
    PS15 = 24640,
    PS05A = 24650,
    PS46A = 24660,
    PS70R = 24705,
    PS171R = 24706,
    PS860 = 24707,
    PS870 = 24709,
    PS890 = 24710,
    PSM33 = 24720,
    PuffBall = 24750,
    QuadradarVI = 24755,
    QW1A = 24757,
    Phazotron1RS21E = 24758,
    PVS200 = 24760,
    PVS2000 = 24761,
    R330ZH = 24768,
    R045 = 24769,
    R76 = 24770,
    R934B = 24771,
    RA20 = 24772,
    RA723 = 24774,
    R41XXX = 24775,
    RAC3D = 24776,
    RAC30 = 24780,
    R423AM = 24781,
    Racal1229 = 24795,
    DECCA1230 = 24800,
    RacalAC2690BT = 24840,
    RacalDecca1216 = 24885,
    RacalDECCA20V909 = 24890,
    RacalDecca360 = 24930,
    RacalDeccaAC1290 = 24975,
    RacalDeccaTM1229 = 25020,
    RacalDeccaTM1626 = 25065,
    RacalDRBN34A = 25110,
    RADAMHR = 25150,
    Radar24 = 25155,
    RADARPILOT1000 = 25170,
    RADARPILOT1100 = 25171,
    RAJENDRA = 25180,
    RAN7S = 25200,
    RAN10S = 25205,
    RAN11LX = 25245,
    Rani = 25250,
    RAPHAELTH = 25259,
    RapierTA = 25260,
    Rapier2000TA = 25265,
    RapierMG = 25270,
    RASCAR3400C = 25273,
    Rashmi = 25275,
    Rasit = 25276,
    Rasit3190B = 25277,
    RAT31DLM = 25278,
    RAT31DL = 25279,
    RAT31S = 25280,
    RAT8S = 25281,
    RAT31SL = 25282,
    RavenES05 = 25283,
    RATAC = 25285,
    RAWL = 25286,
    Rattler = 25287,
    RAWS = 25288,
    RAWL02 = 25289,
    Raytheon1220 = 25290,
    RAWS03 = 25291,
    Raytheon1210xx = 25292,
    Raytheon1302 = 25300,
    Raytheon1500 = 25335,
    Raytheon1645 = 25380,
    Raytheon1650 = 25425,
    Raytheon1900 = 25470,
    Raytheon2502 = 25515,
    RaytheonAnschutzNautoScanNX = 25530,
    RaytheonR41 = 25540,
    RaytheonRM10256X = 25545,
    RaytheonSL72 = 25550,
    RaytheonTM16506X = 25560,
    RaytheonTM166012S = 25605,
    RAY1220XR = 25630,
    RAY1401 = 25635,
    Ray2900 = 25650,
    RaymarineRD218 = 25694,
    Raypath = 25695,
    RaytheonPathfinderSTmk2 = 25698,
    RBE2 = 25735,
    RBE2AA = 25736,
    RCT180 = 25739,
    RDM = 25740,
    RDM3 = 25745,
    RDI = 25750,
    RDY = 25760,
    RDY3 = 25762,
    RDS86 = 25770,
    RDN72 = 25785,
    RDR1A = 25830,
    RDR1E = 25835,
    RDR4A = 25840,
    RDR150 = 25845,
    RDR160XD = 25850,
    RDR230HP = 25853,
    RDR1100 = 25855,
    RDR1150 = 25860,
    RDR1200 = 25875,
    RDR1400 = 25885,
    RDR1400C = 25890,
    RDR4000 = 25891,
    RDR4000 = 25892,
    RDR1500 = 25895,
    RiceCake = 25896,
    RDR1600 = 25897,
    RDR2000 = 25898,
    RDR1700B = 25899,
    Remora = 25900,
    RiceField = 25901,
    REC1A = 25902,
    REC1B = 25903,
    REC1C = 25904,
    ResolveEAS = 25906,
    RiceCupC = 25907,
    REL6E = 25908,
    REC1 = 25909,
    RiceBowl = 25910,
    ImprovedReporter = 25911,
    RiceBug = 25912,
    RiceCup = 25915,
    RiceLamp = 25920,
    REVATHI = 25940,
    REZONANS = 25950,
    RGMUGM109B = 25955,
    RGMUGM109EHomingRadar = 25958,
    RicePad = 25965,
    RKL526 = 25966,
    RKZ764 = 25967,
    RKZ766 = 25968,
    RKL165 = 25969,
    RKL609 = 25970,
    RKL800 = 25971,
    RKZ761 = 25972,
    RKZ2000 = 25973,
    RIS4CA = 25974,
    RL2000 = 25975,
    RL41 = 25976,
    RIR778 = 25977,
    RISAT = 25978,
    RLMS = 25979,
    RimHatESMECMSuite = 25980,
    RiceScoop = 26008,
    RiceScreen = 26010,
    DECCATM1070A = 26011,
    RM370BT = 26015,
    RockwellCollinsFMR200X = 26020,
    RM2312 = 26040,
    RM23123 = 26041,
    RMT0100A = 26043,
    RN222 = 26045,
    ROLAND2 = 26053,
    ROLANDBN = 26055,
    ROLANDMG = 26100,
    ROLANDTA = 26145,
    ROLANDTT = 26190,
    ROTODOME = 26210,
    RoundBall = 26235,
    RP379DTiradaD = 26236,
    RP3 = 26237,
    RP4G = 26238,
    RoundHouse = 26280,
    RoundHouseB = 26325,
    RPR117 = 26326,
    RS0250 = 26327,
    RSR210N = 26328,
    RT0250 = 26330,
    RTA4100 = 26340,
    RTN1A = 26350,
    RTN25X = 26353,
    RTS6400 = 26354,
    RubyRake = 26355,
    RumSling = 26360,
    RumSlingRO = 26361,
    RumSlingTT = 26362,
    RV2 = 26370,
    RV3 = 26415,
    RV5 = 26460,
    RV10 = 26505,
    RV15M = 26506,
    RV17 = 26550,
    RV18 = 26595,
    RV21 = 26596,
    RV21B = 26597,
    RV25 = 26600,
    RV377 = 26610,
    RVUM = 26640,
    RWD8 = 26650,
    RXN260 = 26660,
    RyeHouse = 26665,
    S1810CD = 26670,
    Sahab = 26672,
    Salamandre = 26673,
    SamyungSMR7200 = 26674,
    S1850M = 26675,
    S511 = 26676,
    S512 = 26677,
    S600 = 26678,
    S604 = 26679,
    S763LANZA3D = 26680,
    S613 = 26681,
    S631 = 26682,
    S654 = 26683,
    S669 = 26684,
    SA2Guideline = 26685,
    S244 = 26686,
    S711 = 26687,
    SA3Goa = 26730,
    SA8GeckoDT = 26775,
    SA12TELARILL = 26795,
    SA23TELIlluminator = 26797,
    SABERM60 = 26799,
    Samovar = 26805,
    Sampson = 26810,
    SAN7GadflyTI = 26820,
    SAN11Cads1UN = 26865,
    SaccadeMH = 26900,
    SaltPotAB = 26910,
    SAP14 = 26920,
    SAP518 = 26925,
    SAP518M = 26926,
    SandBar = 26930,
    SAPechora2MTT = 26935,
    SAR = 26945,
    SATRAPE = 26950,
    SATURNEII = 26955,
    ScanCan = 27000,
    ScanFix = 27045,
    ScanOdd = 27090,
    SCANTER1002 = 27095,
    SCANTER2001 = 27100,
    SCANTER2002 = 27101,
    SCANTER2100 = 27102,
    SCANTER4002 = 27109,
    SCANTER4100 = 27110,
    SCANTER5102 = 27111,
    SCANTER5502 = 27113,
    SCANTER6000 = 27115,
    SCANTER6002 = 27116,
    ScanterMil009 = 27125,
    ScanThree = 27135,
    SCANTERMILS = 27137,
    ScanterSMR = 27139,
    SCANTER = 27140,
    SCORADS = 27141,
    Scimitar = 27142,
    STAR2000 = 27143,
    SCOREBOARD = 27150,
    ScoopPair = 27175,
    ScoupPlate = 27180,
    SCOUT = 27183,
    SCR584 = 27190,
    SeaArcher2 = 27225,
    SeaBasedXBand = 27230,
    SeaDragon = 27235,
    SeaEagle = 27239,
    SeaEagleSC = 27240,
    SEAFALCON = 27245,
    SeaGiraffeAMB = 27248,
    Seaguard = 27251,
    SeaHawkSHNX12 = 27260,
    SeaHunter4MG = 27270,
    SeaHunter4TA = 27315,
    SeaHunter4TT = 27360,
    SeaGull = 27405,
    SeaMaster400 = 27430,
    SeaNet = 27450,
    #[deprecated]
    SeaSparrow = 27451,
    SeaSpray = 27495,
    SeaTiger = 27540,
    SeaTigerM = 27550,
    Seastar = 27560,
    Searchwater = 27570,
    Searchwater2000 = 27575,
    SEASONDE = 27580,
    SEASPRAY7000E = 27582,
    SeaVue = 27583,
    SeasprayMk3 = 27584,
    SeleniaOrion7 = 27585,
    Seleniatype912 = 27630,
    SelenniaRAN12LX = 27675,
    SeleniaRAN20S = 27680,
    SelenniaRTN10X = 27720,
    SeliniaARP1645 = 27765,
    SENTIRM20 = 27770,
    SERDAR = 27771,
    SERHAT = 27773,
    Series10CompactSubmarineRadarCSR = 27775,
    SERIES52 = 27780,
    SERIES320 = 27790,
    SG = 27800,
    SGJ02 = 27802,
    SGJ03 = 27803,
    SGR10200 = 27810,
    SGR10302 = 27855,
    SGR104 = 27870,
    Shahed129SAR = 27873,
    SHAHINE = 27875,
    SheetBend = 27900,
    SheetCurve = 27945,
    SHIKRA = 27980,
    ShipGlobe = 27990,
    ShipWheel = 28035,
    SGR114 = 28080,
    ShoreWalkA = 28125,
    ShortHorn = 28170,
    ShotDome = 28215,
    SideGlobeJN = 28260,
    PRV11 = 28280,
    SideWalkA = 28305,
    SignaalDA02 = 28350,
    SignaalDA05 = 28395,
    SignaalDA08 = 28440,
    SignaalDA082LS = 28445,
    SignaalLW04 = 28480,
    SignaalLW08 = 28485,
    SignaalLWOR = 28530,
    SignaalM45 = 28575,
    SignaalMW08 = 28620,
    SignaalSMART = 28665,
    SignaalSTING = 28710,
    SignaalSTIR = 28755,
    SignaalSTIR1_8M = 28760,
    SignaalSTIR24M = 28770,
    SignaalWM202 = 28800,
    SignaalWM25 = 28845,
    SignaalWM27 = 28890,
    SignaalWM28 = 28935,
    SignaalZW01 = 28980,
    SignaalZW06 = 29025,
    SignaalZW07 = 29030,
    SignaalZW0800 = 29035,
    SIMRAD3G = 29043,
    SIMRAD4G = 29045,
     = 29050,
     = 29060,
     = 29070,
     = 29115,
     = 29160,
    SKYFENDER = 29172,
    SkyWave = 29175,
    SkyguardB = 29180,
    SKYGUARDTA = 29185,
    SKYGUARDTT = 29190,
    SkyguardLR = 29191,
    Skymaster = 29200,
     = 29205,
    SkyRanger = 29210,
     = 29215,
    SKYSHIELDTA = 29220,
    SL = 29250,
    SLALQ234 = 29270,
     = 29295,
     = 29297,
    SLC2 = 29300,
    SLC2E = 29301,
    SLC4 = 29305,
     = 29340,
     = 29385,
     = 29400,
     = 29430,
     = 29431,
     = 29432,
     = 29433,
     = 29434,
     = 29435,
     = 29440,
    SM674AUPM = 29450,
     = 29475,
     = 29520,
     = 29565,
     = 29610,
     = 29655,
     = 29700,
     = 29745,
     = 29790,
     = 29835,
     = 29880,
     = 29925,
     = 29970,
     = 30015,
    SR47A = 30016,
     = 30060,
     = 30065,
    SMARTS = 30068,
    SMARTSMk2 = 30069,
    SMARTL = 30070,
    SM932 = 30072,
     = 30075,
     = 30080,
     = 30105,
     = 30140,
     = 30150,
     = 30195,
     = 30200,
     = 30240,
     = 30255,
     = 30285,
     = 30330,
     = 30375,
     = 30420,
     = 30421,
     = 30465,
    9S18M1 = 30470,
    9S18M1E = 30471,
    SPB7 = 30475,
     = 30480,
    SNW10 = 30490,
    SO1 = 30510,
    SO12 = 30520,
    SOACommunist = 30555,
    SO69 = 30580,
     = 30600,
    SOM64 = 30645,
    Sopka = 30650,
     = 30660,
    SorbtsiyaL005 = 30661,
    SorbtsiyaL005S = 30662,
    SPADASIR = 30665,
     = 30670,
    SparrowILL = 30690,
    SPERRYRASCAR = 30691,
    SPECTRA = 30692,
    SPEAR3MMW = 30696,
     = 30700,
     = 30701,
    SPEXER2000 = 30710,
    SPG53F = 30735,
    SPG70 = 30780,
    SPG74 = 30825,
    SPG75 = 30870,
    SPG76 = 30915,
     = 30960,
     = 31005,
     = 31050,
    SPINODADDAWTR = 31070,
    SPJ40 = 31080,
     = 31095,
    SPN2 = 31096,
    SPN4 = 31097,
    SPN30 = 31100,
    SPN35A = 31140,
    SPN41 = 31185,
    SPN42 = 31230,
    SPN43A = 31275,
    SPN43B = 31320,
    SPN44 = 31365,
    SPN46 = 31410,
    SPN703 = 31455,
    SPN720 = 31475,
    SPN7281 = 31500,
    SPN748 = 31545,
    SPN750 = 31590,
    SPO8 = 31592,
    SPN753G = 31593,
     = 31635,
    P12 = 31680,
    P18 = 31681,
    P18 = 31682,
    P18 = 31684,
    P18MH2 = 31685,
     = 31700,
    SPQ712 = 31725,
    SPR2 = 31730,
    SPR51 = 31740,
    SPS5FASOL = 31765,
    SPS6 = 31766,
    SPS6C = 31770,
    SPS10F = 31815,
    SPS12 = 31860,
    SPS22NBUKET = 31870,
    SPS33NBUKET = 31875,
    SPS44NBUKET = 31880,
    SPS55NBUKET = 31890,
    SPS58 = 31905,
    SPS62 = 31925,
    SPS100K = 31935,
    SPS64 = 31950,
    SPS141 = 31951,
    SPS142 = 31952,
    SPS143 = 31953,
    SPS151 = 31955,
    SPS152 = 31956,
    SPS153 = 31957,
    SPS160Geran = 31959,
    SPS161 = 31960,
    SPS95K = 31970,
    SPS171Jammer = 31971,
    SPS172Jammer = 31972,
    SPS768 = 31995,
    SPS540K = 32010,
    SPS550KMF = 32020,
    SPS774 = 32040,
    SPY790 = 32085,
     = 32130,
     = 32175,
     = 32220,
     = 32265,
    Shmel = 32310,
    P15M = 32330,
     = 32355,
    SQUIRE = 32365,
    SR2410C = 32373,
    SR47BG = 32375,
    SREM5 = 32385,
    SRN6 = 32400,
    SRN15 = 32445,
    SRN206 = 32455,
    SRN745 = 32490,
    SRO1 = 32535,
    SRO2 = 32580,
     = 32625,
     = 32670,
     = 32715,
     = 32760,
     = 32805,
     = 32850,
     = 32851,
     = 32852,
     = 32895,
     = 32940,
     = 32985,
     = 33025,
     = 33030,
     = 33075,
     = 33120,
    SSN10AFL10mmWMH = 33125,
    SSN11Nasr1mmWMH = 33140,
     = 33165,
    SSN12YJ83JmmWMH = 33166,
     = 33210,
     = 33230,
     = 33231,
     = 33255,
     = 33300,
     = 33345,
     = 33390,
     = 33435,
     = 33480,
     = 33481,
     = 33483,
    SSN26StrobileMMWMH = 33484,
     = 33485,
     = 33486,
     = 33505,
     = 33510,
     = 33511,
     = 33525,
    STR41 = 33570,
    ST858 = 33580,
    START1M = 33582,
    STENTOR = 33584,
    StormShadowAHR = 33585,
    STRAIGHTFLUSH = 33586,
     = 33590,
     = 33595,
     = 33600,
     = 33615,
     = 33660,
     = 33705,
     = 33750,
     = 33795,
     = 33840,
    SUPERDARN = 33850,
    Superfledermaus = 33860,
    Supersearcher = 33870,
     = 33885,
     = 33930,
    SYMPHONY = 33933,
    SYNAPSISMk2 = 33935,
    SY80 = 33950,
     = 33975,
     = 34020,
     = 34040,
     = 34065,
     = 34110,
     = 34155,
     = 34200,
     = 34245,
     = 34290,
     = 34335,
     = 34380,
     = 34425,
     = 34470,
    TA10K = 34480,
    JY11B = 34500,
    TACANSURF = 34505,
    P14 = 34515,
     = 34516,
     = 34517,
     = 34560,
     = 34605,
    TDR94 = 34607,
     = 34610,
     = 34620,
    TALISMAN = 34624,
     = 34625,
    T1135 = 34626,
    TANCANSURF = 34627,
    TECSAR = 34628,
    TERRASARX = 34629,
    TESAR = 34630,
    THAADGBR = 34640,
    ThalesRDY2 = 34644,
    ThalesNederlandSignaalAPAR = 34645,
    ThalesScorpionJammer = 34646,
    ThalesVariant = 34647,
    ThalesICMSJammer = 34648,
    ThalesIMEWSJammer = 34649,
    THD225 = 34650,
    THD1012 = 34655,
    THD1098 = 34660,
    THD1213 = 34665,
    THD1940 = 34670,
    THD1955Palmier = 34680,
    THD5500 = 34695,
    ThirdofKhordad = 34700,
     = 34740,
    PRV9 = 34785,
    PRV16 = 34786,
     = 34795,
     = 34830,
     = 34875,
     = 34920,
     = 34965,
    ThomsonCSFDomino30 = 34966,
     = 35010,
     = 35055,
     = 35100,
     = 35145,
     = 35190,
     = 35235,
     = 35280,
     = 35325,
     = 35370,
     = 35415,
     = 35460,
    ThomsonENR = 35470,
    ThomsonRDI = 35475,
    TierIIPlus = 35477,
    TPS755 = 35478,
    TPS830K = 35479,
    TRS2105 = 35480,
    TR23K = 35481,
    TR23MR = 35482,
    TRAC2100 = 35483,
    TRAC2300 = 35484,
    HT223 = 35485,
    TRADEX = 35486,
    TRAILXI = 35487,
    TRD1211 = 35488,
    TRD1235 = 35489,
    TRS2100 = 35490,
    TRACNG = 35491,
     = 35505,
    36D6 = 35550,
     = 35570,
    TIRSPONDER = 35580,
    TK25E5 = 35583,
    TMKMk2 = 35585,
    TMXMk2 = 35586,
     = 35595,
     = 35640,
     = 35685,
     = 35730,
     = 35775,
    TokenB = 35785,
     = 35800,
    Tonson = 35810,
     = 35820,
     = 35865,
     = 35910,
     = 35955,
     = 36000,
     = 36045,
     = 36046,
     = 36090,
    TYPE208 = 36120,
     = 36135,
     = 36180,
    TornadoGMR = 36200,
    TornadoTFR = 36201,
     = 36220,
     = 36225,
    TORM2TER = 36226,
     = 36230,
     = 36270,
    TR47C = 36300,
    TORSOM = 36315,
    TQN2 = 36320,
     = 36360,
    TRD1500 = 36365,
     = 36370,
     = 36371,
    TRISPONDE = 36380,
    TRML = 36381,
    TRS2215 = 36382,
    TRML3D = 36383,
    TRMS = 36384,
    TRS2056 = 36385,
    TRS3010 = 36386,
    TRS2060 = 36387,
    TRS2245 = 36388,
    TRS2310 = 36389,
    TritonG = 36390,
    TRS22XX = 36391,
    TRS3030 = 36400,
    TRS3033 = 36405,
    TRS3203 = 36417,
    TRS3405 = 36420,
    TRS3410 = 36425,
    TRS3415 = 36430,
    TRS3D = 36440,
    TRS3D16 = 36441,
    TRS3D16ES = 36442,
    TRS3D32 = 36443,
    TRS4D = 36446,
    TRSC = 36447,
    TRSN = 36450,
    TS4478A = 36460,
    TSE5000 = 36495,
    TSR333 = 36540,
    TSR793 = 36550,
     = 36563,
     = 36585,
    TW1374 = 36590,
    TW1378 = 36595,
    TW1446 = 36600,
     = 36630,
     = 36675,
     = 36720,
     = 36765,
     = 36810,
    Type071LPD = 36821,
    Type212JA = 36827,
    Type221JA = 36830,
    Type223 = 36835,
    Type80ASM1 = 36836,
    Type120 = 36838,
    Type208 = 36840,
    Type222 = 36843,
    Type226 = 36846,
    Type232H = 36850,
    TYPE245 = 36853,
    TYPE262 = 36855,
    TYPE275 = 36900,
    TYPE278 = 36905,
    TYPE293 = 36945,
    Type341 = 36946,
    TYPE313 = 36947,
    Type305A = 36948,
    Type334 = 36960,
    Type342 = 36985,
    TYPE343SUNVISORB = 36990,
    Type344 = 36992,
    Type345 = 37010,
    Type346 = 37011,
    Type349A = 37033,
    TYPE347B = 37035,
    Type347G = 37038,
    Type359 = 37039,
    Type352 = 37040,
    Type360 = 37041,
    Type362ESR1SR47B = 37043,
    Type354 = 37045,
    Type366 = 37047,
    Type363 = 37048,
    Type364 = 37049,
    Type404A = 37050,
    Type405 = 37052,
    TYPE405J = 37053,
    Type408D = 37058,
    Type517B = 37059,
    Type518 = 37060,
    Type589 = 37070,
    TYPE651 = 37073,
    Type753 = 37075,
    Type702 = 37077,
    Type704 = 37078,
    Type753 = 37079,
    Type756 = 37080,
    TYPE713 = 37081,
    TYPE714 = 37082,
    TYPE702D = 37083,
    TYPE760 = 37084,
    Type760 = 37086,
    Type815 = 37090,
    Type793 = 37095,
    Type8A813 = 37100,
    TYPE901M = 37105,
    TYPE902 = 37110,
    Type902B = 37124,
    TYPE903 = 37125,
    TYPE909TI = 37170,
    TYPE909TT = 37215,
    TYPE910 = 37260,
    TYPE931 = 37265,
    TYPE965 = 37305,
    TYPE967 = 37350,
    TYPE968 = 37395,
    TYPE974 = 37440,
    TYPE975 = 37485,
    TYPE978 = 37530,
    Type981 = 37534,
    Type9813 = 37535,
    TYPE982 = 37540,
    Type984 = 37543,
    Type985 = 37544,
    TYPE992 = 37575,
    TYPE993 = 37620,
    TYPE994 = 37665,
    Type996 = 37670,
    Type997Artisan = 37675,
    TYPE1006 = 37710,
    TYPE1006 = 37755,
    TYPE1022 = 37800,
    Type1047 = 37810,
    Type1048 = 37815,
    Type1474 = 37825,
    Type1493 = 37828,
    ULTRA = 37840,
    UKMK10 = 37845,
    UPS220C = 37850,
    UPX110 = 37890,
    UPX27 = 37935,
    URN20 = 37980,
    UTESA = 37985,
    UTEST = 37990,
    URN25 = 38025,
    VIGILANT = 38035,
    VitebskL370Jammer = 38038,
    VOLEXIIIIV = 38045,
    VOLGA = 38046,
    VORONEZHDM = 38047,
    VOSTOK = 38048,
    VOSTOKE = 38049,
    VSR = 38050,
    VOSTOK3D = 38051,
    VSTARPT = 38055,
    W160 = 38058,
    W1028 = 38060,
    W8818 = 38070,
    W8838 = 38115,
    W8852 = 38120,
     = 38140,
     = 38150,
    WAS74S = 38160,
     = 38205,
    WATCHDOG = 38210,
     = 38250,
    Watchman = 38260,
    WAVESTORM = 38270,
    WATCHMANS = 38275,
    WATCHMANT = 38276,
    WEATHERSCOUT2 = 38280,
     = 38295,
     = 38320,
     = 38340,
     = 38385,
     = 38430,
     = 38475,
    WetEye = 38520,
    WetEye2 = 38525,
    WetEyeMod = 38565,
    WF44S = 38568,
    WGU41B = 38570,
    WGU44B = 38572,
     = 38610,
     = 38655,
     = 38700,
     = 38715,
     = 38730,
    WineGlassJammer = 38735,
    WildCard = 38745,
    WILDCAT = 38748,
     = 38790,
     = 38835,
    WLR = 38840,
    WM2XSeries = 38880,
    WM2XSeriesCAS = 38925,
    WR10X = 38930,
    WR2100 = 38935,
    WSR74C = 38950,
    WSR74S = 38955,
    WSR81 = 38957,
    WXR700C = 38960,
    WXR2100 = 38965,
    WXR2100MSTT = 38966,
     = 38970,
    XTAR25 = 38990,
    XTAR3D = 38995,
    YAOGAN3 = 39000,
    Yaogan29 = 39014,
     = 39015,
    YH96 = 39050,
     = 39060,
    YITIANADS = 39061,
    YD3 = 39062,
    YJ12MH = 39063,
    YJ62MH = 39065,
    YJ82MH = 39066,
    YJ83MH = 39067,
    YLC2 = 39070,
    YLC2A = 39071,
    YLC4 = 39073,
    YLC6 = 39074,
    YLC6M = 39075,
    YLC8 = 39080,
    YLC8B = 39081,
    YLC18 = 39085,
     = 39105,
    ZaslonA = 39110,
    ZaslonMultipurpose = 39112,
    ZooPark1 = 39125,
    ZPS6 = 39126,
    ZOOPARK3 = 39127,
    ZD12 = 39131,
    ZW06 = 39150,
    ANALQ1361 = 39200,
    ANALQ1362 = 39201,
    ANALQ1363 = 39202,
    ANALQ1364 = 39203,
    ANALQ1365 = 39204,
    ANALQ1622 = 39210,
    ANALQ1623 = 39211,
    ANALQ1624 = 39212,
    ZhukM = 45300,
    ZHUKMAE = 45303,
    ZHUKME = 45304,
    ZHUKMME = 45305,
    ZhukMSE = 45307,
}

// SISO-REF-010-2023 EmitterSystemFunction [UID 76]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EmitterSystemFunction {
    #[default]
    Other = 0,
    Multifunction = 1,
    EarlyWarningSurveillance = 2,
    HeightFinder = 3,
    FireControl = 4,
    AcquisitionDetection = 5,
    Tracker = 6,
    GuidanceIllumination = 7,
    Firingpointlaunchpointlocation = 8,
    RangeOnly = 9,
    RadarAltimeter = 10,
    Imaging = 11,
    MotionDetection = 12,
    Navigation = 13,
    WeatherMeteorological = 14,
    Instrumentation = 15,
    IdentificationClassification = 16,
    AAAFireControl = 17,
    AirSearchBomb = 18,
    AirIntercept = 19,
    Altimeter = 20,
    AirMapping = 21,
    AirTrafficControl = 22,
    Beacon = 23,
    BattlefieldSurveillance = 24,
    GroundControlApproach = 25,
    GroundControlIntercept = 26,
    CoastalSurveillance = 27,
    DecoyMimic = 28,
    DataTransmission = 29,
    EarthSurveillance = 30,
    GunLayBeacon = 31,
    GroundMapping = 32,
    HarborSurveillance = 33,
    IFF = 34,
    ILS = 35,
    IonosphericSound = 36,
    Interrogator = 37,
    BarrageJamming = 38,
    ClickJamming = 39,
    DeceptiveJamming = 40,
    FrequencySweptJamming = 41,
    Jammer = 42,
    NoiseJamming = 43,
    PulsedJamming = 44,
    RepeaterJamming = 45,
    SpotNoiseJamming = 46,
    MissileAcquisition = 47,
    MissileDownlink = 48,
    Meteorological = 49,
    Space = 50,
    SurfaceSearch = 51,
    ShellTracking = 52,
    Television = 56,
    Unknown = 57,
    VideoRemoting = 58,
    ExperimentalorTraining = 59,
    MissileGuidance = 60,
    MissileHoming = 61,
    MissileTracking = 62,
    Jammingnoise = 64,
    Jammingdeception = 65,
    Decoy = 66,
    NavigationDistanceMeasuringEquipment = 71,
    TerrainFollowing = 72,
    WeatherAvoidance = 73,
    ProximityFuse = 74,
    Instrumentation = 75,
    Radiosonde = 76,
    Sonobuoy = 77,
    BathythermalSensor = 78,
    TowedCounterMeasure = 79,
    DippingSonar = 80,
    TowedAcousticSensor = 81,
    Weaponnonlethal = 96,
    Weaponlethal = 97,
    TestEquipment = 98,
    AcquisitionTrack = 99,
    TrackGuidance = 100,
    GuidanceIlluminationTrackAcquisition = 101,
    SearchAcquisition = 102,
    Dropsonde = 103,
}

// SISO-REF-010-2023 ElectromagneticEmissionStateUpdateIndicator [UID 77]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ElectromagneticEmissionStateUpdateIndicator {
    #[default]
    HeartbeatUpdate = 0,
    ChangedDataUpdate = 1,
}

// SISO-REF-010-2023 ElectromagneticEmissionBeamFunction [UID 78]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ElectromagneticEmissionBeamFunction {
    #[default]
    Other = 0,
    Search = 1,
    HeightFinding = 2,
    Acquisition = 3,
    Tracking = 4,
    Acquisitionandtracking = 5,
    Commandguidance = 6,
    Illumination = 7,
    Ranging = 8,
    Missilebeacon = 9,
    MissileFusing = 10,
    Activeradarmissileseeker = 11,
    Jamming = 12,
    IFF = 13,
    NavigationWeather = 14,
    Meteorological = 15,
    Datatransmission = 16,
    Navigationaldirectionalbeacon = 17,
    TimeSharedSearch = 20,
    TimeSharedAcquisition = 21,
    TimeSharedTrack = 22,
    TimeSharedCommandGuidance = 23,
    TimeSharedIllumination = 24,
    TimeSharedJamming = 25,
}

// SISO-REF-010-2023 HighDensityTrackJam [UID 79]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum HighDensityTrackJam {
    #[default]
    NotSelected = 0,
    Selected = 1,
}

// SISO-REF-010-2023 DesignatorSystemName [UID 80]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DesignatorSystemName {
    #[default]
    NotSpecified = 0,
    ANAAQ4 = 1000,
    ANAAQ7 = 1100,
    ANAAQ8 = 1200,
    ANAAQ14LANTIRN = 1300,
    ANAAQ19 = 1400,
    ANAAQ22ASAFIRE = 1500,
    ANAAQ22BSAFIRELP = 1600,
    ANAAQ22CStarSAFIREI = 1700,
    ANAAQ22DBRITEStar = 1800,
    ANAAQ24DIRCMNemesis = 1900,
    ANAAQ25LTS = 2000,
    ANAAQ28LITENINGII = 2100,
    ANAAQ30 = 2200,
    ANAAQ32 = 2300,
    ANAAQ33Sniper = 2400,
    ANAAQ37 = 2500,
    ANAAQ38 = 2600,
    ANAAQ40 = 2650,
    ANAAS32 = 2700,
    ANAAS35V = 2800,
    ANAAS37 = 2900,
    ANAAS38 = 3000,
    ANAAS44 = 3100,
    ANAAS46 = 3200,
    ANAAS49 = 3300,
    ANAAS51 = 3400,
    ANAAS52MTSA = 3500,
    ANALQ10 = 3600,
    ANASQ228 = 3700,
    ANAVQ25 = 4400,
    ANAVQ26 = 4500,
    ANGVS5 = 4600,
    ANPED1LLDR = 4700,
    TADSLRFD = 4800,
    MMSLRFD = 4900,
    AH1CNITE = 5000,
    MATES = 5100,
    TCV115 = 5200,
    TIM = 5300,
    TMS303 = 5400,
    TMY303 = 5500,
    ALRAD = 5600,
    RFTDL = 5700,
    VVLR = 5800,
    P0705HELL = 6000,
    P0708PULSE = 6100,
    HELD = 6200,
    TYPE105 = 6300,
    TYPE118 = 6400,
    TYPE121 = 6500,
    TYPE126 = 6600,
    TYPE629 = 6700,
    CLDS = 6800,
    TAV38 = 6900,
    TMV630 = 7000,
    ALTM1020 = 7100,
    ALATS = 7200,
    DarkStarLAMPS = 7300,
    GLTDII = 7400,
    MBTELRF = 7500,
    MarkVII = 7600,
    SIREV = 7700,
    ANAAQ16B = 7800,
    ANAAQ16DAESOP = 7900,
    ANAAQ21StarSAFIREIII = 8000,
    ANAAQ22EBRITEStar = 8100,
    ANAAQ36StarSAFIREII = 8200,
    ANAAS38ANiteHawk = 8300,
    ANAAS38BNiteHawk = 8400,
    ANAAS44C = 8500,
    ANAAS53CSP = 8600,
    ANASQ28ATFLIR = 8700,
    ANDAS1MTSB = 8800,
    ANPAQ1LTD = 8900,
    ANPAQ3MULE = 9000,
    ANPEQ1SOFLAM = 9090,
    ANPEQ3 = 9100,
    ANPEQ15ATPIAL = 9140,
    ANPEQ18IZLID1000P = 9150,
    ANTVQ2GVLLD = 9200,
    ANZSQ21EOS = 9300,
    ANZSQ22EOS = 9400,
    CIRCM = 9500,
    Guardian = 9600,
    IZLID200P = 9700,
    IZLID1000PW = 9800,
    MMS = 9900,
    MTADSPNVSArrowhead = 10000,
    RBS70 = 10100,
    RBS90 = 10200,
    TADSPNVS = 10300,
    COLIBRI = 10400,
    Damocles = 10500,
    I251Shkval = 10600,
    KPS53AVEOTS = 10700,
    StarSAFIRE380 = 10800,
    JANUSTEOS = 10900,
    LOTHAREOS = 11000,
    MK46MOD1EOS = 11100,
    MTK201MEEOS = 11200,
    ThalesMiradorMk2EOS = 11300,
    TPN1M4923EOS = 11400,
}

// SISO-REF-010-2023 DesignatorDesignatorCode [UID 81]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DesignatorDesignatorCode {
    #[default]
    Other = 0,
}

// SISO-REF-010-2023 IFFSystemType [UID 82]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IFFSystemType {
    #[default]
    NotUsed = 0,
    MarkXXIIATCRBSTransponder = 1,
    MarkXXIIATCRBSInterrogator = 2,
    SovietTransponder = 3,
    SovietInterrogator = 4,
    RRBTransponder = 5,
    MarkXIIAInterrogator = 6,
    Mode5Interrogator = 7,
    ModeSInterrogator = 8,
    MarkXIIATransponder = 9,
    Mode5Transponder = 10,
    ModeSTransponder = 11,
    MarkXIIACombinedInterrogatorTransponder = 12,
    MarkXIICombinedInterrogatorTransponder = 13,
    TCASACASTransceiver = 14,
}

// SISO-REF-010-2023 IFFSystemName [UID 83]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IFFSystemName {
    #[default]
    NotUsed = 0,
    GenericMarkX = 1,
    GenericMarkXII = 2,
    GenericATCRBS = 3,
    GenericSoviet = 4,
    GenericModeS = 5,
    GenericMarkXXIIATCRBS = 6,
    GenericMarkXXIIATCRBSModeS = 7,
    ARI5954 = 8,
    ARI5983 = 9,
    GenericRRB = 10,
    GenericMarkXIIA = 11,
    GenericMode5 = 12,
    GenericMarkXIIACombinedInterrogatorTransponder = 13,
    GenericMarkXIICombinedInterrogatorTransponder = 14,
    GenericTCASIACASITransceiver = 15,
    GenericTCASIIACASIITransceiver = 16,
    GenericMarkX = 17,
    GenericMarkX = 18,
}

// SISO-REF-010-2023 IFFSystemMode [UID 84]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IFFSystemMode {
    #[default]
    NoStatement = 0,
    Off = 1,
    Standby = 2,
    Normal = 3,
    Emergency = 4,
    LoworLowSensitivity = 5,
}

// SISO-REF-010-2023 IFFLayerSpecificInformation [UID 87]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IFFLayerSpecificInformation {
    #[default]
    NoLayerSpecificInformationIsPresent = 0,
}

// SISO-REF-010-2023 IFFAlternateMode4ChallengeReply [UID 96]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IFFAlternateMode4ChallengeReply {
    #[default]
    NoStatement = 0,
    Valid = 1,
    Invalid = 2,
    Noresponse = 3,
    UnabletoVerify = 4,
}

// SISO-REF-010-2023 IFFSystemType1OperationalParameter1 [UID 97]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IFFSystemType1OperationalParameter1 {
    #[default]
    NoOperationalParameter1Data = 0,
}

// SISO-REF-010-2023 IFFSystemType1OperationalParameter2 [UID 98]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IFFSystemType1OperationalParameter2 {
    #[default]
    NoOperationalParameter2Data = 0,
}

// SISO-REF-010-2023 SubcategoriesforLandCategory200Mammal [UID 100]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforLandCategory200Mammal {
    #[default]
    SmallDog = 1,
    Chihuahua = 2,
    MediumDog = 10,
    AustralianCattleDog = 11,
    LargeDog = 20,
    GermanShepherd = 21,
    VeryLargeDog = 30,
    GiantTurkishKangal = 31,
    Sheep = 40,
    Goat = 41,
    Pig = 50,
    Cow = 60,
    Ox = 61,
    OxWithCart = 70,
    Horse = 80,
    Donkey = 81,
    Mule = 82,
    HorseWithRider = 90,
    HorseWithCargo = 91,
    DonkeyWithRider = 92,
    DonkeyWithCargo = 93,
    MuleWithRider = 94,
    MuleWithCargo = 95,
    Camel = 100,
    DromedaryCamel = 101,
    BactrianCamel = 102,
    DromedaryCamelWithRider = 110,
    DromedaryCamelWithCargo = 111,
    Rat = 200,
}

// SISO-REF-010-2023 SubcategoriesforLandCategory201Reptile [UID 101]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforLandCategory201Reptile {
    #[default]
    NewZealandNorthernTuatara = 1,
    Monitor = 3,
    Gecko = 8,
    Iguana = 13,
    Chameleon = 17,
    NonVenomousSnake = 30,
    Boa = 31,
    Python = 35,
    Bullsnake = 39,
    Kingsnake = 43,
    VenomousSnake = 60,
    Rattlesnake = 61,
    Copperhead = 62,
    Cottonmouth = 63,
    Taipan = 64,
    Viper = 65,
    Cobra = 66,
    AustralianBrownSnake = 67,
    Tortoise = 90,
    Turtle = 100,
    AmericanAlligator = 120,
    Crocodile = 121,
    AustralianFreshwaterCrocodile = 122,
}

// SISO-REF-010-2023 SubcategoriesforLandCategory202Amphibian [UID 102]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforLandCategory202Amphibian {
    #[default]
    Frog = 1,
    Toad = 2,
    Salamander = 170,
    Caecilian = 230,
}

// SISO-REF-010-2023 SubcategoriesforLandCategory203Insect [UID 103]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforLandCategory203Insect {
    #[default]
    Beetle = 1,
    Mantis = 60,
    Cockroach = 70,
    ArmyAnt = 80,
    FireAnt = 81,
    Grasshopper = 90,
    Centipede = 100,
}

// SISO-REF-010-2023 SubcategoriesforLandCategory204Arachnid [UID 104]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforLandCategory204Arachnid {
    #[default]
    Spider = 1,
    Tick = 20,
    Scorpion = 30,
    Harvestmen = 40,
    Mite = 50,
}

// SISO-REF-010-2023 SubcategoriesforLandCategory205Mollusk [UID 105]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforLandCategory205Mollusk {
    #[default]
    Snail = 1,
    Slug = 50,
}

// SISO-REF-010-2023 SubcategoriesforLandCategory206Marsupial [UID 106]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforLandCategory206Marsupial {
    #[default]
    BrownFourEyedOpossum = 1,
    BushyTailedOpossum = 2,
    TatesShrewOpossum = 90,
    GreaterBilby = 100,
    TasmanianDevil = 110,
    BrushTailedRockWallaby = 150,
    EasternWallaroo = 160,
    RedKangaroo = 170,
    QueenslandKoala = 200,
    SouthernHairyNosedWombat = 205,
    BrushtailPossum = 210,
    SugarGlider = 211,
}

// SISO-REF-010-2023 SubcategoriesforAirCategory200Bird [UID 110]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforAirCategory200Bird {
    #[default]
    Penguin = 1,
    Seagull = 2,
    Pelican = 3,
    Albatross = 4,
    Swan = 5,
    Cormorant = 6,
    Heron = 7,
    Crane = 8,
    Osprey = 9,
    Loon = 10,
    Stork = 11,
    Flamingo = 12,
    Duck = 13,
    Ostrich = 20,
    Emu = 21,
    Chicken = 22,
    BlackBird = 30,
    Starling = 31,
    Budgerigar = 32,
    CanadianGoose = 40,
    Crow = 41,
    Eagle = 50,
    Vulture = 55,
    Falcon = 60,
    Hawk = 65,
    Owl = 70,
    Kite = 80,
}

// SISO-REF-010-2023 SubcategoriesforAirCategory201Insect [UID 111]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforAirCategory201Insect {
    #[default]
    Moth = 1,
    Butterfly = 2,
    Fly = 20,
    Mosquito = 30,
    Wasp = 40,
    Bee = 50,
    Beetle = 60,
    Dragonfly = 70,
    Locust = 80,
}

// SISO-REF-010-2023 SubcategoriesforAirCategory202Mammal [UID 112]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforAirCategory202Mammal {
    #[default]
    Bat = 1,
    FlyingSquirrel = 10,
    GlidingPossum = 20,
}

// SISO-REF-010-2023 SubcategoriesforSubsurfaceCategory200Fish [UID 120]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforSubsurfaceCategory200Fish {
    #[default]
    ForageFishSmallSchooling = 1,
    Herring = 2,
    Sardines = 3,
    Krill = 4,
    Squid = 5,
    MediumSchoolingFish = 30,
    Hake = 31,
    Cod = 32,
    Haddock = 33,
    Mackerel = 34,
    LargeSchoolingFish = 60,
    Tuna = 61,
    SmallShark = 90,
    DogfishShark = 91,
    MediumShark = 120,
    MakoShark = 121,
    HammerheadShark = 122,
    LargeShark = 150,
    GreatWhiteShark = 151,
    TigerShark = 152,
    BlueShark = 153,
    WhaleShark = 154,
    Skate = 180,
    Stingray = 181,
    Eel = 190,
    Marlin = 200,
    Swordfish = 201,
}

// SISO-REF-010-2023 SubcategoriesforSubsurfaceCategory201Mammal [UID 121]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforSubsurfaceCategory201Mammal {
    #[default]
    Whale = 1,
    BeakedWhale = 2,
    BelugaWhale = 3,
    BlueWhale = 4,
    BottlenoseWhale = 5,
    NorthernBottlenoseWhale = 6,
    SouthernBottlenoseWhale = 7,
    BowheadWhale = 8,
    BrydesWhale = 9,
    DwarfSpermWhale = 10,
    FinbackWhale = 11,
    GrayWhale = 12,
    HumpbackWhale = 13,
    LongFinnedPilotWhale = 14,
    MinkeWhale = 15,
    NorthernMinkeWhale = 16,
    SouthernMinkeWhale = 17,
    NarwhalWhale = 18,
    OrcaWhale = 19,
    PygmySpermWhale = 20,
    RightWhale = 21,
    NorthAtlanticRightWhale = 22,
    NorthPacificRightWhale = 23,
    SouthernRightWhale = 24,
    SeiWhale = 25,
    ShortFinnedPilotWhale = 26,
    SpermWhale = 27,
    Dolphin = 50,
    BottlenoseDolphin = 51,
    BottlenoseIndoPacificDolphin = 52,
    BottlenoseBurrunanDolphin = 53,
    AtlanticSpottedDolphin = 54,
    AustralianSnubfinDolphin = 55,
    ChileanBlackDolphin = 56,
    ChineseWhiteDolphin = 57,
    ClymeneDolphin = 58,
    Porpoise = 100,
    HarbourPorpoise = 101,
    CalifornianPorpoise = 102,
    DallsPorpoise = 103,
    BurmeistersPorpoise = 104,
    Seal = 120,
    BeardedSeal = 121,
    HarborSeal = 122,
    FurSeal = 123,
    WeddellSeal = 124,
    ElephantSeal = 125,
    SeaLion = 130,
    AustralianSeaLion = 131,
    CaliforniaSeaLion = 132,
    Walrus = 140,
    AtlanticWalrus = 141,
    PacificWalrus = 142,
    Otter = 150,
    SeaOtter = 151,
    Manatee = 160,
    FloridaManatee = 161,
    Dugongs = 162,
    PolarBear = 200,
}

// SISO-REF-010-2023 SubcategoriesforSubsurfaceCategory202Mollusk [UID 122]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforSubsurfaceCategory202Mollusk {
    #[default]
    Snail = 1,
    Slug = 10,
    Octopus = 20,
    Squid = 30,
    Cuttlefish = 40,
    Clam = 50,
    Muscle = 60,
    Oyster = 70,
    Scallop = 80,
}

// SISO-REF-010-2023 SubcategoriesforSubsurfaceCategory203Crustacean [UID 123]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforSubsurfaceCategory203Crustacean {
    #[default]
    Shrimp = 1,
    SnappingShrimp = 2,
    Crayfish = 10,
    Lobster = 20,
    Crab = 30,
}

// SISO-REF-010-2023 SubcategoriesforSubsurfaceCategory204Insect [UID 124]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SubcategoriesforSubsurfaceCategory204Insect {
    #[default]
    SeaSkater = 1,
    WaterBeetle = 2,
}

// SISO-REF-010-2023 AnimalLifeformGroupSizeRangeEnumerationforallDomains [UID 130]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AnimalLifeformGroupSizeRangeEnumerationforallDomains {
    #[default]
    Numberofanimalsrangefrom201to249 = 201,
    Numberofanimalsrangefrom250to299 = 202,
    Numberofanimalsrangefrom300to399 = 203,
    Numberofanimalsrangefrom400to499 = 204,
    Numberofanimalsrangefrom500to999 = 205,
    Numberofanimalsrangefrom1000to1499 = 206,
    Numberofanimalsrangefrom1500to1999 = 207,
    Numberofanimalsrangefrom2000to2999 = 208,
    Numberofanimalsrangefrom3000to4999 = 210,
    Numberofanimalsrangefrom5000to6999 = 212,
    Numberofanimalsrangefrom7000to9999 = 214,
    Numberofanimalsrangefrom10000to19999 = 216,
    Numberofanimalsrangefrom20000to50000 = 218,
    Numberofanimalsrangegreaterthan50000 = 220,
}

// SISO-REF-010-2023 SpecificDimensionEnumerationsforLandAreaSize [UID 131]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SpecificDimensionEnumerationsforLandAreaSize {
    #[default]
    SmallArea = 222,
    SmallAreaDense = 223,
    MediumArea = 224,
    MediumAreaDense = 225,
    LargeArea = 226,
    LargeAreaDense = 227,
}

// SISO-REF-010-2023 SpecificDimensionEnumerationsforAirAreaSize [UID 132]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SpecificDimensionEnumerationsforAirAreaSize {
    #[default]
    SmallFlockSwarm = 222,
    SmallFlockSwarmDense = 223,
    MediumFlockSwarm = 224,
    MediumFlockSwarmDense = 225,
    LargeFlockSwarm = 226,
    LargeFlockSwarmDense = 227,
}

// SISO-REF-010-2023 AddSpecificDimensionEnumerationsforSubsurfaceAreaSize [UID 133]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AddSpecificDimensionEnumerationsforSubsurfaceAreaSize {
    #[default]
    SmallSchool = 222,
    SmallSchoolDense = 223,
    MediumSchool = 224,
    MediumSchoolDense = 225,
    LargeSchool = 226,
    LargeSchoolDense = 227,
}

// SISO-REF-010-2023 AddVariantsforLandCategory200Mammal [UID 134]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AddVariantsforLandCategory200Mammal {
    #[default]
    AnimalwithaMaleChildRider = 1,
    AnimalwithaFemaleChildRider = 2,
    AnimalwithanAdultMaleRider = 3,
    AnimalwithanAdultFemaleRider = 4,
    AnimalHarnessedtoaPlow = 5,
    AnimalHarnessedtoaCart = 6,
}

// SISO-REF-010-2023 VariantsforLandCategoriesReptilesAmphibiansInsectsandArachnids [UID 135]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum VariantsforLandCategoriesReptilesAmphibiansInsectsandArachnids {
    #[default]
    Black = 1,
    Green = 2,
    Spotted = 3,
    Red = 4,
    Brown = 5,
}

// SISO-REF-010-2023 VariantsforAirCategory200Bird [UID 136]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum VariantsforAirCategory200Bird {
    #[default]
    BirdwithFish = 1,
    VPatternFlockShape = 2,
    CircularFlockShape = 3,
    IrregularFlockShape = 4,
}

// SISO-REF-010-2023 AddVariantsforAirCategory201Insect [UID 137]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AddVariantsforAirCategory201Insect {
    #[default]
    VerticalShapedInsectSwarm = 1,
    CircularShapedInsectSwarm = 2,
    IrregularShapedInsectSwarm = 3,
}

// SISO-REF-010-2023 AddVariantsforSubsurfaceCategoriesFishMolluskCrustaceanandInsect [UID 138]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AddVariantsforSubsurfaceCategoriesFishMolluskCrustaceanandInsect {
    #[default]
    Black = 1,
    Green = 2,
    Spotted = 3,
    Red = 4,
    Brown = 5,
    Blue = 6,
    Silver = 7,
    Grey = 8,
}

// SISO-REF-010-2023 VariantsforSubsurfaceCategory201Mammal [UID 139]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum VariantsforSubsurfaceCategory201Mammal {
    #[default]
    Singing = 1,
    Spouting = 2,
}

// SISO-REF-010-2023 UAStateChangeUpdateIndicator [UID 143]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum UAStateChangeUpdateIndicator {
    #[default]
    StateUpdate = 0,
    ChangedDataUpdate = 1,
}

// SISO-REF-010-2023 UAAcousticSystemName [UID 144]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum UAAcousticSystemName {
    #[default]
    Other = 0,
    ANBQQ5 = 1,
    ANSSQ62 = 2,
    ANSQS23 = 3,
    ANSQS26 = 4,
    ANSQS53 = 5,
    ALFS = 6,
    LFA = 7,
    ANAQS901 = 8,
    ANAQS902 = 9,
}

// SISO-REF-010-2023 UAAcousticEmitterSystemFunction [UID 145]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum UAAcousticEmitterSystemFunction {
    #[default]
    Other = 0,
    Platformsearchdetecttrack = 1,
    Navigation = 2,
    Minehunting = 3,
    Weaponsearchdetecttrackdetect = 4,
}

// SISO-REF-010-2023 UAActiveEmissionParameterIndex [UID 146]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum UAActiveEmissionParameterIndex {
    #[default]
    Other = 0,
}

// SISO-REF-010-2023 UAScanPattern [UID 147]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum UAScanPattern {
    #[default]
    Scanpatternnotused = 0,
    Conical = 1,
    Helical = 2,
    Raster = 3,
    Sectorsearch = 4,
    Continuoussearch = 5,
}

// SISO-REF-010-2023 UAPassiveParameterIndex [UID 148]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum UAPassiveParameterIndex {
    #[default]
    Other = 0,
}

// SISO-REF-010-2023 UAAdditionalPassiveActivityParameterIndex [UID 150]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum UAAdditionalPassiveActivityParameterIndex {
    #[default]
    Other = 0,
}

// SISO-REF-010-2023 TransmitterMajorModulation [UID 155]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterMajorModulation {
    #[default]
    NoStatement = 0,
    Amplitude = 1,
    AmplitudeandAngle = 2,
    Angle = 3,
    Combination = 4,
    Pulse = 5,
    Unmodulated = 6,
    CarrierPhaseShiftModulation = 7,
    SATCOM = 8,
}

// SISO-REF-010-2023 TransmitterDetailAmplitudeModulation [UID 156]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterDetailAmplitudeModulation {
    #[default]
    Other = 0,
    AFSK = 1,
    AM = 2,
    CW = 3,
    DSB = 4,
    ISB = 5,
    LSB = 6,
    SSBFull = 7,
    SSBReduc = 8,
    USB = 9,
    VSB = 10,
}

// SISO-REF-010-2023 TransmitterDetailAmplitudeandAngleModulation [UID 157]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterDetailAmplitudeandAngleModulation {
    #[default]
    Other = 0,
    AmplitudeandAngle = 1,
}

// SISO-REF-010-2023 TransmitterDetailAnglemodulation [UID 158]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterDetailAnglemodulation {
    #[default]
    Other = 0,
    FM = 1,
    FSK = 2,
    PM = 3,
    MSK = 4,
}

// SISO-REF-010-2023 TransmitterDetailCombinationModulation [UID 159]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterDetailCombinationModulation {
    #[default]
    Other = 0,
    AmplitudeAnglePulse = 1,
}

// SISO-REF-010-2023 TransmitterDetailPulseModulation [UID 160]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterDetailPulseModulation {
    #[default]
    Other = 0,
    Pulse = 1,
    XBandTACANPulse = 2,
    YBandTACANPulse = 3,
}

// SISO-REF-010-2023 TransmitterDetailUnmodulatedModulation [UID 161]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterDetailUnmodulatedModulation {
    #[default]
    Other = 0,
    ContinuousWaveemissionofanunmodulatedcarrier = 1,
}

// SISO-REF-010-2023 TransmitterDetailCarrierPhaseShiftModulation [UID 162]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterDetailCarrierPhaseShiftModulation {
    #[default]
    Other = 0,
}

// SISO-REF-010-2023 TransmitterModulationTypeSystem [UID 163]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterModulationTypeSystem {
    #[default]
    Other = 0,
    GenericRadioorSimpleIntercom = 1,
    HAVEQUICKI = 2,
    HAVEQUICKII = 3,
    SATURN = 4,
    SINCGARS = 5,
    CCTTSINCGARS = 6,
    EPLRS = 7,
    JTIDSMIDS = 8,
    Link11 = 9,
    Link11B = 10,
    LBandSATCOM = 11,
    EnhancedSINCGARS7_3 = 12,
    NavigationAid = 13,
}

// SISO-REF-010-2023 TransmitterTransmitState [UID 164]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterTransmitState {
    #[default]
    Off = 0,
    Onbutnottransmitting = 1,
    Onandtransmitting = 2,
}

// SISO-REF-010-2023 TransmitterInputSource [UID 165]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterInputSource {
    #[default]
    Other = 0,
    Pilot = 1,
    Copilot = 2,
    FirstOfficer = 3,
    Driver = 4,
    Loader = 5,
    Gunner = 6,
    Commander = 7,
    DigitalDataDevice = 8,
    Intercom = 9,
    AudioJammer = 10,
    DataJammer = 11,
    GPSJammer = 12,
    GPSMeaconer = 13,
    SATCOMUplinkJammer = 14,
}

// SISO-REF-010-2023 TransmitterCryptoSystem [UID 166]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterCryptoSystem {
    #[default]
    NoEncryptionDevice = 0,
    KY28 = 1,
    KY58 = 2,
    NarrowSpectrumSecureVoice = 3,
    WideSpectrumSecureVoice = 4,
    SINCGARSICOM = 5,
    KY75 = 6,
    KY100 = 7,
    KY57 = 8,
    KYV5 = 9,
    Link11KG40AP = 10,
    Link11BKG40AS = 11,
    Link11KG40AR = 12,
    KGV135A = 13,
    TacticalSecureVoice = 14,
}

// SISO-REF-010-2023 TransmitterAntennaPatternType [UID 167]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterAntennaPatternType {
    #[default]
    Isotropic = 0,
    Beam = 1,
    Sphericalharmonic = 2,
    TransmitterRadiationVolume = 4,
    BeamandTransmitterRadiationVolume = 5,
    Omnidirectional = 6,
}

// SISO-REF-010-2023 TransmitterAntennaPatternReferenceSystem [UID 168]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterAntennaPatternReferenceSystem {
    #[default]
    WorldCoordinates = 1,
    EntityCoordinates = 2,
}

// SISO-REF-010-2023 CCTTSINCGARSStartofMessage [UID 170]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum CCTTSINCGARSStartofMessage {
    #[default]
    Notstartofmessage = 0,
    StartofMessage = 1,
}

// SISO-REF-010-2023 CCTTSINCGARSClearChannel [UID 171]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum CCTTSINCGARSClearChannel {
    #[default]
    Notclearchannel = 0,
    Clearchannel = 1,
}

// SISO-REF-010-2023 TimeSlotAllocationLevel [UID 172]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TimeSlotAllocationLevel {
    #[default]
    LowFidelityLevel0 = 0,
    LowFidelityLevel1 = 1,
    MediumFidelityLevel2 = 2,
    MediumFidelityLevel3 = 3,
    HighFidelityLevel4 = 4,
}

// SISO-REF-010-2023 JTIDSMIDSModulationParametersTransmittingTerminalPrimaryMode [UID 173]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum JTIDSMIDSModulationParametersTransmittingTerminalPrimaryMode {
    #[default]
    NTR = 1,
    JTIDSUnitParticipant = 2,
}

// SISO-REF-010-2023 JTIDSMIDSModulationParametersTransmittingTerminalSecondaryMode [UID 174]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum JTIDSMIDSModulationParametersTransmittingTerminalSecondaryMode {
    #[default]
    None = 0,
    NetPositionReference = 1,
    PrimaryNavigationController = 2,
    SecondaryNavigationController = 3,
}

// SISO-REF-010-2023 JTIDSMIDSModulationParametersSynchronizationState [UID 175]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum JTIDSMIDSModulationParametersSynchronizationState {
    #[default]
    NoStatement = 0,
    InitialNetEntry = 1,
    CoarseSynchronization = 2,
    FineSynchronization = 3,
    SynchronizationMaintenance = 4,
}

// SISO-REF-010-2023 MessageTypeIdentifier [UID 176]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MessageTypeIdentifier {
    #[default]
    JTIDSHeaderMessages = 0,
    RTTAB = 1,
    RTTReply = 2,
    JTIDSVoiceCVSD = 3,
    JTIDSVoiceLPC10 = 4,
    JTIDSVoiceLPC12 = 5,
    JTIDSLET = 6,
    VMF = 7,
}

// SISO-REF-010-2023 SignalUserProtocolIdentificationNumber [UID 177]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SignalUserProtocolIdentificationNumber {
    #[default]
    CCSIL = 1,
    A2ATDSINCGARSERF = 5,
    A2ATDCAC2 = 6,
    BattleCommand = 20,
    AFIWCIADSTrackReport = 30,
    AFIWCIADSCommC2Message = 31,
    AFIWCIADSGroundControlInterceptorCommand = 32,
    AFIWCVoiceTextMessage = 35,
    ModSAFTextRadio = 177,
    CCTTSINCGARSERFLOCKOUT = 200,
    CCTTSINCGARSERFHOPSET = 201,
    CCTTSINCGARSOTAR = 202,
    CCTTSINCGARSDATA = 203,
    ModSAFFWAForwardAirController = 546,
    ModSAFThreatADAC3 = 832,
    F16MTCAFAPDProtocol = 1000,
    F16MTCIDLProtocol = 1100,
    AutomaticIdentificationSystem = 1371,
    ModSAFArtilleryFireControl = 4570,
    AGTS = 5361,
    GC3 = 6000,
    WNCPdata = 6010,
    Spokentextmessage = 6020,
    LongbowIDMmessage = 6661,
    ComancheIDMmessage = 6662,
    LongbowAirborneTACFIREMessage = 6663,
    LongbowGroundTACFIREMessage = 6664,
    LongbowAFAPDMessage = 6665,
    LongbowERFmessage = 6666,
    VMFIDM = 7000,
    CSARRadioSurvivorMessage = 7010,
    CSARRadioInterrogatorMessage = 7020,
    ImageFileTransferMessage = 7030,
    GeotagDataMessage = 7040,
    TacticalVideoRegenerationData = 7050,
}

// SISO-REF-010-2023 SignalTDLType [UID 178]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SignalTDLType {
    #[default]
    Other = 0,
    PADIL = 1,
    NATOLink1 = 2,
    ATDL1 = 3,
    Link11B = 4,
    SituationalAwarenessDataLink = 5,
    Link16LegacyFormat = 6,
    Link16LegacyFormat = 7,
    Link11 = 8,
    IJMS = 9,
    Link4A = 10,
    Link4C = 11,
    TIBS = 12,
    ATL = 13,
    ConstantSource = 14,
    AbbreviatedCommandandControl = 15,
    MILSTAR = 16,
    ATHS = 17,
    OTHGOLD = 18,
    TACELINT = 19,
    WeaponsDataLink = 20,
    AbbreviatedCommandandControl = 21,
    EnhancedPositionLocationReportingSystem = 22,
    PositionLocationReportingSystem = 23,
    SINCGARS = 24,
    HAVEQUICKI = 25,
    HAVEQUICKII = 26,
    SATURN = 27,
    IntraFlightDataLink1 = 28,
    IntraFlightDataLink2 = 29,
    ImprovedDataModem = 30,
    AirForceApplicationProgramDevelopment = 31,
    CooperativeEngagementCapability = 32,
    ForwardAreaAirDefense = 33,
    GroundBasedDataLink = 34,
    IntraVehicularInfoSystem = 35,
    MarineTacticalSystem = 36,
    TacticalFireDirectionSystem = 37,
    IntegratedBroadcastService = 38,
    AirborneInformationTransfer = 39,
    AdvancedTacticalAirborneReconnaissanceSystemDataLink = 40,
    BattleGroupPassiveHorizonExtensionSystemDataLink = 41,
    CommonHighBandwidthDataLink = 42,
    GuardrailInteroperableDataLink = 43,
    GuardrailCommonSensorSystemOneDataLink = 44,
    GuardrailCommonSensorSystemTwoDataLink = 45,
    GuardrailCSS2MultiRoleDataLink = 46,
    GuardrailCSS2DirectAirtoSatelliteRelayDataLink = 47,
    LineofSight = 48,
    LightweightCDL = 49,
    L52M = 50,
    RivetReachRivetOwlDataLink = 51,
    SeniorSpan = 52,
    SeniorSpur = 53,
    SeniorStretch_ = 54,
    SeniorYearInteroperableDataLink = 55,
    SpaceCDL = 56,
    TR1modeMISTAirborneDataLink = 57,
    KubandSATCOMDataLinkImplementation = 58,
    MissionEquipmentControlDatalink = 59,
    RadarDataTransmittingSetDataLink = 60,
    SurveillanceandControlDataLink = 61,
    TacticalUAVVideo = 62,
    UHFSATCOMDataLinkImplementation = 63,
    TacticalCommonDataLink = 64,
    LowLevelAirPictureInterface = 65,
    WeaponsDataLink = 66,
    AutomaticIdentificationSystem = 67,
    WeaponsDataLink = 68,
    WeaponsDataLink = 69,
    WeaponsDataLink = 70,
    GC3 = 99,
    Link16StandardizedFormat = 100,
    Link16EnhancedDataRate = 101,
    JTIDSMIDSNetDataLoad = 102,
    Link22 = 103,
    AFIWCIADSCommunicationsLinks = 104,
    F22IntraFlightDataLink = 105,
    LBandSATCOM = 106,
    TSAFCommunicationsLink = 107,
    EnhancedSINCGARS7_3 = 108,
    F35MultifunctionAdvancedDataLink = 109,
    CursoronTarget = 110,
    AllPurposeStructuredEurocontrolSurveillanceInformationExchange = 111,
    VariableMessageFormat = 112,
    Link16SurrogateforNonNATOTDL = 113,
    MQ19CBandLOSUplink = 114,
    MQ19CBandLOSDownlink = 115,
    MQ19KuBandSATCOMUplink = 116,
    MQ19KuBandSATCOMDownlink = 117,
    WeaponsDatalink = 118,
    JTACSAUplink = 119,
    CommonInteractiveBroadcast = 120,
    JointRangeExtensionApplicationProtocolA = 121,
    JPALSDataLink = 125,
    OneSAFIADSCommunicationsLink = 126,
    TacticalTargetingNetworkTechnologyApplication = 127,
}

// SISO-REF-010-2023 ReceiverReceiverState [UID 179]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ReceiverReceiverState {
    #[default]
    Off = 0,
    Onbutnotreceiving = 1,
    Onandreceiving = 2,
}

// SISO-REF-010-2023 IntercomControlControlType [UID 180]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IntercomControlControlType {
    #[default]
    Reserved = 0,
    Status = 1,
    RequestAcknowledgeRequired = 2,
    RequestNoAcknowledge = 3,
    AckRequestGranted = 4,
    NackRequestDenied = 5,
}

// SISO-REF-010-2023 IntercomControlCommunicationsType [UID 181]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IntercomControlCommunicationsType {
    #[default]
    Reserved = 0,
    ConnectionFDX = 1,
    ConnectionHDXDestinationisReceiveOnly = 2,
    ConnectionHDXDestinationisTransmitOnly = 3,
    ConnectionHDX = 4,
}

// SISO-REF-010-2023 IntercomControlCommand [UID 182]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IntercomControlCommand {
    #[default]
    NoCommand = 0,
    Status = 1,
    Connect = 2,
    Disconnect = 3,
    Reset = 4,
    On = 5,
    Off = 6,
}

// SISO-REF-010-2023 IntercomControlTransmitLineState [UID 183]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IntercomControlTransmitLineState {
    #[default]
    TransmitLineStatenotapplicable = 0,
    NotTransmitting = 1,
    Transmitting = 2,
}

// SISO-REF-010-2023 IntercomControlDestinationLineStateCommand [UID 184]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IntercomControlDestinationLineStateCommand {
    #[default]
    None = 0,
    SetLineStateTransmitting = 1,
    SetLineStateNotTransmitting = 2,
    ReturntoLocalLineStateControl = 3,
}

// SISO-REF-010-2023 IntercomControlRecordType [UID 185]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IntercomControlRecordType {
    #[default]
    SpecificDestinationrecord = 1,
    GroupDestinationrecord = 2,
    GroupAssignmentrecord = 3,
}

// SISO-REF-010-2023 CollisionType [UID 189]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum CollisionType {
    #[default]
    Inelastic = 0,
    Elastic = 1,
    Boomnozzlehasclearedthereceiversrefuelingreceptacle = 55,
}

// SISO-REF-010-2023 MinefieldSensorTypes [UID 193]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldSensorTypes {
    #[default]
    Other = 0,
    Optical = 1,
    FLIR = 2,
    RADAR = 3,
    Magnetic = 4,
    Laser = 5,
    SONAR = 6,
    Physical = 7,
    Multispectral = 8,
}

// SISO-REF-010-2023 MinefieldSensorTypesOptical [UID 194]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldSensorTypesOptical {
    #[default]
    UnaidedEyeActivelySearching = 0,
    UnaidedEyeNotActivelySearching = 1,
    Binoculars = 2,
    ImageIntensifier = 3,
    HMMWVoccupantActivelySearching = 4,
    HMMWVoccupantNotActivelySearching = 5,
    TruckoccupantActivelySearching = 6,
    TruckoccupantNotActivelySearching = 7,
    TrackedvehicleoccupantclosedhatchActivelySearching = 8,
    TrackedvehicleoccupantclosedhatchNotActivelySearching = 9,
    TrackedvehicleoccupantopenhatchActivelySearching = 10,
    TrackedvehicleoccupantopenhatchNotActivelySearching = 11,
}

// SISO-REF-010-2023 MinefieldSensorTypesFLIR [UID 195]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldSensorTypesFLIR {
    #[default]
    Generic35 = 0,
    Generic812 = 1,
    ASTAMIDSI = 2,
    ASTAMIDSII = 3,
    GSTAMIDS35 = 4,
    GSTAMIDS812 = 5,
    HSTAMIDS35 = 6,
    HSTAMIDS812 = 7,
    COBRA35 = 8,
    COBRA812 = 9,
}

// SISO-REF-010-2023 MinefieldSensorTypesRADAR [UID 196]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldSensorTypesRADAR {
    #[default]
    Generic = 0,
    GenericGPR = 1,
    GSTAMIDSI = 2,
    GSTAMIDSII = 3,
    HSTAMIDSI = 4,
    HSTAMIDSII = 5,
}

// SISO-REF-010-2023 MinefieldSensorTypesMagnetic [UID 197]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldSensorTypesMagnetic {
    #[default]
    Generic = 0,
    ANPSS11 = 1,
    ANPSS12 = 2,
    GSTAMIDS = 3,
}

// SISO-REF-010-2023 MinefieldSensorTypesLaser [UID 198]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldSensorTypesLaser {
    #[default]
    Generic = 0,
    ASTAMIDS = 1,
}

// SISO-REF-010-2023 MinefieldSensorTypesSONAR [UID 199]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldSensorTypesSONAR {
    #[default]
    Generic = 0,
}

// SISO-REF-010-2023 MinefieldSensorTypesPhysical [UID 200]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldSensorTypesPhysical {
    #[default]
    GenericProbe = 0,
    Probemetalcontent = 1,
    Probenometalcontent = 2,
}

// SISO-REF-010-2023 MinefieldSensorTypesMultispectral [UID 201]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldSensorTypesMultispectral {
    #[default]
    Generic = 0,
}

// SISO-REF-010-2023 AggregateStateAggregateState [UID 204]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AggregateStateAggregateState {
    #[default]
    Other = 0,
    Aggregated = 1,
    Disaggregated = 2,
    Fullydisaggregated = 3,
    Pseudodisaggregated = 4,
    Partiallydisaggregated = 5,
}

// SISO-REF-010-2023 AggregateStateFormation [UID 205]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AggregateStateFormation {
    #[default]
    Other = 0,
    Assembly = 1,
    Vee = 2,
    Wedge = 3,
    Line = 4,
    Column = 5,
}

// SISO-REF-010-2023 AggregateStateAggregateKind [UID 206]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AggregateStateAggregateKind {
    #[default]
    Other = 0,
    MilitaryHierarchy = 1,
    CommonType = 2,
    CommonMission = 3,
    SimilarCapabilities = 4,
    CommonLocation = 5,
}

// SISO-REF-010-2023 AggregateStateSubcategory [UID 208]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AggregateStateSubcategory {
    #[default]
    Other = 0,
    CavalryTroop = 1,
    Armor = 2,
    Infantry = 3,
    MechanizedInfantry = 4,
    Cavalry = 5,
    ArmoredCavalry = 6,
    Artillery = 7,
    SelfPropelledArtillery = 8,
    CloseAirSupport = 9,
    Engineer = 10,
    AirDefenseArtillery = 11,
    AntiTank = 12,
    ArmyAviationFixedwing = 13,
    ArmyAviationRotarywing = 14,
    ArmyAttackHelicopter = 15,
    AirCavalry = 16,
    ArmorHeavyTaskForce = 17,
    MotorizedRifle = 18,
    MechanizedHeavyTaskForce = 19,
    CommandPost = 20,
    CEWI = 21,
    Tankonly = 22,
}

// SISO-REF-010-2023 AggregateStateSpecific [UID 209]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AggregateStateSpecific {
    #[default]
    Noheadquarters = 0,
    Yesaggregateunitcontainsaheadquarters = 1,
}

// SISO-REF-010-2023 IsPartOfNature [UID 210]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IsPartOfNature {
    #[default]
    Other = 0,
    Hostfireablemunition = 1,
    Munitioncarriedascargo = 2,
    Fuelcarriedascargo = 3,
    Gunmountattachedtohost = 4,
    Computergeneratedforcescarriedascargo = 5,
    Vehiclecarriedascargo = 6,
    Emittermountedonhost = 7,
    Mobilecommandandcontrolentitycarriedaboardhost = 8,
    Entitystationedatpositionwithrespecttohost = 9,
    Teammemberinformationwith = 10,
}

// SISO-REF-010-2023 IsPartOfPosition [UID 211]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IsPartOfPosition {
    #[default]
    Other = 0,
    Ontopof = 1,
    Insideof = 2,
}

// SISO-REF-010-2023 IsPartOfStationName [UID 212]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IsPartOfStationName {
    #[default]
    Other = 0,
    AircraftWingstation = 1,
    ShipsForwardGunmount = 2,
    ShipsForwardGunmount = 3,
    ShipsForwardGunmount = 4,
    ShipsAftGunmount = 5,
    ShipsAftGunmount = 6,
    ShipsAftGunmount = 7,
    ForwardTorpedoTube = 8,
    AftTorpedoTube = 9,
    BombBay = 10,
    CargoBay = 11,
    TruckBed = 12,
    TrailerBed = 13,
    WellDeck = 14,
    OnStationRangeandBearing = 15,
    OnStationxyz = 16,
    AirtoAirRefuelingBoom = 17,
    AerialRefuelingReceptacle = 18,
    PortSideRefuelingDrogue = 19,
    StarboardSideRefuelingDrogue = 20,
    CenterRefuelingDrogue = 21,
    AirRefuelingProbe = 22,
}

// SISO-REF-010-2023 IsGroupOfGroupedEntityCategory [UID 213]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IsGroupOfGroupedEntityCategory {
    #[default]
    Undefined = 0,
    BasicGroundCombatVehicle = 1,
    EnhancedGroundCombatVehicle = 2,
    BasicGroundCombatSoldier = 3,
    EnhancedGroundCombatSoldier = 4,
    BasicRotorWingAircraft = 5,
    EnhancedRotorWingAircraft = 6,
    BasicFixedWingAircraft = 7,
    EnhancedFixedWingAircraft = 8,
    GroundLogisticsVehicle = 9,
}

// SISO-REF-010-2023 IsGroupOfRestStatus [UID 214]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IsGroupOfRestStatus {
    #[default]
    Notrested = 0,
    Hassleptanaverageof1hourperdayinthelastthreedays_ = 1,
    Hassleptanaverageof2hoursperdayinthelastthreedays_ = 2,
    Hassleptanaverageof3hoursperdayinthelastthreedays_ = 3,
    Hassleptanaverageof4hoursperdayinthelastthreedays_ = 4,
    Hassleptanaverageof5hoursperdayinthelastthreedays_ = 5,
    Hassleptanaverageof6hoursperdayinthelastthreedays_ = 6,
    Hassleptanaverageof7hoursperdayinthelastthreedays_ = 7,
    Fullyrested = 8,
}

// SISO-REF-010-2023 TransferControlTransferType [UID 224]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransferControlTransferType {
    #[default]
    Other = 0,
    PushTransferEntity = 1,
    AutomaticPullTransferEntity = 2,
    NotUsed = 3,
    PushTransferEnvironmentalProcess = 4,
    AutomaticPullTransferEnvironmentalProcess = 5,
    NotUsed = 6,
    CancelTransfer = 7,
    ManualPullTransferEntity = 8,
    ManualPullTransferEnvironmentalProcess = 9,
    RemoveEntity = 10,
}

// SISO-REF-010-2023 ObjectKind [UID 225]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ObjectKind {
    #[default]
    Other = 0,
    Obstacle = 1,
    PreparedPosition = 2,
    CulturalFeature = 3,
    Passageway = 4,
    TacticalSmoke = 5,
    ObstacleMarker = 6,
    ObstacleBreach = 7,
    EnvironmentalObject = 8,
}

// SISO-REF-010-2023 GriddedDataFieldNumber [UID 243]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum GriddedDataFieldNumber {
    #[default]
}

// SISO-REF-010-2023 GriddedDataCoordinateSystem [UID 244]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum GriddedDataCoordinateSystem {
    #[default]
    RighthandedCartesian = 0,
    LefthandedCartesian = 1,
    LatitudeLongitudeHeight = 2,
    LatitudeLongitudeDepth = 3,
}

// SISO-REF-010-2023 GriddedDataConstantGrid [UID 245]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum GriddedDataConstantGrid {
    #[default]
    Constantgrid = 0,
    Updatedgrid = 1,
}

// SISO-REF-010-2023 GriddedDataSampleType [UID 246]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum GriddedDataSampleType {
    #[default]
    NotSpecified = 0,
}

// SISO-REF-010-2023 GriddedDataDataRepresentation [UID 247]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum GriddedDataDataRepresentation {
    #[default]
    Type0 = 0,
    Type1 = 1,
    Type2 = 2,
}

// SISO-REF-010-2023 EnvironmentalProcessModelType [UID 248]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EnvironmentalProcessModelType {
    #[default]
    NoStatement = 0,
}

// SISO-REF-010-2023 EnvironmentalProcessRecordType [UID 250]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EnvironmentalProcessRecordType {
    #[default]
    COMBICState = 256,
    FlareState = 259,
    BoundingSphereRecord = 65536,
    UniformGeometryRecord = 327680,
    PointRecord1 = 655360,
    LineRecord1 = 786432,
    SphereRecord1 = 851968,
    EllipsoidRecord1 = 1048576,
    ConeRecord1 = 3145728,
    RectangularVolumeRecord1 = 5242880,
    RectangularVolumeRecord3 = 83886080,
    PointRecord2 = 167772160,
    LineRecord2 = 201326592,
    SphereRecord2 = 218103808,
    EllipsoidRecord2 = 268435456,
    ConeRecord2 = 805306368,
    RectangularVolumeRecord2 = 1342177280,
    GaussianPlumeRecord = 1610612736,
    GaussianPuffRecord = 1879048192,
}

// SISO-REF-010-2023 SignalEncodingClass [UID 270]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SignalEncodingClass {
    #[default]
    Encodedaudio = 0,
    RawBinaryData = 1,
    ApplicationSpecificData = 2,
    Databaseindex = 3,
}

// SISO-REF-010-2023 SignalEncodingType [UID 271]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SignalEncodingType {
    #[default]
    8bitmulaw = 1,
    CVSD = 2,
    ADPCM = 3,
    16bitLinearPCM2sComplementBigEndian = 4,
    8bitLinearPCMUnsigned = 5,
    VQ = 6,
     = 7,
    GSMFullRate = 8,
    GSMHalfRate = 9,
    SpeexNarrowBand = 10,
    Opus = 11,
    LPC10 = 12,
    16bitLinearPCM2sComplementLittleEndian = 100,
     = 255,
}

// SISO-REF-010-2023 RepairGroups [UID 272]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum RepairGroups {
    #[default]
    GeneralRepairCodes = 0,
    DriveTrain = 1,
    HullAirframeBody = 2,
    InterfaceswithEnvironment = 3,
    Weapons = 4,
    FuelSystems = 5,
    Electronics = 6,
    LifeSupportSystems = 7,
    HydraulicSystemsandActuators = 8,
    AuxiliaryCraft = 9,
}

// SISO-REF-010-2023 EnvironmentRecordTypeGroups [UID 273]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EnvironmentRecordTypeGroups {
    #[default]
    State = 0,
    Geometry = 1,
}

// SISO-REF-010-2023 PlatformAirCivilianUltralightNonrigidWingAircraftSubcategories [UID 274]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformAirCivilianUltralightNonrigidWingAircraftSubcategories {
    #[default]
    HangGliderUnpowered = 1,
    HangGliderPowered = 2,
    ParagliderUnpowered = 3,
    ParagliderPowered = 4,
    PoweredParachute = 5,
}

// SISO-REF-010-2023 PlatformAirCivilianUltralightRigidWingAircraftSubcategories [UID 275]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformAirCivilianUltralightRigidWingAircraftSubcategories {
    #[default]
    Weightshiftcontrol = 1,
    Controlsurfacecontrol = 2,
}

// SISO-REF-010-2023 PlatformAirCivilianGliderSubcategories [UID 276]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformAirCivilianGliderSubcategories {
    #[default]
    SailPlane = 1,
    MotorGlider = 2,
}

// SISO-REF-010-2023 PlatformAirCivilianFixedWingAircraftSubcategories [UID 277]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformAirCivilianFixedWingAircraftSubcategories {
    #[default]
    SinglePistonEngine = 11,
    TwinPistonEngine = 12,
    SingleEngineTurboprop = 21,
    TwinEngineTurboprop = 22,
    FourEngineTurboprop = 24,
    TwinJet = 32,
    TriJet = 33,
    FourEngineJet = 34,
}

// SISO-REF-010-2023 PlatformAirCivilianHelicopterSubcategories [UID 278]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformAirCivilianHelicopterSubcategories {
    #[default]
    SingleRotorPistonEngine = 11,
    SingleRotorTurboshaftEngineConventionalTailRotor = 12,
    SingleRotorTurboshaftEngineShroudedTailRotor = 13,
    SingleRotorTurboshaftEngineNoTailRotor = 14,
    TandemRotor = 21,
    CoaxialRotor = 22,
    IntermeshingRotor = 23,
}

// SISO-REF-010-2023 PlatformAirCivilianLighterthanAirBalloonSubcategories [UID 279]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformAirCivilianLighterthanAirBalloonSubcategories {
    #[default]
    Gasfilledfree = 1,
    Gasfilledtethered = 2,
    HotAir = 3,
    RoziereBalloon = 4,
    Helikite = 5,
}

// SISO-REF-010-2023 PlatformAirCivilianLighterthanAirAirshipSubcategories [UID 280]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformAirCivilianLighterthanAirAirshipSubcategories {
    #[default]
    Nonrigid = 1,
    Semirigid = 2,
    Rigid = 3,
    Hybrid = 4,
}

// SISO-REF-010-2023 APAParameterIndexAPAStatus [UID 281]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum APAParameterIndexAPAStatus {
    #[default]
    DeselectedOff = 0,
    APAValueChangeOnly = 1,
    StateChange = 2,
    RecordActivation = 3,
}

// SISO-REF-010-2023 SeparationVPReasonforSeparation [UID 282]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SeparationVPReasonforSeparation {
    #[default]
    NoStatement = 0,
    AttachedPartSeparation = 1,
    SubmunitionSeparation = 2,
}

// SISO-REF-010-2023 SeparationVPPreEntityIndicator [UID 283]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SeparationVPPreEntityIndicator {
    #[default]
    NoStatement = 0,
    EntityIDExistedPriortoSeparationwithoutEntityStatePDU = 1,
    EntityIDExistedPriortoSeparationwithEntityStatePDUIssued = 2,
    EntityInitiallyCreatedatSeparationEvent = 3,
}

// SISO-REF-010-2023 IOActionIOWarfareType [UID 285]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IOActionIOWarfareType {
    #[default]
    NoStatement = 0,
    ElectronicWarfare = 1,
    ComputerNetworkOperations = 2,
    PsychologicalOperations = 3,
    MilitaryDeception = 4,
    OperationsSecurity = 5,
    PhysicalAttack = 6,
}

// SISO-REF-010-2023 IOActionIOSimulationSource [UID 286]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IOActionIOSimulationSource {
    #[default]
    NoStatement = 0,
}

// SISO-REF-010-2023 IOActionIOActionType [UID 287]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IOActionIOActionType {
    #[default]
    NoStatement = 0,
    IOAttackProfileData = 1,
    IOAttackComputedEffects = 2,
    IntentBasedEW = 3,
    IntentBasedEWComputedEffects = 4,
}

// SISO-REF-010-2023 IOActionIOActionPhase [UID 288]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IOActionIOActionPhase {
    #[default]
    NoStatement = 0,
    StartAttackProfile = 1,
    EndAttackProfile = 2,
    ContinueAttackProfilewithChanges = 3,
    StartAttackEffects = 4,
    EndAttackedEffects = 5,
    ContinueAttackEffectswithChanges = 6,
}

// SISO-REF-010-2023 IOReportIOReportType [UID 289]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IOReportIOReportType {
    #[default]
    NoStatement = 0,
    InitialReport = 1,
    UpdateReport = 2,
    FinalReport = 3,
}

// SISO-REF-010-2023 IOEffectsRecordIOStatus [UID 290]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IOEffectsRecordIOStatus {
    #[default]
    NoStatement = 0,
    EffectonSender = 1,
    EffectonReceiver = 2,
    EffectonSenderandReceiver = 3,
    EffectonMessage = 4,
    EffectonSenderandMessage = 5,
    EffectonReceiverandMessage = 6,
    EffectonSenderReceiverandMessage = 7,
}

// SISO-REF-010-2023 IOEffectsRecordIOLinkType [UID 291]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IOEffectsRecordIOLinkType {
    #[default]
    NoStatement = 0,
    LogicalLink = 1,
    PhysicalNode = 2,
    PhysicalLink = 3,
}

// SISO-REF-010-2023 IOEffectsRecordIOEffect [UID 292]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IOEffectsRecordIOEffect {
    #[default]
    NoStatement = 0,
    Denial = 1,
    Degradation = 2,
    Disruption = 3,
    TerminateEffect = 255,
}

// SISO-REF-010-2023 IOEffectsRecordIOProcess [UID 293]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IOEffectsRecordIOProcess {
    #[default]
    NoStatement = 0,
}

// SISO-REF-010-2023 IOCommsNodeRecordCommsNodeType [UID 294]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IOCommsNodeRecordCommsNodeType {
    #[default]
    NoStatement = 0,
    SenderNodeID = 1,
    ReceiverNodeID = 2,
    SenderReceiverNodeID = 3,
}

// SISO-REF-010-2023 DISAttributeActionCode [UID 295]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DISAttributeActionCode {
    #[default]
    NoStatement = 0,
}

// SISO-REF-010-2023 DRParametersType [UID 296]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DRParametersType {
    #[default]
    None = 0,
    LocalEulerAngles = 1,
    WorldOrientationQuaternion = 2,
}

// SISO-REF-010-2023 HighFidelityHAVEQUICKTODTransmitIndicator [UID 297]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum HighFidelityHAVEQUICKTODTransmitIndicator {
    #[default]
    NoTODIsBeingTransmitted = 0,
    TODTransmissioninProgress = 1,
}

// SISO-REF-010-2023 NETIDRecordMode [UID 298]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum NETIDRecordMode {
    #[default]
    AHAVEQUICKIorHAVEQUICKIICOMBAT = 1,
    BSATURNCOMBAT = 2,
    TTRAINING = 3,
}

// SISO-REF-010-2023 NETIDRecordFrequencyTable [UID 299]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum NETIDRecordFrequencyTable {
    #[default]
    HQIOperations = 0,
    HQIINATOEuropeAreaOperations = 1,
    HQIINonNATOEuropeAreaOperations = 2,
    SATURNOperations = 3,
}

// SISO-REF-010-2023 EEAttributeStateIndicator [UID 300]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EEAttributeStateIndicator {
    #[default]
    HeartbeatUpdate = 0,
    ChangedData = 1,
    HasCeased = 2,
}

// SISO-REF-010-2023 DISPDUStatusTransferredEntityIndicator(TEI) [UID 301]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DISPDUStatusTransferredEntityIndicator(TEI) {
    #[default]
    NoDifference = 0,
    Difference = 1,
}

// SISO-REF-010-2023 LVCIndicator [UID 302]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LVCIndicator {
    #[default]
    NoStatement = 0,
    Live = 1,
    Virtual = 2,
    Constructive = 3,
}

// SISO-REF-010-2023 DISPDUStatusCoupledExtensionIndicator(CEI) [UID 303]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DISPDUStatusCoupledExtensionIndicator(CEI) {
    #[default]
    NotCoupled = 0,
    Coupled = 1,
}

// SISO-REF-010-2023 DISPDUStatusFireTypeIndicator(FTI) [UID 304]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DISPDUStatusFireTypeIndicator(FTI) {
    #[default]
    Munition = 0,
    Expendable = 1,
}

// SISO-REF-010-2023 DISPDUStatusDetonationTypeIndicator(DTI) [UID 305]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DISPDUStatusDetonationTypeIndicator(DTI) {
    #[default]
    Munition = 0,
    Expendable = 1,
    NonmunitionExplosion = 2,
}

// SISO-REF-010-2023 RadioAttachedIndicator [UID 306]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum RadioAttachedIndicator {
    #[default]
    NoStatement = 0,
    Unattached = 1,
    Attached = 2,
}

// SISO-REF-010-2023 DISPDUStatusIntercomAttachedIndicator(IAI) [UID 307]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DISPDUStatusIntercomAttachedIndicator(IAI) {
    #[default]
    NoStatement = 0,
    Unattached = 1,
    Attached = 2,
}

// SISO-REF-010-2023 DISPDUStatusIFFSimulationMode(ISM) [UID 308]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DISPDUStatusIFFSimulationMode(ISM) {
    #[default]
    Regeneration = 0,
    Interactive = 1,
}

// SISO-REF-010-2023 ExplosiveMaterialGroups [UID 309]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ExplosiveMaterialGroups {
    #[default]
    General = 0,
    LiquidAviationMissileFuels = 1,
    LiquidOtherFuels = 2,
    LiquidExplosiveMaterial = 3,
    Solid = 4,
    Gaseous = 5,
    DustMaterial = 6,
}

// SISO-REF-010-2023 ExplosiveMaterialCategories [UID 310]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ExplosiveMaterialCategories {
    #[default]
    NoStatement = 0,
    AVGAS = 10,
    JetFuel = 11,
    JP4 = 12,
    JP5 = 13,
    JP7 = 14,
    JP8 = 15,
    JP10MissileFuel = 16,
    JPTS = 17,
    JetA = 18,
    JetA1 = 19,
    JetB = 20,
    JetBiofuel = 21,
    GasolinePetrol = 151,
    DieselFuel = 152,
    Ethanol = 153,
    E85Ethanol = 154,
    FuelOil = 155,
    Kerosene = 156,
    CrudeOil = 157,
    LightCrudeOil = 158,
    LiquidPetroleumGas = 159,
    RP1RocketFuel = 160,
    LH2RocketFuel = 161,
    LOXRocketFuel = 162,
    Alcohol = 164,
    Hydrogen = 166,
    Nitroglycerin = 301,
    ANFO = 302,
    Dynamite = 451,
    TNT = 452,
    RDX = 453,
    PETN = 454,
    HMX = 455,
    C4 = 456,
    CompositionC4 = 457,
    NaturalGas = 601,
    Butane = 602,
    Propane = 603,
    Helium = 604,
    Hydrogen = 605,
    Dust = 801,
    GrainDust = 802,
    FlourDust = 803,
    SugarDust = 804,
}

// SISO-REF-010-2023 DEPrecisionAimpointBeamSpotType [UID 311]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DEPrecisionAimpointBeamSpotType {
    #[default]
    Other = 0,
    Gaussian = 1,
    TopHat = 2,
}

// SISO-REF-010-2023 DEFirePulseShape [UID 312]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DEFirePulseShape {
    #[default]
    Other = 0,
    SquareWave = 1,
    ContinuousWave = 2,
    Gaussian = 3,
}

// SISO-REF-010-2023 EntityDamageStatusComponentIdentification [UID 314]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EntityDamageStatusComponentIdentification {
    #[default]
    EntityCenter = 0,
    EntityStructure = 1,
    ControlSystem = 2,
    ControlSurface = 3,
    EnginePropulsionSystem = 4,
    CrewMember = 5,
    Fuse = 6,
    AcquisitionSensor = 7,
    TrackingSensor = 8,
    FuelTankSolidRocketMotor = 9,
}

// SISO-REF-010-2023 DEDamageDescriptionComponentDamageStatus [UID 315]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DEDamageDescriptionComponentDamageStatus {
    #[default]
    NoDamage = 0,
    MinorDamage = 1,
    MediumDamage = 2,
    MajorDamage = 3,
    Destroyed = 4,
}

// SISO-REF-010-2023 DEDamageDescriptionComponentVisualSmokeColor [UID 316]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DEDamageDescriptionComponentVisualSmokeColor {
    #[default]
    NoSmoke = 0,
    White = 1,
    Gray = 2,
    Black = 3,
}

// SISO-REF-010-2023 BeamStatusBeamState [UID 318]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum BeamStatusBeamState {
    #[default]
    Active = 0,
    Deactivated = 1,
}

// SISO-REF-010-2023 EntityAssociationAssociationStatus [UID 319]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EntityAssociationAssociationStatus {
    #[default]
    NotSpecified = 0,
    PhysicalAssociation = 1,
    FunctionalAssociation = 2,
    AssociationBroken = 3,
    PhysicalAssociation = 4,
    FunctionalAssociation = 5,
    FunctionalAssociation = 6,
}

// SISO-REF-010-2023 EntityVPRecordChangeIndicator [UID 320]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EntityVPRecordChangeIndicator {
    #[default]
    InitialReportorNoChangeSinceLastIssuance = 0,
    ChangeSinceLastIssuance = 1,
}

// SISO-REF-010-2023 EntityAssociationGroupMemberType [UID 321]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EntityAssociationGroupMemberType {
    #[default]
    NotPartofaGroup = 0,
    GroupLeader = 1,
    GroupMember = 2,
    FormationLeader = 3,
    FormationMember = 4,
    ConvoyLeader = 5,
    ConvoyMember = 6,
}

// SISO-REF-010-2023 PhysicalAssociationTypeGroups [UID 322]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PhysicalAssociationTypeGroups {
    #[default]
    NotSpecified = 0,
    TowedMountedSlingLoad = 1,
    Restrained = 2,
    Mission = 3,
    OtherConnections = 4,
}

// SISO-REF-010-2023 EntityAssociationPhysicalAssociationType [UID 323]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EntityAssociationPhysicalAssociationType {
    #[default]
    NotSpecified = 0,
    TowedinAir = 1,
    TowedonLand = 2,
    TowedonWaterSurface = 3,
    TowedUnderwater = 4,
    MountedAttached = 5,
    MountedUnattachedandUnsupported = 6,
    MountedUnattachedandSupported = 7,
    TowedinAir = 8,
    TowedinAir = 9,
    TowedinAir = 10,
    TowedinAir = 11,
    TowedinAir = 12,
    TowedinAir = 13,
    TowedinAir = 14,
    Hoisted = 15,
    RestrainedtoaLifeform = 30,
    RestrainedtoaPlatform = 31,
    RestrainedtoanObject = 32,
    RefuelingOperation = 61,
    SearchandRescueBasket = 62,
    SearchandRescueRescueCollar = 63,
    EngagementObject2isBeingEngaged = 64,
    ReturnToBaseObject2istheDestinationObject = 65,
    LinebetweenCommunicationTowers = 90,
    LineBetweenPowerTowers = 91,
    Indoors = 92,
    TopSurface = 93,
}

// SISO-REF-010-2023 EntityAssociationPhysicalConnectionType [UID 324]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EntityAssociationPhysicalConnectionType {
    #[default]
    NotSpecified = 0,
    AttachedDirectlytoSurface = 1,
    CableWire = 2,
    Rope = 3,
    Chain = 4,
    PowerLine = 5,
    TelephoneLine = 6,
    CableLine = 7,
    RefuelingDrogue = 8,
    RefuelingBoom = 9,
    Handcuffs = 10,
    InContactWith = 11,
    FastRope = 12,
}

// SISO-REF-010-2023 SensorRecordSensorTypeOtherActiveSensors [UID 325]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SensorRecordSensorTypeOtherActiveSensors {
    #[default]
    Undefined = 0,
}

// SISO-REF-010-2023 SensorRecordSensorTypePassiveSensors [UID 326]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SensorRecordSensorTypePassiveSensors {
    #[default]
    ALR400 = 60000,
    ANAAR47 = 60001,
    ANAAR50 = 60002,
    ANAAR54 = 60003,
    ANAAR56 = 60004,
    ANAAR57 = 60005,
    ANALQ142 = 60006,
    ANALR45 = 60007,
    ANALR46 = 60008,
    ANALR56 = 60009,
    ANALR59 = 60010,
    ANALR64 = 60011,
    ANALR66 = 60012,
    ANALR67 = 60013,
    ANALR69 = 60014,
    ANALR73 = 60015,
    ANALR76 = 60016,
    ANALR91 = 60017,
    ANALR93 = 60018,
    ANALR94 = 60019,
    ANALR801 = 60020,
    ANAPR39 = 60021,
    ANAYR2 = 60022,
    ARI18223 = 60023,
    BOW21 = 60024,
    ChaparralIRST = 60025,
     = 60026,
     = 60027,
     = 60028,
     = 60029,
     = 60030,
     = 60031,
    KJ200 = 60032,
    KJ8602 = 60033,
    L150Pastel = 60034,
    Serval = 60035,
    Sherloc = 60036,
    Sherlocvf = 60037,
    Sirena2 = 60038,
    Sirena3 = 60039,
    Sirena3M = 60040,
    SkyGuardian = 60041,
    SPO15 = 60042,
    SPS200 = 60043,
    Tarang = 60044,
    ANAAQ29A = 60045,
    101KSUMAW = 60046,
     = 60047,
     = 60048,
    ANAAQ13LANTIRNFLIR = 60049,
    ANALR74 = 60050,
    ANALR90 = 60051,
    ANAPR48 = 60052,
    ELT156X = 60053,
    101KSV = 60054,
    TP23ML = 60055,
     = 60056,
     = 60057,
     = 60058,
     = 60059,
    L136Mak = 60060,
     = 60061,
     = 60062,
     = 60063,
     = 60064,
     = 60065,
    TornadoRWR = 60066,
    TOES521FLIR = 60067,
     = 60068,
}

// SISO-REF-010-2023 MunitionExpendableStatus [UID 327]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MunitionExpendableStatus {
    #[default]
    Other = 0,
    Ready = 1,
    Inventory = 2,
}

// SISO-REF-010-2023 FuelMeasurementUnits [UID 328]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum FuelMeasurementUnits {
    #[default]
    Other = 0,
    Liter = 1,
    Kilogram = 2,
}

// SISO-REF-010-2023 FuelLocation [UID 329]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum FuelLocation {
    #[default]
    Other = 0,
}

// SISO-REF-010-2023 EntityAssociationAssociationType [UID 330]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum EntityAssociationAssociationType {
    #[default]
    Other = 0,
    TargetEntity = 1,
    TargetLocation = 2,
    HomeBaseLocation = 3,
    CAPPointLocation = 4,
    FlightLeader = 5,
    FlightMember = 6,
    IPPoint = 7,
    RendezvousPoint = 8,
    OnStationLocation = 9,
    LandingZoneLocation = 10,
    DownedPilot = 11,
    TankerEntitythatIsCurrentlyRefuelingtheTransferredEntity = 12,
    TankerEntitytheTransferredEntityIsHeadedtowardstoRefuel = 13,
    EntityHeadedtowardstoJoinUpWith = 14,
}

// SISO-REF-010-2023 SensorOnOffStatus [UID 331]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SensorOnOffStatus {
    #[default]
    Off = 0,
    On = 1,
}

// SISO-REF-010-2023 OwnershipStatus [UID 332]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum OwnershipStatus {
    #[default]
    Other = 0,
    NewOwner = 1,
    OwnershipQueryResponse = 2,
    OwnershipConflict = 3,
    LocalEntityCancelledAutoResolveConflict = 4,
    LocalEntityCancelledManualResolveConflict = 5,
    LocalEntityCancelledRemoveEntityTCRReceived = 6,
}

// SISO-REF-010-2023 RecordREventType [UID 333]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum RecordREventType {
    #[default]
    Other = 0,
}

// SISO-REF-010-2023 RecordQueryREventType [UID 334]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum RecordQueryREventType {
    #[default]
    Periodic = 0,
    InternalEntityStateData = 1,
}

// SISO-REF-010-2023 UAPropulsionPlantConfigurationConfiguration [UID 335]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum UAPropulsionPlantConfigurationConfiguration {
    #[default]
    Other = 0,
    DieselElectric = 1,
    Diesel = 2,
    Battery = 3,
    TurbineReduction = 4,
    Steam = 6,
    GasTurbine = 7,
}

// SISO-REF-010-2023 MinefieldStateProtocolMode [UID 336]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldStateProtocolMode {
    #[default]
    HeartbeatMode = 0,
    QRPMode = 1,
}

// SISO-REF-010-2023 TransponderInterrogatorIndicator [UID 337]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransponderInterrogatorIndicator {
    #[default]
    Transponder = 0,
    Interrogator = 1,
}

// SISO-REF-010-2023 IFFSimulationMode [UID 338]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IFFSimulationMode {
    #[default]
    Regeneration = 0,
    Interactive = 1,
}

// SISO-REF-010-2023 IFFApplicableModes [UID 339]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IFFApplicableModes {
    #[default]
    NoApplicableModesData = 0,
    AllModes = 1,
}

// SISO-REF-010-2023 ModeCAltitudeIndicator [UID 340]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ModeCAltitudeIndicator {
    #[default]
    PositiveAltitudeAboveMSL = 0,
    NegativeAltitudeBelowMSLUseAlternateModeCAltitude = 1,
}

// SISO-REF-010-2023 TCASACASBasicAdvancedIndicator [UID 341]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TCASACASBasicAdvancedIndicator {
    #[default]
    Basic = 0,
    Advanced = 1,
}

// SISO-REF-010-2023 TCASACASIndicator [UID 342]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TCASACASIndicator {
    #[default]
    TCAS = 0,
    ACAS = 1,
}

// SISO-REF-010-2023 TCASACASSoftwareVersion [UID 343]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TCASACASSoftwareVersion {
    #[default]
    NoStatement = 0,
    6_0_2 = 1,
    7_0 = 2,
}

// SISO-REF-010-2023 TCASACASType [UID 344]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TCASACASType {
    #[default]
    NoStatement = 0,
    ACASI = 1,
    ACASII = 2,
}

// SISO-REF-010-2023 TCASIIIType [UID 345]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TCASIIIType {
    #[default]
    TCASI = 0,
    TCASII = 1,
}

// SISO-REF-010-2023 Mode5IFFMission [UID 346]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Mode5IFFMission {
    #[default]
    NoStatement = 0,
    SurveillanceSHORAD = 1,
    SHORADassociatedwithaWeaponsSystem = 2,
    WeaponSystem = 3,
    AirborneandSurfaceSurveillancePlatforms = 4,
    AirborneandSurfaceWeaponsPlatforms = 5,
    GroundtoGround = 6,
}

// SISO-REF-010-2023 ModeSInterrogatorStatusTransmitState [UID 347]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ModeSInterrogatorStatusTransmitState {
    #[default]
    NoStatement = 0,
    RollCall = 1,
    AllCall = 2,
    LockoutOverride = 3,
    TemporaryLockout = 4,
    IntermittentLockout = 5,
}

// SISO-REF-010-2023 ModeSInterrogatorIdentifierICType [UID 348]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ModeSInterrogatorIdentifierICType {
    #[default]
    II = 0,
    SI = 1,
}

// SISO-REF-010-2023 ISLSAntennaType [UID 349]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ISLSAntennaType {
    #[default]
    NoStatement = 0,
    MonopulseAntenna = 1,
}

// SISO-REF-010-2023 Mode5Reply [UID 350]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Mode5Reply {
    #[default]
    NoResponse = 0,
    Valid = 1,
    Invalid = 2,
    UnabletoVerify = 3,
}

// SISO-REF-010-2023 AntennaSelection [UID 351]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AntennaSelection {
    #[default]
    NoStatement = 0,
    Top = 1,
    Bottom = 2,
    Diversity = 3,
}

// SISO-REF-010-2023 Mode5SquitterType [UID 352]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Mode5SquitterType {
    #[default]
    NotCapable = 0,
    Short = 1,
    Extended = 2,
}

// SISO-REF-010-2023 Level2SquitterStatus [UID 353]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Level2SquitterStatus {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

// SISO-REF-010-2023 ModeSSquitterType [UID 354]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ModeSSquitterType {
    #[default]
    NotCapable = 0,
    Acquisition = 1,
    Extended = 2,
    Short = 3,
}

// SISO-REF-010-2023 ModeSSquitterRecordSource [UID 355]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ModeSSquitterRecordSource {
    #[default]
    Layer4IFFDataRecords = 0,
    Layer5GICBIFFDataRecords = 1,
}

// SISO-REF-010-2023 AircraftPresentDomain [UID 356]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AircraftPresentDomain {
    #[default]
    NoStatement = 0,
    Airborne = 1,
    OnGroundSurface = 2,
}

// SISO-REF-010-2023 AircraftIdentificationType [UID 357]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AircraftIdentificationType {
    #[default]
    NoStatement = 0,
    FlightNumber = 1,
    TailNumber = 2,
}

// SISO-REF-010-2023 CapabilityReport [UID 358]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum CapabilityReport {
    #[default]
    NoCommunicationsCapability = 0,
    Reserved = 1,
    Reserved = 2,
    Reserved = 3,
    SignifiesatLeastCommAandCommBCapabilityandAbilitytoSetCACode7andontheGround = 4,
    SignifiesatLeastCommAandCommBcapabilityandAbilitytoSetCACode7andAirborne = 5,
    SignifiesatLeastCommAandCommBcapabilityandAbilitytoSetCACode7andEitherAirborneorontheGround = 6,
    SignifiestheDownlinkRequestFieldEquals234or5andEitherAirborneorontheGround = 7,
    NoStatement = 255,
}

// SISO-REF-010-2023 NavigationSource [UID 359]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum NavigationSource {
    #[default]
    NoStatement = 0,
    GPS = 1,
    INS = 2,
    INSGPS = 3,
}

// SISO-REF-010-2023 IFFDataRecordAvailable [UID 360]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IFFDataRecordAvailable {
    #[default]
    ComputeLocally = 0,
    IFFDataRecordAvailable = 1,
}

// SISO-REF-010-2023 Mode5SAltitudeResolution [UID 361]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Mode5SAltitudeResolution {
    #[default]
    100foot = 0,
    25foot = 1,
}

// SISO-REF-010-2023 DeltaMode5SAltitudePositiveNegativeIndicator [UID 362]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DeltaMode5SAltitudePositiveNegativeIndicator {
    #[default]
    Positive = 0,
    Negative = 1,
}

// SISO-REF-010-2023 FormatType [UID 363]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum FormatType {
    #[default]
    NoData = 0,
    IdentityFormat = 4,
    SurfaceFormat5meterRNP = 5,
    SurfaceFormat100meterRNP = 6,
    AirborneFormat5meterRNP25footBarometricAltitude = 7,
    AirborneFormat100meterRNP25footBarometricAltitude = 8,
    AirborneFormat0_25nmiRNP25footBarometricAltitude = 9,
    AirborneFormat1_0nmiRNP25footBarometricAltitude = 10,
    AirborneFormat5meterRNP100footBarometricAltitude = 11,
    AirborneFormat100meterRNP100footBarometricAltitude = 12,
    AirborneFormat0_25nmiRNP100footBarometricAltitude = 13,
    AirborneFormat1_0nmiRNP100footBarometricAltitude = 14,
    AirborneFormat5meterRNPGPSHeight = 15,
    AirborneFormat100meterRNPGPSHeight = 16,
}

// SISO-REF-010-2023 AircraftAddressSource [UID 364]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AircraftAddressSource {
    #[default]
    ModeSAircraftAddressFieldValue = 0,
    GICBIFFDataRecordAvailable = 1,
}

// SISO-REF-010-2023 SurveillanceStatus [UID 365]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SurveillanceStatus {
    #[default]
    NoInformation = 0,
    EmergencyLossofCommunications = 1,
    SPI = 2,
    ATCRBSCodeChange = 3,
}

// SISO-REF-010-2023 TurnRateSource [UID 366]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TurnRateSource {
    #[default]
    ComputeLocally = 0,
    LessThan1DegreeTurnorNotTurning = 1,
    1DegreeorGreaterTurnRate = 2,
}

// SISO-REF-010-2023 TimeTypeSource [UID 367]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TimeTypeSource {
    #[default]
    ComputeLocally = 0,
    EvenSecond = 1,
    OddSecond = 2,
}

// SISO-REF-010-2023 AircraftTypeWake [UID 368]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AircraftTypeWake {
    #[default]
    NoStatement = 0,
}

// SISO-REF-010-2023 DataCategory [UID 369]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DataCategory {
    #[default]
    NoStatement = 0,
    FunctionalData = 1,
    TransponderInterrogatorDataLinkMessages = 2,
}

// SISO-REF-010-2023 TILinkType [UID 370]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TILinkType {
    #[default]
    NotUsed = 0,
    GroundInitiatedCommunicationsB = 1,
    AutomaticDependentSurveillance = 2,
    GlobalNavigationSatelliteSystem = 3,
    DataLinkInitiationCapability = 4,
    AircraftCommunicationsAddressingandReportingSystem = 5,
    ATCCommunicationsManagement = 6,
    VHFDigitalLink = 7,
    AeronauticalTelecommunicationNetwork = 8,
    ModeSelect = 9,
    AirborneCollisionAvoidanceSystems = 10,
    TrafficCollisionAvoidanceSystems = 11,
    AutomaticDependentSurveillanceB = 12,
}

// SISO-REF-010-2023 AntennaStatus [UID 371]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AntennaStatus {
    #[default]
    NoStatement = 0,
    NotAbletoEmit = 1,
    AbletoEmit = 2,
}

// SISO-REF-010-2023 TransmissionIndicator [UID 372]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmissionIndicator {
    #[default]
    NoStatement = 0,
    OriginalInterrogation = 1,
    InterrogationReply = 2,
    SquitterTransmission = 3,
}

// SISO-REF-010-2023 ReplyAmplification [UID 373]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ReplyAmplification {
    #[default]
    NoStatement = 0,
    Complete = 1,
    Limted = 2,
    UnabletoRespond = 3,
}

// SISO-REF-010-2023 DEFireFlagsStateUpdateFlag [UID 374]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DEFireFlagsStateUpdateFlag {
    #[default]
    UpdateDuetoHeartbeatTimer = 0,
    StateChange = 1,
}

// SISO-REF-010-2023 ComponentVisualDamageStatusSmoke [UID 375]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ComponentVisualDamageStatusSmoke {
    #[default]
    NoSmoke = 0,
    LightSmoke = 1,
    ModerateSmoke = 2,
    HeavySmoke = 3,
}

// SISO-REF-010-2023 ComponentVisualDamageStatusSurfaceDamage [UID 376]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ComponentVisualDamageStatusSurfaceDamage {
    #[default]
    NormalAppearance = 0,
    LightCharring = 1,
    HeavyCharring = 2,
    OneorMoreHolesBurnedCompletelythroughSurface = 3,
}

// SISO-REF-010-2023 GridAxisDescriptorAxisType [UID 377]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum GridAxisDescriptorAxisType {
    #[default]
    RegularAxis = 0,
    IrregularAxis = 1,
}

// SISO-REF-010-2023 AppearancePaintScheme [UID 378]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearancePaintScheme {
    #[default]
    UniformColor = 0,
    Camouflage = 1,
}

// SISO-REF-010-2023 AppearanceDamage [UID 379]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceDamage {
    #[default]
    NoDamage = 0,
    SlightDamage = 1,
    ModerateDamage = 2,
    Destroyed = 3,
}

// SISO-REF-010-2023 Mode5MessageFormatsStatus [UID 380]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Mode5MessageFormatsStatus {
    #[default]
    Capability = 0,
    ActiveInterrogation = 1,
}

// SISO-REF-010-2023 AppearanceTrailingEffects [UID 381]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceTrailingEffects {
    #[default]
    None = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
}

// SISO-REF-010-2023 AppearanceHatch [UID 382]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceHatch {
    #[default]
    NotApplicable = 0,
    Closed = 1,
    Popped = 2,
    PoppedandPersonIsVisible = 3,
    Open = 4,
    OpenandPersonIsVisible = 5,
}

// SISO-REF-010-2023 AppearanceLauncherOperational [UID 383]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceLauncherOperational {
    #[default]
    NotRaisedNotOperational = 0,
    RaisedOperational = 1,
}

// SISO-REF-010-2023 AppearanceCamouflageType [UID 384]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceCamouflageType {
    #[default]
    DesertCamouflage = 0,
    WinterCamouflage = 1,
    ForestCamouflage = 2,
    Other = 3,
}

// SISO-REF-010-2023 AppearanceConcealedPosition [UID 385]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceConcealedPosition {
    #[default]
    NotConcealed = 0,
    PreparedConcealedPosition = 1,
}

// SISO-REF-010-2023 AppearanceEntityorObjectState [UID 386]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceEntityorObjectState {
    #[default]
    Active = 0,
    Deactivated = 1,
}

// SISO-REF-010-2023 AppearanceCanopy [UID 387]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceCanopy {
    #[default]
    NotApplicable = 0,
    SingleCanopySingleTroopDoorClosed = 1,
    FrontandRearCanopyLeftandRightTroopDoorClosed = 2,
    FrontCanopyLeftTroopDoorOpen = 3,
    SingleCanopySingleTroopDoorOpen = 4,
    RearCanopyRightTroopDoorOpen = 5,
    FrontandRearCanopyLeftandRightTroopDoorOpen = 6,
}

// SISO-REF-010-2023 AppearanceSubsurfaceHatch [UID 388]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceSubsurfaceHatch {
    #[default]
    NotApplicable = 0,
    HatchIsClosed = 1,
    HatchIsOpen = 4,
}

// SISO-REF-010-2023 DISPDUStatusActiveInterrogationIndicator(AII) [UID 389]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DISPDUStatusActiveInterrogationIndicator(AII) {
    #[default]
    NotActive = 0,
    Active = 1,
}

// SISO-REF-010-2023 AppearanceLifeformHealth [UID 390]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceLifeformHealth {
    #[default]
    NoInjury = 0,
    SlightInjury = 1,
    ModerateInjury = 2,
    FatalInjury = 3,
}

// SISO-REF-010-2023 AppearanceLifeFormComplianceStatus [UID 391]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceLifeFormComplianceStatus {
    #[default]
    NotSpecified = 0,
    Detained = 1,
    Surrender = 2,
    UsingFists = 3,
    VerbalAbuseLevel1 = 4,
    VerbalAbuseLevel2 = 5,
    VerbalAbuseLevel3 = 6,
    PassiveResistanceLevel1 = 7,
    PassiveResistanceLevel2 = 8,
    PassiveResistanceLevel3 = 9,
    UsingNonLethalWeapon1 = 10,
    UsingNonLethalWeapon2 = 11,
    UsingNonLethalWeapon3 = 12,
    UsingNonLethalWeapon4 = 13,
    UsingNonLethalWeapon5 = 14,
    UsingNonLethalWeapon6 = 15,
}

// SISO-REF-010-2023 AppearanceLifeFormPosture [UID 392]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceLifeFormPosture {
    #[default]
    NotSpecified = 0,
    UprightStandingStill = 1,
    UprightWalking = 2,
    UprightRunning = 3,
    Kneeling = 4,
    Prone = 5,
    Crawling = 6,
    Swimming = 7,
    Parachuting = 8,
    Jumping = 9,
    Sitting = 10,
    Squatting = 11,
    Crouching = 12,
    Wading = 13,
    Surrender = 14,
    Detained = 15,
}

// SISO-REF-010-2023 AppearanceLifeFormWeaponImplement [UID 393]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceLifeFormWeaponImplement {
    #[default]
    NotPresent = 0,
    Stowed = 1,
    DeployedActive = 2,
    FiringPositionInUse = 3,
}

// SISO-REF-010-2023 AppearanceConcealedMovement [UID 394]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceConcealedMovement {
    #[default]
    OpenMovement = 0,
    RushesBetweenCoveredPositions = 1,
}

// SISO-REF-010-2023 AppearanceEnvironmentalDensity [UID 395]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceEnvironmentalDensity {
    #[default]
    Clear = 0,
    Hazy = 1,
    Dense = 2,
    VeryDense = 3,
    Opaque = 4,
}

// SISO-REF-010-2023 Mode5PlatformType [UID 396]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Mode5PlatformType {
    #[default]
    GroundVehicle = 0,
    AirVehicle = 1,
}

// SISO-REF-010-2023 AppearanceAntiCollisionDayNight [UID 397]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceAntiCollisionDayNight {
    #[default]
    Day = 0,
    Night = 1,
}

// SISO-REF-010-2023 AppearanceNavigationPositionBrightness [UID 398]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceNavigationPositionBrightness {
    #[default]
    Dim = 0,
    Bright = 1,
}

// SISO-REF-010-2023 AppearanceSupplyDeployed [UID 399]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceSupplyDeployed {
    #[default]
    NotApplicable = 0,
    Stowed = 1,
    Deployed = 2,
    DeployedandActive = 3,
}

// SISO-REF-010-2023 AppearanceNVGMode [UID 400]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceNVGMode {
    #[default]
    OvertLighting = 0,
    CovertLighting = 1,
}

// SISO-REF-010-2023 Parachute [UID 401]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Parachute {
    #[default]
    None = 0,
    Deployed = 1,
    Collapsed = 2,
    MalfunctionStreamer = 3,
}

// SISO-REF-010-2023 FlareSmokeColor [UID 402]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum FlareSmokeColor {
    #[default]
    White = 0,
    Red = 1,
    Green = 2,
    IR = 3,
}

// SISO-REF-010-2023 FlareSmokeStatus [UID 403]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum FlareSmokeStatus {
    #[default]
    NotIgnited = 0,
    Burning = 1,
    BurnedOut = 2,
}

// SISO-REF-010-2023 SpotChaffStatus [UID 404]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SpotChaffStatus {
    #[default]
    None = 0,
    Deployed = 1,
    Malfunction = 2,
}

// SISO-REF-010-2023 AppearanceObjectGeneralDamage [UID 405]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceObjectGeneralDamage {
    #[default]
    NoDamage = 0,
    Damaged = 1,
    Destroyed = 2,
}

// SISO-REF-010-2023 AppearanceObjectGeneralPredistributed [UID 406]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceObjectGeneralPredistributed {
    #[default]
    ObjectCreatedDuringtheExercise = 0,
    ObjectPredistributedPriortoExerciseStart = 1,
}

// SISO-REF-010-2023 AppearanceObjectSpecificBreachState [UID 407]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceObjectSpecificBreachState {
    #[default]
    NoBreaching = 0,
    Breached = 1,
    Cleared = 2,
}

// SISO-REF-010-2023 AppearanceObjectSpecificChemicalType [UID 408]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceObjectSpecificChemicalType {
    #[default]
    Other = 0,
    Hydrochloric = 1,
    WhitePhosphorous = 2,
    RedPhosphorous = 3,
}

// SISO-REF-010-2023 AppearanceLinearObjectTankDitchBreach [UID 409]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceLinearObjectTankDitchBreach {
    #[default]
    NoBreaching = 0,
    SlightBreaching = 1,
    ModerateBreached = 2,
    Cleared = 3,
}

// SISO-REF-010-2023 AppearanceLinearObjectLaneMarkerVisible [UID 410]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceLinearObjectLaneMarkerVisible {
    #[default]
    LeftSideIsVisible = 0,
    RightSideIsVisible = 1,
    BothSidesAreVisible = 2,
}

// SISO-REF-010-2023 AppearanceObjectGeneralIEDPresent [UID 411]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AppearanceObjectGeneralIEDPresent {
    #[default]
    None = 0,
    Visible = 1,
    PartiallyHidden = 2,
    CompletelyHidden = 3,
}

// SISO-REF-010-2023 Mode5LevelSelection [UID 412]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Mode5LevelSelection {
    #[default]
    Mode5Level1 = 0,
    Mode5Level2 = 1,
}

// SISO-REF-010-2023 SupplyFuelType [UID 413]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SupplyFuelType {
    #[default]
    Other = 0,
    Gasoline = 1,
    DieselFuel = 2,
    JP4 = 3,
    FuelOil = 4,
    JP8 = 5,
    FogOil = 6,
    MultiSpectralFogOil = 7,
    JP5 = 8,
    JPTS = 9,
    TS1 = 10,
}

// SISO-REF-010-2023 SensorTypeSource [UID 414]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SensorTypeSource {
    #[default]
    OtherActiveSensors = 0,
    Electromagnetic = 1,
    PassiveSensors = 2,
    MinefieldSensors = 3,
    UnderwaterAcoustics = 4,
    Lasers = 5,
}

// SISO-REF-010-2023 AttachedPartDetachedIndicator [UID 415]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AttachedPartDetachedIndicator {
    #[default]
    Attached = 0,
    Detached = 1,
}

// SISO-REF-010-2023 IntercomControlCommunicationsClass [UID 416]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IntercomControlCommunicationsClass {
    #[default]
    SimulatedCommunicationsChannel = 0,
    SimulationSupportCommunicationsChannel = 1,
}

// SISO-REF-010-2023 DISLiveEntitySubprotocolNumber [UID 417]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DISLiveEntitySubprotocolNumber {
    #[default]
    NoSubprotocol = 0,
}

// SISO-REF-010-2023 MinefieldAppearanceMinefieldType [UID 418]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldAppearanceMinefieldType {
    #[default]
    MixedAntiPersonnelandAntiTankMinefield = 0,
    PureAntiPersonnelMinefield = 1,
    PureAntiTankMinefield = 2,
}

// SISO-REF-010-2023 MinefieldAppearanceActiveStatus [UID 419]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldAppearanceActiveStatus {
    #[default]
    Active = 0,
    Inactive = 1,
}

// SISO-REF-010-2023 MinefieldAppearanceLane [UID 420]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldAppearanceLane {
    #[default]
    MinefieldHasActiveLane = 0,
    MinefieldHasanInactiveLane = 1,
}

// SISO-REF-010-2023 MinefieldAppearanceState [UID 421]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldAppearanceState {
    #[default]
    Active = 0,
    Deactivated = 1,
}

// SISO-REF-010-2023 MinefieldFusingFuseType [UID 422]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldFusingFuseType {
    #[default]
    NoFuse = 0,
    Other = 1,
    Pressure = 2,
    Magnetic = 3,
    TiltRod = 4,
    Command = 5,
    TripWire = 6,
}

// SISO-REF-010-2023 Mode5LocationErrors [UID 423]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Mode5LocationErrors {
    #[default]
    NoLocationErrors = 0,
    IFFDataRecordPresent = 1,
}

// SISO-REF-010-2023 MinefieldPaintSchemeAlgae [UID 424]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldPaintSchemeAlgae {
    #[default]
    None = 0,
    Light = 1,
    Moderate = 2,
    Heavy = 3,
}

// SISO-REF-010-2023 MinefieldPaintSchemePaintScheme [UID 425]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum MinefieldPaintSchemePaintScheme {
    #[default]
    Other = 0,
    Standard = 1,
    CamouflageDesert = 2,
    CamouflageJungle = 3,
    CamouflageSnow = 4,
    CamouflageGravel = 5,
    CamouflagePavement = 6,
    CamouflageSand = 7,
    NaturalWood = 8,
    Clear = 9,
    Red = 10,
    Blue = 11,
    Green = 12,
    Olive = 13,
    White = 14,
    Tan = 15,
    Black = 16,
    Yellow = 17,
    Brown = 18,
}

// SISO-REF-010-2023 CoverShroudStatus [UID 426]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum CoverShroudStatus {
    #[default]
    Closed = 0,
    Opening = 1,
    CoverShroudBlownDetached = 2,
    OpenAttached = 3,
}

// SISO-REF-010-2023 PlatformLandMotorcycleSubcategories [UID 427]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandMotorcycleSubcategories {
    #[default]
    Other = 0,
    Scooter = 1,
    SportStreet = 2,
    Cruiser = 3,
    DirtBike = 4,
    Standard = 5,
    Touring = 6,
    DualPurpose = 7,
}

// SISO-REF-010-2023 PlatformLandCarSubcategories [UID 428]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandCarSubcategories {
    #[default]
    Other = 0,
    Generic = 10,
    GenericMiniMicrocar = 11,
    GenericEconomyCompact = 12,
    GenericIntermediateStandard = 13,
    GenericFullPremiumLuxury = 14,
    GenericOversize = 15,
    2Door = 20,
    2Door = 21,
    2Door = 22,
    2Door = 23,
    3DoorHatchback = 30,
    3DoorHatchbackMiniMicrocar = 31,
    3DoorHatchbackEconomyCompact = 32,
    4DoorSedan = 40,
    4DoorSedanMiniMicrocar = 41,
    4DoorSedanEconomyCompact = 42,
    4DoorSedanIntermediateStandard = 43,
    4DoorSedanFullPremiumLuxury = 44,
    4DoorSedanOversize = 45,
    5DoorHatchback = 50,
    5DoorHatchbackMiniMicrocar = 51,
    5DoorHatchbackEconomyCompact = 52,
    5DoorHatchbackIntermediateStandard = 53,
    5DoorHatchbackFullPremiumLuxury = 54,
    Wagon = 60,
    WagonEconomyCompact = 62,
    WagonIntermediateStandard = 63,
    WagonFullPremiumLuxury = 64,
    Minivan = 70,
    Limousine = 80,
    LimousineFullPremiumLuxury = 84,
    LimousineOversize = 85,
    Sports = 90,
    Convertible = 100,
    ConvertibleMiniMicrocar = 101,
    ConvertibleEconomyCompact = 102,
    ConvertibleIntermediateStandard = 103,
    ConvertibleFullPremiumLuxury = 104,
    SportsUtilityVehicle = 110,
    SportsUtilityVehicle = 112,
    SportsUtilityVehicle = 113,
    SportsUtilityVehicle = 114,
    SportsUtilityVehicleOversize = 115,
}

// SISO-REF-010-2023 PlatformLandBusSubcategories [UID 429]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandBusSubcategories {
    #[default]
    Other = 0,
    Commuter = 1,
    Commuter = 2,
    Shuttle = 3,
    DoubleDecker = 4,
    Guided = 5,
    Kneeling = 6,
    Midibus = 7,
    Minibus = 8,
    MiniWheelchair = 9,
    Motorcoach = 10,
    PrisonBus = 11,
    Schoolbus = 12,
    SchoolWheelchair = 13,
    Tour = 14,
    TramParkingLot = 15,
    Trolley = 16,
    AirportTransport = 17,
    Articulated = 18,
}

// SISO-REF-010-2023 PlatformLandSingleUnitCargoTruckSubcategories [UID 430]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandSingleUnitCargoTruckSubcategories {
    #[default]
    Other = 0,
    PickupTruckMini = 1,
    PickupTruckMidSize = 2,
    PickupTruckFullSize = 3,
    PickupTruckCrewCab = 4,
    PickupTruckExtendedCab = 5,
    PickupTruckLongBed = 6,
    PickupTruckCabForward = 7,
    CargoTruck = 10,
    CargoTruckupto2_5Ton = 11,
    CargoTruckupto5Ton = 12,
    CargoTruckupto7_5Ton = 13,
    CargoTruckupto10Ton = 14,
    CargoTruckover10Ton = 15,
    Tanker = 20,
    SemiTrailerCab = 30,
    Van = 70,
    VanExtended = 71,
    VanCompact = 72,
    VanWheelchair = 73,
    VanDelivery = 74,
    DeliveryTruck = 100,
    DeliveryTruckBox = 101,
    DeliveryTruckFlatbed = 102,
    DeliveryTruckStakeBed = 103,
    MessTruck = 104,
    TruckPalletisedLoadSystem = 105,
    TruckPetroleumOilandLubricants = 106,
    TruckPetroleumOilandLubricantsSurveillance = 107,
    RefrigeratedTruckSmall = 108,
    RefrigeratedTruckMedium = 109,
    RefrigeratedTruckLarge = 110,
}

// SISO-REF-010-2023 PlatformLandSingleUnitUtilityEmergencyTruckSubcategories [UID 431]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandSingleUnitUtilityEmergencyTruckSubcategories {
    #[default]
    Other = 0,
    AmbulanceTruck = 1,
    FireParamedicTruck = 2,
    AmbulanceAdvancedLifeSupport = 3,
    AmbulancePickupTruck = 4,
    FireEngine = 10,
    AerialLadderFireEngine = 11,
    AirportFireEngine = 12,
    WildlandFireEngine = 13,
    FireChief = 14,
    PolicePaddyWagon = 20,
    PoliceSWAT = 21,
    PoliceBombSquad = 22,
    PolicePickupTruck = 23,
    Hazmat = 30,
    WreckerNormalHookandChain = 40,
    WreckerNormalBoom = 41,
    WreckerNormalWheelLift = 42,
    WreckerNormalFlatbed = 43,
    WreckerNormalIntegrated = 44,
    WreckerHeavyHookandChain = 45,
    WreckerHeavyBoom = 46,
    WreckerHeavyWheelLift = 47,
    WreckerHeavyFlatbed = 48,
    WreckerHeavyIntegrated = 49,
    PostalTruck = 60,
    StreetSweeper = 70,
    StreetSweeperThreeWheeled = 71,
    WasteCollectionOther = 80,
    WasteCollectionFrontLoader = 81,
    WasteCollectionRearLoader = 82,
    WasteCollectionAutomatedSideLoader = 83,
    WasteCollectionPneumaticCollection = 84,
    WasteCollectionGrapple = 85,
    UtilityTruck = 90,
    UtilityTruckwBoom = 91,
    AerialWorkPlatformOther = 100,
    AerialWorkPlatformScissorLift = 101,
    AerialWorkPlatformTelescoping = 102,
    MaintenanceTruck = 120,
    DecontaminationTruck = 121,
    WaterCannonTruck = 122,
    WaterPurificationTruck = 123,
    SmokeGeneratorTruck = 124,
    AutoRickshaw = 150,
}

// SISO-REF-010-2023 PlatformLandMultipleUnitCargoTruckSubcategories [UID 432]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandMultipleUnitCargoTruckSubcategories {
    #[default]
    Other = 0,
    TractorTrailer = 1,
    Tanker = 2,
}

// SISO-REF-010-2023 PlatformLandMultipleUnitUtilityEmergencyTruckSubcategories [UID 433]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandMultipleUnitUtilityEmergencyTruckSubcategories {
    #[default]
    Other = 0,
    FireEngineHookAndLadder = 1,
}

// SISO-REF-010-2023 PlatformLandConstructionSpecialtyVehicleSubcategories [UID 434]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandConstructionSpecialtyVehicleSubcategories {
    #[default]
    Other = 0,
    Tug = 1,
    Forklift = 2,
    Loader = 3,
    LoaderBackhoe = 4,
    CraneTractorMounted = 5,
    CraneWheeled = 6,
    Grader = 7,
    RoadRollerOther = 8,
    RoadRollerDoubleDrumSmooth = 9,
    RoadRollerSingleDrumSmooth = 10,
    RoadRollerDoubleDrumSheeps = 11,
    RoadRollerSingleDrumSheeps = 12,
    RoadRollerPneumaticTired = 13,
    ExcavatorOther = 14,
    ExcavatorDragline = 15,
    ExcavatorLongReach = 16,
    ExcavatorMobileTire = 17,
    MiniExcavator = 18,
    ExcavatorGiant = 19,
    BulldozerTractorMounted = 20,
    BulldozerTracked = 21,
    Scraper = 22,
    SkidSteer = 23,
    DumpTruckOther = 24,
    DumpTruckArticulated = 25,
    DumpTruckTransfer = 26,
    DumpTruckSuper = 27,
    DumpTruckOffRoad = 28,
    Paver = 29,
    DrillingMachine = 30,
    ConcreteMixerOther = 31,
    ConcreteMixerRearDischarge = 32,
    ConcreteMixerFrontDischarge = 33,
    ConcreteMixerSixAxle = 34,
    ConcreteMixerLongReachBoom = 35,
    ConcreteMixerVolumetric = 36,
    TrencherChain = 37,
    TrencherRockwheel = 38,
    Snowcat = 39,
    CraneTracked = 40,
    CraneShovel = 41,
    SweeperRotary = 42,
    RollerVibratoryCompactor = 43,
    ForkLiftTruck = 44,
    ForkLiftRoughtTerrain = 45,
    Transloader = 46,
    TruckWaterConstruction = 47,
    TruckFuelDelivery = 48,
    TruckSawmill = 49,
    TruckLineMarkingConstruction = 50,
    TractorIndustrial = 51,
    CompactorHighSpeed = 52,
    TruckDrilling = 53,
    TruckDrillingSupport = 54,
    CraneConstruction = 55,
}

// SISO-REF-010-2023 PlatformLandFarmSpecialtyVehicleSubcategories [UID 435]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandFarmSpecialtyVehicleSubcategories {
    #[default]
    Other = 0,
    Tractor = 1,
    HarvesterReaper = 2,
    Skidder = 3,
    Forwarder = 4,
    LawnMowerOther = 5,
    LawnMowerRiding = 6,
    LawnMowerStanding = 7,
    LawnMowerPush = 8,
}

// SISO-REF-010-2023 PlatformLandTrailerSubcategories [UID 436]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandTrailerSubcategories {
    #[default]
    Other = 0,
    TrailerFlatbed = 1,
    TrailerContainer = 2,
    TrailerContainerRefrigerated = 3,
    TrailerDouble = 4,
    TrailerAutoTransport = 5,
    TrailerArticulated = 6,
    TrailerTanker = 7,
    TrailerTankerSmall = 8,
    TrailerTankerLarge = 9,
    TrailerTankerGasoline = 10,
    TrailerTankerMilk = 11,
    TrailerTankerWater = 12,
    TrailerTankerSeptic = 13,
    TrailerBoat = 14,
    TrailerBoatSmall = 15,
    TrailerBoatLarge = 16,
    TrailerRecreational = 17,
    TrailerRecreationalConventional = 18,
    TrailerRecreationalTravelExpandable = 19,
    TrailerRecreationalFifthWheelTravel = 20,
    TrailerRecreationalFoldingCamping = 21,
    TrailerRecreationalTruckCamper = 22,
    TrailerAerostatMooringPlatform = 23,
    TrailerHousehold = 24,
    TrailerKitchen = 25,
    TrailerUltraLightAircraft = 26,
    TrailerHeavyEquipment = 27,
}

// SISO-REF-010-2023 PlatformLandRecreationalSubcategories [UID 437]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandRecreationalSubcategories {
    #[default]
    Other = 0,
    ATV2X4 = 1,
    ATV4X4 = 2,
    ATV6X6 = 3,
    ATV3wheeled = 4,
    ToyOther = 5,
    ToyCar = 6,
    ToyATV = 7,
    GolfCart = 8,
    Snowmobile = 9,
    RecreationalVehicle = 10,
    RecreationalVehicleTypeAMotorhome = 11,
    RecreationalVehicleTypeBMotorhome = 12,
    RecreationalVehicleTypeCMotorhome = 13,
    ConversionVan = 14,
}

// SISO-REF-010-2023 PlatformLandNonmotorizedSubcategories [UID 438]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandNonmotorizedSubcategories {
    #[default]
    Other = 0,
    Unicycle = 1,
    Bicycle = 2,
    BicycleMountain = 3,
    BicycleRacing = 4,
    Tricycle = 5,
    Quadricycle = 6,
    RickshawTwoPerson = 7,
    RickshawOnePerson = 8,
    TandemBicycle = 9,
    CycleTrailer = 10,
    CycleSidecar = 11,
    Sled = 12,
    Skis = 13,
    Snowboard = 14,
    Skateboard = 15,
    Skates = 16,
    SkatesInLine = 17,
    WagonCart = 18,
    Dolly = 19,
    Handtruck = 20,
    PushCart = 21,
    Wheelbarrow = 22,
    KickScooter = 23,
    Wheelchair = 24,
}

// SISO-REF-010-2023 PlatformLandTrainsSubcategories [UID 439]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandTrainsSubcategories {
    #[default]
    Other = 0,
    Engine = 1,
    BoxCar = 2,
    Tanker = 3,
    Flatcar = 4,
    Caboose = 5,
    PassengerCar = 6,
    Hopper = 7,
}

// SISO-REF-010-2023 PlatformLandUtilityEmergencyCarSubcategories [UID 440]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformLandUtilityEmergencyCarSubcategories {
    #[default]
    Other = 0,
    AmbulanceCar = 1,
    PoliceCar = 2,
    PoliceChief = 3,
    Hearse = 4,
    Taxi = 5,
}

// SISO-REF-010-2023 PlatformSurfacePassengerVesselSubcategories [UID 441]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformSurfacePassengerVesselSubcategories {
    #[default]
    CruiseShip = 1,
    CruiseFerry = 2,
    HighSpeedFerry = 3,
    Ferry = 4,
    OceanLiner = 5,
}

// SISO-REF-010-2023 PlatformSurfaceDryCargoShipSubcategories [UID 442]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformSurfaceDryCargoShipSubcategories {
    #[default]
    CommonDryCargoShip = 1,
    DryBulkCargoShip = 2,
    ContainerShip = 3,
    ReeferShip = 4,
    RoRoShip = 5,
    Barge = 6,
    HeavyLiftShip = 7,
}

// SISO-REF-010-2023 PlatformSurfaceTankerSubcategories [UID 443]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformSurfaceTankerSubcategories {
    #[default]
    LiquidPetroleumGasTanker = 1,
    ChemicalTanker = 2,
    LiquidNaturalGasTanker = 3,
    CoastalTradingVessel = 4,
    CrudeOilTanker = 5,
    LiquidBulkTanker = 6,
    VeryLargeCrudeCarrier = 7,
    UltraLargeCrudeCarrier = 8,
    CondensateStorageTanker = 9,
}

// SISO-REF-010-2023 PlatformSurfaceSupportVesselSubcategories [UID 444]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformSurfaceSupportVesselSubcategories {
    #[default]
    PlatformSupplyVessel = 1,
    TenderVessel = 2,
    Tugboat = 3,
    DiveSupportVessel = 4,
    Fireboat = 5,
    WellStimulationVessel = 6,
    AnchorHandlingTugSupplyVessel = 7,
    OffshoreConstructionVessel = 8,
    EmergencyResponseandRescueVessel = 9,
}

// SISO-REF-010-2023 PlatformSurfacePrivateMotorboatSubcategories [UID 445]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformSurfacePrivateMotorboatSubcategories {
    #[default]
    SmallMotorboat = 1,
    MediumMotorboat = 2,
    LargeMotorboat = 3,
    VeryLargeMotorboat = 4,
}

// SISO-REF-010-2023 PlatformSurfacePrivateSailboatSubcategories [UID 446]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformSurfacePrivateSailboatSubcategories {
    #[default]
    SmallSailboat = 1,
    MediumSailboat = 2,
    LargeSailboat = 3,
    VeryLargeSailboat = 4,
}

// SISO-REF-010-2023 PlatformSurfaceFishingVesselSubcategories [UID 447]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformSurfaceFishingVesselSubcategories {
    #[default]
    SmallFishingVessel = 1,
    MediumFishingVessel = 2,
    LargeFishingVessel = 3,
    FishProcessingVessel = 4,
    MastedFishingVessel = 5,
}

// SISO-REF-010-2023 PlatformSurfaceOtherVesselsSubcategories [UID 448]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformSurfaceOtherVesselsSubcategories {
    #[default]
    GoFastBoat = 1,
    ResearchVessel = 2,
    HydrofoilVessel = 3,
    CableLayerVessel = 4,
    DredgerVessel = 5,
    JunkDhowVessel = 6,
    Catamaran = 7,
    Pontoon = 8,
    PersonalWaterCraft = 9,
    RefugeeRaft = 10,
}

// SISO-REF-010-2023 CryptoKeyIDCryptoMode [UID 449]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum CryptoKeyIDCryptoMode {
    #[default]
    Baseband = 0,
    Diphase = 1,
}

// SISO-REF-010-2023 Color [UID 463]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Color {
    #[default]
    NotSpecified = 0,
    WhiteVGA = 1,
    RedVGA = 2,
    YellowVGA = 3,
    LimeVGA = 4,
    CyanVGA = 5,
    BlueVGA = 6,
    MagentaVGA = 7,
    GreyVGA = 8,
    SilverVGA = 9,
    MaroonVGA = 10,
    OliveVGA = 11,
    GreenVGA = 12,
    TealVGA = 13,
    NavyVGA = 14,
    PurpleVGA = 15,
    Reserved = 16,
    Reserved = 17,
    Reserved = 18,
    Reserved = 19,
    Black = 20,
    Navy = 21,
    DarkBlue = 22,
    MediumBlue = 23,
    Blue = 24,
    DarkGreen = 25,
    Green = 26,
    Teal = 27,
    DarkCyan = 28,
    DeepSkyBlue = 29,
    DarkTurquoise = 30,
    MediumSpringGreen = 31,
    Lime = 32,
    SpringGreen = 33,
    Cyan = 34,
    MidnightBlue = 35,
    DodgerBlue = 36,
    LightSeaGreen = 37,
    ForestGreen = 38,
    SeaGreen = 39,
    DarkSlateGray = 40,
    LimeGreen = 41,
    MediumSeaGreen = 42,
    Turquoise = 43,
    RoyalBlue = 44,
    SteelBlue = 45,
    DarkSlateBlue = 46,
    MediumTurquoise = 47,
    Indigo = 48,
    DarkOliveGreen = 49,
    CadetBlue = 50,
    CornflowerBlue = 51,
    MediumAquamarine = 52,
    DimGray = 53,
    SlateBlue = 54,
    OliveDrab = 55,
    SlateGray = 56,
    LightSlateGray = 57,
    MediumSlateBlue = 58,
    LawnGreen = 59,
    Chartreuse = 60,
    Aquamarine = 61,
    Maroon = 62,
    Purple = 63,
    Olive = 64,
    Gray = 65,
    Grey = 66,
    SkyBlue = 67,
    LightSkyBlue = 68,
    BlueViolet = 69,
    DarkRed = 70,
    DarkMagenta = 71,
    SaddleBrown = 72,
    DarkSeaGreen = 73,
    LightGreen = 74,
    MediumPurple = 75,
    DarkViolet = 76,
    PaleGreen = 77,
    DarkOrchid = 78,
    YellowGreen = 79,
    Sienna = 80,
    Brown = 81,
    DarkGray = 82,
    LightBlue = 83,
    GreenYellow = 84,
    PaleTurquoise = 85,
    LightSteelBlue = 86,
    PowderBlue = 87,
    FireBrick = 88,
    DarkGoldenRod = 89,
    MediumOrchid = 90,
    RosyBrown = 91,
    DarkKhaki = 92,
    Silver = 93,
    MediumVioletRed = 94,
    IndianRed = 95,
    Peru = 96,
    Chocolate = 97,
    Tan = 98,
    LightGray = 99,
    PaleVioletRed = 100,
    Thistle = 101,
    Orchid = 102,
    GoldenRod = 103,
    Crimson = 104,
    Gainsboro = 105,
    Plum = 106,
    BurlyWood = 107,
    LightCyan = 108,
    Lavender = 109,
    DarkSalmon = 110,
    Violet = 111,
    PaleGoldenRod = 112,
    LightCoral = 113,
    Khaki = 114,
    AliceBlue = 115,
    HoneyDew = 116,
    Azure = 117,
    SandyBrown = 118,
    Wheat = 119,
    Beige = 120,
    WhiteSmoke = 121,
    MintCream = 122,
    GhostWhite = 123,
    Salmon = 124,
    AntiqueWhite = 125,
    Linen = 126,
    LightGoldenRodYellow = 127,
    OldLace = 128,
    Red = 129,
    Fuchsia = 130,
    Magenta = 131,
    DeepPink = 132,
    OrangeRed = 133,
    Tomato = 134,
    HotPink = 135,
    Coral = 136,
    DarkOrange = 137,
    LightSalmon = 138,
    Orange = 139,
    LightPink = 140,
    Pink = 141,
    Gold = 142,
    PeachPuff = 143,
    NavajoWhite = 144,
    Moccasin = 145,
    Bisque = 146,
    MistyRose = 147,
    BlanchedAlmond = 148,
    PapayaWhip = 149,
    LavenderBlush = 150,
    SeaShell = 151,
    Cornsilk = 152,
    LemonChiffon = 153,
    FloralWhite = 154,
    Snow = 155,
    Yellow = 156,
    LightYellow = 157,
    Ivory = 158,
    White = 159,
}

// SISO-REF-010-2023 BuildingPaintScheme [UID 464]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum BuildingPaintScheme {
    #[default]
    Default = 0,
}

// SISO-REF-010-2023 Season [UID 465]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Season {
    #[default]
    Summer = 0,
    Winter = 1,
    Spring = 2,
    Autumn = 3,
}

// SISO-REF-010-2023 Material [UID 466]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Material {
    #[default]
    NotSpecified = 0,
    Plastic = 1,
    Rubber = 2,
    Road = 3,
}

// SISO-REF-010-2023 Link1111BFidelityLevel [UID 467]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link1111BFidelityLevel {
    #[default]
    FidelityLevel0 = 0,
    FidelityLevel1 = 1,
    FidelityLevel2 = 2,
}

// SISO-REF-010-2023 Link11TerminalMode [UID 468]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link11TerminalMode {
    #[default]
    NoStatement = 0,
    NetworkControlStation = 1,
    Picket = 2,
}

// SISO-REF-010-2023 Link11DataTerminalSetIndicator [UID 469]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link11DataTerminalSetIndicator {
    #[default]
    NoStatement = 0,
    Transmit = 1,
    Receive = 2,
    NetBusy = 3,
    TransmitDataError = 4,
    ReceiveDataError = 5,
    CodeError = 6,
    SynchronizationComplete = 7,
}

// SISO-REF-010-2023 Link11ModeofOperation [UID 470]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link11ModeofOperation {
    #[default]
    NoStatement = 0,
    NetSync = 1,
    NetTest = 2,
    RollCall = 3,
    ShortBroadcast = 4,
    Broadcast = 5,
}

// SISO-REF-010-2023 LifeFormsSubcategoryIranianWeapons [UID 471]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormsSubcategoryIranianWeapons {
    #[default]
    Misagh2 = 1,
    RBS70 = 2,
}

// SISO-REF-010-2023 LifeFormLandCategories [UID 472]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormLandCategories {
    #[default]
    ConventionalArmedForces = 10,
    Army = 11,
    NavalInfantry = 12,
    AirForce = 13,
    Navy = 14,
    CoastGuard = 15,
    UnitedNations = 16,
    SpecialOperationsForces = 30,
    LawEnforcement = 50,
    NonMilitaryNationalGovernmentAgencies = 70,
    RegionalLocalForces = 90,
    IrregularForces = 100,
    TerroristCombatant = 101,
    Insurgent = 102,
    ParamilitaryForces = 110,
    HumanitarianOrganizations = 120,
    Civilian = 130,
    EmergencyMedicalTechnician = 131,
    Firefighter = 132,
    Press = 133,
    Mammal = 200,
    Reptile = 201,
    Amphibian = 202,
    Insect = 203,
    Arachnid = 204,
    Mollusk = 205,
    Marsupial = 206,
}

// SISO-REF-010-2023 LifeFormHumanSubcategoryEquipmentClass [UID 473]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSubcategoryEquipmentClass {
    #[default]
    None = 0,
    WeaponNonspecific = 1,
    AssaultRifles = 5,
    HighPowerRifles = 10,
    SniperRifles = 15,
    AntiMaterielRifle = 17,
    SubMachineGuns = 20,
    ShotGuns = 25,
    GrenadeLaunchers = 30,
    MachineGuns = 35,
    GrenadeLaunchingMachineGun = 40,
    AntiTankRockets = 45,
    AntiTankMissiles = 50,
    AntiTankGuns = 55,
    FlameRockets = 60,
    FlameThrowers = 65,
    RocketLaunchers = 70,
    Mortars = 75,
    HandGuns = 80,
    ManPortableAirDefenseSystem = 85,
    RecoillessRifles = 90,
    DroneGuns = 95,
    EquipmentNonspecific = 150,
    Sensors = 151,
    SignalSensor = 152,
    Lasers = 153,
    AnimalCompanion = 160,
    PersonalElectronics = 171,
    LogisticsEquipment = 172,
}

// SISO-REF-010-2023 LifeFormHumanSpecificAssaultRifles [UID 474]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LifeFormHumanSpecificAssaultRifles {
    #[default]
    Other = 0,
    _4_5mmInterdynamicsMKR = 1,
    _5_45mmAK74 = 10,
    _5_45mmAKS74 = 11,
    _5_45mmAK74M = 12,
    _5_45mmKbkwz_1988Tantal = 13,
    _5_56mmAK101 = 30,
    _5_56mmDiemacoC7 = 31,
    _5_56mmColtCanadaC8Carbine = 32,
    _5_56mmGIATFAMASG2 = 33,
    _5_56mmFNFNC = 34,
    _5_56mmHKG36 = 35,
    _5_56mmIMIGalil = 36,
    _5_56mmINSAS = 37,
    _5_56mmDaewooK1 = 38,
    _5_56mmDaewooK2 = 39,
    _5_56mmM16A1 = 40,
    _5_56mmM16A2A3A4 = 41,
    _5_56mmColtM4 = 42,
    _5_56mmColtM4SpecialOperationsPeculiarModification = 43,
    _5_56mmRugerMini14 = 44,
    _5_56mmEnfieldSA80A2 = 45,
    _5_56mmPindadSS1V1 = 46,
    _5_56mmPindadSS1V2 = 47,
    _5_56mmPindadSS1V3 = 48,
    _5_56mmSteyrAUGA1 = 49,
    _5_56mmT65 = 50,
    _5_56mmT91 = 51,
    _5_56mmTavorTAR21 = 52,
    _5_56mmTypeCQM311 = 53,
    _5_56mmDaewooK11 = 54,
    _5_56mmAusteyrF88 = 55,
    _5_56mmAusteyrF88GLA = 56,
    _5_56mmAusteyrF88SA1 = 57,
    _5_56mmAusteyrF88SA2 = 58,
    _5_56mmAusteyrF88C = 59,
    _5_56mmAusteyrF88SA1C = 60,
    _5_56mmAusteyrF88SA1LTR = 61,
    _5_56mmAusteyrEF88 = 62,
    _5_56mmBushmasterXM15 = 63,
    _5_56mmHK416 = 64,
    _5_56mmF90 = 65,
    _5_56mmF90 = 66,
    _5_56mmF90M = 67,
    _5_56mmF90M = 68,
    _5_56mmF90CQB = 69,
    _5_56mmMK17SCARL = 70,
    _5_8mmQBZ95 = 100,
    _7_62mmAK103 = 110,
    _7_62mmAK104 = 111,
    _7_62mmAK47 = 112,
    _7_62mmAKM = 113,
    _7_62mmAKS47 = 114,
    _7_62mmHKG3A3 = 115,
    _7_62mmIMIGalil = 116,
    _7_62mmKLS = 117,
    _7_62mmSKS = 118,
    _7_62mmType56 = 119,
    _7_62mmType6368 = 120,
    _7_62mmType81 = 121,
    _7_62mmMK17SCARH = 122,
    _8mmLebelM16 = 240,
}

// SISO-REF-010-2023 LifeFormHumanSpecificHighPowerRifles [UID 475]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificHighPowerRifles {
    #[default]
    Other = 0,
    _7_62mmM14 = 10,
    _7_62mmRemington700 = 11,
    _7_62mmSIGSauerSSG2000 = 12,
    _7_62mmStonerSR25 = 13,
    _7_62mmMosinNagantModel189130 = 14,
    _7_62mmHK417 = 15,
    _7_62mmHK41716Recce = 16,
    _7_65mmBARM1918 = 50,
    _7_65mmM1Garand = 51,
}

// SISO-REF-010-2023 LifeFormCategoriesUS [UID 476]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormCategoriesUS {
    #[default]
    USArmy = 11,
    USMarineCorps = 12,
    USAirForce = 13,
    USNavy = 14,
    USCoastGuard = 15,
    Rangers = 31,
    ARSOF = 32,
    ForceReconnaissance = 33,
    NavySEAL = 34,
    AFSOF = 35,
    DeltaForce = 36,
    FederalBureauofInvestigation = 51,
    CentralIntelligenceAgency = 52,
    DepartmentofHomelandSecurity = 53,
    BureauofAlcoholTobaccoFirearmsandExplosives = 54,
    USSecretService = 55,
    USMarshal = 56,
    StatePolice = 71,
    CountySheriffPolice = 72,
    MunicipalPolice = 73,
    RedCross = 124,
}

// SISO-REF-010-2023 LifeFormExtraPersonalData [UID 477]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormExtraPersonalData {
    #[default]
    NotSpecified = 0,
    Asian = 1,
    PacificIslander = 2,
    Black = 3,
    EastAsian = 4,
    Hispanic = 5,
    White = 6,
    Arab = 7,
    HomogenousCountryCode = 8,
    IndigenousCountryCode = 9,
    Infant = 10,
    Toddler = 20,
    Child = 30,
    Adolescent = 40,
    Teenager = 50,
    YoungAdult = 60,
    Adult = 70,
    SeniorAdult = 80,
    Elderly = 90,
    Female = 100,
    Asian = 101,
    PacificIslander = 102,
    Black = 103,
    EastAsian = 104,
    Hispanic = 105,
    White = 106,
    Arab = 107,
    HomogenousCountryCode = 108,
    IndigenousCountryCode = 109,
    Infant = 110,
    Toddler = 120,
    Child = 130,
    Adolescent = 140,
    Teenager = 150,
    YoungAdult = 160,
    Adult = 170,
    SeniorAdult = 180,
    Elderly = 190,
}

// SISO-REF-010-2023 LifeFormAirCategories [UID 478]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormAirCategories {
    #[default]
    Bird = 200,
    Insect = 201,
    Mammal = 202,
}

// SISO-REF-010-2023 LifeFormSubsurfaceCategories [UID 479]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormSubsurfaceCategories {
    #[default]
    Fish = 200,
    Mammal = 201,
    Mollusk = 202,
    Crustacean = 203,
    Insect = 204,
}

// SISO-REF-010-2023 LifeFormHumanSpecificSniper [UID 481]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LifeFormHumanSpecificSniper {
    #[default]
    Other = 0,
    _5_8mmQBU88 = 1,
    _7_62mmC3 = 30,
    _7_62mmFRF2 = 31,
    _7_62mmAWMF = 32,
    _7_62mmG3SG1 = 33,
    _7_62mmGalilSniper = 34,
    _7_62mmL96A1 = 35,
    _7_62mmM14DMR = 36,
    _7_62mmM24SniperWeaponSystem = 37,
    _7_62mmM40A1A3 = 38,
    _7_62mmSteyrSSG69 = 39,
    _7_62mmSVD = 40,
    _7_62mmTYPE79 = 41,
    _7_62mmSR25MK11 = 42,
    _7_62mmAWSR98 = 43,
    _7_62mmBlaserR93 = 44,
    _7_62mmM2010EnhancedSniperRifle = 45,
    _7_62mmM110SemiAutomaticSniperSystem = 46,
    _7_62mmL129A1 = 47,
    _7_7mmTYPE99 = 100,
    _8_58mmBlaserR93Tactical2 = 105,
    _9mmVSSVintorez = 110,
    _12_7mmSteyrHS_50 = 170,
    _12_7mmM82A1ASpecialApplicationsScopedRifle = 171,
    _12_7mmNSV = 172,
    _12_7mmOSV96 = 173,
    _12_7mmRangemaster50 = 174,
    _12_7mmV94 = 175,
    _12_7mmM107 = 176,
    _20mmDenelNTW20 = 200,
}

// SISO-REF-010-2023 LifeFormHumanSpecificSubMachineGun [UID 482]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificSubMachineGun {
    #[default]
    Other = 0,
    _5_45mmAKS74U = 10,
    _5_56mmDaewooK1A = 20,
    _9mmDaewooK7 = 60,
    _9mmMAC10 = 61,
    _9mmMadsenMKII = 62,
    _9mmMiniUzi = 63,
    _9mmModel83SkorpionSMG = 64,
    _9mmMP5A2 = 65,
    _9mmMP5N = 66,
    _9mmSterlingSMG = 67,
    _9mmTypeCF05 = 68,
    _9mmUzi = 69,
}

// SISO-REF-010-2023 AustralianCategoryOverlay [UID 500]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AustralianCategoryOverlay {
    #[default]
    AustralianArmy = 11,
    RoyalAustralianAirForce = 13,
    RoyalAustralianNavy = 14,
    AustralianSpecialOperationsCommand = 30,
    AustralianDepartmentofHomeAffairs = 51,
    AustralianFederalPolice = 52,
}

// SISO-REF-010-2023 LifeFormCategoriesAfghanistan [UID 501]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormCategoriesAfghanistan {
    #[default]
    AfghanNationalArmy = 11,
    AfghanAirForce = 13,
    NationalDirectorateofSecurity = 51,
    AfghanNationalPolice = 52,
    AfghanBorderPolice = 53,
    AfghanNationalCivilOrderPolice = 54,
    AfghanPublicProtectionForce = 55,
    NonMilitaryNationalGovernmentAgencies = 70,
    TerroristCombatant = 101,
    HumanitarianOrganizations = 120,
    RedCrescent = 121,
    Civilian = 130,
    Press = 133,
}

// SISO-REF-010-2023 LifeFormHumanSpecificEquipmentClass [UID 505]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificEquipmentClass {
    #[default]
    SignalSmoke = 1,
    FlashLight = 2,
    SignalMirror = 3,
    IRStrobe = 4,
    IRIlluminator = 5,
    Spotlight = 6,
}

// SISO-REF-010-2023 PlatformSubsurfaceCivilianSubmarineSubcategories [UID 506]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformSubsurfaceCivilianSubmarineSubcategories {
    #[default]
}

// SISO-REF-010-2023 PlatformSubsurfaceCivilianSubmersibleSubcategories [UID 507]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformSubsurfaceCivilianSubmersibleSubcategories {
    #[default]
}

// SISO-REF-010-2023 PlatformSubsurfaceCivilianSemiSubmersiblesSubcategories [UID 508]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum PlatformSubsurfaceCivilianSemiSubmersiblesSubcategories {
    #[default]
    NarcoSubmarine = 1,
}

// SISO-REF-010-2023 LeafCoverage [UID 509]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LeafCoverage {
    #[default]
    Normal = 0,
    Bare = 1,
}

// SISO-REF-010-2023 LifeFormHumanSpecificAntiMaterielRifles [UID 510]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificAntiMaterielRifles {
    #[default]
    Other = 0,
    12_7mmAW50 = 10,
    12_7mmAW50F = 11,
}

// SISO-REF-010-2023 LifeFormHumanSpecificShotGuns [UID 511]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificShotGuns {
    #[default]
    Other = 0,
    BrowningSuperposedOU = 20,
    BrowningCynergy = 21,
    BrowningAuto5 = 22,
    18_5mmBrowningCitoriOU12Gauge = 23,
    16_8mmBrowningCitoriOU16Gauge = 24,
    15_6mmBrowningCitoriOU20Gauge = 25,
    14mmBrowningCitoriOU28Gauge = 26,
    10_4mmBrowningCitoriOU_410Bore = 27,
    18_5mmBrowningDoubleAutomatic12Gauge = 28,
    18_5mmIthaca3712Gauge = 29,
    16_8mmIthaca3716Gauge = 30,
    15_6mmIthaca3720Gauge = 31,
    14mmIthaca3728Gauge = 32,
    19_7mmIthacaMag10SA10Gauge = 33,
    19_7mmMarlinModel5510Gauge = 34,
    18_5mmMarlinModel5512Gauge = 35,
    16_8mmMarlinModel5516Gauge = 36,
    15_6mmMarlinModel5520Gauge = 37,
    18_5mmMossberg50012Gauge = 38,
    15_6mmMossberg50020Gauge = 39,
    10_4mmMossberg500_410Bore = 40,
    18_5mmMossberg59012Gauge = 41,
    15_6mmMossberg59020Gauge = 42,
    10_4mmMossberg590_410Bore = 43,
    18_5mmMossberg930SA12Gauge = 44,
    RemingtonModel11SA = 45,
    RemingtonModel1012Gauge = 46,
    15_6mmRemingtonModel1720Gauge = 47,
    RemingtonModel31 = 48,
    RemingtonModel1148SA = 49,
    18_5mmRemington87012Gauge = 50,
    16_8mmRemington87016Gauge = 51,
    15_6mmRemington87020Gauge = 52,
    14mmRemington87028Gauge = 53,
    10_4mmRemington870_410Bore = 54,
    RemingtonModel58SA = 55,
    18_5mmRemington878SA12Gauge = 56,
    18_5mmRemingtonModel1100SA12Gauge = 57,
    16_8mmRemingtonModel1100SA16Gauge = 58,
    15_6mmRemingtonModel1100SA20Gauge = 59,
    14mmRemingtonModel1100SA28Gauge = 60,
    10_4mmRemingtonModel1100SA_410Bore = 61,
    18_5mmRemington1187SA12Gauge = 62,
    15_6mmRemington1187SA20Gauge = 63,
    19_7mmRemingtonModelSP10SA10Gauge = 64,
    18_5mmRemington88712Gauge = 65,
    18_5mmRemingtonSparta100SxS12Gauge = 70,
    15_6mmRemingtonSparta100SxS20Gauge = 71,
    10_4mmRemingtonSparta100SxS_410Bore = 72,
    18_5mmRemingtonSpartan310OU12Gauge = 73,
    15_6mmRemingtonSpartan310OU20Gauge = 74,
    14mmRemingtonSpartan310OU28Gauge = 75,
    10_4mmRemingtonSpartan310OU_410Bore = 76,
    18_5mmRemingtonSpartan453SA12Gauge = 77,
    18_5mmWinchesterModel120012Gauge = 80,
    16_8mmWinchesterModel120016Gauge = 81,
    15_6mmWinchesterModel120020Gauge = 82,
    WinchesterModel18871901 = 83,
    WinchesterModel1897 = 84,
    WinchesterModel1912 = 85,
    WinchesterModel21SxS = 86,
    WinchesterModel37SxS = 87,
    18_5mmHRUltraslugSxS12Gauge = 88,
    15_6mmHRUltraslugSxS20Gauge = 89,
    18_5mmCienerUltimateOU12Gauge = 90,
    18_5mmCoachGunSxSDoubleBarrel12Gauge = 91,
    18_5mmRugerGoldLabelSxS12Gauge = 92,
    18_5mmHighStandardModel10SA12Gauge = 93,
    18_5mmKelTexKSG12Gauge = 94,
    18_5KACMasterkey12Gauge = 95,
    18_5mmM26M_A_S_S_12Gauge = 96,
    18_5mmSRMArmsM1216SA12Gauge = 97,
    18_5mmAA12FAAtchissonAssault = 98,
    18_5mmPancorJackhammerFA12Gauge = 99,
    18_5mmUSAS12FA12Gauge = 110,
    18_5mmMAULSA12Gauge = 111,
    18_5mmFNSLPSA12Gauge = 112,
    18_5mmFNTPS12Gauge = 113,
    18_5mmENARMPentagunSA12Gauge = 115,
    StevensModel520620 = 116,
    StoegerCoachGunSxS = 117,
    StoegerCondorOU = 118,
    18_5mmArmscorModel30SA12Gauge = 120,
    WeatherbySA08SA = 121,
    18_5mmFabarmSDASSTactical12Gauge = 122,
    18_5mmMAG712Gauge = 123,
    18_5mmNeostead12Gauge = 124,
    18_5mmArmselStrikerSA12Gauge = 125,
    18_5mmParkerHaleRogunSA12Gauge = 127,
    26mmRGA86Revolver = 130,
    18_5mmSjorgrenSA12Gauge = 131,
    18_5mmAkdalMKA1919SA12Gauge = 132,
    18_5mmRetayMasaiMaraSA12Gauge = 133,
    18_5mmSafirT14SA12Gauge = 134,
    18_5mmBenelliM1Super90SA12Gauge = 150,
    15_6mmBenelliM1Super90SA20Gauge = 151,
    18_5mmBenelliM3Super90SA12Gauge = 152,
    15_6mmBenelliM3Super90SA20Gauge = 153,
    18_5mmBenelliM4Super90SA12Gauge = 154,
    18_5mmBenelliNova12Gauge = 155,
    15_6mmBenelliNove20Gauge = 156,
    18_5mmBenelliRaffaelloSA12Gauge = 157,
    18_5mmBenelliSupernova12Gauge = 158,
    18_5mmBenelliVinciSA12Gauge = 159,
    18_5mmBeretta1201FPSA12Gauge = 160,
    18_5mmBeretta682OU12Gauge = 161,
    15_6mmBeretta682OU20Gauge = 162,
    14mmBeretta682OU28Gauge = 163,
    10_4mmBeretta682OU_410Bore = 164,
    18_5mmBerettaA303SA12Gauge = 165,
    18_5mmBerettaAL391SA12Gauge = 166,
    15_6mmBerettaAL391SA20Gauge = 167,
    18_5mmBerettaDT10OU12Gauge = 168,
    BerettaSilverPigeonOU = 169,
    18_5mmBerettaXtrema2SA12Gauge = 170,
    15_6mmFranchiAL48SA20Gauge = 171,
    14mmFranchiAL48SA28Gauge = 172,
    10_4mmFranchimod_410FA_410Bore = 173,
    18_5mmFranchiSPAS12SA12Gauge = 174,
    18_5mmFranchiSPAS15SA12Gauge = 175,
    18_5mmValtroPM5PM535012Gauge = 176,
    BlazerF3OU = 180,
    18_5mmHKFABARMFP612Gauge = 181,
    18_5mmHKCAWSFA12Gauge = 182,
    18_5mmBaikalMP153SA12Gauge = 200,
    18_5mmBandayevskyRB1212Gauge = 201,
    18_5mmMolotBekasM12Gauge = 202,
    16_8mmMolotBekasM16Gauge = 203,
    18_5mmTOZ19412Gauge = 204,
    23mmKS23 = 205,
    MTs255Revoler12Gauge = 206,
    18_5mmRMB9312Gauge = 207,
    18_5mmSaiga12SA12Gauge = 208,
    15_6mmSaiga12SA20Gauge = 209,
    10_4mmSaiga12SA_410Bore = 210,
    18_5mmVepr12SA12Gauge = 211,
    18_5mmFort50012Gauge = 212,
    18_5mmNorincoHP9112Gauge = 220,
}

// SISO-REF-010-2023 LifeFormHumanSpecificMortars [UID 512]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificMortars {
    #[default]
    Others = 0,
    _60mmM224 = 30,
    _81mmF2 = 50,
    _81mmL16 = 51,
    _81mmM252 = 52,
}

// SISO-REF-010-2023 LifeFormHumanSpecificHandGuns [UID 513]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificHandGuns {
    #[default]
    Other = 0,
    _5_45mmPSM = 1,
    _9mmMK3SLP = 30,
    _9mmBeretta92S92FS = 31,
    _9mmHKUSP = 32,
    _9mmStechkinAPS = 33,
    _9mmMakarovPM = 34,
    _9mmSmithandWessonSD = 35,
    _9mmGlock17 = 36,
    _9mmSIGSauerM17 = 37,
    _9mmSIGPro = 38,
    _9mmSmithandWessonSW1911 = 39,
    _9mmSmithandWesson5900series = 40,
    _45CalM1911 = 41,
    _9_07mmRugerGP100 = 50,
    _10mmGlock20 = 60,
}

// SISO-REF-010-2023 LifeFormHumanSpecificWeaponNonspecific [UID 514]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificWeaponNonspecific {
    #[default]
    Other = 0,
    Knife = 10,
    Machete = 50,
    ExplosiveVest = 100,
    M18A1Claymore = 150,
}

// SISO-REF-010-2023 LifeFormHumanSpecificGrenadeLaunchers [UID 515]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificGrenadeLaunchers {
    #[default]
    Other = 0,
    _40x46mmArsenalUGGLM1 = 1,
    _40x46mmArsenalMSGL = 2,
    _40mmVOGArsenalMSGL = 3,
    _40x46mmArsenalUBGLM16 = 4,
    _40x46mmArsenalUBGLM8 = 5,
    _40x46mmArsenalUBGLM7 = 6,
    _30mmBS1Tishina = 10,
    _40mmBTS203 = 11,
    _40mmIndumilIMC40 = 12,
    _40mmVOGBG15 = 20,
    _40mmVOGGP25Kostoyor = 21,
    _40mmVOGGP30Obuvka = 22,
    _40mmVOGGP34 = 23,
    _40mmVOGRGM40Kastet = 24,
    _40mmVOGRG6 = 25,
    _40x46mmM79 = 30,
    _40x46mmM203 = 31,
    _40x36mmM320 = 32,
    _40x46mmCIS40GL = 35,
    _40x46mmEAGLEGL = 36,
    _40x46mmHKAG36 = 37,
    _40x46mmHKAGCGLM = 38,
    _40x46mmHK69A1 = 39,
    _40x46mmBerettaGLX160 = 40,
    _40x46mmARDEUBGL = 41,
    _40x46mmXML148 = 42,
    _40x46mmChinaLakeGL = 43,
    _40x46mmHawkMM1 = 44,
    _25x40mmXM25CDTE = 50,
    _37mmMilkor3738LLStopper = 60,
    _40mmMilkor40GL = 61,
    _40mmMilkorMGL = 62,
    _40x47mmPalladwz1974 = 65,
    _40x47mmPalladwz1983 = 66,
    UGL200CanisterRWGL3 = 70,
    _20x30mmSTDaewooK11 = 80,
    _35mmType91BreechLoadGL = 90,
    _40x53mmCZW40 = 95,
    _45mmDP64 = 100,
    _20x42mmNeopupPAW20 = 105,
}

// SISO-REF-010-2023 LifeFormHumanSpecificMachineGuns [UID 516]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LifeFormHumanSpecificMachineGuns {
    #[default]
    Other = 0,
    _5_56x45mmXM214MicrogunSixPak = 10,
    _7_62x51mmM134XM196Minigun = 11,
    _5_56x45mmM249FNMinimiSAWLMG = 20,
    _5_56x45mmFNMinimiMk3LMG = 21,
    _7_62x51mmFNMinimi7_62Mk3GPMG = 22,
    _7_62x63mmM1941Johnson = 25,
    _7_62x63mmM1918BAR = 26,
    _7_62x51mmM1919A4Mk21Mod0BrowningMMG = 27,
    _7_62x63mmM1919A6BrowningMMG = 28,
    _7_62x51mmM37BrowningMMG = 29,
    _5_56x45mmAresShrike5_56LMG = 30,
    _5_56x45mmLSATLMG = 31,
    _5_56x45mmCMG1LMG = 32,
    _5_56x45mmCMG2LMG = 33,
    _5_56x45mmStoner63ALMG = 34,
    _5_56x45mmUltimax100LMG = 35,
    _5_56x54mmBerettaAS7090LMG = 36,
    _5_56x45mmCETMEAmeliLMG = 37,
    _5_56x45mmIMINegevLMG = 38,
    _5_56x45mmINSASLMG = 39,
    _5_56x45mmAUGLMG = 40,
    _5_56x45mmAUGHBARLMG = 41,
    _5_56x45mmHKMG4LMG = 43,
    _5_56x45mmHK23GR9LMG = 44,
    _5_56x45mmM27IARSAW = 46,
    _5_56x45mmL86LSW = 47,
    _5_56x45mmDaewooK3LMG = 48,
    _5_56x45mmVectorMiniSSGPMG = 49,
    _7_62x51mmM60GPMG = 50,
    _7_62x51mmM60E3GPMG = 51,
    _7_62x51mmM60E4GPMG = 52,
    _7_62x51mmM60E6GPMG = 53,
    _7_62x51mmMark48GMPG = 55,
    _7_62x51mmM240FNMAG58GPMG = 58,
    _7_62x51mmM240E4M240BGPMG = 59,
    _7_62x51mmM240E1M240DGPMG = 60,
    _7_62x51mmM240GGPMG = 61,
    _7_62x51mmM240E5M240HGPMG = 62,
    _7_62x51mmM240LGPMG = 63,
    _7_62x39mmKk62LMG = 65,
    _7_62x51mmVectorSS77GPMG = 70,
    _7_62x51mmSIGMG7103GPMG = 71,
    _7_62x51mmSterling7_62GMPG = 72,
    _7_62x51mmSumitomoType62GPMG = 73,
    _7_62x51mmDaewooK12GPMG = 74,
    _7_62x51mmMG51GPMG = 75,
    _7_62x51mmRheinmetallMG3 = 76,
    _7_62x51mmRheinmetallMG3KWS = 77,
    _7_62x51mmMG5HK121GPMG = 80,
    _7_62x51mmHK21GPMG = 81,
    _7_62x51mmAA52GPMP = 85,
    _7_62x51mmUKM2000GPMG = 86,
    _7_62x54mmUkvz_59GPMG = 88,
    _7_92x57mmMG42GPMG = 89,
    _12_7x99mmM2A1BrowningHMG = 100,
    _12_7x99mmM2HBBrowningHMG = 101,
    _12_7x99mmM2HBQCBBrowningHMG = 102,
    _12_7x99mmM85CHMG = 105,
    _12_7x99mmRheinmetallRMG_50HMG = 108,
    _12_7x99mmHK25HMG = 110,
    _12_7x99mmCIS50MG = 112,
    _5_45x39mmIP2LMG = 120,
    _5_45x39mmNikonovLMG = 121,
    _5_45x39mmM74RPK = 122,
    _7_62x39mmM43RPK = 125,
    _7_62x39mmRPDSAW = 126,
    _7_62x39mmZastavaM72 = 127,
    _7_62x39mmType81LMG = 128,
    _7_62x51mmZastavaM77 = 135,
    _7_62x54mmPKGPMG = 140,
    _7_62x54mmAEK999GPMP = 141,
    _7_62x54mmPechenegGPMG = 142,
    _7_62x54mmZastavaM84 = 143,
    _7_62x54mmType67GPMG = 144,
    _7_62x54mmType80GPMG = 145,
    _12_7x108mmNSVHMG = 150,
    _12_7x108mmKordHMG = 151,
    _12_7x108mmKPD12_7HMG = 152,
    _12_7x108mmZastavaM02CoyotoeHMG = 153,
    _12_7x108mmZastavaM87 = 154,
    _12_7x108mmType77HMG = 155,
    _12_7x108mmW85HMG = 156,
    _12_7x108mmType90HMG = 157,
    _5_8x42mmQJY88LMG = 164,
    _5_8x42mmQBB95DBP87LMG = 165,
    _5_56x45mmQBB951LMG = 166,
}

// SISO-REF-010-2023 LifeFormHumanSpecificGrenadeLaunchingMachineGun [UID 517]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificGrenadeLaunchingMachineGun {
    #[default]
    Other = 0,
    _40x53mmHKGMG = 20,
    _40x53mmMk47Striker = 25,
    _40mmM75 = 26,
    _40mmM129 = 27,
    _40x46mmXM174 = 28,
    _40x46mmMk18Mod0 = 29,
    _40x53mmMk19 = 30,
    _40x46mmMk20Mod0 = 31,
    _30x29mmRAG30SAG30 = 40,
    _30x29mmAGS17Plamya = 41,
    _30x29mmAGS30Atlant = 42,
    _40mmVOGAGS40Balkan = 43,
    _40x53mmSBLAG40 = 44,
    _40x53mmVektorY3 = 50,
    _40x53mmCIS40 = 55,
    _40x56mmHowaType96 = 60,
    _40x53mmDaewooPrecisionIndustriesK4 = 65,
    _25x59mmXM307AdvancedCrewServedWeapon = 70,
    _35x32mmQLZ87 = 80,
}

// SISO-REF-010-2023 LifeFormHumanSpecificAntiTankRockets [UID 518]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificAntiTankRockets {
    #[default]
    Other = 0,
    _82mmB300 = 10,
    _82mmShipon = 11,
    _83mmMK153Mod0SMAW = 12,
    _66mmM72LAW = 20,
    _66mmM72A1LAW = 21,
    _66mmM72A2LAW = 22,
    _66mmM72A3LAW = 23,
    _66mmM72A4LAW = 24,
    _66mmM72A5LAW = 25,
    _66mmM72A6LAW = 26,
    _66mmM72A7LAW = 27,
    _66mmM72E8LAW = 28,
    _66mmM72E9LAW = 29,
    _66mmM72E10LAW = 30,
    _66mmM72ASLAW = 31,
    _94mmLAW80 = 35,
    _60mmM1Bazooka = 40,
    _60mmM1A1Bazooka = 41,
    _60mmM9Bazooka = 42,
    _60mmM9A1Bazooka = 43,
    _89mmM20SuperBazooka = 44,
    _89mmM20A1SuperBazooka = 45,
    _89mmM20B1SuperBazooka = 46,
    _89mmM20A1B1SuperBazooka = 47,
    _89mmM25ThreeShotBazooka = 48,
    _89mmInstalazaM65 = 49,
    _90mmInstalazaC90 = 50,
    _90mmC90CR = 51,
    _90mmC90CRAM = 52,
    _90mmC90CRBK = 53,
    _90mmC90CRIN = 54,
    _60mmPzF3 = 60,
    _60mmPzF3IT = 61,
    _60mmPzF3Bunkerfaust = 62,
    _44mmPzF44 = 65,
    _30mmPanzerfaust30 = 70,
    _50mmPanzerfaust60 = 71,
    _60mmPanzerfaust100 = 72,
    _60mmPanzerfaust150 = 73,
    _88mmPanzerschreckRPzB = 75,
    _83mmRL83Blindicide = 80,
    _100mmRL100Blindicide = 81,
    _90mmM79Osa = 85,
    _64mmM80Zolja = 86,
    _67mmArmburstCrossbow = 90,
    _40mmType69RPG = 93,
    _89mmPIAT = 95,
    _40mmRPG2 = 100,
    _64mmRPG18Mukha = 101,
    _72_5mmRPG22Netto = 102,
    _72_5mmRPG26Aglen = 103,
    _105mmRPG29Vampir = 104,
    _105mmRPG30Kryuk = 105,
    _105mmRPG32Nashshab = 106,
    _40mmRPG7 = 110,
    _40mmPSRL1 = 111,
    _40mmGS777PSRL2 = 112,
    _68mmRPG76Komar = 120,
    _120mmSEPDard120 = 125,
    _58mmWASP58 = 128,
    _73mmLRAC7350 = 130,
    _89mmLRAC89F1STRIM = 131,
    _90mmMATADOR = 135,
    _90mmMATADORMP = 136,
    _90mmMATADORWB = 137,
    _90mmMATADORAS = 138,
    _78mmMARAAntiTankRocketLauncher = 140,
    _120mmType98PF98 = 145,
}

// SISO-REF-010-2023 LifeFormHumanSpecificAntiTankMissiles [UID 519]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
#[allow(non_camel_case_types)]
pub enum LifeFormHumanSpecificAntiTankMissiles {
    #[default]
    Other = 0,
    _120mmType64MATKAM3 = 30,
    _153mmType79JyuMATKAM9 = 31,
    _120mmType87ChuMAT = 32,
    _140mmType01LMAT = 33,
    _140mmM47Dragon = 58,
    _140mmSaeghe12 = 59,
    _127mmFGM148Javelin = 60,
    _139mmFGM172SRAW = 63,
    _139mmFGM172BSRAWMPV = 64,
    _152mmBGM71TOW = 68,
    _152mmOrevTOWII = 69,
    _120mmVickersVigilantClevite = 75,
    _110mmBantam = 80,
    _150mmRBS56BILL1 = 81,
    _150mmRBS562BILL2 = 82,
    _130mmSpikeSR = 85,
    _130mmSpikeMR = 86,
    _130mmSpikeLR = 87,
    _60mmMosquito = 95,
    _160mmSS_10 = 98,
    _103mmMILAN = 100,
    _115mmMILAN2 = 101,
    _115mmMILAN2T = 102,
    _115mmMILAN3 = 103,
    _115mmMILANER = 104,
    _136mmERYX = 105,
    _152mmEntac = 107,
    _125mmRAAD = 110,
    _125mmIRAADT = 111,
    _152mmToophan = 112,
    _152mmToophan2 = 113,
    _152mmToophan5 = 114,
    _136mmBumbar = 120,
    _130mmShershenPK2 = 125,
    _152mmShershenQP2B = 126,
    _130mmMectronMSS1_2 = 130,
    _120mmHJ8 = 140,
    _120mmHJ8A = 141,
    _120mmHJ8B = 142,
    _120mmHJ8C = 143,
    _120mmHJ8D = 144,
    _120mmHJ8E = 145,
    _120mmHJ8F = 146,
    _120mmHJ8FAE = 147,
    _120mmHJ8L = 148,
    _120mmHJ8H = 149,
    _120mmHJ8S = 150,
    _120mmBaktarShikan = 151,
    _120mmHJ11 = 152,
    _152mmHJ9A = 153,
    _135mmHJ12RedArrow = 154,
    _125mmHJ73MCLOS = 155,
    _125mmHJ73BSACLOS = 156,
    _125mmHJ73CSACLOSERA = 157,
    _125mmAT3SaggerA9M14Malyutka = 170,
    _125mmAT3BSaggerB9M14MMalyutkaM = 171,
    _125mmAT3CSaggerC9M14PMalyutkaP = 172,
    _125mmAT3DSaggerD9M142Malyutka2 = 173,
    _125mmSusongPo = 174,
    _125mmAT3CPOLK = 175,
    _125mmKunWu1 = 176,
    _125mmMaliutkaM2T = 177,
    _120mmAT4ASpigotA9M111Fagot = 178,
    _120mmAT4BSpigotB9M1112Fagot = 179,
    _120mmAT4CSpigotC9M111MFaktoriya = 180,
    _135mmAT5ASpandrel9M113Kronkurs = 181,
    _135mmAT5BSpandrel9M113MKronkursM = 182,
    _135mmTosan = 183,
    _94mmAT7Saxhorn9K115Metis = 184,
    _130mmAT13Saxhorn29K1152MetisM = 185,
    _152mmAT14Spriggan9M133Kornet = 186,
    _152mmDehlavie = 187,
    _102mmMathogo = 200,
}

// SISO-REF-010-2023 LifeFormHumanSpecificManPortableAirDefenseSystem [UID 520]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificManPortableAirDefenseSystem {
    #[default]
    Other = 0,
    70mmFIM43Redeye = 1,
    70mmFIM92Stinger = 2,
    76mmBlowpipe = 10,
    76mmStarburst = 11,
    130mmStarstreakHVM = 12,
    90mmMistral = 15,
    72mm9K32MStrela2 = 20,
    72mm9K36Strela3 = 21,
    72mm9K38Igla = 22,
    72mm9K310IglaM = 23,
    72mm9K333Verba = 24,
    72mm9K338IglaS = 25,
    72mm9K32MStrela2M = 26,
    72mmHN5HongYing5 = 30,
    72mmQW1Vanguard = 31,
    72mmQW2Vanguard2 = 32,
    90mmQW3 = 33,
    72mmFN6 = 34,
    71mmMisagh1 = 45,
    71mmMisagh2 = 46,
    80mmType91KinSAM = 50,
    80mmKPSAMShunGung = 55,
    106mmRBS70 = 60,
}

// SISO-REF-010-2023 LifeFormHumanSpecificRecoillessRifles [UID 521]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificRecoillessRifles {
    #[default]
    Other = 0,
    84mmM136AT4CS = 15,
    57mmM18RR = 20,
    75mmM20RR = 21,
    120mmM28DavyCrockett = 22,
    155mmM29DavyCrockett = 23,
    106mmM40RecoillessRifle = 24,
    82mmM60RR = 25,
    90mmM67RR = 26,
    84mmM1CarlGustav = 30,
    84mmM2CarlGustav = 31,
    84mmM3CarlGustav = 32,
    84mmM4CarlGustav = 33,
    74mmPansarskottm68Miniman = 35,
    84mmALAC = 40,
    82mmB10RR = 45,
    107mmB11RR = 46,
    80mmBredaFolgore = 50,
    120mmBATRR = 55,
    73mmSPG9Kopye = 60,
    88mmRCL3_45in = 65,
    90mmPvpj110 = 70,
    50mmJagdfaust = 75,
    30mmRheinmetallRMK30 = 80,
    88mm55S55Raikka = 90,
    95mm95S5861 = 91,
    73mmLG40 = 95,
    105mmLG40 = 96,
    105mmLG42 = 97,
}

// SISO-REF-010-2023 LifeFormHumanSpecificFlameRockets [UID 522]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificFlameRockets {
    #[default]
    Other = 0,
    66mmM202Flash = 20,
    62mmFHJ84 = 30,
    90mmC90CRFIM = 40,
    93mmRPOAShmel = 50,
    93mmRPOZShmel = 51,
    93mmRPODShmel = 52,
}

// SISO-REF-010-2023 LifeFormHumanSpecificFlameThrowers [UID 523]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificFlameThrowers {
    #[default]
    Other = 0,
    Handflammpatrone = 10,
    FmW41 = 11,
    M1A1 = 20,
    M2A17 = 21,
    M9A17 = 22,
    LPO50 = 30,
    KPattern = 35,
    PortableNo2AckPack = 36,
    Marsden = 37,
    Harvey = 38,
    ROKS2 = 45,
    ROKS3 = 46,
    Type93 = 50,
    Type100 = 51,
}

// SISO-REF-010-2023 LifeFormHumanSpecificDroneGuns [UID 524]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificDroneGuns {
    #[default]
    Other = 0,
    DroneGunTactical = 15,
    DroneGunMKII = 16,
}

// SISO-REF-010-2023 LifeFormHumanSpecificLogisticsEQClass [UID 525]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificLogisticsEQClass {
    #[default]
    SlingLoadPendant = 1,
}

// SISO-REF-010-2023 LifeFormHumanSpecificPersonalElectronicsClass [UID 526]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificPersonalElectronicsClass {
    #[default]
    CellPhone = 1,
}

// SISO-REF-010-2023 LifeFormHumanSpecificLasersClass [UID 527]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeFormHumanSpecificLasersClass {
    #[default]
    GenericLaserDesignator = 1,
    GenericLaserPointer = 2,
}

// SISO-REF-010-2023 TransmitterDetailSATCOMModulation [UID 589]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum TransmitterDetailSATCOMModulation {
    #[default]
    Other = 0,
    NoDelay = 1,
}

// SISO-REF-010-2023 SupplyDomain [UID 600]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SupplyDomain {
    #[default]
    NotUsed = 0,
    Class1Subsistence = 1,
    Class2ClothingIndividualEquipmentToolsAdminSupplies = 2,
    Class3PetroleumOilsLubricants = 3,
    Class4ConstructionMaterials = 4,
    Class5Ammunition = 5,
    Class6PersonnelDemandItems = 6,
    Class7MajorItems = 7,
    Class8MedicalMaterial = 8,
    Class9RepairPartsandComponents = 9,
    Class10MaterialtoSupportNonMilitaryPrograms = 10,
    Class11Supplies = 11,
    Class12SlingLoads = 12,
}

// SISO-REF-010-2023 Class1SupplyCategorySubsistence [UID 601]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Class1SupplyCategorySubsistence {
    #[default]
    Other = 1,
    ANonPerishable = 2,
    CCombatRations = 3,
    RRefrigerated = 4,
    SOtherNonRefrigerated = 5,
    WWater = 6,
}

// SISO-REF-010-2023 Class2SupplyCategoryClothingIndividualEquipmentToolsAdminSupplies [UID 602]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Class2SupplyCategoryClothingIndividualEquipmentToolsAdminSupplies {
    #[default]
    Other = 1,
    AAir = 2,
    BGroundSupportMateriel = 3,
    EGeneralSupplies = 4,
    FClothing = 5,
    GElectronics = 6,
    MWeapons = 7,
    TIndustrialSupplies = 8,
}

// SISO-REF-010-2023 Class3SupplyCategoryPetroleumOilsLubricants [UID 603]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Class3SupplyCategoryPetroleumOilsLubricants {
    #[default]
    Other = 1,
    APOLforAirVehicles = 2,
    WPOLforLandVehicles = 3,
    PPackagedPOL = 4,
}

// SISO-REF-010-2023 Class4SupplyCategoryConstructionMaterials [UID 604]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Class4SupplyCategoryConstructionMaterials {
    #[default]
    Other = 1,
    AConstruction = 2,
    BBarrier = 3,
}

// SISO-REF-010-2023 Class6SupplyCategoryPersonnelDemandItems [UID 606]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Class6SupplyCategoryPersonnelDemandItems {
    #[default]
    Other = 1,
}

// SISO-REF-010-2023 Class7SupplyCategoryMajorItems [UID 607]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Class7SupplyCategoryMajorItems {
    #[default]
    Other = 1,
    AAir = 2,
    BGroundSupportMateriel = 3,
    DAdminVehicles = 4,
    GElectronics = 5,
    JRacksAdaptorsPylons = 6,
    KTacticalVehicles = 7,
    LMissiles = 8,
    MWeapons = 9,
    NSpecialWeapons = 10,
    XAircraftEngines = 11,
    DropTank = 20,
    ConformalFuelTank = 21,
    LuggagePod = 22,
    ECMPod = 23,
    ParaDrogue = 24,
    TargetingPod = 25,
    Fairing = 26,
    AirRefuellingPod = 27,
    HeavyAirdrop = 28,
    ContainerDeliverySystemAirdrop = 29,
    RocketPodLauncher = 30,
    TacticalPod = 31,
    RECCEpod = 32,
    FLIRpod = 33,
}

// SISO-REF-010-2023 Class8SupplyCategoryMedicalMaterial [UID 608]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Class8SupplyCategoryMedicalMaterial {
    #[default]
    Other = 1,
    AMedicalMateriel = 2,
    BBloodFluids = 3,
}

// SISO-REF-010-2023 Class9SupplyCategoryRepairPartsandComponents [UID 609]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Class9SupplyCategoryRepairPartsandComponents {
    #[default]
    Other = 1,
    AAir = 2,
    BGroundSupportMateriel = 3,
    DAdminVehicles = 4,
    GElectronics = 5,
    KTacticalVehicles = 6,
    LMissiles = 7,
    MWeapons = 8,
    NSpecialWeapons = 9,
    XAircraftEngines = 10,
}

// SISO-REF-010-2023 Class10SupplyCategoryMaterialtoSupportNonMilitaryPrograms [UID 610]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Class10SupplyCategoryMaterialtoSupportNonMilitaryPrograms {
    #[default]
    Other = 1,
}

// SISO-REF-010-2023 Class11SupplyCategorySupplies(NonDoctrinal) [UID 611]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Class11SupplyCategorySupplies(NonDoctrinal) {
    #[default]
    Other = 1,
    Pallets = 2,
    FuelTanksDrumsandBladders = 3,
    Chests = 4,
    Boxes = 5,
}

// SISO-REF-010-2023 Class12SupplyCategorySlingLoads(NonDoctrinal) [UID 612]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Class12SupplyCategorySlingLoads(NonDoctrinal) {
    #[default]
    Other = 1,
    SlingLoadBlivet = 2,
    SlingLoadCrate = 3,
    SlingLoadWaterBucket = 4,
    SlingLoadVehicles = 5,
    SlingLoadHowitzer = 6,
    SlingLoadCollapsible = 7,
    SlingLoadBladder = 8,
    SlingLoadPalletofCrates = 9,
    SlingLoadHelicopters = 10,
    SlingLoadHoist = 11,
    SlingLoadConcreteBlock = 12,
}

// SISO-REF-010-2023 LifeSavingEquipment [UID 633]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum LifeSavingEquipment {
    #[default]
    Lifeboat = 1,
    Liferaft = 2,
    MOBBoat = 3,
    Lifebuoy = 4,
}

// SISO-REF-010-2023 IslandSubcategory [UID 715]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum IslandSubcategory {
    #[default]
    Other = 0,
    Islands10002499km2 = 1,
    Islands25004999km2 = 2,
    Islands50009999km2 = 3,
    Islands1000024999km2 = 4,
    Islands2500099999km2 = 5,
    Islands100000km2andGreater = 6,
}

// SISO-REF-010-2023 Link11MessageSubType [UID 730]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link11MessageSubType {
    #[default]
    NoStatement = 0,
    Interrogation = 1,
    DataStart = 2,
    Data = 3,
    DataStop = 4,
}

// SISO-REF-010-2023 Link11MessageTypeIdentifier [UID 731]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link11MessageTypeIdentifier {
    #[default]
    NoStatement = 0,
    NetTest = 1,
    RollCall = 2,
    PicketReply = 3,
    ShortBroadcast = 4,
    Broadcast = 5,
    NetSync = 6,
}

// SISO-REF-010-2023 Link11DataSignallingRate [UID 732]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link11DataSignallingRate {
    #[default]
    NoStatement = 0,
    _1364bps = 1,
    _2250bps = 2,
    _1200bps = 3,
    _2400bps = 4,
}

// SISO-REF-010-2023 Link11SignalIntegrationInterval [UID 733]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link11SignalIntegrationInterval {
    #[default]
    NoStatement = 0,
    _9ms = 1,
    _18ms = 2,
}

// SISO-REF-010-2023 Link11SignalWaveform [UID 734]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link11SignalWaveform {
    #[default]
    NoStatementCLEWFormat = 0,
    ConventionalLinkElevenWaveform = 1,
    SingleToneLinkElevenWaveform = 2,
}

// SISO-REF-010-2023 Link1111BEncryptionFlag [UID 735]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link1111BEncryptionFlag {
    #[default]
    NoEncryptionUsed = 0,
    EncryptionUsed = 1,
}

// SISO-REF-010-2023 SISOSTD002Version [UID 736]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum SISOSTD002Version {
    #[default]
    SISOSTD0022006 = 0,
    SISOSTD0022021 = 1,
}

// SISO-REF-010-2023 Link11BLinkState [UID 737]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link11BLinkState {
    #[default]
    NoStatement = 0,
    Inactive = 1,
    Ready = 2,
    Active = 3,
    Operational = 4,
}

// SISO-REF-010-2023 Link11BModeofOperation [UID 738]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link11BModeofOperation {
    #[default]
    NoStatement = 0,
    FullTransmissionofData = 1,
    LimitedTransmissionofData = 2,
    Receiveonly = 3,
}

// SISO-REF-010-2023 Link11BMessageSubType [UID 739]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link11BMessageSubType {
    #[default]
    NoStatement = 0,
    TransmissionFrame = 1,
    StandbySignal = 2,
}

// SISO-REF-010-2023 Link11BDataSignalingRate [UID 740]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link11BDataSignalingRate {
    #[default]
    NoStatement = 0,
    NotUsed = 1,
    NotUsed = 2,
    _1200bps = 3,
    _2400bps = 4,
    _600bps = 5,
}

// SISO-REF-010-2023 Link11BModulationStandard [UID 741]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Link11BModulationStandard {
    #[default]
    NoStatement = 0,
    CCITTV_23 = 1,
}

// SISO-REF-010-2023 CIGIExtensionPacketID [UID 780]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum CIGIExtensionPacketID {
    #[default]
    ImageCaptureRequestpacketID = 4096,
    ImageCaptureResponsepacketID = 4097,
    StateNotificationRequestPacketID = 4098,
    StateNotificationResponsePacketID = 4099,
    GlobalRefFrameDefPacketID = 5000,
}

// SISO-REF-010-2023 Link16Version [UID 800]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum Link16Version {
    #[default]
    NoStatement = 0,
    MILSTD6016C = 1,
    MILSTD6016D = 2,
    MILSTD6016E = 3,
    MILSTD6016F = 4,
    MILSTD6016FC1 = 5,
    STANAG5516Ed3 = 103,
    STANAG5516Ed4 = 104,
    STANAG5516Ed5 = 105,
    STANAG5516Ed6 = 106,
    STANAG5516Ed8 = 108,
}

// SISO-REF-010-2023 AircraftIDSource [UID 801]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum AircraftIDSource {
    #[default]
    ModeSAircraftIdentificationFieldValue = 0,
    GICBIFFDataRecordAvailable = 1,
}

// SISO-REF-010-2023 ClothingIRSignature [UID 802]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum ClothingIRSignature {
    #[default]
    StandardClothing = 0,
    Camouflage = 1,
    ThermalBlanket = 2,
    Other = 3,
}

// SISO-REF-010-2023 DamageArea [UID 889]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, PartialEq)]
pub enum DamageArea {
    #[default]
    DamageArea1orNotApplicableifDamageAreasarenotdefined_ = 0,
    DamageArea2 = 1,
    DamageArea3 = 2,
    DamageArea4 = 3,
    DamageArea5 = 4,
    DamageArea6 = 5,
    DamageArea7 = 6,
    DamageArea8 = 7,
}

