# Changelog

## [1.0.0] - YYYY-MM-DD

The first official release of open-dis-rust!

### Added

- Wire encoding through [deku](https://crates.io/crates/deku): enums, PDU headers, and every record is written and read using deku
- `common::wire` with `read`, `write`, and `encoded_len` for using those types with `bytes` buffers
  - `read` rejects undefined enum values
- `DISError::WireFormat` for truncated or malformed input
- `impl_pdu!` for PDUs defined as plain structs with deku attributes
- `IFFLayer2` and `IFFLayers` for the IFF PDU's layers after `IFFLayer1`

### Changed

- `TimeSpacePositionInformationPdu`: the fields after the location are optional as indicated by the TSPI flag specifies. They are `Option`s,
  and `serialize` sets the flag (now `tspi_flag: TspiFlag`, formerly `tspi_flag: u8`) from the fields that are present. A default TSPI PDU
  is 25 octects
  - `system_specific_data` is now an `Option<Vec<u8>>` and its length is written automatically, so `system_specific_data_length` has been removed
- `AppearancePdu`: optional fields are read when their flags are set, and `serialize` sets the flags from the fields that are present
- `AppearancePdu::capabilities` and `EntityStatePdu::entity_capabilities` are raw `u32` capability bits instead of `EntityCapabilities`
- `VariableTransmitterParameters::record_type` is `VariableRecordTypes` (UID 66)

### Removed

- The `pdu_macro` module: `FieldSerialize`, `FieldDeserialize`, `FieldLen`, `FieldDeserializeWithLen`, `define_pdu!`, `impl_record_fields!`, and
  `impl_u8_flag_fields!`. Define PDUs as structs deriving `deku::DekuRead` and `deku::DekuWrite` with `impl_pdu!`
  - `pdu_tests!` is unaffected
- The `serialize`/`deserialize` methods on records: use `common::wire::write` and `common::wire::read`
- The `deserialize` methods on enums, which turned undefined values into defaults
- The unused `PduBody` trait 

### Fixed

- `EntityMarking` always encodes to 12 octets, and decoding reads its 11 characters instead of storing the number of bytes left in the buffer
- `VariableDatumRecord` decoding no longer doubles the value and now skips the padding
- `DeadReckoningParameters` encodes `dead_reckoning_other_paramters` instead of 15 octets of `0`
- `SimulationIdentifier` encodes `reference_number` instead of `0`
- `VariableTransmitterParameters` decodes its record type as 32 bits, matching how it is encoded
- Counts and lengths read from the wire are checkec against the maximum PDU size before anything is allocated
- Decoding a truncated or malformed record no longer panics
- Lists and optional fields in every PDU now decode instead of coming back empty
- `AggregateStatePdu` pads its ID lists to a 32-bit boundary according to how many IDs they hold

## [0.4.3] - 2026-07-03

### Added

- New `get_pdu_type` function in `PduHeader` to retrieve the `PduType` from a raw byte buffer, if possible
- Unit tests for `get_pdu_type` function validation
- Examples demonstrating new functionality

### Fixed

- Parameter for `deserialize` function in `GenericHeader` had a typo; now correctly named `buf`

## [0.4.2] - 2026-02-07

### Added

- Introduced some missing constants that were previously missing according to IEEE 1278.1-2012 Section 6.1.8

### Fixed

- Corrected improper formatting and rendering of the book
- Updated the casing of enumeration variants for better readability and clarity

## [0.4.1] - 2026-02-04

### Fixed

- The PduHeader record was missing a byte of padding at the end, leading to serialization and deserialization issues

## [0.4.0] - 2025-12-04

### Added

- `FixedDatumRecord` and `VariableDatumRecord` according to sections 6.2.37 and 6.2.93
- `SimulationIdentifier` record according to section 6.2.81
- `AttributePdu` according to section 7.2.6
- `AttributeRecord` and `AttributeRecordSet` according to section 6.2.10
- `AntennaPattern`, `ModulationParameters`, and `VariableTransmitterParameters` according to sections 6.2.8, 6.2.58, and 6.2.95
- `ObjectIdentifier` according to section 6.2.63
- All PDUs now have a `new` method to discourage use of the `default` method
- A `length` method has been added to the `Pdu` trait enabling the calculation of a PDU's serialized length

### Changed

- A `define_pdu` macro is now used for defining all the associated serialization and deserialization methods for PDUs,
  massively cleaning up the code
- The file structure of this package has been flattened; PDUs in the same protocol family are now defined in the same module file
- All data types and records have been migrated to the `common` module under `common::data_types`
- A `GenericHeader` trait has been defined in preparation for a future release that will be adding the Live Entity protocol family
- `pdu_header` field is private in all PDUs now
  - Mutable and immutable accessor methods are now available for this field
- Deserialization has been genericized by allowing any `B: Buf` type to be used
- `serialize` now returns a `Result<(), DISError>` to handle the cases when serialization may fail
- Updated SECURITY.md
- Updated copyright year to 2025
- Updated book theme (now uses catppuccin mocha)

### Fixed

- Many data types and PDUs had incorrect default lengths that have since been corrected according to the standard
  - Related to this, many of the `check_default_pdu_length` unit tests were incorrect

### Removed

- `create_header` and `deserialize_header` unit tests have been removed
- Documentation comments have been removed from `default` impl to discourage its use when constructing PDUs

## [0.3.0] - 2025-11-04

### Added

- Added `common::constants` module which contains useful constants defined in the IEEE 1278.1-2012 standard (#66)
- Re-exported all modules in `common` and the various protocol family modules (#66)
- More `DISError` variants

### Changed

- Renamed all `decode` methods to `deserialize` (#66)

### Fixed

- `PduHeader` was missing the `PduStatusRecord` defined in section 5.2.7 of the IEEE 1278.1-2012 standard (#66)

### Removed

- `serde` is no longer a dependency, and all `serde` derive macros have been removed (#66)

## [0.2.0] - 2025-02-28

### Changed

- Updated to the Rust 2024 edition (#62)

## [0.1.1] - 2024-12-10

### Added

- Created a book that will generate with `mdbook` in the CI pipeline upon each tagged release (#58)

### Changed

- Bumped supported Rust compiler version to `1.82` (#52)
- Improved code documentation (#58)
  - Added top-level crate docs to `lib.rs` that will display on the front page of docs.rs
  - Added module-level documentation for the protocol family modules

### Fixed

- Fixed improperly sized message fields (#57)
  - Changed type of `beam_parameter_index` in `ElectromagneticEmissionBeamData` from `u8` to `u16`
  - Changed type of `emissions_padding2` in `ElectromagneticEmissionSystemData` from `u8` to `u16`
  - Changed type of `padding2` in `DesignatorPdu` from `i8` to `u16`
  - Changed type of `padding_for_emissions_pdu` in `ElectromagneticEmissionsPdu` from `u8` to `u16`
  - Changed type of `pad` in `UnderwaterAcousticPdu` from `i8` to `u8`
- Updated default size of `ElectromagneticEmissionsPdu` from 28 to 31 bytes (#57)
- Properly assigned enumeration types to certain fields (#57)
  - Changed type of `code_name` in `DesignatorPdu` from `u8` to `DesignatorSystemName`
  - Changed type of `designator_code` in `DesignatorPdu` from `u8` to `DesignatorCode`
  - Changed type of `state_change_indicator` in `UnderwaterAcousticPdu` from `i8` to `UAStateChangeUpdateIndicator`
  - Changed type of `passive_parameter_index` in `UnderwaterAcousticPdu` from `u16` to `UAPassiveParameterIndex`

## [0.1.0] - 2024-10-26

No changes.

## [0.1.0-beta.1] - 2024-10-26

### Added

- Added missing documentation and updated existing documentation for
  all PDUs and associated data types (#44)

### Changed

- Updated `bytes` dependency to v1.8.0 (#48)
- Updated `futures` dependency to v0.3.31 (#48)

### Fixed

- Patched security errors flagged by Clippy (#46)

## [0.1.0-beta.0] - 2024-08-05

### Changed

- Move Non-PDU Data Types into Separate Data Type Directory (#39)

### Fixed

- Clippy fixes (#41)

## [0.1.0-alpha.10] - 2024-08-04

### Changed

- Finished Definition of Gridded Data PDU (#35)
- Move All Enums into Separate Module (#30)

### Fixed

- Fix Default PDU Sizes and Adjust Size When Serializing (#36)
