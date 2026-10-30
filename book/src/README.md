# Introduction

**open-dis-rust** is an open-source Rust implementation of the IEEE 1278.1-2012
Distributed Interactive Simulation (DIS) application protocol. This crate defines
strongly typed Rust structures for each Protocol Data Unit (PDU) along with the
records and SISO-REF-010 enumerations associated with them. It also gives you
the code to turn those structs into bytes on the wire and back again.

```rust,ignore
let mut pdu = EntityStatePdu::new();
pdu.entity_id = EntityId::new(1, 1, 1);
pdu.force_id = ForceId::Friendly;

let mut bytes = BytesMut::new();
pdu.serialize(&mut bytes)?;
```

## What is DIS?

DIS is an IEEE standard that enables independently constructed simulations share
one virtual world in real time. Flight simulators, ground vehicle trainers,
constructive wargames, radio simulators, and live range instrumentation can all
join the same exercise. Each one broadcasts PDUs that describe its own entities
and events and reads the PDUs from other participants in the simulation.

A few PDUs carry most of the traffic in a typical exercise:

| PDU | Purpose |
| --- | --- |
| Entity State | Describes a simulation entity, including what it is, where it is, how it is moving, and what it looks like |
| Fire / Detonation | A munition was fired, and it detonated at a certain location |
| Transmitter / Signal / Receiver | Simulates radio traffic, including actual audio |
| Start/Resume, Stop/Freeze, Acknowledge | Simulation management for controlling the exercise |

DIS has no central server. Simlators usually exchange PDUs over UDP broadcast or multicast,
and every participant decides for itself which PDUs to act on.

## What this crate does

- Defines all 72 PDU types defined in the IEEE 1278.1-2012 across every protocol family that
  has PDUs, including Live Entity, Information Operations, and Simulation Management with
  Reliability, to name a few.
- Encodes and decodes PDUs in network (big-endian) byte order, and checks the declared PDU
  length against the actual contents.
- Fills in derived fields for you when serializing. These include the PDU type, protocol
  family, total length, record counts, and padding.
- Provides SISO-REF-010 enumerations implemented as Rust enums.
- Avoids `unsafe` code and panics. Malformed input comes back as `DISError` instead of
  crashing the simulation.

## What this crate leaves to you

This crate is a protocol codec, not a complete DIS framework. Any consumer application is
responsible for:

- **Networking.** Any transport mode works, but it is strongly recommended to use UDP in
  distributed simulation environments. The examples in this crate all use UDP either via
  the Rust standard library or with `tokio`.
- **Timing.** This includes heartbeats and timing out entities that have gone stale.
- **Dead reckoning.** This crate carries the dead reckoning parameters, but extrapolating
  positions between updates is up to the consumer.
- **Coordinate conversion.** DIS locations are geocentric (ECEF) coordinates in meters.
  Converting to and from latitude, longitude, and altitude needs a separate crate or
  custom implementation.

## How this book is organized

- **[Installation](acquiring.md)** and the **[Quickstart](quickstart.md)** get you sending
  and receiving PDUs in a few minutes.
- **Guides** walk through common tasks in more detail.
- **Development** explains how the crate is built, for contributors.

For the full API, see the [API documentation on docs.rs](https://docs.rs/open-dis-rust).

## Project status and feedback

This crate is free and open source under the BSD 2-Clause License. If you find a PDU,
record, or enumeration that doesn't match the standard, or one that another DIS
implementation is unable to parse, please [open an issue](https://github.com/crhowell3/open-dis-rust/issues)
and attach a packet capture if you can. Captures from real exercises are the fastest
way to find and fix interoperability problems. Since DIS is commonly used in military
exercises, ensure that you do NOT post any classified data when submitting tickets.
