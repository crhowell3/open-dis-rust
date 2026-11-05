//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! DIS timestamps

use std::{
    fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// Timestamp ticks in one hour: bits 31 to 1 count units of 3600/2^31 seconds
const TICKS_PER_HOUR: u128 = 1 << 31;
const NANOS_PER_HOUR: u128 = 3_600_000_000_000;
const SECONDS_PER_HOUR: u64 = 3600;
const ABSOLUTE_BIT: u32 = 1;

/// A DIS timestamp, as carried in every PDU header and in [`ClockTime`] records.
///
/// A timestamp counts the time since the start of the current hour in units of
/// about 1.676 microseconds.
///
/// Because the count restarts every hour, timestamps can't be ordered by
/// comparing them directly. Use [`duration_since`](Self::duration_since), which
/// allows for the hour wrapping around.
///
/// ```
/// use std::time::Duration;
/// use open_dis_rust::common::Timestamp;
///
/// let start = Timestamp::absolute(Duration::from_secs(3599)); // 59:59
/// let later = Timestamp::absolute(Duration::from_secs(1));    // 00:01, in the next hour
/// // Accurate to within a tick, about 1.7 µs.
/// assert_eq!(later.duration_since(start).as_secs_f64().round(), 2.0);
/// assert!(later.is_absolute());
/// ```
///
/// [`ClockTime`]: crate::common::data_types::ClockTime
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct Timestamp {
    raw: u32,
}

impl Timestamp {
    /// An absolute timestamp for the current time from the system clock
    ///
    /// The result is only as accurate as the system clock, so sync it with
    /// something like NTP before other simulators compare their timestamps
    /// with yours.
    #[must_use]
    pub fn now() -> Self {
        let since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        Self::absolute(since_epoch)
    }

    /// An absolute timestamp for the given time past the hour, in UTC.
    ///
    /// Whole hours are discarded, so a time since midnight, or since the Unix
    /// epoch, gives the same result as the time since the start of its hour.
    #[must_use]
    pub fn absolute(time_past_hour: Duration) -> Self {
        Self {
            raw: Self::ticks(time_past_hour) << 1 | ABSOLUTE_BIT,
        }
    }

    /// A relative timestamp for the given time on the simulation's own clock, for
    /// example the time since the simulation started.
    ///
    /// Whole hours are discarded, as for [`absolute`](Self::absolute).
    #[must_use]
    pub fn relative(time: Duration) -> Self {
        Self {
            raw: Self::ticks(time) << 1,
        }
    }

    /// A timestamp from its 32-bit wire value.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self { raw }
    }

    /// The timestamp's 32-bit wire value.
    #[must_use]
    pub const fn to_raw(self) -> u32 {
        self.raw
    }

    /// Whether the timestamp is absolute (measured against UTC) rather than relative.
    #[must_use]
    pub const fn is_absolute(self) -> bool {
        self.raw & ABSOLUTE_BIT == ABSOLUTE_BIT
    }

    /// The time since the start of the hour that the timestamp gives.
    #[must_use]
    pub fn time_past_hour(self) -> Duration {
        Self::duration(self.raw >> 1)
    }

    /// How long after `earlier` this timestamp is, allowing for the hour wrapping
    /// around.
    ///
    /// The result is always less than an hour. A timestamp from 59:59, compared
    /// with one from 00:01, is 2 seconds earlier, not 59 minutes and 58 seconds
    /// later. Like every timestamp, it's accurate to within about 1.7 µs, so a
    /// whole number of seconds can come out a fraction of a microsecond short.
    ///
    /// If `earlier` is actually slightly *after* this timestamp, for example
    /// because the sender's clock runs fast, the result is just under an hour.
    /// Treat results above about 30 minutes as meaning `earlier` is in the future.
    ///
    /// Only compare timestamps of the same kind: absolute with absolute, or
    /// relative with relative from the same simulation.
    #[must_use]
    pub fn duration_since(self, earlier: Self) -> Duration {
        let ticks = (self.raw >> 1).wrapping_sub(earlier.raw >> 1) & (u32::MAX >> 1);
        Self::duration(ticks)
    }

    /// Converts a duration to the nearest tick since the start of its hour. Always
    /// below 2³¹: a time that rounds up to the next hour becomes tick 0
    fn ticks(time: Duration) -> u32 {
        let past_hour = time.as_nanos().checked_rem(NANOS_PER_HOUR).unwrap_or(0);
        let ticks = past_hour
            .saturating_mul(TICKS_PER_HOUR)
            .saturating_add(NANOS_PER_HOUR / 2)
            .checked_div(NANOS_PER_HOUR)
            .unwrap_or(0)
            .checked_rem(TICKS_PER_HOUR)
            .unwrap_or(0);
        u32::try_from(ticks).unwrap_or(0)
    }

    /// Converts ticks to a duration. `ticks` is below 2³¹, so this never overflows.
    fn duration(ticks: u32) -> Duration {
        // 3600 s / 2³¹ per tick: whole seconds first, then the remainder in nanoseconds
        let total = u64::from(ticks).saturating_mul(SECONDS_PER_HOUR);
        let seconds = total >> 31;
        let remainder = total & ((1 << 31) - 1);
        // Round to the nearest nanosecond. `Duration::new` carries 10⁹ nanoseconds
        // into the seconds
        let nanos = remainder
            .saturating_mul(1_000_000_000)
            .saturating_add(1 << 30)
            >> 31;
        Duration::new(seconds, u32::try_from(nanos).unwrap_or(0))
    }
}

impl From<Timestamp> for u32 {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.raw
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timestamp")
            .field("time_past_hour", &self.time_past_hour())
            .field("absolute", &self.is_absolute())
            .finish()
    }
}

/// Formats the time past the hour as minutes and seconds, to the nearest microsecond,
/// followed by `abs` or `rel`: for example `35:26.232076 abs`. A timestamp's
/// resolution is about 1.676 µs, so the last digit isn't always exact.
impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Round to the nearest microsecond, but never up to 60:00
        let micros = self
            .time_past_hour()
            .as_nanos()
            .saturating_add(500)
            .checked_div(1000)
            .unwrap_or(0)
            .min(3_599_999_999);
        let seconds = micros / 1_000_000;
        write!(
            f,
            "{:02}:{:02}.{:06} {}",
            seconds / 60,
            seconds % 60,
            micros % 1_000_000,
            if self.is_absolute() { "abs" } else { "rel" }
        )
    }
}

#[cfg(test)]
#[allow(
    clippy::duration_suboptimal_units,
    reason = "Duration::from_mins and from_hours need Rust 1.91, above the crate's minimum"
)]
mod tests {
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
            // Rounding to the nearest tick is off by at most half a tick
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
        // `earlier` two seconds in the future comes out as just under an hour
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
}
