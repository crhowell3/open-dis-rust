# Enumerations (SISO-REF-010)

This crate adheres to the IEEE 1278.1-2012 DIS standard, and that standard leverages enumerations
defined in a separate document, **SISO-REF-010**, *Reference for Enumerations for Simulation Interoperability*.

SISO-REF-010 is published by the [Simulation Interoperability Standards Organization (SISO)](https://www.sisostandards.org/)
and is free to download. It is revised regularly to add new countries, platforms, weapons, and emitters. Unlike the IEEE standard,
it changes from year to year.

This crate specifically leverages the enumerations defined in **SISO-REF-010-2023**.

## How enumerations appear in the crate

All enumerations are in `common::enums`. Each comes from one table in SISO-REF-010, which
identifies its tables by a **UID** number. They take three forms:

- **Rust enums** for fields with a fixed list of possible values. Most enumerated fields in a PDU
  use these types directly.

```rust,ignore
  use open_dis_rust::common::enums::ForceId;

  pdu.force_id = ForceId::Friendly;
```

- **Bit flags** for fields where several options can be set at once. These use the
  [`bitflags`](https://docs.rs/bitflags) crate, so you combine them with `|`.

```rust,ignore
  use open_dis_rust::common::enums::FrozenBehavior;

  // Keep the simulation clock running and keep sending updates while frozen.
  stop.frozen_behavior = FrozenBehavior::RunSimulationClock | FrozenBehavior::TransmitUpdates;
```

- **Plain integers** for fields whose meaning depends on another field. An entity type's category,
  for example, means one thing for a land vehicle and another for an aircraft. The crate has an enum
  for each of those tables, but the field itself is a plain old data type, such as a `u8`.

## Finding a value

Variant names follow the names in SISO-REF-010, with spaces and punctuation removed. For example,
"United States of America" bcomes `UnitedStatesofAmerica`, and "Able to Comply" becomes `AbleToComply`.
Search the [API documentation](https://docs.rs/open-dis-rust) for the name, or search `enums.rs`
for the table name or its UID number.

## Values unknown to the crate

When decoding, a value that is not in the crate's table is an error. The whole PDU faisl with
`DISError::WireFormat`. This usually happens when another simulator uses a newer version of
SISO-REF-010. See [Conformance Notes](../reference/conformance.md#decoding-behavior) for details.

If you encounter an issue like this, please [report it](../get-in-touch.md#encoding-and-interoperability-problems)
with the value and the PDU type.

## Deprecated values

When SISO retires a value, it stays in the tables so that old traffic can still be
read, but it should not be used in new PDUs. The crate marks these variants `#[deprecated]`,
so the compiler warns you if you use one:

```text
warning: use of deprecated unit variant `open_dis_rust::common::enums::LandDomainCategories::CivilianVehicle`: Deprecated in SISO-REF-010-2023
```
