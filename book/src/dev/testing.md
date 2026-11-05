# Testing Strategy

A DIS codec has two separate jobs: its encoder and decoder must agree with each other,
and both must agree with the standard and with other DIS implementations. The current
tests cover the first objective well. This page describes them, then the tests needed for
the second objective.

## What the tests cover today

Run everything with `cargo test`.

### PDU tests

Every PDU has a `pdu_tests!` entry at the bottom of its module, which generates these
tests:

| Test | Checks |
| --- | --- |
| `cast_to_any` | `as_any()` returns the right type. |
| `new_sets_pdu_type` | `new()` sets the PDU type in the header. |
| `new_sets_default_length` | `new()` sets the header length to the expected default size. |
| `serialized_length_matches_header` | The encoded size equals the header's Length field. |
| `round_trip_is_byte_identical` | Encoding, decoding and encoding again produces identical bytes and an identical header. |
| `rejects_wrong_pdu_type` | Decoding a different PDU type as this one fails. |

The **default length** test is the only one that checks the crate against the standard,
and only for an empty PDU. Its expected size should be worked out from the
standard by hand.

### Enumeration tests

`common/enums/deku_tests.rs` checks that every enumeration encodes to the right number
and that undefined numbers are rejected. 8- and 16-bit enums are tested with every
possible value. 32-bit enums are tested at every defined value, the values next to
them, and a few far-off ones.

### Unit tests

A few modules, including `pdu_header.rs` and `constants.rs` have their own unit tests
for specific behavior such as `get_pdu_type`.

## What round-trip tests cannot catch

A round-trip test passes whenever the encoder and decoder agree, even if both are wrong.
If a record has a padding field that is 16 bits instead of 8, the encoder writes one
extra octet, the decoder reads one extra octet, and the round trip passes. Only
another DIS implementation would notice the error.

Every problem listed in [Conformance Notes](../reference/conformance.md) is of this
kind, and all of them pass the current tests.

## Golden-byte tests

A golden-byte test compares the crate's output with bytes that came from outside
the crate; that is, written out by hand from the standard's field tables, or captured
from another DIS implementation.

Put them in `tests/golden.rs`. Write each fixture as hex, laid out one field per line with
a comment, so a reviewer can check it against the standard line by line:

```rust,ignore
#![allow(clippy::unwrap_used, reason = "test helpers")]
 
use open_dis_rust::{
    common::{
        Pdu,
        data_types::EntityId,
        enums::{AcknowledgeFlag, AcknowledgeResponseFlag},
    },
    simulation_management::AcknowledgePdu,
};
 
const ACKNOWLEDGE: &str = "
    07 01 0F 05   00 00 00 00   00 20 00 00   # header: v7, exercise 1, type 15, family 5, timestamp 0, length 32
    00 01 00 01 00 00                         # originating entity 1:1:0
    00 01 00 02 00 00                         # receiving entity 1:2:0
    00 01                                     # acknowledge flag: Create Entity
    00 01                                     # response flag: Able to Comply
    00 00 00 2A                               # request ID 42
";

/// Parses a fixture: hex digits, with whitespace and `# comments` ignored.
fn fixture(s: &str) -> Vec<u8> {
    let digits: Vec<u8> = s
        .lines()
        .flat_map(|line| line.split('#').next().unwrap().bytes())
        .filter(|b| !b.is_ascii_whitespace())
        .collect();
    digits
        .chunks(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}
 
#[test]
fn acknowledge_encodes_to_golden_bytes() {
    let mut pdu = AcknowledgePdu::new();
    pdu.header_mut().timestamp = Timestamp::from_raw(0); // fixed, so the bytes are repeatable
    pdu.originating_entity_id = EntityId::new(1, 1, 0);
    pdu.receiving_entity_id = EntityId::new(1, 2, 0);
    pdu.acknowledge_flag = AcknowledgeFlag::CreateEntity;
    pdu.response_flag = AcknowledgeResponseFlag::AbleToComply;
    pdu.request_id = 42;
 
    assert_eq!(pdu.to_bytes().unwrap(), fixture(ACKNOWLEDGE));
}
 
#[test]
fn acknowledge_decodes_from_golden_bytes() {
    let bytes = fixture(ACKNOWLEDGE);
    let pdu = AcknowledgePdu::deserialize(&mut bytes.as_slice()).unwrap();
    assert_eq!(pdu.receiving_entity_id, EntityId::new(1, 2, 0));
    assert_eq!(pdu.response_flag, AcknowledgeResponseFlag::AbleToComply);
    assert_eq!(pdu.request_id, 42);
}
```

Test both directions for each fixture. **Encoding** checks that the crate writes
exactly these bytes. **Decoding** checks that it reads the right values from them.

Set the timestamp to a fixed value, such as 0, before encoding. Otherwise, the bytes
change on every run.

### Where fixtures come from

In order of preference:

1. **Captures from other implementations,** such as
  [OpenDIS for Java, C++ or Python](https://github.com/open-dis), or a commercial tool.
  These test real interoperability. Save the PDU's bytes from a Wireshark capture
  (right-click the DIS layer, then **Copy -> ...as a Hex Stream**) and note where it came
  from in a comment.
2. **Hand-written from the standard,** as in the example above. Fill every field with a
  different, recognizable value, so that a field in the wrong place shows up clearly.
3. **Never from open-dis-rust itself.** A fixture produced by the encoder under test
  only checks that the encoder hasn't changed.

Prioritize golden tests for PDUs with variable-length records, padding inside lists,
or length fields measured in bits or 32-bit words. Those are where the current bugs
are.

## Fuzzing

Decoding must never panic no matter what bytes arrive. The crate's lints rule out most
panics, but fuzzing checks for the rest along with unbounded memory use.
[cargo-fuzz](https://rust-fuzz.github.io/book/cargo-fuzz.html) works well for this. A
useful target decodes arbitrary bytes as each PDU type and, when decoding succeeds,
checks that encoding the result reproduces the input:

```rust,ignore
#![no_main]
use libfuzzer_sys::fuzz_target;
use open_dis_rust::{common::Pdu, entity_information::EntityStatePdu};
 
fuzz_target!(|data: &[u8]| {
    let mut input = data;
    if let Ok(mut pdu) = EntityStatePdu::deserialize(&mut input) {
        let mut out = bytes::BytesMut::new();
        pdu.to_bytes().expect("a decoded PDU must re-encode");
    }
});
```

## Checking against Wireshark

Wireshark's DIS dissector is an independent implementation of the standard, and its
command-line version, `tshark`, can run in CI. Write the crate's PDUs to a `.pcap` file
in a test, then run `tshark -r out.pcap -V` and fail if it reports any packet as
malformed. This catches layout errors in PDUs that don't have golden-byte tests yet.

## Continuous integration

CI runs on every pull request and push to `main`. It currently:
 
- runs `cargo test` on nightly Rust
- builds on stable and beta
- runs clippy and uploads the results to GitHub code scanning

Before you push, run the same checks locally:
 
```shell
cargo fmt --check
cargo clippy --all-targets
cargo test
cargo build --examples
mdbook build book
```
 
Improvements the CI would benefit from:
 
- **Run the tests on stable**, the toolchain users build with, as well as nightly.
- **Fail the build on clippy warnings.** The clippy step currently has
  `continue-on-error: true`, so the strict lints in `Cargo.toml` aren't enforced.
- **Check formatting** with `cargo fmt --check`.
- **Build the examples and the book,** so the Quickstart code and the book's code
  samples will not fall out of date.
