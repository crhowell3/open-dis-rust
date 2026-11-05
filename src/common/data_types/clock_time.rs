//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use std::time::{SystemTime, UNIX_EPOCH};

use crate::common::Timestamp;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
/// Implemented according to IEEE 1278.1-2012 §6.2.14
pub struct ClockTime {
    /// The hours since 0000h 1 January 1970 UTC (The Epoch)
    pub hour: u32,
    /// Time past the hour indicated in the hour field
    pub time_past_hour: Timestamp,
}

impl ClockTime {
    #[must_use]
    pub const fn new(hour: u32, time_past_hour: Timestamp) -> Self {
        Self {
            hour,
            time_past_hour,
        }
    }

    /// The current UTC time, from the system clock, with an absolute timestamp
    #[must_use]
    pub fn now() -> Self {
        let since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let hour = u32::try_from(since_epoch.as_secs() / 3600).unwrap_or(u32::MAX);
        Self::new(hour, Timestamp::absolute(since_epoch))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn now_counts_hours_since_the_epoch() {
        let now = ClockTime::now();
        // November 2026 is about 498,000 hours after 1970, so any current time is past this
        assert!(now.hour > 495_000, "{now:?}");
        assert!(now.time_past_hour.is_absolute());
    }
}
