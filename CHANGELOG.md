# Open DIS Rust Changelog

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
