# Quickstart

In this walkthrough, you'll run two small programs. One broadcasts an Entity State PDU
for a tank once a second. The other listens for PDUs and prints what it hears. This
walkthrough takes about 5 minutes.

Both programs live in the repository's `examples` directory. You can run them from a
clone of the repository, as shown at the end of this page, or copy them into your
own project.

## 1. Send an Entity State PDU

An Entity State PDU is the heartbeat of DIS. Every simulator sends one for each entity
it owns, at regular intervals, so that everyone else knows where that entity is and
what it is doing.

```rust,ignore
{{#include ../../examples/quickstart_send.rs}}
```

A few things to notice:

- **`EntityStatePdu::new()`** gives you a PDU with sensible defaults and the header
  already set up. You only fill in the fields you care about.
- **Entity IDs** consist of three numbers: site, application, and entity IDs. Together
  they must be unique within the exercise. It should be noted that this crate does NOt
  enforce this constraint. That is completely up to the consumer to ensure uniqueness.
- **Entity types** come from the SISO-REF-010 enumerations.
