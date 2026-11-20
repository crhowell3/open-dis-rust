# Changelog

## [1.0.0] - 2026-11-19

The first official release of open-dis-rust!

### Added

- Wire encoding through [deku](https://crates.io/crates/deku): enums, PDU headers, and every record is written and read using deku
  - Decoding rejects undefined enum values
  - deku is an implementation detail and is not part of the public API
- `Pdu::to_bytes` method, which encodes a PDU into a new `Vec<u8>`
- `records::Timestamp`, a type for DIS timestamps, also in the prelude. `Timestamp::now()`, `absolute()` and
  `relative()` create one; `is_absolute()`, `time_past_hour()` and `duration_since()` read one, and
  `duration_since()` allows for the hour wrapping around
- `ClockTime::now()`
- `DISError::WireFormat` and `WireFormatError` for truncated or malformed input
- `RecordReliablePdu` (Record-R, PDU type 63), so every PDU type in IEEE 1278.1-2012 is implemented
- The `prelude` module for importing the most common traits and types
- `IffLayer2` and `IffLayers` for the IFF PDU's layers after layer 1
- Every record in §6.2 of the standard, in `records` in section order, including the bit records
  `BeamStatus`, `ChangeOptions`, `InformationLayers`, `ProtocolMode`, `SystemStatus` and `JammingTechnique`,
  and `MinefieldSensorType`. Each bit record has a getter and setter per field and keeps unused bits as received
- `ExerciseIdentifier` for the header's exercise ID, which defaults to 1, and `RequestIdentifier`
- `EntityAppearance` and `EntityCapabilities`, 32-bit records kept as received, and typed views of entity appearance
  for each SISO-REF-010-2023 layout (UIDs 31-43 and 480), such as `LandPlatformAppearance` and `HumanLifeFormAppearance`.
  `EntityAppearance::view` picks the layout from an `EntityType`
- `ArealObjectStateMinefieldAppearance` (UID 239), `ArealObjectStateModification` (UID 242), and `ComponentVisualDamageStatus` (UID 317)
- `StandardPdu` and `LiveEntityPdu`, implemented for every PDU with a `PduHeader` and a `LiveEntityPduHeader`
  respectively, to tell the two kinds of PDU apart in generic code. Both are in the prelude
- `Pdu::pdu_type()` and `Pdu::protocol_family()`, which give the PDU's type and family without reading its header
- `AnyPdu`, which decodes a PDU of any type. It has one variant per PDU type, named after the `PduType` it encapsulates.
  `AnyPdu::deserialize` decodes one PDU, picking its type from the header, and `AnyPdu::deserialize_all` decodes every
  PDU in a datagram. Every PDU converts into an `AnyPdu` with `From`, and `AnyPdu` is in the prelude

### Changed

- The public API uses standard library types instead of the `bytes` crate, and `bytes` is no longer a direct dependency
  - `Pdu::serialize` appends to a `&mut Vec<u8>` instead of a `&mut BytesMut`; on error, the buffer is left unchanged
  - `Pdu::deserialize` and `Pdu::deserialize_without_header` read from a `&mut &[u8]` instead of any `B: Buf`
  - `PduHeader::serialize` and `PduHeader::deserialize`, and the same on `LiveEntityPduHeader`, change the same way
  - `Vec<u8>`, `&[u8]`, `bytes::Bytes` and `bytes::BytesMut` all work: pass `&mut &buf[..]` to decode from any of them
- `TimeSpacePositionInformationPdu`: the fields after the location are optional as indicated by the TSPI flag. They are `Option`s,
  and `serialize` sets the flag (now `tspi_flag: TspiFlag`, formerly `tspi_flag: u8`) from the fields that are present. A default TSPI PDU
  is 25 octets
  - `system_specific_data` is now an `Option<Vec<u8>>` and its length is written automatically, so `system_specific_data_length` has been removed
- Header timestamps and `ClockTime::time_past_hour` are `Timestamp`s instead of `u32`s. Use `Timestamp::now()`
  where you used `PduHeader::calculate_dis_timestamp()`, and `Timestamp::from_raw()` and `to_raw()` for the wire value
- Fields that encoding derives are read-only, so they can no longer be set and then overwritten:
  - The header's PDU type, protocol family and length. Read them with the header's `pdu_type()`,
    `protocol_family()` and `length()` methods
  - Record and blob lengths, such as `Attribute::record_length` and `TransmitterPdu::antenna_pattern_length`
  - The Signal and Intercom Signal PDUs' `data_length` stays writable, because it's in bits
- `Pdu::finalize` has no default implementation
- `PduHeader::new` and `LiveEntityPduHeader::new` take only an exercise ID
- `AppearancePdu`: optional fields are read when their flags are set, and `serialize` sets the flags from the fields that are present
- `AppearancePdu::capabilities` and `EntityStatePdu::entity_capabilities` are `records::EntityCapabilities`, and entity
  appearances are `records::EntityAppearance`, instead of `u32`s
- The PDU Status record is `records::PduStatus`, in the header's `pdu_status` field (formerly `status_record`), encoded
  with deku. Its accessors drop the `get_` prefix (`dti()`,
  `rai()`, ...), `dti()`, `rai()` and `iai()` return `None` for the undefined value 3, and `new`, `from_raw` and `to_raw`
  replace `new_zero`, `from_u8` and `to_u8`
- `ObjectStateAppearanceGeneral` (UID 229) has a getter and setter for each field, such as `percent_complete()` and
  `damage()`, instead of one flag per field. `PointObjectStatePdu::general_object_appearance` uses it instead of `u16`
- `PointObjectStatePdu`'s object IDs are `ObjectIdentifier`s and `MinefieldResponseNackPdu::minefield_id` is a
  `MinefieldIdentifier`, matching the other object state and minefield PDUs
- `TransmitterPdu::antenna_pattern` is a `BeamAntennaPattern`, read when `antenna_pattern_length` is 40 and set by
  `serialize`
- `IFFFundamentalParameterData::applicable_modes` is `IFFApplicableModes` (UID 339)
- `VariableTransmitterParameters::record_type` is `VariableRecordTypes` (UID 66)
- `RecordREventType` is now a `u16` to match the standard
- `EntityId` is now `EntityIdentifier`
- The modules are organized by where their contents come from, and every public type has one path:
  - IEEE 1278.1: the PDU family modules, and the top-level `records`, `live_entity_records` and `constants` modules,
    which replace `common::records`, `common::live_entity_records` and `common::constants`
  - The crate: `Pdu`, `StandardPdu`, `LiveEntityPdu`, `DISError`, `DISResult` and `WireFormatError` are at the crate
    root, instead of in `common`, `common::pdu` and `common::dis_error`
  - The `common` and `common::data_types` modules are gone. The records that were in `data_types` are in `records`,
    and the parts of a single PDU are in that PDU's module: the Electromagnetic Emission and Underwater Acoustic
    system and beam data, `ApaData` and `ShaftRPMs` in `distributed_emissions`, `AttributeRecordSet` in
    `entity_information`, and `Point` in `minefield`
- Everything SISO-REF-010 defines is in a new top-level `siso` module:
  - The enumerations are in `siso::enums` instead of `common::enums`
  - The entity appearance views, `ObjectStateAppearanceGeneral` (UID 229) and `ArealObjectStateMinefieldAppearance`
    (UID 239) are in `siso::appearance`
  - `FrozenBehavior`, `ArealObjectStateModification`, and `ComponentVisualDamageStatus` are in `siso::bit_records`
- The Live Entity records, `LiveEntityPduHeader`, and `FixedBinary8` and `FixedBinary16` records are all in `live_entity_records`.
  `LiveEntityRelativeLocation` and `LiveMunitionDescriptor` moved there from `live_entity_information`

### Removed

- `Pdu::as_any`. Use `AnyPdu` to handle PDUs whose type is not known at compile time
- The constants the standard doesn't define, `BITS_PER_BYTE`, `MAX_ARTICULATION_PARAMS` and
  `MAX_ENTITY_MARKING_LENGTH`, and the unused helpers `validate_pdu_size`, `total_pdu_size` and `validate_marking_length`
- The `pdu_macro` module: `FieldSerialize`, `FieldDeserialize`, `FieldLen`, `FieldDeserializeWithLen`, `define_pdu!`, `impl_record_fields!`, and
  `impl_u8_flag_fields!`. PDUs are now defined internally with deku, and the macros that define and test them are
  private to the crate
- The `bytes` and `chrono` dependencies
- `PduHeader::calculate_dis_timestamp` and `LiveEntityPduHeader::calculate_dis_timestamp`: use `Timestamp::now()` instead
- The `SerializedLength` trait. Every PDU header is 12 octets: use `constants::PDU_HEADER_SIZE` which is also in the prelude
- The `GenericHeader` trait. `PduHeader` and `LiveEntityPduHeader` have `pdu_type()`, `protocol_family()`,
  `length()`, `serialize()`, and `deserialize()` as their own methods, and `Pdu::pdu_type()` and `Pdu::protocol_family()`
  cover generic code
- The `modular-bitfield` dependency, and the `From` conversions between `u8` and the PDU Status enums
- `enums::EntityCapabilities` (UID 55 lists the capability layouts; its values are not sent on the wire) and the
  `bitflags` `enums::LandPlatformAppearance`, which could not hold its multi-bit fields
- `live_entity_information::PlatformAppearance`, `FrozenBehavior::as_u8` and `from_u8`, and the `as_u16` and
  `from_u16` of `ObjectStateAppearanceGeneral`
- `PduState` and the `DISError` variants the crate never returned: `InvalidStateTransition`,
  `ProtocolVersionMismatch`, `InvalidFieldValue` (with `DISError::invalid_field`) and `NetworkError`
- The `deserialize` methods on enums, which turned undefined values into defaults
- The unused `PduBody` trait
- `Vector3Double` and `VelocityVector` in favor of the `Vector` record described in the standard

### Fixed

- `EntityMarking` always encodes to 12 octets, and decoding reads its 11 characters instead of storing the number of bytes left in the buffer
- `VariableDatumRecord` decoding no longer doubles the value and now skips the padding
- `DeadReckoningParameters` encodes `dead_reckoning_other_parameters` instead of 15 octets of `0`
- `SimulationIdentifier` encodes `reference_number` instead of `0`
- `VariableTransmitterParameters` decodes its record type as 32 bits, matching how it is encoded
- Counts and lengths read from the wire are checked against the maximum PDU size before anything is allocated
- Decoding a truncated or malformed record no longer panics
- Lists and optional fields in every PDU now decode instead of coming back empty
- `AggregateStatePdu` pads its ID lists to a 32-bit boundary according to how many IDs they hold
- `DataQueryPdu` carries 32-bit datum IDs instead of full datum records
- `GridData` records' sample type and data representation are 16 bits
- `IFFFundamentalParameterData::burst_length` is a 32-bit integer
- The Appearance PDU's Flag-2 marks Appearance-EM with bit 0 and Appearance-Audio with bit 1, instead of bits 1 and 2
- `LinearSegmentParameter::general_segment_appearance` is the 16-bit General Object Appearance record (`ObjectStateAppearanceGeneral`) instead of a `u32`, so each segment is 64 octets
- `IOEffectsRecordIOProcess` is a 16-bit enumeration, so the IO Effect record is 16 octets, matching its Record length
- `Sensor::station` is a 32-bit enumeration value (`u32`)
- `MinefieldResponseNackPdu` carries 8-bit missing PDU sequence numbers followed by padding to a 32-bit boundary, instead of 64-bit numbers with no padding
- `IntercomControlPdu::intercom_parameters_length` is the length of the parameters in octets instead of a record count, and `IntercomCommunicationsParameters` carries `record_length` octets of record-specific fields padding to a 32-bit boundary instead of one `u32`. `IntercomCommunicationsParameters::new` takes the record type and the fields
- `DirectedEnergyFirePdu` carries its DE records as Standard Variable records in `de_records`, and `EntityDamageStatusPdu::damage_descriptions` holds Standard Variable records, so DE Area and Precision Aimpoint records and other record types decode. They were fixed 40-octet Damage Description records. `StandardVariableRecord::from_record` and `to_record` convert between typed records and Standard Variable records
- `AggregateStatePdu` carries its silent aggregate systems as `SilentAggregateSystem`s (a count, padding and an Aggregate Type) and its silent entity systems as `SilentEntitySystem` records, instead of bare `EntityType`s
- `Environment` records carry their geometry or state record in `geometry_or_state`, padded to a 64-bit boundary, with `length` in bits set by `serialize`, so Environmental Process PDUs with real environment records decode. `Environment::new` takes the type, index and geometry or state octets
- `serialize` sets the EE PDU's System Data Length and Beam Data Length and the UA PDU's Emitter System Data Length and Beam Data Length in 32-bit words, or 0 for an EE system or beam over 255 words, instead of sending whatever was set (0 by default)
- `LiveEntityFirePdu` and `LiveEntityDetonationPdu` follow their flag fields: optional fields are `Option`s that `serialize` signals in `LEFireFlags`, `LEDetonationFlags1` and `LEDetonationFlags2`, munition and event IDs leave out site and application numbers equal to the firing entity's, the Detonation PDU carries one of its two locations, and neither has a padding octet. `LiveMunitionDescriptor` has optional `WarheadAndFuse` and `QuantityAndRate` parts, and the Detonation result is a `DetonationResult`
- `DEAreaAimpoint` is padded to a 64-bit boundary, `serialize` sets its `record_length`, and `DEAreaAimpoint::new` sets its record type (4001)

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
- All data types and records have been migrated to the `common` module under `common::data_types` and `common::records`
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
