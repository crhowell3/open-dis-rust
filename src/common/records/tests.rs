//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use super::*;

/// 3600 s / 2³¹, rounded up.
const ONE_TICK: Duration = Duration::from_nanos(1677);

#[test]
fn absolute_sets_bit_zero_and_relative_clears_it() {
    assert!(Timestamp::absolute(Duration::from_secs(10)).is_absolute());
    assert!(!Timestamp::relative(Duration::from_secs(10)).is_absolute());
    assert_eq!(Timestamp::relative(Duration::ZERO).to_raw(), 0);
    assert_eq!(Timestamp::absolute(Duration::ZERO).to_raw(), 1);
}

#[test]
fn half_an_hour_is_half_the_tick_range() {
    let half = Timestamp::relative(Duration::from_secs(1800));
    assert_eq!(half.to_raw(), (1u32 << 30) << 1);
    assert_eq!(half.time_past_hour(), Duration::from_secs(1800));
}

#[test]
fn whole_hours_are_discarded() {
    let a = Timestamp::absolute(Duration::from_secs(5 * 3600 + 90));
    assert_eq!(a, Timestamp::absolute(Duration::from_secs(90)));
    assert!(a.time_past_hour().abs_diff(Duration::from_secs(90)) < ONE_TICK);
}

#[test]
fn a_time_that_rounds_up_to_the_hour_wraps_to_zero() {
    let just_before = Duration::from_secs(3600) - Duration::from_nanos(1);
    assert_eq!(Timestamp::relative(just_before).to_raw(), 0);
}

#[test]
fn largest_value_is_just_under_an_hour() {
    let last = Timestamp::from_raw(u32::MAX);
    assert!(last.is_absolute());
    assert!(last.time_past_hour() < Duration::from_secs(3600));
    assert!(last.time_past_hour() > Duration::from_secs(3599));
}

#[test]
fn round_trips_to_within_one_tick() {
    for micros in [0, 1, 999_999, 1_234_567_891, 3_599_999_999] {
        let time = Duration::from_micros(micros);
        let back = Timestamp::relative(time).time_past_hour();
        // Rounding to the nearest tick is off by at most half a tick.
        assert!(
            time.abs_diff(back) <= ONE_TICK / 2,
            "{time:?} came back as {back:?}"
        );
    }
}

#[test]
fn duration_since_wraps_at_the_hour() {
    let late = Timestamp::absolute(Duration::from_secs(3599));
    let early = Timestamp::absolute(Duration::from_secs(1));
    assert!(early.duration_since(late).abs_diff(Duration::from_secs(2)) < ONE_TICK);
    assert_eq!(late.duration_since(late), Duration::ZERO);
    // `earlier` two seconds in the future comes out as just under an hour.
    assert!(
        late.duration_since(early)
            .abs_diff(Duration::from_secs(3598))
            < ONE_TICK
    );
}

#[test]
fn now_is_absolute_and_within_the_hour() {
    let now = Timestamp::now();
    assert!(now.is_absolute());
    assert!(now.time_past_hour() < Duration::from_secs(3600));
}

#[test]
fn display_shows_minutes_seconds_and_kind() {
    // 30 minutes is exactly 2³⁰ ticks.
    let t = Timestamp::relative(Duration::from_secs(1800));
    assert_eq!(t.to_string(), "30:00.000000 rel");
    // Most times aren't a whole number of ticks, so the microseconds can be off by one.
    let t = Timestamp::absolute(Duration::from_millis(35 * 60_000 + 26_500)).to_string();
    assert!(t.starts_with("35:26.") && t.ends_with(" abs"), "{t}");
}

#[test]
fn encodes_as_its_raw_value() {
    let mut buf = Vec::new();
    crate::common::wire::write(&Timestamp::from_raw(0x0102_0304), &mut buf).unwrap();
    assert_eq!(buf, [1, 2, 3, 4]);
}

#[test]
fn now_counts_hours_since_the_epoch() {
    let now = ClockTime::now();
    // November 2026 is about 498,000 hours after 1970, so any current time is past this
    assert!(now.hour > 495_000, "{now:?}");
    assert!(now.time_past_hour.is_absolute());
}

mod jamming_techniques {
    use super::*;

    /// Every constant, with the code SISO-REF-010 gives it.
    const TABLE: &[(JammingTechnique, [u8; 4])] = &[
        (JammingTechnique::NOISE, [1, 0, 0, 0]),
        (JammingTechnique::AMPLITUDE_MODULATION_NOISE, [1, 5, 0, 0]),
        (JammingTechnique::BARRAGE_NOISE, [1, 10, 0, 0]),
        (JammingTechnique::CLICK, [1, 10, 5, 0]),
        (JammingTechnique::SOURCE_NOISE, [1, 10, 10, 0]),
        (JammingTechnique::BISTATIC_CLUTTER, [1, 15, 0, 0]),
        (JammingTechnique::COMB, [1, 20, 0, 0]),
        (JammingTechnique::COOPERATIVE_BLINKED_NOISE, [1, 25, 0, 0]),
        (JammingTechnique::DOPPLER_NOISE, [1, 30, 0, 0]),
        (
            JammingTechnique::FREQUENCY_MODULATION_BY_NOISE,
            [1, 35, 0, 0],
        ),
        (JammingTechnique::IMPULSE_NOISE, [1, 40, 0, 0]),
        (JammingTechnique::PARTIAL_BAND, [1, 45, 0, 0]),
        (JammingTechnique::PSEUDORANDOM_AM, [1, 50, 0, 0]),
        (JammingTechnique::PULSE_NOISE, [1, 55, 0, 0]),
        (JammingTechnique::QUASI_NOISE, [1, 60, 0, 0]),
        (JammingTechnique::RANGE_BIN_MASKING, [1, 65, 0, 0]),
        (
            JammingTechnique::RANGE_BIN_MASKING_WITH_VELOCITY_BIN_MASKING,
            [1, 65, 5, 0],
        ),
        (JammingTechnique::NOISE_REPEATER_NOISE, [1, 70, 0, 0]),
        (JammingTechnique::NARROWBAND_REPEATER_NOISE, [1, 70, 5, 0]),
        (JammingTechnique::WIDE_BAND_REPEATER_NOISE, [1, 70, 10, 0]),
        (JammingTechnique::SPOT_NOISE, [1, 75, 0, 0]),
        (JammingTechnique::AUTOMATIC_SPOT_NOISE, [1, 75, 5, 0]),
        (JammingTechnique::BLINKING_SPOT_NOISE, [1, 75, 10, 0]),
        (JammingTechnique::BURST_SPOT_NOISE, [1, 75, 15, 0]),
        (JammingTechnique::DOPPLER_SPOT_NOISE, [1, 75, 20, 0]),
        (JammingTechnique::SKIRT_FREQUENCY, [1, 75, 25, 0]),
        (JammingTechnique::SWEPT_NOISE, [1, 80, 0, 0]),
        (JammingTechnique::FREQUENCY_SWEPT, [1, 80, 5, 0]),
        (JammingTechnique::SWEPT_AM, [1, 80, 10, 0]),
        (JammingTechnique::VELOCITY_BIN_MASKING, [1, 85, 0, 0]),
        (JammingTechnique::DECEPTION, [2, 0, 0, 0]),
        (JammingTechnique::ANALYZER, [2, 5, 0, 0]),
        (JammingTechnique::ANGLE, [2, 10, 0, 0]),
        (JammingTechnique::ANGLE_GATE_WALK_OFF, [2, 10, 5, 0]),
        (JammingTechnique::COOPERATIVE_ANGLE, [2, 10, 10, 0]),
        (JammingTechnique::CROSS_EYE, [2, 10, 15, 0]),
        (JammingTechnique::CROSS_POLARIZATION, [2, 10, 20, 0]),
        (JammingTechnique::DELTA, [2, 10, 25, 0]),
        (JammingTechnique::INVERSE_GAIN, [2, 10, 30, 0]),
        (JammingTechnique::SEA_BOUNCED, [2, 10, 35, 0]),
        (JammingTechnique::SWEPT_SQUARE_WAVE, [2, 10, 40, 0]),
        (JammingTechnique::TERRAIN_BOUNCE, [2, 10, 45, 0]),
        (JammingTechnique::ANGLE_AND_GATE_STEALER, [2, 15, 0, 0]),
        (
            JammingTechnique::CROSS_POLARIZATION_AND_RANGE_GATE_PULL_OFF,
            [2, 15, 5, 0],
        ),
        (
            JammingTechnique::CROSS_POLARIZATION_AND_VELOCITY_GATE_PULL_OFF,
            [2, 15, 10, 0],
        ),
        (
            JammingTechnique::CROSS_POLARIZATION_RGPO_AND_VGPO,
            [2, 15, 15, 0],
        ),
        (JammingTechnique::INVERSE_GAIN_AND_RGPO, [2, 15, 20, 0]),
        (
            JammingTechnique::INVERSE_GAIN_AND_RGPO_AND_VGPO,
            [2, 15, 25, 0],
        ),
        (JammingTechnique::INVERSE_GAIN_AND_VGPO, [2, 15, 30, 0]),
        (JammingTechnique::RGPO_AND_SSW, [2, 15, 35, 0]),
        (JammingTechnique::SSW_AND_VGPO, [2, 15, 40, 0]),
        (JammingTechnique::ANGLE_AND_FALSE_TARGET, [2, 20, 0, 0]),
        (
            JammingTechnique::ANGLE_AND_VELOCITY_FALSE_TARGETS,
            [2, 20, 5, 0],
        ),
        (JammingTechnique::INVERSE_GAIN_AND_VFT, [2, 20, 5, 5]),
        (JammingTechnique::SSW_AND_VFT, [2, 20, 5, 10]),
        (
            JammingTechnique::RANGE_FALSE_TARGETS_AND_INVERSE_GAIN,
            [2, 20, 10, 0],
        ),
        (JammingTechnique::RFT_AND_SSW, [2, 20, 15, 0]),
        (
            JammingTechnique::ANGLE_AND_RANDOM_RANGE_PROGRAMS,
            [2, 25, 0, 0],
        ),
        (JammingTechnique::RANRAP_AND_SSW, [2, 25, 5, 0]),
        (JammingTechnique::ANGLE_AND_VELOCITY, [2, 30, 0, 0]),
        (JammingTechnique::INVERSE_GAIN_AND_VBM, [2, 30, 5, 0]),
        (JammingTechnique::SSW_AND_VBM, [2, 30, 10, 0]),
        (JammingTechnique::AUTOMATIC_GAIN_CONTROL, [2, 35, 0, 0]),
        (JammingTechnique::AGC_AND_GATE_STEALER, [2, 40, 0, 0]),
        (JammingTechnique::AGC_AND_RGPO, [2, 40, 5, 0]),
        (JammingTechnique::AGC_AND_VGPO, [2, 40, 10, 0]),
        (JammingTechnique::AGC_AND_RGPO_AND_VGPO, [2, 40, 15, 0]),
        (JammingTechnique::COLINEAR, [2, 45, 0, 0]),
        (JammingTechnique::CONSTANT_FALSE_ALARM_RATE, [2, 50, 0, 0]),
        (JammingTechnique::DOUBLE_CROSS, [2, 55, 0, 0]),
        (JammingTechnique::DOWN_LINK, [2, 60, 0, 0]),
        (JammingTechnique::FALSE_TARGET, [2, 65, 0, 0]),
        (JammingTechnique::COHERENT_FALSE_TARGETS, [2, 65, 5, 0]),
        (JammingTechnique::FALSE_DOPPLER_TARGET, [2, 65, 10, 0]),
        (JammingTechnique::MULTIPLE_FALSE_TARGETS, [2, 65, 15, 0]),
        (JammingTechnique::RANGE_FALSE_TARGETS, [2, 65, 20, 0]),
        (JammingTechnique::TRANSPONDER, [2, 65, 25, 0]),
        (JammingTechnique::VELOCITY_FALSE_TARGETS, [2, 65, 30, 0]),
        (JammingTechnique::FIGURE_EIGHT, [2, 70, 0, 0]),
        (JammingTechnique::GATE_STEALER, [2, 75, 0, 0]),
        (JammingTechnique::CHIRP_GATE_STEALER, [2, 75, 5, 0]),
        (JammingTechnique::RANGE_GATE_PULL_OFF, [2, 75, 10, 0]),
        (JammingTechnique::RGPO_AND_VGPO, [2, 75, 15, 0]),
        (JammingTechnique::VGPO, [2, 75, 20, 0]),
        (JammingTechnique::GATE_STEALER_AND_REPEATER, [2, 80, 0, 0]),
        (
            JammingTechnique::REPEATER_SWEPT_AMPLITUDE_MODULATION_AND_VGPO,
            [2, 80, 5, 0],
        ),
        (JammingTechnique::GLINT_ENHANCE, [2, 85, 0, 0]),
        (JammingTechnique::IMAGE_FREQUENCY, [2, 90, 0, 0]),
        (
            JammingTechnique::JITTERED_PULSE_REPETITION_FREQUENCY,
            [2, 95, 0, 0],
        ),
        (JammingTechnique::JITTERED_PULSE_WIDTH, [2, 100, 0, 0]),
        (JammingTechnique::PSEUDORANDOM_NOISE, [2, 105, 0, 0]),
        (JammingTechnique::PULSE, [2, 110, 0, 0]),
        (
            JammingTechnique::PULSE_COMPRESSION_DECEPTION,
            [2, 115, 0, 0],
        ),
        (JammingTechnique::RANDOM_RANGE_PROGRAMS, [2, 120, 0, 0]),
        (JammingTechnique::REFRACTION, [2, 125, 0, 0]),
        (JammingTechnique::REPEATER, [2, 130, 0, 0]),
        (JammingTechnique::CONTINUOUS_WAVE_REPEATER, [2, 130, 5, 0]),
        (JammingTechnique::DECEPTION_REPEATER_NOISE, [2, 130, 10, 0]),
        (
            JammingTechnique::MULTIPLE_FREQUENCY_REPEATER,
            [2, 130, 15, 0],
        ),
        (
            JammingTechnique::NARROW_BAND_REPEATER_NOISE,
            [2, 130, 20, 0],
        ),
        (JammingTechnique::RANDOM_DOPPLER, [2, 130, 25, 0]),
        (
            JammingTechnique::REPEATER_DIGITAL_RADIO_FREQUENCY_MEMORY,
            [2, 130, 30, 0],
        ),
        (JammingTechnique::DRFM_LEVEL_A, [2, 130, 30, 1]),
        (JammingTechnique::DRFM_LEVEL_B, [2, 130, 30, 2]),
        (JammingTechnique::DRFM_LEVEL_C, [2, 130, 30, 3]),
        (JammingTechnique::DRFM_LEVEL_D, [2, 130, 30, 4]),
        (JammingTechnique::DRFM_LEVEL_E, [2, 130, 30, 5]),
        (
            JammingTechnique::REPEATER_SWEPT_AMPLITUDE_MODULATION,
            [2, 130, 35, 0],
        ),
        (JammingTechnique::SCINTILLATION, [2, 135, 0, 0]),
        (JammingTechnique::SERRODYNE, [2, 140, 0, 0]),
        (JammingTechnique::VELOCITY, [2, 145, 0, 0]),
        (JammingTechnique::DECEPTION_AND_NOISE, [3, 0, 0, 0]),
        (JammingTechnique::ANGLE_AND_NOISE, [3, 5, 0, 0]),
        (JammingTechnique::ANGLE_AND_BARRAGE, [3, 5, 5, 0]),
        (JammingTechnique::BARRAGE_AND_INVERSE_GAIN, [3, 5, 5, 5]),
        (JammingTechnique::BARRAGE_AND_SSW, [3, 5, 5, 10]),
        (JammingTechnique::ANGLE_AND_FM_BY_NOISE, [3, 10, 0, 0]),
        (
            JammingTechnique::FM_BY_NOISE_AND_INVERSE_GAIN,
            [3, 10, 5, 0],
        ),
        (JammingTechnique::FM_BY_NOISE_AND_SSW, [3, 10, 10, 0]),
        (JammingTechnique::ANGLE_AND_PSEUDORANDOM_AM, [3, 15, 0, 0]),
        (
            JammingTechnique::INVERSE_GAIN_AND_PSEUDORANDOM_AM,
            [3, 15, 5, 0],
        ),
        (JammingTechnique::PSEUDORANDOM_AM_AND_SSW, [3, 15, 10, 0]),
        (JammingTechnique::ANGLE_AND_SPOT, [3, 20, 0, 0]),
        (
            JammingTechnique::INVERSE_GAIN_AND_LOW_LEVEL_NOISE,
            [3, 20, 5, 0],
        ),
        (
            JammingTechnique::INVERSE_GAIN_AND_SPOT_NOISE,
            [3, 20, 10, 0],
        ),
        (JammingTechnique::SPOT_AND_SSW, [3, 20, 15, 0]),
        (JammingTechnique::GATE_STEALER_AND_NOISE, [3, 25, 0, 0]),
        (JammingTechnique::NOISE_AND_RGPO, [3, 25, 5, 0]),
        (JammingTechnique::LOW_LEVEL_NOISE_AND_RGPO, [3, 25, 5, 5]),
        (JammingTechnique::NOISE_AND_VGPO, [3, 25, 10, 0]),
        (JammingTechnique::LOW_LEVEL_NOISE_AND_VGPO, [3, 25, 10, 5]),
        (
            JammingTechnique::FALSE_TARGET_DECEPTION_AND_SWEPT_NOISE,
            [3, 30, 0, 0],
        ),
        (JammingTechnique::SPECIAL, [4, 0, 0, 0]),
        (JammingTechnique::SUPER_JAM, [4, 1, 0, 0]),
    ];

    #[test]
    fn every_constant_has_its_code_and_a_name() {
        assert_eq!(TABLE.len(), 134);
        for &(technique, [kind, category, subcategory, specific]) in TABLE {
            assert_eq!(
                technique,
                JammingTechnique::new(kind, category, subcategory, specific)
            );
            assert!(technique.name().is_some(), "{technique:?} has no name");
        }
    }

    #[test]
    fn unknown_codes_have_no_name_but_still_display() {
        let unknown = JammingTechnique::new(1, 200, 0, 0);
        assert_eq!(unknown.name(), None);
        assert_eq!(unknown.to_string(), "1.200");
        assert_eq!(JammingTechnique::default().to_string(), "0");
    }

    #[test]
    fn display_drops_trailing_zero_levels() {
        assert_eq!(JammingTechnique::NOISE.to_string(), "1 Noise");
        assert_eq!(
            JammingTechnique::BARRAGE_NOISE.to_string(),
            "1.10 Barrage Noise"
        );
        assert_eq!(
            JammingTechnique::DRFM_LEVEL_E.to_string(),
            "2.130.30.5 DRFM Level E"
        );
    }

    #[test]
    fn is_within_follows_the_hierarchy() {
        let drfm_e = JammingTechnique::DRFM_LEVEL_E;
        assert!(drfm_e.is_within(JammingTechnique::DECEPTION));
        assert!(drfm_e.is_within(JammingTechnique::REPEATER));
        assert!(drfm_e.is_within(JammingTechnique::REPEATER_DIGITAL_RADIO_FREQUENCY_MEMORY));
        assert!(drfm_e.is_within(drfm_e));
        assert!(!drfm_e.is_within(JammingTechnique::NOISE));
        assert!(!drfm_e.is_within(JammingTechnique::DRFM_LEVEL_A));
        assert!(!JammingTechnique::REPEATER.is_within(drfm_e));
        // The two techniques named "Repeater Noise" are in different kinds.
        assert!(JammingTechnique::NOISE_REPEATER_NOISE.is_within(JammingTechnique::NOISE));
        assert!(JammingTechnique::DECEPTION_REPEATER_NOISE.is_within(JammingTechnique::REPEATER));
    }

    #[test]
    fn encodes_as_four_octets_in_order() {
        let mut buf = Vec::new();
        crate::common::wire::write(&JammingTechnique::DRFM_LEVEL_E, &mut buf).unwrap();
        assert_eq!(buf, [2, 130, 30, 5]);
        let decoded: JammingTechnique = crate::common::wire::read(&mut buf.as_slice()).unwrap();
        assert_eq!(decoded, JammingTechnique::DRFM_LEVEL_E);
    }
}
