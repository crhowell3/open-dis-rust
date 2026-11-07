# Architecture

This page explains how open-dis-rust is put together: where things live, how a PDU
becomes bytes and vice versa, and the rules the code follows. Read it before making
changes to the crate.

## Layout

```text
src/
├── lib.rs                         one module per protocol family
├── entity_information.rs          ┐
├── warfare.rs                     │
├── logistics.rs                   │  the PDUs, grouped by family
├── …                              │
├── information_operations.rs      ┘
└── common/
    ├── pdu.rs                     the Pdu trait and the impl_pdu! and pdu_tests! macros
    ├── pdu_header.rs              PduHeader and PduStatusRecord
    ├── live_entity_pdu_header.rs  LiveEntityPduHeader
    ├── generic_header.rs          the GenericHeader trait both headers implement
    ├── wire.rs                    bridges byte slices and vectors to deku, plus encoding helpers
    ├── records.rs                 the records PDUs are built from
    ├── data_types.rs              other non-record data types defined or implied in the standard
    ├── live_entity_records/       the compact records used by Live Entity PDUs
    ├── enums.rs                   SISO-REF-010 enumerations
    ├── enums/deku_tests.rs        encoding tests for every enumeration
    ├── constants.rs               special values and size limits
    └── dis_error.rs               DISError
```

PDUs live in the module for their family. Records shared between PDUs live in `common/data_types/`,
each in its own file and re-exported from `data_types/mod.rs`.

## Deku

This crate leverages `deku` for most of its data encoding. PDUs and records are plain structs
that derive [deku](https://docs.rs/deku)'s `DekuRead` and `DekuWrite`, and deku generates the 
code to read and write their fields in order. Attributes on each field describe anything that
is not a fixed-size value: counts, lengths, optional fields, and padding.

There are two attribute forms.

**PDU structs** are always big-endian:

```rust,ignore
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct ServiceRequestPdu { … }
```

**Records** take their byte order from whatever contains them, and default to
big-endian when used on their own:
 
```rust,ignore
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct EntityIdentifier {
    pub simulation_address: SimulationAddress,
    pub entity_id: u16,
}
```

## Header and body

Every PDU struct starts with its header, marked `#[deku(skip)]`:
 
```rust,ignore
pub struct ServiceRequestPdu {
    #[deku(skip)]
    header: PduHeader,
    pub receiving_entity_id: EntityIdentifier,
    …
}
```
 
deku only handles the **body**. The header is read and written separately by the code
that `impl_pdu!` generates, because its length field depends on the size of the body,
and because receivers need to read the header before they know which PDU type to
decode.

## `impl_pdu!`
 
`impl_pdu!` turns a deku struct into a PDU. One line per PDU:
 
```rust,ignore
crate::common::pdu::impl_pdu!(ServiceRequestPdu, PduHeader, PduType::ServiceRequest, ProtocolFamily::Logistics);
```

It implements the `Pdu` trait and adds a `new()` constructor. The generated methods
work like this:
 
**Encoding (`serialize`):**
 
1. Set the header's PDU type and protocol family.
2. Run deku's update rules (`DekuUpdate::update`), which recompute derived fields such
   as counts, lengths and flags from the data. See
   [Derived fields](#derived-fields-and-update-rules).
3. Append the header and then the body to the caller's `Vec<u8>`.
4. Compute the total length, header plus body, and fail if it's over 8192 octets.
5. Rewrite the header in place with the real length. If any step fails, the vector is truncated
   back to its original length so callers never see a partial PDU.

**Decoding (`deserialize`):**
 
1. Read the 12-octet header.
2. Check that its PDU type matches the struct being decoded.
3. Take exactly the number of octets the Length field gives. Fail if the buffer is
   shorter.
4. Decode the body from those octets only, so a bad PDU can't read into the next one.
5. Fail if any of those octets weren't used.
6. Attach the header to the decoded body and advance the caller's slice past the PDU.

`new()` is `Default::default()` plus the PDU type, family and length, so a new PDU is
always valid to send.

## Derived fields and update rules

The values of some fields are determined by the value of others: a count by the length
of the associated list, a record length by its contents, or a flag by whether an
optional field is present. Users should never have to set these, so each one has a
deku `update` rule:

```rust,ignore
#[deku(update = "self.supplies.len()")]
pub number_of_supply_types: u8,
padding: u16,
#[deku(count = "number_of_supply_types")]
pub supplies: Vec<SupplyQuantity>,
```

- `update` says how to compute the field when encoding.
- `count` says how many items to read when decoding.

When a record inside a list has its own derived fields, the list's count field runs
the record's update rules too, using `wire::update_each`:

```rust,ignore
#[deku(update = "crate::common::wire::update_each(&mut self.systems)?")]
pub number_of_systems: u8,
```

`update_each` updates every record and returns the list's length, so one rule covers
both jobs.

## Other field patterns

| Situation | Pattern | Example |
| --- | --- | --- |
| Padding | A private field named `padding`, `padding2` and so on, with the width the standard gives | `CollisionPdu` |
| Optional field controlled by a flag | `Option<T>` with `#[deku(cond = "…")]`, plus an `update` rule that sets the flag | `TimeSpacePositionInformationPdu` |
| A 16- or 32-bit count read from the wire | Add `assert = "crate::common::wire::fits_in_pdu(*count)"` | `AggregateStatePdu` |
| Variable-length record with a length field | Use the `wire` helpers `padded_record_length`, `record_fields_len` and `write_padded_fields` | `AttributeRecord` |
| A layout attributes can't describe | Custom `reader` and `writer` functions on the field | `MinefieldDataPdu`, `IFFPdu` |
| A record that needs a fully hand-written codec | Implement `DekuReader`, `DekuWriter` and `DekuUpdate` yourself, then call `impl_deku_default_endian!` | `GridAxisDescriptor` |

## Enumerations

The enumerations in `enums.rs` take three forms:
 
- **Ordinary enums** derive `FromPrimitive` and get their deku code from
  `impl_deku_enum!(Name, u8, from_u8)`. The macro is hand-written rather than derived
  because deku's derived reader for the largest enums, which have thousands of
  variants, needs megabytes of stack in debug builds.
- **Bit flags** use the `bitflags!` macro, with `impl_deku_bitflags!` for encoding.
  Unknown bits are kept, so flags from newer revisions round-trip unchanged.
- **PDU Status bits** derive modular-bitfield's `Specifier` so they can be packed into
  `PduStatusRecord`.

[Updating Enumerations](enumerations.md) covers how to change them.

## What is public and what is not

The public API is the PDU and record structs, the enumerations, the `Pdu` and
`GenericHeader` traits, and `DISError`. Everything that names a `deku` type is
internal: the `wire` module, `impl_pdu!`, `pdu_tests!` and `impl_deku_default_endian!`
are all private to the crate, and `DISError::WireFormat` wraps deku's error in the
opaque `WireFormatError` variant.

This keeps deku, which hasn't reached 1.0, out of the crate's stable API, so the crate
can upgrade deku without a breaking release. Keep it that way:

- Don't add a public function, trait method, or `From` impl that takes or returns a
  deku type. Convert deku errors with `DISError::from_wire`, which is `pub(crate)`.
- Public encoding and decoding go through `Vec<u8>` and `&[u8]`, not through deku's
  readers and writers or another crate's buffer types.

The structs still implement deku's `DekuRead` and `DekuWrite` because they're
derived. Those impls are visible in the API documentation, but they aren't part of the
stable API, and users shouldn't call them.

## Rules the code follows

This crate denies clippy's `pedantic` and `nursery` groups, plus a set of lints that
rule out panics. These are configured in `Cargo.toml`, and they shape how the code is
written:

- **No panics on any input.** No `unwrap`, `expect`, `panic!`, `unreachable!`, or
  `todo!` outside tests. Errors are propagated up to the caller as `DISError`.
- **No indexing.** Use `.get()` instead of `[i]`, and `.get(..n)` instead of slicing.
- **No unchecked arithmetic.** Use `checked_*` or `saturating_*` methods instead of
  `+`, `-`, and `*`.
- **No `as` casts.** Use `From` and `TryFrom` so that a value that does not fit is an
  error instead of being truncated. Generated code that needs `as` allows it within that
  scope.
- **No trust of count values from the wire.** A count field in a received PDU can say
  anything, especially if the PDU was constructed with a separate crate/library. Check
  it against the PDU size limit before allocating, with `fits_in_pdu` or `wire::read_n`,
  so a malformed/malicious PDU cannot make a receiver allocate an unreasonable amount
  of memory.
- **Keep encoding and decoding symmetric.** Every field read must be written the same
  way. The round-trip tests check this for every PDU.
