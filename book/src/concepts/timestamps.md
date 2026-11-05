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

## The `Timestamp` type

The crate represents timestamps with `common::Timestamp`, which is also in the
prelude. Header timestamps and the time inside Clock Time records both use it.

**Creating timestamps:**

| Call | Gives |
| --- | --- |
| `Timestamp::now()` | An absolute timestamp for the current UTC time, from the system clock |
| `Timestamp::absolute(time_past_hour)` | An absolute timestamp for a given time, as a `Duration` |
| `Timestamp::relative(time)` | A relative timestamp, for example from the time since your simulation started |
| `Timestamp::from_raw(u32)` | A timestamp from its 32-bit wire value |

`absolute` and `relative` discard whole hours, so you can pass a time since midnight,
or since the simulation started, directly.

**Reading timestamps:**

| Call | Gives |
| --- | --- |
| `is_absolute()` | Whether bit 0 is set |
| `time_past_hour()` | The time since the start of the hour, as a `Duration` |
| `later.duration_since(earlier)` | The time between two timestamps, allowing for the hour wrapping around |
| `to_raw()` | The 32-bit wire value |

`Timestamp` implements `Display`, which prints the time past the hour and its kind,
such as `35:26.232076 abs`, which is handy for logging.

A timestamp's resolution is about 1.676 µs, so a value converted to a `Timestamp` and
back can differ from the original by up to about 0.84 µs.

## Setting the timestamp before sending

`new()` timestamps a PDU with `Timestamp::now()` once, when the PDU is created. If you
keep a PDU and send it repeatedly, as you normally would for Entity State heartbeats,
set the timestamp again before each send:

```rust,ignore
use open_dis_rust::prelude::*;

pdu.header_mut().timestamp = Timestamp::now();
pdu.serialize(&mut buf)?;
```

The timestamp is only as accurate as your system clock, so make sure the machine is
synchronized.

To send **relative** timestamps, build them from your own simulation clock:

```rust,ignore
use std::time::Instant;

let started = Instant::now();
// ...
pdu.header_mut().timestamp = Timestamp::relative(started.elapsed());
```

## How old is a PDU?

`duration_since` handles the hour boundary: a timestamp from 59:59, compared with one
from 00:01, is 2 seconds earlier, not almost an hour.

```rust,ignore
use std::time::Duration;

let age = Timestamp::now().duration_since(pdu.header().timestamp);
if age < Duration::from_secs(30 * 60) {
    // Extrapolate the entity's position forward by `age` using its dead reckoning parameters
}
```

Because timestamps repeat every hour, a timestamp slightly *ahead* of your clock, from
a sender whose clock runs fast, comes out as an age of nearly an hour. Treat ages above
30 minutes as being from the future, and don't extrapolate those entities backwards.

Only compare timestamps of the same kind: absolute with absolute, or relative with
relative from the same simulator.

## Clock Time records

Some PDUs need to say *which* hour, not just the time within it. Simulation Management PDUs
such as Start/Resume use a **Clock Time** record (`ClockTime`) for this. It has two fields:

- `hour`: the number of hours since 00:00 UTC on 1 January 1970.
- `time_past_hour`: the time within that hour, as a `Timestamp`.

To build one for the current time, use `ClockTime::now()`:

```rust,ignore
use open_dis_rust::common::data_types::ClockTime;

start.real_world_time = ClockTime::now();
```

To build one yourself, pass the hour and a `Timestamp`:

```rust,ignore
use std::time::Duration;
use open_dis_rust::common::{Timestamp, data_types::ClockTime};

// 06:00 UTC on 1 January 2026 is 490,902 hours after the Unix epoch
let dawn = ClockTime::new(490_902, Timestamp::absolute(Duration::ZERO));
```
