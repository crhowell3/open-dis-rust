//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};

#[derive(Copy, Clone, Debug)]
pub struct EmitterSystem {
    pub emitter_name: EmitterName,
    pub function: EmitterSystemFunction,
    pub emitter_id_number: u8,
}

impl EmitterSystem {
    pub fn new(name: EmitterName, function: EmitterSystemFunction, id: u8) -> Self {
        EmitterSystem {
            emitter_name: name,
            function,
            emitter_id_number: id,
        }
    }

    pub fn default() -> Self {
        EmitterSystem {
            emitter_name: EmitterName::E12456X,
            function: EmitterSystemFunction::Other,
            emitter_id_number: 0,
        }
    }

    pub fn serialize(&self, buf: &mut BytesMut) {
        buf.put_u16(self.emitter_name as u16);
        buf.put_u8(self.function as u8);
        buf.put_u8(self.emitter_id_number);
    }

    pub fn decode(buf: &mut BytesMut) -> EmitterSystem {
        EmitterSystem {
            emitter_name: EmitterName::decode(buf.get_u16()),
            function: EmitterSystemFunction::decode(buf.get_u8()),
            emitter_id_number: buf.get_u8(),
        }
    }
}

// TODO: There are a LOT of values. Need to populate this.
#[derive(Copy, Clone, Debug)]
pub enum EmitterName {
    E12456X = 2,
    E1L117 = 3,
    E1L121E = 4,
    E1l250 = 5,
    E1L220U = 6,
    E1L1221E = 7,
}

impl EmitterName {
    pub fn decode(word: u16) -> EmitterName {
        match word {
            2 => EmitterName::E12456X,
            3 => EmitterName::E1L117,
            4 => EmitterName::E1L121E,
            5 => EmitterName::E1l250,
            6 => EmitterName::E1L220U,
            7 => EmitterName::E1L1221E,
            _ => EmitterName::E12456X,
        }
    }
}

#[derive(Copy, Clone, Debug)]
#[allow(deprecated)]
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
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
    IdentifyFriendOrFoe = 34,
    InstrumentLandingSystem = 35,
    IonosphericSound = 36,
    Interrogator = 37,
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
    BarrageJamming = 38,
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
    ClickJamming = 39,
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
    DeceptiveJamming = 40,
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
    FrequencySweptJamming = 41,
    Jammer = 42,
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
    NoiseJamming = 43,
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
    PulsedJamming = 44,
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
    RepeaterJamming = 45,
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
    SpotNoiseJamming = 46,
    MissileAcquisition = 47,
    MissileDownlink = 48,
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
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
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
    JammingNoise = 64,
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
    JammingDeception = 65,
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
    Decoy = 66,
    NavigationDistanceMeasuringEquipment = 71,
    TerrainFollowing = 72,
    WeatherAvoidance = 73,
    ProximityFuse = 74,
    #[deprecated(note = "Deprecated in SISO-REF-010-2020")]
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

impl EmitterSystemFunction {
    pub fn decode(byte: u8) -> EmitterSystemFunction {
        match byte {
            0 => EmitterSystemFunction::Other,
            1 => EmitterSystemFunction::MultiFunction,
            2 => EmitterSystemFunction::EarlyWarningSurveillance,
            3 => EmitterSystemFunction::HeightFinder,
            4 => EmitterSystemFunction::FireControl,
            5 => EmitterSystemFunction::AcquisitionDetection,
            6 => EmitterSystemFunction::Tracker,
            7 => EmitterSystemFunction::GuidanceIllumination,
            8 => EmitterSystemFunction::FiringPointLaunchPointLocation,
            9 => EmitterSystemFunction::RangeOnly,
            10 => EmitterSystemFunction::RadarAltimeter,
            11 => EmitterSystemFunction::Imaging,
            12 => EmitterSystemFunction::MotionDetection,
            13 => EmitterSystemFunction::Navigation,
            14 => EmitterSystemFunction::WeatherMeteorological,
            15 => EmitterSystemFunction::Instrumentation,
            16 => EmitterSystemFunction::IdentificationClassification,
            17 => EmitterSystemFunction::AAAFireControl,
            18 => EmitterSystemFunction::AirSearchBomb,
            19 => EmitterSystemFunction::AirIntercept,
            20 => EmitterSystemFunction::Altimeter,
            21 => EmitterSystemFunction::AirMapping,
            22 => EmitterSystemFunction::AirTrafficControl,
            23 => EmitterSystemFunction::Beacon,
            24 => EmitterSystemFunction::BattlefieldSurveillance,
            25 => EmitterSystemFunction::GroundControlApproach,
            26 => EmitterSystemFunction::GroundControlIntercept,
            27 => EmitterSystemFunction::CoastalSurveillance,
            28 => EmitterSystemFunction::DecoyMimic,
            29 => EmitterSystemFunction::DataTransmission,
            30 => EmitterSystemFunction::EarthSurveillance,
            31 => EmitterSystemFunction::GunLayBeacon,
            32 => EmitterSystemFunction::GroundMapping,
            33 => EmitterSystemFunction::HarborSurveillance,
            35 => EmitterSystemFunction::InstrumentLandingSystem,
            36 => EmitterSystemFunction::IonosphericSound,
            37 => EmitterSystemFunction::Interrogator,
            42 => EmitterSystemFunction::Jammer,
            47 => EmitterSystemFunction::MissileAcquisition,
            48 => EmitterSystemFunction::MissileDownlink,
            50 => EmitterSystemFunction::Space,
            51 => EmitterSystemFunction::SurfaceSearch,
            52 => EmitterSystemFunction::ShellTracking,
            56 => EmitterSystemFunction::Television,
            57 => EmitterSystemFunction::Unknown,
            58 => EmitterSystemFunction::VideoRemoting,
            59 => EmitterSystemFunction::ExperimentalOrTraining,
            60 => EmitterSystemFunction::MissileGuidance,
            61 => EmitterSystemFunction::MissileHoming,
            62 => EmitterSystemFunction::MissileTracking,
            71 => EmitterSystemFunction::NavigationDistanceMeasuringEquipment,
            72 => EmitterSystemFunction::TerrainFollowing,
            73 => EmitterSystemFunction::WeatherAvoidance,
            74 => EmitterSystemFunction::ProximityFuse,
            76 => EmitterSystemFunction::Radiosonde,
            77 => EmitterSystemFunction::Sonobuoy,
            78 => EmitterSystemFunction::BathythermalSensor,
            79 => EmitterSystemFunction::TowedCounterMeasure,
            80 => EmitterSystemFunction::DippingSonar,
            81 => EmitterSystemFunction::TowedAcousticSensor,
            96 => EmitterSystemFunction::WeaponNonLethal,
            97 => EmitterSystemFunction::WeaponLethal,
            98 => EmitterSystemFunction::TestEquipment,
            99 => EmitterSystemFunction::AcquisitionTrack,
            100 => EmitterSystemFunction::TrackGuidance,
            101 => EmitterSystemFunction::GuidanceIlluminationTrackAcquisition,
            102 => EmitterSystemFunction::SearchAcquisition,
            _ => EmitterSystemFunction::Other,
        }
    }
}
