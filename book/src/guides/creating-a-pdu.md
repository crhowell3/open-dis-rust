# Creating a PDU

IEEE 1278.1-2012 defines 72 PDUs, and open-dis-rust implements all of them.

## Importing PDUs

Each protocol family is a module, and each PDU is a struct in its family's module.
[Protocol Families](../concepts/families.md) lists them. Import the PDU you need along
with the prelude for the common types and traits:

```rust,ignore
use open_dis_rust::{prelude::*, simulation_management::AcknowledgePdu};
```

## Constructing PDUs

`new()` gives a PDU that is ready to send: its header has the correct PDU type, protocol
family, and length, exercise ID 1, and the current time. Every other field has a sensible
default value.

```rust,ignore
let mut ack = AcknowledgePdu::new();
```

## Set the header

Read the header with `header()` and change it with `header_mut()`. Set the exercise ID
and, for a PDU you send more than once, the timestamp:

```rust,ignore
ack.header_mut().exercise_id = ExerciseIdentifier::new(3);
ack.header_mut().timestamp = Timestamp::now();
```

The PDU type, protocol family, and length are read-only. Encoding fills them in from the
PDU, so they cannot disagree with its contents.

## Set the fields

Every other field is public. Simple fields take a plain value:

```rust,ignore
ack.request_id = 14;
```

Fields that are records, such as identifiers, take a record. Build it with its `new`
function, or change its fields one at a time:

```rust,ignore
let me = SimulationAddress::new(1, 100);
ack.originating_entity_id = EntityIdentifier::new(me, 0);
ack.receiving_entity_id.entity_id = 12;
```

Counts and lengths that depend on other fields, such as the number of records in a
list, are set when the PDU is encoded, so you only fill in the list itself.
[Wire Format Conventions](../reference/wire-format.md#what-encoding-fills-in-for-you)
lists them.

## Send it

[Sending PDUs](sending-pdus.md) shows how to encode the PDU and send it over UDP.
