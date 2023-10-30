use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug, Default)]
pub struct EmitterSystem {
    pub emitter_name: EmitterName,
    pub function: EmitterSystemFunction,
    pub emitter_id_number: ,
}

impl EmitterSystem {
    pub fn new(name: EmitterName, function: EmitterSystemFunction, id: EmitterIdNumber) -> Self {
        EmitterSystem {
            emitter_name: name,
            function,
            emitter_id_number: id,
        }
    }
}

pub enum EmitterName {
    
}

pub enum EmitterSystemFunction {
    Other = 0,
    MultiFunction = 1,
    EarlyWarningSurveillance = 2,
    HeightFinder = 3,
    FireControl = 4,
    AcquisitionDetection = 5,
    Tracker = 6,
    GuidanceIllumination = 7,
    FiringPointLaunchPointLocation = 8,
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
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    IdentifyFriendOrFoe = 34,
    InstrumentLandingSystem = 35,
    IonosphericSound = 36,
    Interrogator = 37,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    BarrageJamming = 38,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    ClickJamming = 39,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    DeceptiveJamming = 40,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    FrequencySweptJamming = 41,
    Jammer = 42,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    NoiseJamming = 43,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    PulsedJamming = 44,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    RepeaterJamming = 45,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    SpotNoiseJamming = 46,
    MissileAcquisition = 47,
    MissileDownlink = 48,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    Meteorological = 49,
    Space = 50,
    SurfaceSearch = 51,
    ShellTracking = 52,
    Television = 56,
    Unknown = 57,
    VideoRemoting = 58,
    ExperimentalOrTraining = 59,
    MissileGuidance = 60,
    MissileHoming = 61,
    MissileTracking = 62,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    JammingNoise = 64,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    JammingDeception = 65,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    Decoy = 66,
    NavigationDistanceMeasuringEquipment = 71,
    TerrainFollowing = 72,
    WeatherAvoidance = 73,
    ProximityFuse = 74,
    #[deprecated(note="Deprecated in SISO-REF-010-2020")]
    Instrumentation1 = 75,
    Radiosonde = 76,
    Sonobuoy = 77,
    BathythermalSensor = 78,
    TowedCounterMeasure = 79,
    DippingSonar = 80,
    TowedAcousticSensor = 81,
    WeaponNonLethal = 96,
    WeaponLethal = 97,
    TestEquipment = 98,
    AcquisitionTrack = 99,
    TrackGuidance = 100,
    GuidanceIlluminationTrackAcquisition = 101,
    SearchAcquisition = 102,
}

pub enum EmitterIdNumber {
    
}
