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
