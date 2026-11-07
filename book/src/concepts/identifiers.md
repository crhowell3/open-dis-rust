# Identifiers & Exercises

Every PDU says which exercise it belongs to and who sent it. This page explains
the numbers DIS uses for that and they map to types in open-dis-rust.

## Exercises

An **exercise** is one chared simulated world. Several exercises and run on the
same network at once, such as a training event and a test. They are kept discrete
and disjoint via the **exercise ID** in the header of every PDU.

- The exercise ID is one octet, so it ranges from 1 to 255.
- Every participant in an exercise must use the same ID. The exercise organizer
  usually assigns it.
- Receivers ignore PDUs from other exercises. Nothing in the protocol enforces this,
  so your application(s) must handle this on their own.

PDUs created with `new()` use exercise ID 1 by default. Set your own before sending:

```rust,ignore
pdu.header_mut().exercise_id = 3;
```

The exercise ID is octet 1 of every PDU, so you can filter before decoding anything:

```rust,ignore
fn is_same_exercise(datagram: &[u8], exercise_id: u8) -> bool {
    datagram.get(1) == Some(&exercise_id)
}
```

## Sites and applications

Within an exercise, every simulation program has a **simulation address**: a pair of
16-bit numbers used for identification.

- The **site** usually identifies a facility or organization.
- The **application** identifies one running simulation program at that site.

Together they must be unique in the exercise. Like the exercise ID, they are agreed in
advance rather than assigned by the protocol. In open-dis-rust, they are stored in
`SimulationAddress`:

```rust,ignore
use open_dis_rust::common::data_types::SimulationAddress;

let me = SimulationAddress::new(1, 100); // site 1, application 100
```

## Entities

An **entity** is anything in the simulated world that a simulation owns and reports on,
such as a vehicle, an aircraft, a person, or a munition in flight. Its **entity ID** is
its simulation address plus a 16-bit entity number:

```rust,ignore
use open_dis_rust::common::data_types::EntityIdentifier;

let tank = EntityIdentifier::new(1, 100, 7); // entity 7 of site 1, application 100
```

The application that owns an entity picks its entity numbers. Use each number for only
one entity during an exercise. If you reuse a number after an entity is destroyed, other
simulators may confuse the new entity with the old one.

Entity number 0 means "no entity". An entity ID with entity number 0 refers to the
simulation application itself, which is how simulation management PDUs address a
whole simulator:

```rust,ignore
let my_application = EntityIdentifier::new(1, 100, 0);
```

## Special values

These constants in `common::constants` have fixed meanings:

| Constant | Value | Meaning |
| --- | --- | --- |
| `NO_SITE`, `NO_APPLIC`, `NO_ENTITY` | 0 | None, or not applicable. |
| `ALL_SITES`, `ALL_APPLIC`, `ALL_ENTITIES` | 0xFFFF | Every site, application, or entity. |
| `RQST_ASSIGN_ID` | 0xFFFE | Ask the receiver to assign an entity number. |

For example, to send a Start/Resume PDU to every simulator in the exercise:

```rust,ignore
use open_dis_rust::common::constants::{ALL_APPLIC, ALL_ENTITIES, ALL_SITES};

let mut start = StartResumePdu::new();
start.originating_entity_id = EntityIdentifier::new(1, 100, 0);
start.receiving_entity_id = EntityIdentifier::new(ALL_SITES, ALL_APPLIC, ALL_ENTITIES);
```

Other PDUs use 0 to mean "no target". A Fire PDU for a shot with no intended target
sets the target entity ID to all zeros.

## Other identifiers

Several other things in DIS are identified the same way: a simulation address plus
a number that the owning application assigns.

| Identifier | Rust type | Identifies | Used in |
| --- | --- | --- | --- |
| Entity ID | `EntityIdentifier` | An entity | Most PDUs |
| Event ID | `EventId` | One event, such as a shot | Fire, Detonation, Collision, emissions PDUs |
| Object ID | `ObjectIdentifier` | An environmental object, such as a minefield marker or a crater | Linear and Areal Object State |
| Aggregate ID | `AggregateId` | A group of entities, such as a platoon | Aggregate State |
| Minefield ID | `MinefieldIdentifier` | A minefield | Minefield PDUs |
| Simulation ID | `SimulationIdentifier` | A simulation application | Transfer Ownership, Information Operations |

**Event IDs** link related PDUs. When an entity fires, the Fire PDU and the Detonation
PDU for that shot carry the same event ID, so receivers can match the two. Generally,
you should monotonically increment the number for each successive event, wrapping around
once you reach the 16-bit maximum (65535).

**Radios and intercoms** are identified by an entity ID plus a 16-bit radio or
intercom number because one entity can carry several. The Transmitter, Signal, and
Receiver PDUs for one radio all use the same `entity_id` and `radio_id`.

## Live entity identifiers

The Live Entity PDUs (types 46-50) use smaller fields to save bandwidth on
instrumented live ranges. `LEEntityId` has an 8-bit site number, an 8-bit
application number, and a 16-bit entity number. If your simulation exchanges data with
live entities, keep its site and application numbers below 256 so that they can be
represented in both forms.
