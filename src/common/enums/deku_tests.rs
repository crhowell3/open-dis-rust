//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! Checks that every deku-enabled enum encodes exactly as the existing
//! `FromPrimitive` mapping does, and that undefined values are rejected.
//!
//! There is one line per enum in `deku_matches_from_primitive_for_every_enum`;
//! add one when an enum gains an `impl_deku_enum!`.

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    clippy::too_many_lines,
    clippy::as_conversions,
    clippy::unreadable_literal
)]

use std::fmt::Debug;

use deku::{DekuError, DekuReader, DekuWriter, no_std_io::Cursor, reader::Reader, writer::Writer};
use num_traits::FromPrimitive;

use super::*;

/// Decode one `T` from `bytes`, returning it and the number of bytes read.
fn decode<T: for<'a> DekuReader<'a, ()>>(bytes: &[u8]) -> Result<(T, usize), DekuError> {
    let mut cursor = Cursor::new(bytes);
    let mut reader = Reader::new(&mut cursor);
    let value = T::from_reader_with_ctx(&mut reader, ())?;
    Ok((value, reader.bits_read / 8))
}

fn encode<T: DekuWriter<()>>(value: &T) -> Vec<u8> {
    let mut out = Vec::new();
    let mut cursor = Cursor::new(&mut out);
    let mut writer = Writer::new(&mut cursor);
    value.to_writer(&mut writer, ()).unwrap();
    writer.finalize().unwrap();
    out
}

trait Wire: for<'a> DekuReader<'a, ()> + DekuWriter<()> + PartialEq + Debug {}
impl<T: for<'a> DekuReader<'a, ()> + DekuWriter<()> + PartialEq + Debug> Wire for T {}

/// Decode `bytes` with deku and compare against the expected value.
fn check_one<T: Wire>(bytes: &[u8], expected: Option<T>) {
    let name = std::any::type_name::<T>();
    match (decode::<T>(bytes), expected) {
        (Ok((got, read)), Some(want)) => {
            assert_eq!(read, bytes.len(), "{name}: {bytes:?} read {read} bytes");
            assert_eq!(got, want, "{name}: wrong variant for {bytes:?}");
            assert_eq!(encode(&got), bytes, "{name}: re-encode differs");
        }
        (Err(_), None) => {}
        (Ok((got, _)), None) => panic!("{name}: undefined value {bytes:?} decoded as {got:?}"),
        (Err(e), Some(want)) => panic!("{name}: {bytes:?} should decode as {want:?}: {e}"),
    }
}

fn check_u8<T: Wire>(from: fn(u8) -> Option<T>) {
    for v in 0..=u8::MAX {
        check_one(&[v], from(v));
    }
}

fn check_u16<T: Wire>(from: fn(u16) -> Option<T>) {
    for v in 0..=u16::MAX {
        check_one(&v.to_be_bytes(), from(v));
    }
}

/// u32 enums are too wide to sweep, so check every defined value plus the
/// values just around them and a few far-off ones.
fn check_u32<T: Wire>(defined: &[u32], from: fn(u32) -> Option<T>) {
    let probes = defined
        .iter()
        .flat_map(|&d| [d.wrapping_sub(1), d, d.wrapping_add(1)])
        .chain([0, 0x00FF_FFFF, 0x7FFF_FFFF, u32::MAX]);
    for v in probes {
        check_one(&v.to_be_bytes(), from(v));
    }
}

#[test]
fn deku_matches_from_primitive_for_every_enum() {
    check_u8::<ProtocolVersion>(ProtocolVersion::from_u8);
    check_u8::<PduType>(PduType::from_u8);
    check_u8::<ProtocolFamily>(ProtocolFamily::from_u8);
    check_u8::<ForceId>(ForceId::from_u8);
    check_u8::<EntityKind>(EntityKind::from_u8);
    check_u8::<OtherKinds>(OtherKinds::from_u8);
    check_u8::<LandDomainCategories>(LandDomainCategories::from_u8);
    check_u8::<AirDomainCategories>(AirDomainCategories::from_u8);
    check_u8::<SurfaceDomainCategories>(SurfaceDomainCategories::from_u8);
    check_u8::<SubsurfaceDomainCategories>(SubsurfaceDomainCategories::from_u8);
    check_u8::<SpaceDomainCategories>(SpaceDomainCategories::from_u8);
    check_u8::<MunitionKind>(MunitionKind::from_u8);
    check_u8::<MunitionCategory>(MunitionCategory::from_u8);
    check_u8::<USWeaponSubcategories>(USWeaponSubcategories::from_u8);
    check_u8::<RussiaWeaponSubcategories>(RussiaWeaponSubcategories::from_u8);
    check_u8::<UKWeaponSubcategories>(UKWeaponSubcategories::from_u8);
    check_u8::<FrenchWeaponSubcategories>(FrenchWeaponSubcategories::from_u8);
    check_u8::<LifeFormsSubcategoryGermanWeapons>(LifeFormsSubcategoryGermanWeapons::from_u8);
    check_u8::<EnvironmentalSubcategory>(EnvironmentalSubcategory::from_u8);
    check_u8::<RadioCategory>(RadioCategory::from_u8);
    check_u8::<RadioSubcategory>(RadioSubcategory::from_u8);
    check_u8::<ExpendableAirCategory>(ExpendableAirCategory::from_u8);
    check_u8::<ExpendableSurfaceCategory>(ExpendableSurfaceCategory::from_u8);
    check_u8::<ExpendableSubsurfaceCategory>(ExpendableSubsurfaceCategory::from_u8);
    check_u8::<SensorEmitterCategory>(SensorEmitterCategory::from_u8);
    check_u16::<Country>(Country::from_u16);
    check_u8::<DeadReckoningAlgorithm>(DeadReckoningAlgorithm::from_u8);
    check_u8::<EntityMarkingCharacterSet>(EntityMarkingCharacterSet::from_u8);
    check_u32::<EntityCapabilities>(
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        EntityCapabilities::from_u32,
    );
    check_u8::<VariableParameterRecordType>(VariableParameterRecordType::from_u8);
    check_u32::<AttachedParts>(
        &[
            0, 1, 512, 640, 768, 896, 897, 898, 899, 900, 901, 902, 903, 904, 905,
        ],
        AttachedParts::from_u32,
    );
    check_u8::<ArticulatedPartsTypeMetric>(ArticulatedPartsTypeMetric::from_u8);
    check_u32::<ArticulatedPartsTypeClass>(
        &[
            0, 1024, 1056, 1088, 1120, 1152, 1184, 1216, 1248, 1280, 1312, 1344, 1376, 1408, 1440,
            1472, 1504, 1536, 1568, 1600, 1632, 1664, 1696, 1728, 1760, 1792, 1824, 1856, 1888,
            1920, 1952, 1984, 2048, 2080, 2112, 2144, 2176, 2208, 2240, 2272, 2304, 2336, 3072,
            3104, 3136, 3168, 3200, 3232, 3264, 3296, 3328, 3360, 3392, 3424, 3456, 3488, 3520,
            3552, 3584, 3616, 3648, 3680, 3712, 3744, 3776, 3808, 3840, 3872, 4096, 4128, 4160,
            4192, 4224, 4256, 4288, 4320, 4352, 4384, 4416, 4448, 4480, 4512, 4544, 4576, 4608,
            4640, 4672, 4704, 4736, 4768, 4800, 4832, 4864, 4896, 4928, 4960, 4992, 5024, 5056,
            5088, 5120, 5152, 5184, 5216, 5248, 5280, 5312, 5344, 5376, 5408, 5440, 5472, 5504,
            5536, 5568, 5600, 5632, 5664, 5696, 5728, 5760, 5792, 5824, 5856, 5888, 5920, 5952,
            5984, 6016, 6048, 6080, 6112, 6144, 6176, 6208, 6240, 6272, 6304, 6336, 6368, 6400,
            6432, 6464, 6496, 6528, 6560, 6592, 6624, 6656, 6688, 6720, 6752, 6784, 6816, 6848,
            6880, 6912, 6944, 6976, 7008, 7040, 7072, 7104, 7136, 7168, 7200, 7232, 7264, 7296,
            7328, 7360, 7392, 7424, 7456, 7488, 7520, 7552, 7584, 7616, 7648, 7680, 7712, 7744,
            7776, 7808, 7840, 7872, 7904, 7936, 7968, 8000, 8032, 8064, 8096, 8128, 8160, 8192,
            8224, 8256, 8288, 8320, 8352, 8384, 8416, 8448, 8480, 8512, 8544, 8576, 8608, 8640,
            8672, 8704, 8736, 8768, 8800, 8832, 8864, 8896, 8928, 8960, 8992, 9024, 9056, 9088,
            9120, 9152, 9184, 9216, 9248, 9280, 9312, 9344, 9376, 9408, 9440, 9472, 9504, 9536,
            9568, 9600, 9632, 9664, 9696, 9728, 9760, 9792, 9824, 9856, 9888, 9920, 9952, 9984,
            10016, 10048, 10080, 10112, 10144,
        ],
        ArticulatedPartsTypeClass::from_u32,
    );
    check_u16::<MunitionDescriptorWarhead>(MunitionDescriptorWarhead::from_u16);
    check_u16::<MunitionDescriptorFuse>(MunitionDescriptorFuse::from_u16);
    check_u8::<DetonationResult>(DetonationResult::from_u8);
    check_u8::<ServiceRequestServiceTypeRequested>(ServiceRequestServiceTypeRequested::from_u8);
    check_u16::<RepairCompleteRepair>(RepairCompleteRepair::from_u16);
    check_u8::<RepairResponseRepairResult>(RepairResponseRepairResult::from_u8);
    check_u32::<VariableRecordTypes>(
        &[
            1, 1001, 1002, 1003, 1004, 1005, 1006, 1007, 1009, 1051, 1052, 1053, 1054, 1055, 1056,
            1057, 1058, 1059, 1061, 1111, 1112, 1114, 1115, 1116, 1118, 1119, 1120, 1200, 3000,
            3500, 3501, 3502, 4000, 4001, 4500, 5000, 5001, 5002, 5003, 5004, 5005, 5006, 5007,
            5008, 5009, 5010, 5011, 5012, 5013, 5014, 5500, 5501, 10000, 10010, 10020, 10030,
            10110, 10115, 10140, 10150, 10157, 10160, 10170, 10190, 10250, 10254, 10256, 10258,
            10259, 10262, 10263, 10264, 10267, 10268, 10280, 10290, 10300, 10306, 10307, 10308,
            10310, 10311, 10312, 10313, 10314, 10315, 10316, 10400, 10600, 11000, 11100, 11110,
            11120, 11130, 11140, 11150, 11160, 11170, 11180, 11200, 11300, 11500, 11501, 11502,
            11503, 11504, 12000, 12110, 12120, 12130, 12140, 12150, 12160, 12170, 12300, 13000,
            13100, 13200, 14000, 14200, 14300, 14400, 14500, 14520, 14540, 14600, 14700, 14800,
            15000, 15100, 15200, 15300, 15400, 15500, 15510, 15600, 15800, 19177, 20000, 21000,
            21100, 21200, 21300, 23000, 23100, 23105, 24000, 24001, 24002, 24003, 24004, 24005,
            24006, 24007, 24008, 24009, 24010, 24060, 24100, 24110, 24120, 24130, 24140, 24150,
            24160, 24300, 25000, 26000, 27000, 28000, 30000, 30010, 30100, 30300, 30400, 30401,
            30402, 30403, 30404, 30405, 30406, 30407, 30408, 30409, 30410, 30411, 30412, 30413,
            30414, 30415, 30416, 30417, 30418, 30419, 30420, 30421, 30422, 30423, 30424, 30425,
            30426, 30427, 30428, 30429, 30430, 30431, 31000, 31010, 31100, 31200, 31210, 31220,
            31230, 31300, 31400, 31500, 31510, 31520, 31530, 31600, 31700, 31800, 31900, 32000,
            32100, 32200, 32300, 32500, 32600, 32700, 33000, 33100, 33101, 33200, 33300, 33400,
            33500, 33600, 33710, 33720, 33730, 34000, 34100, 34110, 35000, 35050, 35100, 35200,
            35300, 36000, 36100, 36200, 36300, 36400, 37000, 37100, 37200, 37300, 38100, 39000,
            40000, 40010, 40015, 40020, 41000, 42000, 42100, 42200, 42300, 43000, 43100, 43200,
            43300, 43410, 43420, 43430, 43440, 44000, 44100, 44110, 44120, 44130, 44140, 44200,
            44210, 44220, 44230, 44240, 44300, 44310, 44320, 44330, 44340, 44400, 44410, 44420,
            44430, 44440, 44500, 44510, 44520, 44530, 44540, 45000, 45005, 45010, 45020, 45030,
            45040, 45050, 45060, 45070, 45080, 46000, 46010, 50000, 50103, 50106, 50118, 50120,
            50124, 50126, 50600, 50700, 51000, 51010, 51100, 51110, 51111, 52000, 52001, 52100,
            52200, 52300, 52305, 52310, 52320, 52330, 52340, 52350, 52360, 52370, 52380, 52400,
            52410, 52420, 52430, 52440, 52500, 52510, 52511, 52520, 52530, 52600, 52601, 52602,
            52610, 52620, 52630, 53000, 53050, 53060, 53100, 53200, 53250, 53300, 53350, 53400,
            53410, 53420, 53430, 54000, 54100, 55000, 55100, 55101, 55105, 55200, 55300, 55401,
            55410, 55420, 55425, 55430, 55435, 56000, 56100, 56105, 56110, 56111, 56120, 56121,
            56130, 56140, 56200, 56205, 56210, 56220, 56221, 56230, 56231, 56240, 56310, 56320,
            56330, 56340, 56350, 57200, 57300, 57400, 57410, 57411, 57420, 57421, 57500, 57610,
            57611, 57620, 57621, 58000, 58100, 58105, 58200, 58430, 58435, 59000, 59100, 59110,
            59115, 59120, 59125, 59130, 59135, 59140, 59145, 59150, 59155, 59160, 60000, 61005,
            61100, 61101, 61200, 61300, 61400, 61500, 61600, 61700, 62100, 62101, 62200, 62300,
            62400, 62500, 63101, 63102, 63103, 63104, 63105, 63106, 63107, 63108, 63109, 63110,
            63111, 63112, 63113, 63114, 63115, 70000, 71000, 71100, 71200, 71300, 72000, 72100,
            72600, 72700, 73000, 74000, 75000, 75100, 75200, 90000, 90050, 90070, 90100, 90200,
            90300, 90400, 90500, 90600, 90700, 90800, 90900, 91000, 91100, 91200, 91300, 91400,
            91500, 91600, 100001, 100002, 100003, 100004, 100005, 100008, 100010, 100011, 100012,
            100014, 100047, 100048, 100058, 100061, 100063, 100068, 100100, 100160, 100161, 100162,
            100163, 100164, 100165, 100166, 100167, 100169, 100170, 100171, 100172, 100200, 100205,
            100206, 100207, 100208, 100213, 100214, 100218, 100300, 100400, 100500, 100510, 100520,
            100600, 100610, 100620, 100631, 100632, 100633, 100634, 100640, 100650, 100660, 100661,
            100662, 100700, 100800, 100900, 101000, 101100, 101200, 101300, 101310, 101320, 101321,
            101330, 101340, 101350, 101360, 101370, 101400, 101500, 102100, 102200, 200201, 200202,
            200203, 200204, 200205, 200207, 200210, 200211, 200213, 200214, 200215, 200216, 200217,
            200301, 200302, 200303, 200304, 200305, 200307, 200310, 200311, 200313, 200314, 200315,
            200316, 200317, 200400, 200401, 200402, 200403, 200404, 200405, 200406, 200407, 200408,
            200409, 200410, 200411, 200412, 200413, 200414, 200415, 200416, 200417, 200418, 200419,
            200420, 200421, 200422, 200423, 200424, 200425, 200426, 200427, 200428, 200429, 200430,
            200431, 200432, 200433, 200434, 200435, 200436, 200437, 200501, 200502, 200503, 200504,
            200505, 200506, 200507, 200508, 200509, 200510, 200511, 200512, 200513, 200514, 200515,
            200516, 200517, 200518, 200519, 200521, 200522, 200523, 200524, 200525, 200600, 200601,
            200602, 200603, 200604, 200605, 200606, 200607, 200608, 200609, 200610, 200611, 200612,
            200613, 240000, 240001, 240002, 240003, 240004, 240005, 240006, 240007, 240008, 240009,
            240010, 240011, 240012, 240013, 240014, 240015, 240016, 240017, 240018, 240019, 240020,
            240021, 240022, 240023, 240024, 240025, 240026, 240027, 240028, 240029, 240030, 240031,
            240032, 240033, 240034, 240035, 240036, 240037, 240038, 240039, 240040, 240041, 240042,
            240043, 240044, 240045, 240046, 240047, 240048, 240049, 240050, 240051, 240052, 240053,
            240054, 240055, 240056, 240057, 240058, 240059, 240060, 240061, 240062, 240063, 240064,
            240065, 240066, 240067, 240068, 240069, 240070, 240071, 240072, 240073, 240074, 240075,
            240076, 240077, 240078, 240079, 240080, 240081, 240082, 240083, 240084, 240085, 240086,
            240087, 240088, 240089, 240090, 240091, 240092, 240093, 240094, 240095, 240096, 240097,
            240098, 240099, 240100, 240101, 240102, 240103, 240104, 240105, 240106, 240107, 240108,
            240109, 240110, 240111, 240112, 240113, 240114, 240115, 240116, 240117, 240118, 240119,
            240120, 240121, 240122, 240123, 240124, 240125, 240126, 240127, 240128, 240129, 240130,
            240131, 240132, 240133, 240134, 240135, 240136, 240137, 240138, 240139, 240140, 240141,
            240142, 240143, 240144, 240145, 240146, 240147, 240148, 240149, 240150, 240151, 240152,
            240153, 270115, 270116, 270117, 270150, 270151, 270152, 270153, 270154, 270155, 270156,
            270157, 270158, 270159, 270160, 270161, 270162, 270163, 270164, 270165, 270166, 300000,
            300001, 300002, 300003, 300004, 300005, 300006, 300007, 300008, 300009, 300010, 300011,
            300012, 300013, 300014, 300016, 300017, 300018, 300019, 300020, 300021, 300022, 300023,
            300024, 300025, 300026, 300027, 301100, 301130, 301140, 301150, 301151, 301152, 301160,
            301170, 301171, 301174, 301175, 301176, 301177, 301178, 301179, 301180, 301181, 301182,
            301183, 301184, 301185, 301186, 301187, 301189, 301190, 301191, 301197, 301198, 301199,
            301200, 301201, 301202, 301220, 301221, 301222, 301223, 301224, 301225, 301226, 301227,
            301228, 301229, 301234, 301237, 301238, 301239, 301240, 301241, 301242, 301243, 301244,
            301255, 301256, 400008, 400009, 400010, 400011, 400012, 400013, 400014, 400015, 400016,
            400017, 400018, 400019, 400020, 400021, 400022, 400023, 400024, 400025, 400026, 400027,
            400028, 400029, 400030, 400031, 400032, 400033, 500001, 500002, 500003, 500004, 500005,
            500006, 500007, 500008, 500009, 500010, 500011, 500012, 500013, 500014, 500015, 500016,
            500017, 500018, 500019, 500021, 500022, 500023, 500024, 500025, 500026, 500027, 500028,
            500029, 500200, 500201, 500202, 500203, 500204, 500205, 551001, 551002, 551003, 551004,
            551005, 551006, 551007, 552001, 552002, 553001, 553002, 553003, 553004, 553005, 553006,
            553007, 553008, 553009, 553010, 553011, 553012, 553013, 553014, 553015, 553016, 553017,
            553018, 553019, 553020, 553021, 553022, 553028, 553029, 554001, 554002, 554003, 554004,
            554005, 554006, 554007, 554008, 554009, 554010, 554100, 554101, 554102, 554103, 554104,
            554105, 554106, 554107, 554108, 554109, 554110, 555001, 555002, 555003, 555004, 555005,
            555006, 555007, 555008, 556001, 556007, 556008, 556016, 556017, 556018, 556019, 556020,
            556029, 557001, 557002, 557003, 557004, 557005, 557006, 557007, 557008, 557009, 557010,
            559001, 559002, 559003, 559004, 559005, 559006, 559007, 559008, 559009, 559010, 559011,
            559012, 559999, 600001, 600002, 600210, 600212, 600214, 600215, 600216, 600218, 600219,
            600220, 600300, 600301, 600302, 600303, 600304, 600305, 600306, 600307, 600308, 610006,
            610007, 610008, 610009, 610010, 610011, 610012, 610013, 610015, 610016, 610017, 610025,
            610026, 610027, 610029, 610032, 610035, 610036, 610037, 610040, 610042, 610043, 610044,
            610045, 610046, 610047, 610048, 610049, 610050, 613002, 613003, 613004, 613005, 613006,
            613007, 613008, 613013, 613016, 613017, 613020, 613021, 613022, 613027, 613028, 613029,
            613030, 613031, 615000, 615001, 615002, 615003, 615004, 615005, 615006, 615007, 615008,
            615009, 615010, 615011, 615012, 615013, 615015, 615017, 615018, 615019, 615020, 615021,
            615022, 615023, 615024, 615025, 615026, 615027, 615028, 615029, 615030, 615031, 615032,
            615033, 615034, 615035, 615036, 615037, 615038, 700000, 700001, 700002, 700003, 700004,
            700005, 1000620, 2000000, 2000010, 2000020, 2000030, 2000031, 2000032, 2000033,
            2000034, 2000035, 2000036, 2000037, 2000038, 2000039, 2000040, 2000041, 2000042,
            2000043, 2000044, 2000045, 2000050, 5005001, 5005002, 5005003, 5005006, 5005008,
            5005009, 5005010, 5005011, 5005012, 5005014, 5005055, 5005060, 5005061, 5005063,
            5005070, 5005080, 5005081, 5007010, 5007020, 5007030, 5007040, 5007050, 5008010,
            5008020, 5008030, 5008040, 5008050, 5008060, 5008070, 5008080, 5009010, 5009020,
            5507010,
        ],
        VariableRecordTypes::from_u32,
    );
    check_u8::<Reason>(Reason::from_u8);
    check_u16::<AcknowledgeFlag>(AcknowledgeFlag::from_u16);
    check_u16::<AcknowledgeResponseFlag>(AcknowledgeResponseFlag::from_u16);
    check_u32::<ActionRequestActionID>(
        &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 100, 101, 4303, 4304, 4305,
        ],
        ActionRequestActionID::from_u32,
    );
    check_u32::<ActionResponseRequestStatus>(
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 100, 101, 201],
        ActionResponseRequestStatus::from_u32,
    );
    check_u32::<EventType>(
        &[
            0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
        EventType::from_u32,
    );
    check_u8::<RequiredReliabilityService>(RequiredReliabilityService::from_u8);
    check_u16::<EmitterName>(EmitterName::from_u16);
    check_u8::<EmitterSystemFunction>(EmitterSystemFunction::from_u8);
    check_u8::<ElectromagneticEmissionStateUpdateIndicator>(
        ElectromagneticEmissionStateUpdateIndicator::from_u8,
    );
    check_u8::<ElectromagneticEmissionBeamFunction>(ElectromagneticEmissionBeamFunction::from_u8);
    check_u8::<HighDensityTrackJam>(HighDensityTrackJam::from_u8);
    check_u16::<DesignatorSystemName>(DesignatorSystemName::from_u16);
    check_u16::<DesignatorCode>(DesignatorCode::from_u16);
    check_u16::<IFFSystemType>(IFFSystemType::from_u16);
    check_u16::<IFFSystemName>(IFFSystemName::from_u16);
    check_u8::<IFFSystemMode>(IFFSystemMode::from_u8);
    check_u8::<IFFLayerSpecificInformation>(IFFLayerSpecificInformation::from_u8);
    check_u8::<IFFAlternateMode4ChallengeReply>(IFFAlternateMode4ChallengeReply::from_u8);
    check_u8::<IFFSystemType1OperationalParameter1>(IFFSystemType1OperationalParameter1::from_u8);
    check_u8::<IFFSystemType1OperationalParameter2>(IFFSystemType1OperationalParameter2::from_u8);
    check_u8::<SubcategoriesforLandCategory200Mammal>(
        SubcategoriesforLandCategory200Mammal::from_u8,
    );
    check_u8::<SubcategoriesforLandCategory201Reptile>(
        SubcategoriesforLandCategory201Reptile::from_u8,
    );
    check_u8::<SubcategoriesforLandCategory202Amphibian>(
        SubcategoriesforLandCategory202Amphibian::from_u8,
    );
    check_u8::<SubcategoriesforLandCategory203Insect>(
        SubcategoriesforLandCategory203Insect::from_u8,
    );
    check_u8::<SubcategoriesforLandCategory204Arachnid>(
        SubcategoriesforLandCategory204Arachnid::from_u8,
    );
    check_u8::<SubcategoriesforLandCategory205Mollusk>(
        SubcategoriesforLandCategory205Mollusk::from_u8,
    );
    check_u8::<SubcategoriesforLandCategory206Marsupial>(
        SubcategoriesforLandCategory206Marsupial::from_u8,
    );
    check_u8::<SubcategoriesforAirCategory200Bird>(SubcategoriesforAirCategory200Bird::from_u8);
    check_u8::<SubcategoriesforAirCategory201Insect>(SubcategoriesforAirCategory201Insect::from_u8);
    check_u8::<SubcategoriesforAirCategory202Mammal>(SubcategoriesforAirCategory202Mammal::from_u8);
    check_u8::<SubcategoriesforSubsurfaceCategory200Fish>(
        SubcategoriesforSubsurfaceCategory200Fish::from_u8,
    );
    check_u8::<SubcategoriesforSubsurfaceCategory201Mammal>(
        SubcategoriesforSubsurfaceCategory201Mammal::from_u8,
    );
    check_u8::<SubcategoriesforSubsurfaceCategory202Mollusk>(
        SubcategoriesforSubsurfaceCategory202Mollusk::from_u8,
    );
    check_u8::<SubcategoriesforSubsurfaceCategory203Crustacean>(
        SubcategoriesforSubsurfaceCategory203Crustacean::from_u8,
    );
    check_u8::<SubcategoriesforSubsurfaceCategory204Insect>(
        SubcategoriesforSubsurfaceCategory204Insect::from_u8,
    );
    check_u8::<AnimalLifeformGroupSizeRangeEnumerationforallDomains>(
        AnimalLifeformGroupSizeRangeEnumerationforallDomains::from_u8,
    );
    check_u8::<SpecificDimensionEnumerationsforLandAreaSize>(
        SpecificDimensionEnumerationsforLandAreaSize::from_u8,
    );
    check_u8::<SpecificDimensionEnumerationsforAirAreaSize>(
        SpecificDimensionEnumerationsforAirAreaSize::from_u8,
    );
    check_u8::<AddSpecificDimensionEnumerationsforSubsurfaceAreaSize>(
        AddSpecificDimensionEnumerationsforSubsurfaceAreaSize::from_u8,
    );
    check_u8::<AddVariantsforLandCategory200Mammal>(AddVariantsforLandCategory200Mammal::from_u8);
    check_u8::<VariantsforLandCategoriesReptilesAmphibiansInsectsandArachnids>(
        VariantsforLandCategoriesReptilesAmphibiansInsectsandArachnids::from_u8,
    );
    check_u8::<VariantsforAirCategory200Bird>(VariantsforAirCategory200Bird::from_u8);
    check_u8::<AddVariantsforAirCategory201Insect>(AddVariantsforAirCategory201Insect::from_u8);
    check_u8::<AddVariantsforSubsurfaceCategoriesFishMolluskCrustaceanandInsect>(
        AddVariantsforSubsurfaceCategoriesFishMolluskCrustaceanandInsect::from_u8,
    );
    check_u8::<VariantsforSubsurfaceCategory201Mammal>(
        VariantsforSubsurfaceCategory201Mammal::from_u8,
    );
    check_u8::<UAStateChangeUpdateIndicator>(UAStateChangeUpdateIndicator::from_u8);
    check_u16::<UAAcousticSystemName>(UAAcousticSystemName::from_u16);
    check_u8::<UAAcousticEmitterSystemFunction>(UAAcousticEmitterSystemFunction::from_u8);
    check_u16::<UAActiveEmissionParameterIndex>(UAActiveEmissionParameterIndex::from_u16);
    check_u16::<UAScanPattern>(UAScanPattern::from_u16);
    check_u16::<UAPassiveParameterIndex>(UAPassiveParameterIndex::from_u16);
    check_u16::<UAAdditionalPassiveActivityParameterIndex>(
        UAAdditionalPassiveActivityParameterIndex::from_u16,
    );
    check_u16::<TransmitterMajorModulation>(TransmitterMajorModulation::from_u16);
    check_u16::<TransmitterDetailAmplitudeModulation>(
        TransmitterDetailAmplitudeModulation::from_u16,
    );
    check_u16::<TransmitterDetailAmplitudeandAngleModulation>(
        TransmitterDetailAmplitudeandAngleModulation::from_u16,
    );
    check_u16::<TransmitterDetailAngleModulation>(TransmitterDetailAngleModulation::from_u16);
    check_u16::<TransmitterDetailCombinationModulation>(
        TransmitterDetailCombinationModulation::from_u16,
    );
    check_u16::<TransmitterDetailPulseModulation>(TransmitterDetailPulseModulation::from_u16);
    check_u16::<TransmitterDetailUnmodulatedModulation>(
        TransmitterDetailUnmodulatedModulation::from_u16,
    );
    check_u16::<TransmitterDetailCarrierPhaseShiftModulation>(
        TransmitterDetailCarrierPhaseShiftModulation::from_u16,
    );
    check_u16::<TransmitterModulationTypeSystem>(TransmitterModulationTypeSystem::from_u16);
    check_u8::<TransmitterTransmitState>(TransmitterTransmitState::from_u8);
    check_u8::<TransmitterInputSource>(TransmitterInputSource::from_u8);
    check_u16::<TransmitterCryptoSystem>(TransmitterCryptoSystem::from_u16);
    check_u16::<TransmitterAntennaPatternType>(TransmitterAntennaPatternType::from_u16);
    check_u8::<TransmitterAntennaPatternReferenceSystem>(
        TransmitterAntennaPatternReferenceSystem::from_u8,
    );
    check_u8::<CCTTSINCGARSStartofMessage>(CCTTSINCGARSStartofMessage::from_u8);
    check_u8::<CCTTSINCGARSClearChannel>(CCTTSINCGARSClearChannel::from_u8);
    check_u8::<TimeSlotAllocationLevel>(TimeSlotAllocationLevel::from_u8);
    check_u8::<JTIDSMIDSModulationParametersTransmittingTerminalPrimaryMode>(
        JTIDSMIDSModulationParametersTransmittingTerminalPrimaryMode::from_u8,
    );
    check_u8::<JTIDSMIDSModulationParametersTransmittingTerminalSecondaryMode>(
        JTIDSMIDSModulationParametersTransmittingTerminalSecondaryMode::from_u8,
    );
    check_u8::<JTIDSMIDSModulationParametersSynchronizationState>(
        JTIDSMIDSModulationParametersSynchronizationState::from_u8,
    );
    check_u8::<MessageTypeIdentifier>(MessageTypeIdentifier::from_u8);
    check_u32::<SignalUserProtocolIdentificationNumber>(
        &[
            1, 5, 6, 20, 30, 31, 32, 35, 177, 200, 201, 202, 203, 546, 832, 1000, 1100, 1371, 4570,
            5361, 6000, 6010, 6020, 6661, 6662, 6663, 6664, 6665, 6666, 7000, 7010, 7020, 7030,
            7040, 7050,
        ],
        SignalUserProtocolIdentificationNumber::from_u32,
    );
    check_u16::<SignalTDLType>(SignalTDLType::from_u16);
    check_u16::<ReceiverReceiverState>(ReceiverReceiverState::from_u16);
    check_u8::<IntercomControlControlType>(IntercomControlControlType::from_u8);
    check_u8::<IntercomControlCommunicationsType>(IntercomControlCommunicationsType::from_u8);
    check_u8::<IntercomControlCommand>(IntercomControlCommand::from_u8);
    check_u8::<IntercomControlTransmitLineState>(IntercomControlTransmitLineState::from_u8);
    check_u8::<IntercomControlDestinationLineStateCommand>(
        IntercomControlDestinationLineStateCommand::from_u8,
    );
    check_u16::<IntercomControlRecordType>(IntercomControlRecordType::from_u16);
    check_u8::<CollisionType>(CollisionType::from_u8);
    check_u8::<MinefieldSensorTypes>(MinefieldSensorTypes::from_u8);
    check_u16::<MinefieldSensorTypesOptical>(MinefieldSensorTypesOptical::from_u16);
    check_u16::<MinefieldSensorTypesFLIR>(MinefieldSensorTypesFLIR::from_u16);
    check_u16::<MinefieldSensorTypesRADAR>(MinefieldSensorTypesRADAR::from_u16);
    check_u16::<MinefieldSensorTypesMagnetic>(MinefieldSensorTypesMagnetic::from_u16);
    check_u16::<MinefieldSensorTypesLaser>(MinefieldSensorTypesLaser::from_u16);
    check_u16::<MinefieldSensorTypesSONAR>(MinefieldSensorTypesSONAR::from_u16);
    check_u16::<MinefieldSensorTypesPhysical>(MinefieldSensorTypesPhysical::from_u16);
    check_u16::<MinefieldSensorTypesMultispectral>(MinefieldSensorTypesMultispectral::from_u16);
    check_u8::<AggregateStateAggregateState>(AggregateStateAggregateState::from_u8);
    check_u32::<AggregateStateFormation>(&[0, 1, 2, 3, 4, 5], AggregateStateFormation::from_u32);
    check_u8::<AggregateStateAggregateKind>(AggregateStateAggregateKind::from_u8);
    check_u8::<AggregateStateSubcategory>(AggregateStateSubcategory::from_u8);
    check_u8::<AggregateStateSpecific>(AggregateStateSpecific::from_u8);
    check_u16::<IsPartOfNature>(IsPartOfNature::from_u16);
    check_u16::<IsPartOfPosition>(IsPartOfPosition::from_u16);
    check_u16::<IsPartOfStationName>(IsPartOfStationName::from_u16);
    check_u8::<IsGroupOfGroupedEntityCategory>(IsGroupOfGroupedEntityCategory::from_u8);
    check_u8::<IsGroupOfRestStatus>(IsGroupOfRestStatus::from_u8);
    check_u8::<TransferControlTransferType>(TransferControlTransferType::from_u8);
    check_u8::<ObjectKind>(ObjectKind::from_u8);
    check_u16::<GriddedDataCoordinateSystem>(GriddedDataCoordinateSystem::from_u16);
    check_u8::<GriddedDataConstantGrid>(GriddedDataConstantGrid::from_u8);
    check_u8::<GriddedDataSampleType>(GriddedDataSampleType::from_u8);
    check_u8::<GriddedDataDataRepresentation>(GriddedDataDataRepresentation::from_u8);
    check_u8::<EnvironmentalProcessModelType>(EnvironmentalProcessModelType::from_u8);
    check_u32::<EnvironmentalProcessRecordType>(
        &[
            256, 259, 65536, 327680, 655360, 786432, 851968, 1048576, 3145728, 5242880, 83886080,
            167772160, 201326592, 218103808, 268435456, 805306368, 1342177280, 1610612736,
            1879048192,
        ],
        EnvironmentalProcessRecordType::from_u32,
    );
    check_u8::<SignalEncodingClass>(SignalEncodingClass::from_u8);
    check_u8::<SignalEncodingType>(SignalEncodingType::from_u8);
    check_u8::<RepairGroups>(RepairGroups::from_u8);
    check_u8::<EnvironmentRecordTypeGroups>(EnvironmentRecordTypeGroups::from_u8);
    check_u8::<PlatformAirCivilianUltralightNonrigidWingAircraftSubcategories>(
        PlatformAirCivilianUltralightNonrigidWingAircraftSubcategories::from_u8,
    );
    check_u8::<PlatformAirCivilianUltralightRigidWingAircraftSubcategories>(
        PlatformAirCivilianUltralightRigidWingAircraftSubcategories::from_u8,
    );
    check_u8::<PlatformAirCivilianGliderSubcategories>(
        PlatformAirCivilianGliderSubcategories::from_u8,
    );
    check_u8::<PlatformAirCivilianFixedWingAircraftSubcategories>(
        PlatformAirCivilianFixedWingAircraftSubcategories::from_u8,
    );
    check_u8::<PlatformAirCivilianHelicopterSubcategories>(
        PlatformAirCivilianHelicopterSubcategories::from_u8,
    );
    check_u8::<PlatformAirCivilianLighterthanAirBalloonSubcategories>(
        PlatformAirCivilianLighterthanAirBalloonSubcategories::from_u8,
    );
    check_u8::<PlatformAirCivilianLighterthanAirAirshipSubcategories>(
        PlatformAirCivilianLighterthanAirAirshipSubcategories::from_u8,
    );
    check_u8::<APAParameterIndexAPAStatus>(APAParameterIndexAPAStatus::from_u8);
    check_u8::<SeparationVPReasonforSeparation>(SeparationVPReasonforSeparation::from_u8);
    check_u8::<SeparationVPPreEntityIndicator>(SeparationVPPreEntityIndicator::from_u8);
    check_u16::<IOActionIOWarfareType>(IOActionIOWarfareType::from_u16);
    check_u16::<IOActionIOSimulationSource>(IOActionIOSimulationSource::from_u16);
    check_u16::<IOActionIOActionType>(IOActionIOActionType::from_u16);
    check_u16::<IOActionIOActionPhase>(IOActionIOActionPhase::from_u16);
    check_u8::<IOReportIOReportType>(IOReportIOReportType::from_u8);
    check_u8::<IOEffectsRecordIOStatus>(IOEffectsRecordIOStatus::from_u8);
    check_u8::<IOEffectsRecordIOLinkType>(IOEffectsRecordIOLinkType::from_u8);
    check_u8::<IOEffectsRecordIOEffect>(IOEffectsRecordIOEffect::from_u8);
    check_u8::<IOEffectsRecordIOProcess>(IOEffectsRecordIOProcess::from_u8);
    check_u8::<IOCommsNodeRecordCommsNodeType>(IOCommsNodeRecordCommsNodeType::from_u8);
    check_u8::<DISAttributeActionCode>(DISAttributeActionCode::from_u8);
    check_u8::<DRParametersType>(DRParametersType::from_u8);
    check_u8::<HighFidelityHAVEQUICKTODTransmitIndicator>(
        HighFidelityHAVEQUICKTODTransmitIndicator::from_u8,
    );
    check_u8::<NETIDRecordMode>(NETIDRecordMode::from_u8);
    check_u8::<NETIDRecordFrequencyTable>(NETIDRecordFrequencyTable::from_u8);
    check_u8::<EEAttributeStateIndicator>(EEAttributeStateIndicator::from_u8);
    check_u8::<ExplosiveMaterialGroups>(ExplosiveMaterialGroups::from_u8);
    check_u16::<ExplosiveMaterialCategories>(ExplosiveMaterialCategories::from_u16);
    check_u8::<DEPrecisionAimpointBeamSpotType>(DEPrecisionAimpointBeamSpotType::from_u8);
    check_u8::<DEFirePulseShape>(DEFirePulseShape::from_u8);
    check_u8::<ComponentIdentification>(ComponentIdentification::from_u8);
    check_u8::<ComponentDamageStatus>(ComponentDamageStatus::from_u8);
    check_u8::<ComponentVisualSmokeColor>(ComponentVisualSmokeColor::from_u8);
    check_u8::<BeamStatusBeamState>(BeamStatusBeamState::from_u8);
    check_u8::<EntityAssociationAssociationStatus>(EntityAssociationAssociationStatus::from_u8);
    check_u8::<EntityVPRecordChangeIndicator>(EntityVPRecordChangeIndicator::from_u8);
    check_u8::<EntityAssociationGroupMemberType>(EntityAssociationGroupMemberType::from_u8);
    check_u8::<PhysicalAssociationTypeGroups>(PhysicalAssociationTypeGroups::from_u8);
    check_u8::<EntityAssociationPhysicalAssociationType>(
        EntityAssociationPhysicalAssociationType::from_u8,
    );
    check_u8::<EntityAssociationPhysicalConnectionType>(
        EntityAssociationPhysicalConnectionType::from_u8,
    );
    check_u8::<SensorRecordSensorTypeOtherActiveSensors>(
        SensorRecordSensorTypeOtherActiveSensors::from_u8,
    );
    check_u16::<SensorRecordSensorTypePassiveSensors>(
        SensorRecordSensorTypePassiveSensors::from_u16,
    );
    check_u8::<MunitionExpendableStatus>(MunitionExpendableStatus::from_u8);
    check_u8::<FuelMeasurementUnits>(FuelMeasurementUnits::from_u8);
    check_u8::<FuelLocation>(FuelLocation::from_u8);
    check_u8::<EntityAssociationAssociationType>(EntityAssociationAssociationType::from_u8);
    check_u8::<SensorOnOffStatus>(SensorOnOffStatus::from_u8);
    check_u8::<OwnershipStatus>(OwnershipStatus::from_u8);
    check_u16::<RecordREventType>(RecordREventType::from_u16);
    check_u16::<RecordQueryREventType>(RecordQueryREventType::from_u16);
    check_u8::<UAPropulsionPlantConfigurationConfiguration>(
        UAPropulsionPlantConfigurationConfiguration::from_u8,
    );
    check_u16::<MinefieldStateProtocolMode>(MinefieldStateProtocolMode::from_u16);
    check_u8::<TransponderInterrogatorIndicator>(TransponderInterrogatorIndicator::from_u8);
    check_u8::<IFFSimulationMode>(IFFSimulationMode::from_u8);
    check_u8::<IFFApplicableModes>(IFFApplicableModes::from_u8);
    check_u8::<ModeCAltitudeIndicator>(ModeCAltitudeIndicator::from_u8);
    check_u8::<TCASACASBasicAdvancedIndicator>(TCASACASBasicAdvancedIndicator::from_u8);
    check_u8::<TCASACASIndicator>(TCASACASIndicator::from_u8);
    check_u8::<TCASACASSoftwareVersion>(TCASACASSoftwareVersion::from_u8);
    check_u8::<TCASACASType>(TCASACASType::from_u8);
    check_u8::<TCASIIIType>(TCASIIIType::from_u8);
    check_u8::<Mode5IFFMission>(Mode5IFFMission::from_u8);
    check_u8::<ModeSInterrogatorStatusTransmitState>(ModeSInterrogatorStatusTransmitState::from_u8);
    check_u8::<ModeSInterrogatorIdentifierICType>(ModeSInterrogatorIdentifierICType::from_u8);
    check_u8::<ISLSAntennaType>(ISLSAntennaType::from_u8);
    check_u8::<Mode5Reply>(Mode5Reply::from_u8);
    check_u8::<AntennaSelection>(AntennaSelection::from_u8);
    check_u8::<Mode5SquitterType>(Mode5SquitterType::from_u8);
    check_u8::<Level2SquitterStatus>(Level2SquitterStatus::from_u8);
    check_u8::<ModeSSquitterType>(ModeSSquitterType::from_u8);
    check_u8::<ModeSSquitterRecordSource>(ModeSSquitterRecordSource::from_u8);
    check_u8::<AircraftPresentDomain>(AircraftPresentDomain::from_u8);
    check_u8::<AircraftIdentificationType>(AircraftIdentificationType::from_u8);
    check_u8::<CapabilityReport>(CapabilityReport::from_u8);
    check_u8::<NavigationSource>(NavigationSource::from_u8);
    check_u8::<IFFDataRecordAvailable>(IFFDataRecordAvailable::from_u8);
    check_u8::<Mode5SAltitudeResolution>(Mode5SAltitudeResolution::from_u8);
    check_u8::<DeltaMode5SAltitudePositiveNegativeIndicator>(
        DeltaMode5SAltitudePositiveNegativeIndicator::from_u8,
    );
    check_u8::<FormatType>(FormatType::from_u8);
    check_u8::<AircraftAddressSource>(AircraftAddressSource::from_u8);
    check_u8::<SurveillanceStatus>(SurveillanceStatus::from_u8);
    check_u8::<TurnRateSource>(TurnRateSource::from_u8);
    check_u8::<TimeTypeSource>(TimeTypeSource::from_u8);
    check_u8::<AircraftTypeWake>(AircraftTypeWake::from_u8);
    check_u8::<DataCategory>(DataCategory::from_u8);
    check_u8::<TILinkType>(TILinkType::from_u8);
    check_u8::<AntennaStatus>(AntennaStatus::from_u8);
    check_u8::<TransmissionIndicator>(TransmissionIndicator::from_u8);
    check_u8::<ReplyAmplification>(ReplyAmplification::from_u8);
    check_u8::<DEFireFlagsStateUpdateFlag>(DEFireFlagsStateUpdateFlag::from_u8);
    check_u8::<ComponentVisualDamageStatusSmoke>(ComponentVisualDamageStatusSmoke::from_u8);
    check_u8::<ComponentVisualDamageStatusSurfaceDamage>(
        ComponentVisualDamageStatusSurfaceDamage::from_u8,
    );
    check_u8::<GridAxisDescriptorAxisType>(GridAxisDescriptorAxisType::from_u8);
    check_u8::<AppearancePaintScheme>(AppearancePaintScheme::from_u8);
    check_u8::<AppearanceDamage>(AppearanceDamage::from_u8);
    check_u8::<Mode5MessageFormatsStatus>(Mode5MessageFormatsStatus::from_u8);
    check_u8::<AppearanceTrailingEffects>(AppearanceTrailingEffects::from_u8);
    check_u8::<AppearanceHatch>(AppearanceHatch::from_u8);
    check_u8::<AppearanceLauncherOperational>(AppearanceLauncherOperational::from_u8);
    check_u8::<AppearanceCamouflageType>(AppearanceCamouflageType::from_u8);
    check_u8::<AppearanceConcealedPosition>(AppearanceConcealedPosition::from_u8);
    check_u8::<AppearanceEntityorObjectState>(AppearanceEntityorObjectState::from_u8);
    check_u8::<AppearanceCanopy>(AppearanceCanopy::from_u8);
    check_u8::<AppearanceSubsurfaceHatch>(AppearanceSubsurfaceHatch::from_u8);
    check_u8::<AppearanceLifeformHealth>(AppearanceLifeformHealth::from_u8);
    check_u8::<AppearanceLifeFormComplianceStatus>(AppearanceLifeFormComplianceStatus::from_u8);
    check_u8::<AppearanceLifeFormPosture>(AppearanceLifeFormPosture::from_u8);
    check_u8::<AppearanceLifeFormWeaponImplement>(AppearanceLifeFormWeaponImplement::from_u8);
    check_u8::<AppearanceConcealedMovement>(AppearanceConcealedMovement::from_u8);
    check_u8::<AppearanceEnvironmentalDensity>(AppearanceEnvironmentalDensity::from_u8);
    check_u8::<Mode5PlatformType>(Mode5PlatformType::from_u8);
    check_u8::<AppearanceAntiCollisionDayNight>(AppearanceAntiCollisionDayNight::from_u8);
    check_u8::<AppearanceNavigationPositionBrightness>(
        AppearanceNavigationPositionBrightness::from_u8,
    );
    check_u8::<AppearanceSupplyDeployed>(AppearanceSupplyDeployed::from_u8);
    check_u8::<AppearanceNVGMode>(AppearanceNVGMode::from_u8);
    check_u8::<Parachute>(Parachute::from_u8);
    check_u8::<FlareSmokeColor>(FlareSmokeColor::from_u8);
    check_u8::<FlareSmokeStatus>(FlareSmokeStatus::from_u8);
    check_u8::<SpotChaffStatus>(SpotChaffStatus::from_u8);
    check_u8::<AppearanceObjectGeneralDamage>(AppearanceObjectGeneralDamage::from_u8);
    check_u8::<AppearanceObjectGeneralPredistributed>(
        AppearanceObjectGeneralPredistributed::from_u8,
    );
    check_u8::<AppearanceObjectSpecificBreachState>(AppearanceObjectSpecificBreachState::from_u8);
    check_u8::<AppearanceObjectSpecificChemicalType>(AppearanceObjectSpecificChemicalType::from_u8);
    check_u8::<AppearanceLinearObjectTankDitchBreach>(
        AppearanceLinearObjectTankDitchBreach::from_u8,
    );
    check_u8::<AppearanceLinearObjectLaneMarkerVisible>(
        AppearanceLinearObjectLaneMarkerVisible::from_u8,
    );
    check_u8::<AppearanceObjectGeneralIEDPresent>(AppearanceObjectGeneralIEDPresent::from_u8);
    check_u8::<Mode5LevelSelection>(Mode5LevelSelection::from_u8);
    check_u8::<SupplyFuelType>(SupplyFuelType::from_u8);
    check_u8::<SensorTypeSource>(SensorTypeSource::from_u8);
    check_u8::<AttachedPartDetachedIndicator>(AttachedPartDetachedIndicator::from_u8);
    check_u8::<IntercomControlCommunicationsClass>(IntercomControlCommunicationsClass::from_u8);
    check_u8::<DISLiveEntitySubprotocolNumber>(DISLiveEntitySubprotocolNumber::from_u8);
    check_u8::<MinefieldAppearanceMinefieldType>(MinefieldAppearanceMinefieldType::from_u8);
    check_u8::<MinefieldAppearanceActiveStatus>(MinefieldAppearanceActiveStatus::from_u8);
    check_u8::<MinefieldAppearanceLane>(MinefieldAppearanceLane::from_u8);
    check_u8::<MinefieldAppearanceState>(MinefieldAppearanceState::from_u8);
    check_u8::<MinefieldFusingFuseType>(MinefieldFusingFuseType::from_u8);
    check_u8::<Mode5LocationErrors>(Mode5LocationErrors::from_u8);
    check_u8::<MinefieldPaintSchemeAlgae>(MinefieldPaintSchemeAlgae::from_u8);
    check_u8::<MinefieldPaintSchemePaintScheme>(MinefieldPaintSchemePaintScheme::from_u8);
    check_u8::<CoverShroudStatus>(CoverShroudStatus::from_u8);
    check_u8::<PlatformLandMotorcycleSubcategories>(PlatformLandMotorcycleSubcategories::from_u8);
    check_u8::<PlatformLandCarSubcategories>(PlatformLandCarSubcategories::from_u8);
    check_u8::<PlatformLandBusSubcategories>(PlatformLandBusSubcategories::from_u8);
    check_u8::<PlatformLandSingleUnitCargoTruckSubcategories>(
        PlatformLandSingleUnitCargoTruckSubcategories::from_u8,
    );
    check_u8::<PlatformLandSingleUnitUtilityEmergencyTruckSubcategories>(
        PlatformLandSingleUnitUtilityEmergencyTruckSubcategories::from_u8,
    );
    check_u8::<PlatformLandMultipleUnitCargoTruckSubcategories>(
        PlatformLandMultipleUnitCargoTruckSubcategories::from_u8,
    );
    check_u8::<PlatformLandMultipleUnitUtilityEmergencyTruckSubcategories>(
        PlatformLandMultipleUnitUtilityEmergencyTruckSubcategories::from_u8,
    );
    check_u8::<PlatformLandConstructionSpecialtyVehicleSubcategories>(
        PlatformLandConstructionSpecialtyVehicleSubcategories::from_u8,
    );
    check_u8::<PlatformLandFarmSpecialtyVehicleSubcategories>(
        PlatformLandFarmSpecialtyVehicleSubcategories::from_u8,
    );
    check_u8::<PlatformLandTrailerSubcategories>(PlatformLandTrailerSubcategories::from_u8);
    check_u8::<PlatformLandRecreationalSubcategories>(
        PlatformLandRecreationalSubcategories::from_u8,
    );
    check_u8::<PlatformLandNonmotorizedSubcategories>(
        PlatformLandNonmotorizedSubcategories::from_u8,
    );
    check_u8::<PlatformLandTrainsSubcategories>(PlatformLandTrainsSubcategories::from_u8);
    check_u8::<PlatformLandUtilityEmergencyCarSubcategories>(
        PlatformLandUtilityEmergencyCarSubcategories::from_u8,
    );
    check_u8::<PlatformSurfacePassengerVesselSubcategories>(
        PlatformSurfacePassengerVesselSubcategories::from_u8,
    );
    check_u8::<PlatformSurfaceDryCargoShipSubcategories>(
        PlatformSurfaceDryCargoShipSubcategories::from_u8,
    );
    check_u8::<PlatformSurfaceTankerSubcategories>(PlatformSurfaceTankerSubcategories::from_u8);
    check_u8::<PlatformSurfaceSupportVesselSubcategories>(
        PlatformSurfaceSupportVesselSubcategories::from_u8,
    );
    check_u8::<PlatformSurfacePrivateMotorboatSubcategories>(
        PlatformSurfacePrivateMotorboatSubcategories::from_u8,
    );
    check_u8::<PlatformSurfacePrivateSailboatSubcategories>(
        PlatformSurfacePrivateSailboatSubcategories::from_u8,
    );
    check_u8::<PlatformSurfaceFishingVesselSubcategories>(
        PlatformSurfaceFishingVesselSubcategories::from_u8,
    );
    check_u8::<PlatformSurfaceOtherVesselsSubcategories>(
        PlatformSurfaceOtherVesselsSubcategories::from_u8,
    );
    check_u8::<CryptoKeyIDCryptoMode>(CryptoKeyIDCryptoMode::from_u8);
    check_u8::<Color>(Color::from_u8);
    check_u8::<BuildingPaintScheme>(BuildingPaintScheme::from_u8);
    check_u8::<Season>(Season::from_u8);
    check_u8::<Material>(Material::from_u8);
    check_u8::<Link11_11BFidelityLevel>(Link11_11BFidelityLevel::from_u8);
    check_u8::<Link11TerminalMode>(Link11TerminalMode::from_u8);
    check_u8::<Link11DataTerminalSetIndicator>(Link11DataTerminalSetIndicator::from_u8);
    check_u16::<Link11ModeofOperation>(Link11ModeofOperation::from_u16);
    check_u8::<LifeFormsSubcategoryIranianWeapons>(LifeFormsSubcategoryIranianWeapons::from_u8);
    check_u8::<LifeFormLandCategories>(LifeFormLandCategories::from_u8);
    check_u8::<LifeFormHumanSubcategoryEquipmentClass>(
        LifeFormHumanSubcategoryEquipmentClass::from_u8,
    );
    check_u8::<LifeFormHumanSpecificAssaultRifles>(LifeFormHumanSpecificAssaultRifles::from_u8);
    check_u8::<LifeFormHumanSpecificHighPowerRifles>(LifeFormHumanSpecificHighPowerRifles::from_u8);
    check_u8::<LifeFormCategoriesUS>(LifeFormCategoriesUS::from_u8);
    check_u8::<LifeFormExtraPersonalData>(LifeFormExtraPersonalData::from_u8);
    check_u8::<LifeFormAirCategories>(LifeFormAirCategories::from_u8);
    check_u8::<LifeFormSubsurfaceCategories>(LifeFormSubsurfaceCategories::from_u8);
    check_u8::<LifeFormHumanSpecificSniper>(LifeFormHumanSpecificSniper::from_u8);
    check_u8::<LifeFormHumanSpecificSubMachineGun>(LifeFormHumanSpecificSubMachineGun::from_u8);
    check_u8::<AustralianCategoryOverlay>(AustralianCategoryOverlay::from_u8);
    check_u8::<LifeFormCategoriesAfghanistan>(LifeFormCategoriesAfghanistan::from_u8);
    check_u8::<LifeFormHumanSpecificEquipmentClass>(LifeFormHumanSpecificEquipmentClass::from_u8);
    check_u8::<PlatformSubsurfaceCivilianSemiSubmersiblesSubcategories>(
        PlatformSubsurfaceCivilianSemiSubmersiblesSubcategories::from_u8,
    );
    check_u8::<LeafCoverage>(LeafCoverage::from_u8);
    check_u8::<LifeFormHumanSpecificAntiMaterielRifles>(
        LifeFormHumanSpecificAntiMaterielRifles::from_u8,
    );
    check_u8::<LifeFormHumanSpecificShotGuns>(LifeFormHumanSpecificShotGuns::from_u8);
    check_u8::<LifeFormHumanSpecificMortars>(LifeFormHumanSpecificMortars::from_u8);
    check_u8::<LifeFormHumanSpecificHandGuns>(LifeFormHumanSpecificHandGuns::from_u8);
    check_u8::<LifeFormHumanSpecificWeaponNonspecific>(
        LifeFormHumanSpecificWeaponNonspecific::from_u8,
    );
    check_u8::<LifeFormHumanSpecificGrenadeLaunchers>(
        LifeFormHumanSpecificGrenadeLaunchers::from_u8,
    );
    check_u8::<LifeFormHumanSpecificMachineGuns>(LifeFormHumanSpecificMachineGuns::from_u8);
    check_u8::<LifeFormHumanSpecificGrenadeLaunchingMachineGun>(
        LifeFormHumanSpecificGrenadeLaunchingMachineGun::from_u8,
    );
    check_u8::<LifeFormHumanSpecificAntiTankRockets>(LifeFormHumanSpecificAntiTankRockets::from_u8);
    check_u8::<LifeFormHumanSpecificAntiTankMissiles>(
        LifeFormHumanSpecificAntiTankMissiles::from_u8,
    );
    check_u8::<LifeFormHumanSpecificManPortableAirDefenseSystem>(
        LifeFormHumanSpecificManPortableAirDefenseSystem::from_u8,
    );
    check_u8::<LifeFormHumanSpecificRecoillessRifles>(
        LifeFormHumanSpecificRecoillessRifles::from_u8,
    );
    check_u8::<LifeFormHumanSpecificFlameRockets>(LifeFormHumanSpecificFlameRockets::from_u8);
    check_u8::<LifeFormHumanSpecificFlameThrowers>(LifeFormHumanSpecificFlameThrowers::from_u8);
    check_u8::<LifeFormHumanSpecificDroneGuns>(LifeFormHumanSpecificDroneGuns::from_u8);
    check_u8::<LifeFormHumanSpecificLogisticsEQClass>(
        LifeFormHumanSpecificLogisticsEQClass::from_u8,
    );
    check_u8::<LifeFormHumanSpecificPersonalElectronicsClass>(
        LifeFormHumanSpecificPersonalElectronicsClass::from_u8,
    );
    check_u8::<LifeFormHumanSpecificLasersClass>(LifeFormHumanSpecificLasersClass::from_u8);
    check_u16::<TransmitterDetailSATCOMModulation>(TransmitterDetailSATCOMModulation::from_u16);
    check_u8::<SupplyDomain>(SupplyDomain::from_u8);
    check_u8::<Class1SupplyCategorySubsistence>(Class1SupplyCategorySubsistence::from_u8);
    check_u8::<Class2SupplyCategoryClothingIndividualEquipmentToolsAdminSupplies>(
        Class2SupplyCategoryClothingIndividualEquipmentToolsAdminSupplies::from_u8,
    );
    check_u8::<Class3SupplyCategoryPetroleumOilsLubricants>(
        Class3SupplyCategoryPetroleumOilsLubricants::from_u8,
    );
    check_u8::<Class4SupplyCategoryConstructionMaterials>(
        Class4SupplyCategoryConstructionMaterials::from_u8,
    );
    check_u8::<Class6SupplyCategoryPersonnelDemandItems>(
        Class6SupplyCategoryPersonnelDemandItems::from_u8,
    );
    check_u8::<Class7SupplyCategoryMajorItems>(Class7SupplyCategoryMajorItems::from_u8);
    check_u8::<Class8SupplyCategoryMedicalMaterial>(Class8SupplyCategoryMedicalMaterial::from_u8);
    check_u8::<Class9SupplyCategoryRepairPartsandComponents>(
        Class9SupplyCategoryRepairPartsandComponents::from_u8,
    );
    check_u8::<Class10SupplyCategoryMaterialToSupportNonMilitaryPrograms>(
        Class10SupplyCategoryMaterialToSupportNonMilitaryPrograms::from_u8,
    );
    check_u8::<Class11SupplyCategorySupplies>(Class11SupplyCategorySupplies::from_u8);
    check_u8::<Class12SupplyCategorySlingLoads>(Class12SupplyCategorySlingLoads::from_u8);
    check_u8::<LifeSavingEquipment>(LifeSavingEquipment::from_u8);
    check_u16::<IslandSubcategory>(IslandSubcategory::from_u16);
    check_u8::<Link11MessageSubType>(Link11MessageSubType::from_u8);
    check_u8::<Link11MessageTypeIdentifier>(Link11MessageTypeIdentifier::from_u8);
    check_u8::<Link11DataSignallingRate>(Link11DataSignallingRate::from_u8);
    check_u8::<Link11SignalIntegrationInterval>(Link11SignalIntegrationInterval::from_u8);
    check_u8::<Link11SignalWaveform>(Link11SignalWaveform::from_u8);
    check_u8::<Link11_11BEncryptionFlag>(Link11_11BEncryptionFlag::from_u8);
    check_u8::<SISOSTD002Version>(SISOSTD002Version::from_u8);
    check_u8::<Link11BLinkState>(Link11BLinkState::from_u8);
    check_u16::<Link11BModeofOperation>(Link11BModeofOperation::from_u16);
    check_u8::<Link11BMessageSubType>(Link11BMessageSubType::from_u8);
    check_u8::<Link11BDataSignalingRate>(Link11BDataSignalingRate::from_u8);
    check_u8::<Link11BModulationStandard>(Link11BModulationStandard::from_u8);
    check_u16::<CIGIExtensionPacketID>(CIGIExtensionPacketID::from_u16);
    check_u8::<Link16Version>(Link16Version::from_u8);
    check_u8::<AircraftIDSource>(AircraftIDSource::from_u8);
    check_u8::<ClothingIRSignature>(ClothingIRSignature::from_u8);
    check_u8::<DamageArea>(DamageArea::from_u8);
}

#[test]
fn undefined_value_is_a_wire_format_error() {
    // PduType defines 0..=72, so 200 is undefined.
    let err = decode::<PduType>(&[200]).unwrap_err();
    let err = crate::common::DISError::from(err);
    assert!(matches!(err, crate::common::DISError::WireFormat(_)));
    assert!(
        err.to_string()
            .contains("200 is not a defined PduType value"),
        "unexpected message: {err}"
    );
}

#[test]
fn multi_byte_enums_are_big_endian() {
    // EmitterName is a 16-bit enum; its default variant is 2.
    assert_eq!(encode(&EmitterName::default()), vec![0x00, 0x02]);
}

#[test]
fn truncated_input_is_an_error() {
    assert!(decode::<EmitterName>(&[0x00]).is_err());
    assert!(decode::<ForceId>(&[]).is_err());
}
