# Changelog

## [0.4.0] - 2025-12-04

### Added

- `FixedDatumRecord` and `VariableDatumRecord` according to sections 6.2.37 and 6.2.93
- `SimulationIdentifier` record according to section 6.2.81
- `AttributePdu` according to section 7.2.6
- `AttributeRecord` and `AttributeRecordSet` according to section 6.2.10
- `AntennaPattern`, `ModulationParameters`, and `VariableTransmitterParameters` according to sections 6.2.8, 6.2.58, and 6.2.95
- `ObjectIdentifier` according to section 6.2.63
- All PDUs now have a `new` method to discourage use of the `default` method
- All PDUs now have a private `deserialize_body` method that is invoked when calling either `deserialize` or `deserialize_without_header`
- A `length` method has been added to the `Pdu` trait enabling the calculation of a PDU's serialized length
- All PDUs and most data types now have a `SerializedLength` trait that is used for determining the length of the data when serialized

### Changed

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
