# Open DIS Rust Changelog

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
