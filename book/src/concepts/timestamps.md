# Timestamps

Every PDU header has a 32-bit **timestamp** saying when the data in it was valid.
Receivers use it to tell how old the data is, to put PDUs in order, and to
extrapolate entity positions forward to the present.

## Time since the hour

A DIS timestamp counts time since the **start of the current hour**, not since a fixed
date. Bits 31 to 1 hold that time in units of 3600/2³¹ seconds, which is about
1.676 microseconds. Bit 0 says what kind of timestamp it is.

```text
 31                                                           1   0
┌─────────────────────────────────────────────────────────────┬───┐
│ time since the start of the hour, in units of 3600/2³¹ s    │ A │
└─────────────────────────────────────────────────────────────┴───┘
                                                  A = 1: absolute, 0: relative
```

At the start of each hour, the count resets to 0. A receiver comparing timestamps
must accommodate for this, as show [below](#working-with-timestamps).

## Absolute and relative time

**Absolute** timestamps (bit 0 set) are measured against real-world UTC. They only
work if every participant's clock is synchronized, usually with RTP or, for tighter
accuracy, PTP. Because every PDU then shares one time base, receivers can compare
timestamps from different simulators directly. This is the usual choice for
networked exercises.

**Relative** timestamps (bit 0 clear) are measured against each simulator's own clock,
so they can only be compared with other timestamps from the same simulator. This is
useful when clocks cannot be synchronized or when a simulation runs faster or slower
than real time.

Whichever is chosen, every participant in an exercise should agree on it.

## How open-dis-rust sets timestamps

`PduHeader::calculate_dis_timestamp()` returns an **absolute** timestamp for the current
UTC time using the current system clock. `new()` calls it once when the PDU is created.
If a PDU is kept and sent repeatedly, set the timestamp again before each send:

```rust,ignore
pdu.header_mut().timestamp = PduHeader::calculate_dis_timestamp();
pdu.serialize(&mut bytes)?;
```

The timestamp is only as accurate as the system clock, so make sure the machine is synchronized
as well.

To send **relative** timestamps, compute the value from your own simulation clock and
leave bit 0 clear:

```rust,ignore
use std::time::Duration;
 
const TICKS_PER_HOUR: u128 = 1 << 31;
const NANOS_PER_HOUR: u128 = 3_600_000_000_000;
 
/// Encodes time since the start of the simulation as a relative DIS timestamp.
fn relative_timestamp(since_start: Duration) -> u32 {
    let past_hour = since_start.as_nanos() % NANOS_PER_HOUR;
    let ticks = past_hour * TICKS_PER_HOUR / NANOS_PER_HOUR; // always below 2³¹
    u32::try_from(ticks).unwrap_or(0) << 1 // bit 0 clear: relative
}
```

## Working with timestamps

The crate stores timestamps as a raw `u32`. These helpers convert them into `std::time::Duration`:

```rust,ignore
use std::time::Duration;
 
const TICKS_PER_HOUR: u128 = 1 << 31;
const NANOS_PER_HOUR: u128 = 3_600_000_000_000;
 
fn ticks_to_duration(ticks: u32) -> Duration {
    let nanos = u128::from(ticks) * NANOS_PER_HOUR / TICKS_PER_HOUR;
    Duration::from_nanos(u64::try_from(nanos).unwrap_or(u64::MAX))
}
 
/// Returns the time past the hour, and whether the timestamp is absolute.
fn decode_timestamp(timestamp: u32) -> (Duration, bool) {
    (ticks_to_duration(timestamp >> 1), timestamp & 1 == 1)
}
 
/// How long before `now` a timestamp was, allowing for the hour wrapping around.
fn age(timestamp: u32, now: u32) -> Duration {
    let ticks = (now >> 1).wrapping_sub(timestamp >> 1) & 0x7FFF_FFFF;
    ticks_to_duration(ticks)
}
```

`age` handles the hour boundary correctly: a timestamp from 59:59 compared with a
current time of 00:01 gives an age of 2 seconds.

Because timestamps repeat every hour, a timestamp slightly *ahead* of your clock from
a sender whose clock runs fast comes out as an age of nearly an hour. Treat ages above
30 minutes as being from the future, and do not extrapolate those entities backwards.

```rust,ignore
let now = PduHeader::calculate_dis_timestamp();
let age = age(pdu.header().timestamp, now);
if age < Duration::from_secs(1800) {
    // Extrapolate the entity's position forward by `age` using its dead reckoning parameters.
}
```

## Clock Time records

Some PDUs need to say *which* hour, not just the time within it. Simulation Management PDUs
such as Start/Resume use a **Clock Time** record (`ClockTime`) for this. It has two fields:

- `hour`: the number of hours since 00:00 UTC on 1 January 1970.
- `time_past_hour`: the time within that hour, in the same format as a header timestamp.

To build one for the current time:

```rust,ignore
use std::time::{SystemTime, UNIX_EPOCH};
use open_dis_rust::common::{PduHeader, data_types::ClockTime};
 
fn clock_time_now() -> ClockTime {
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let hours = u32::try_from(since_epoch.as_secs() / 3600).unwrap_or(u32::MAX);
    ClockTime::new(hours, PduHeader::calculate_dis_timestamp())
}
```
