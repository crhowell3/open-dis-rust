# Wire Format Conventions

This page summarizes how DIS PDUs are laid out on the wire and what open-dis-rust
does for you when it encodes them. This is a quick reference; for a complete guide,
check out the IEEE 1278.1-2012 documentation.

## Basics

- **Byte order.** Every multi-octet field is big-endian (network byte order).
- **Floating point.** 32- and 64-bit floats are in IEEE 754 format.
- **Units.** Unless the original standard document specifies otherwise, lengths are
  in meters, angles in radians, time in seconds, and velocities in meters per second.
- **Coordinates.**  World coordinates are geocentric (ECEF) coordinates in meters
  encoded as 64-bit floats.
- **Maximum size.** A PDU can be at most 8192 octets. Encoding a larger PDU fails
  with `DISError::PduSizeExceeded`.

## The PDU header

Every PDU starts with a 12-octet header.

| Octets | Field | Size | Notes |
| --- | --- | --- | --- |
| 0 | Protocol Version | 8 bits | Encoded as `7` for IEEE 1278.1-2012 |
| 1 | Exercise ID | 8 bits | Must match across all participants in an exercise |
| 2 | PDU Type | 8 bits | See [PDU Coverage](pdu-coverage.md) |
| 3 | Protocol Family | 8 bits | |
| 4-7 | Timestamp | 32 bits | See [Timestamps](#timestamps) |
| 8-9 | Length | 16 bits | Total PDU length in octets, including the header |
| 10 | PDU Status | 8 bits | See [PDU Status](#pdu-status) |
| 11 | Padding | 8 bits | |

In `LiveEntityPduHeader`, octet 10 holds the subprotocol number instead of the PDU Status
record.

To check a datagram's PDU type without decoding the whole PDU, use `PduHeader::get_pdu_type`.
It reads only the header.

### Timestamps

The tiemstamp counts time since the start of the current hour. Bits 31 to 1 hold the time
in units of 3600/2³¹ seconds, which is about 1.676 µs. Bit 0 is `1` for an absolute
timestamp (synchronized to real time) and `0` for a relative.

`PduHeader::calculate_dis_timestamp()` returns an absolute timestamp for the current UTC time.
`new()` sets it once when you create a PDU, so set it again before each send if you reuse
a PDU:

```rust,ignore
pdu.header_mut().timestamp = PduHeader::calculate_dis_timestamp();
```

To send relative timestamps, compute the value yourself and leave bit 0 clear.

### PDU Status

| Bits | Field | Applies to |
| --- | --- | --- |
| 0 | Transferred Entity Indicator (TEI) | PDUs that identify an entity |
| 1-2 | LVC Indicator | All PDUs |
| 3 | Coupled Extension Indicator (CEI) | All PDUs |
| 4-5 | Depends on PDU type | See below |
| 6-7 | Reserved | |

Bits 4 and 5 mean different things depending on the PDU:

| PDU | Bits 4-5 | Accessor |
| --- | --- | --- |
| Fire | Fire Type Indicator (bit 4) | `get_fti` / `set_fti` |
| Detonation | Detonation Type Indicator | `get_dti` / `set_dti` |
| Transmitter, Signal, Receiver | Radio Attached Indicator | `get_rai` / `set_rai` |
| Intercom Signal, Intercom Control | Intercom Attached Indicator | `get_iai` / `set_iai` |
| IFF | IFF Simulation Mode (bit 4), Active Interrogation Indicator (bit 5) | `get_ism` / `set_ism`, `get_aii` / `set_aii` |

The accessors are on `PduStatusRecord`, which you reach through `header.status_record`.

## What encoding fills in for you

`serialize()` sets these fields from the PDU's contents, so you never set them yourself:

- The header's **PDU type**, **protocol family**, and **length**.
- Every **count field**, such as the number of articulation parameters or datum records,
  from the length of the matching `Vec`.
- **Record length** fields in variable-length records, such as attribute records and
  variable transmitter parameters.
- **Flag fields** that say which optional fields are present. PDUs that have optional
  fields include the TSPI PDU, the Appearance PDU, and the Minefield Data PDU.
- **Padding**, which is always written as `0`.

You set the **exercise ID**, the **timestamp**, and the **PDU Status** bits. The protocol
version defaults to `7`, which represents `IEEE 1278.1-2012`.

`new()` also runs these updates, so a freshly created PDU already has the right length in
its header.

## Strings

**Entity Marking** is one character-set octect followed by exactly 11 character octets.
Shorter strings are padded when encoded. Longer ones are cut to 11 octets, so keep markings
short.

## Several PDUs in one datagram

The standard allows more than one PDU in a single datagram. `deserialize` reads exactly
the number of octets given by the PDU's `length` field and leaves the rest in the
buffer, so you can decode a bundle in a loop:

```rust,ignore
let mut rest: &[u8] = datagram;

while rest.len() >= PduHeader::length() {
    match PduHeader::get_pdu_type(rest) {
        Some(PduType::EntityState) => {
            let pdu = EntityStatePdu::deserialize(&mut rest)?;
            // ...
        }
        _ => {
            // Skip a PDU we do not want to handle, using the length field (octets 8-9)
            let len = rest.get(8..10).map_or(0, |b| usize::from(u16::from_be_bytes([b[0], b[1]])));
            if len < PduHeader::length() || len > rest.len() {
                break; // malformed: drop the rest of the datagram
            }
            rest = &rest[len..];
        }
    }
}
```

To send a bundle, serialize each PDU into the same buffer, one after another, and keep
the total under the network's maximum datagram size.
