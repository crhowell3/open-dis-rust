# Errors

Every fallible operation in open-dis-rust returns `Result<T, DISError>`. The crate
never panics on bad input, so a malformed or unexpected PDU always comes back to you
as an error you can handle.

`DISError` inplements `std::error::Error` and `Display`. The alias `common::dis_error::DISResult<T>`
can be imported as a shorthand for `Result<T, DISError>`.

## Possibly encountered errors

These are the variants the crate currently returns. The messages are the actual `Display` output.

### `WireFormat`

The bytes do not form a valid PDU. This is the most common decoding error. It has three typical causes:

- **The buffer ends partway through a field**, usually because the datagram was
  shorter than the 12-octet header:
```text
  PDU wire format error: Not enough data, need 16 bits (or 2 bytes)
```
 
- **An enumerated field holds a value the crate doesn't define.** This includes the
  protocol version in the header:
```text
  PDU wire format error: Parse error: 99 is not a defined ProtocolVersion value
  PDU wire format error: Parse error: 32513 is not a defined AcknowledgeFlag value
```
 
- **A record's fields are inconsistent**, for example a count or length field that
  doesn't match the data. This also happens when *encoding* a record you built
  yourself:
```text
  PDU wire format error: VariableDatumRecord: 16 bits needs 2 octets, but value has 3
```

This variant wraps the underlying `deku::DekuError`. Match on `WireFormat(_)` rather than on the 
inner error type because its type may change in a future release.

### `InvalidHeader`

The header was read but does not fit the PDU you asked for.

```text
Invalid PDU header: expected PDU type EntityState, got Acknowledge
Invalid PDU header: PDU length 4 is shorter than its 12-octet header
```

The first is a normal event on a busy network. It means you called `deserialize` on
the wrong type, which you can avoid by checking `PduHeader::get_pdu_type` first.

### `BufferUnderflow`

The header's `length` field promises more octets than the buffer holds. This usually
means the datagram was truncated, or your receive buffer was too small.

```text
Buffer underflow: tried to read 20 bytes, but only 8 bytes remain
```

Make receive buffers at least `MAX_PDU_SIZE_OCTETS` (8192) octets.

### `DeserializationError`

The PDU decoded, but its `length` field covers more octets than its fields use.

```text
PDU deserialization error: PDU length 36 is 4 octets longer than its fields
```

This can mean the sender uses a newer or extended version of the PDU, or that the crate
is missing a field. See [Conformance Notes](conformance.md).

### `PduSizeExceeded`

Encoding would produce a PDU larger than the 8192-octet limit, usually because of too 
many records or a very large datum value.

```text
PDU size exceeds maximum allowed: 9104 > 8192
```

Split the data across several PDUs.

## Reserved variants

`SerializationError`, `InvalidFieldValue`, `ProtocolVersionMismatch`, and `InvalidStateTransition`
exist but are not returned by the crate at this time. Keep a catch-all arm in your `match` so
new variants do not break your code. It is possible that variants are added, modified, or removed
in future releases.

## Errors that are not reported

`new()` and `finalize()` cannot return errors. If a PDU's contents are invalid, they leave the
header length at 0 and carry on. `serialize()` runs the same checks and does report the error,
so always check its result.

## Handling errors in a receiver

On a shared network, some traffic will always fail to decode. It might come from another exercise,
a newer simulator or a damaged packet. A receiver should log the problem and move on like so:

```rust,ignore
use open_dis_rust::prelude::*;
use open_dis_rust::entity_information::EntityStatePdu;

fn handle(datagram: &[u8]) {
    let Some(PduType::EntityState) = PduHeader::get_pdu_type(datagram) else {
        return; // not a PDU we care about
    };

    let mut bytes = datagram;

    match EntityStatePdu::deserialize(&mut bytes) {
        Ok(pdu) => {
            // use the PDU
        }
        Err(DISError::BufferUnderflow { .. }) => {
            eprintln!("truncated Entity State PDU; is the receive buffer big enough?");
        }
        Err(DISError::WireFormat(e)) => {
            eprintln!("undecodable Entity State PDU: {e}");
        }
        Err(e) => eprintln!("dropping Entity State PDU: {e}"),
    }
}
```
