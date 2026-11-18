//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The symbolic names and size limits of IEEE 1278.1-2012 (§5 and Table 25), such as
//! `ALL_SITES` for an identifier that means every site.

/// An entity number representing all entities
pub const ALL_ENTITIES: u16 = 0xFFFF;
/// An application number representing all applications
pub const ALL_APPLIC: u16 = 0xFFFF;
/// A site number representing all sites
pub const ALL_SITES: u16 = 0xFFFF;

/// An entity number representing no entity, or not applicable
pub const NO_ENTITY: u16 = 0x0000;
/// An application number representing no application, or not applicable
pub const NO_APPLIC: u16 = 0x0000;
/// A site number representing no site, or not applicable
pub const NO_SITE: u16 = 0x0000;

/// The largest PDU, in octets
pub const MAX_PDU_SIZE_OCTETS: usize = 8192;
/// The largest PDU, in bits
pub const MAX_PDU_SIZE_BITS: usize = 65_536;

/// The size of the PDU header, in octets. The Live Entity PDU header is the same size
pub const PDU_HEADER_SIZE: usize = 12;

/// An aggregate number that means every aggregate
pub const ALL_AGGS: u16 = 0xFFFF;
/// An emitter number that means every emitter
pub const ALL_EMITTERS: u16 = 0xFF;
/// A beam number that means every beam
pub const ALL_BEAMS: u16 = 0xFF;
/// An object number that means every object
pub const ALL_OBJECTS: u16 = 0xFFFF;

/// An Environmental Process PDU sequence number that means the process is not sent
/// in a sequence
pub const EP_NO_SEQUENCE: u16 = 0xFFFF;

/// An aggregate number that means no aggregate
pub const NO_AGG: u16 = 0x0000;
/// A beam number that means no beam
pub const NO_BEAM: u16 = 0x0000;
/// A category that means no category
pub const NO_CATEGORY: u16 = 0x0000;
/// An emitter number that means no emitter
pub const NO_EMITTER: u16 = 0x0000;
/// A fire mission index that means no fire mission
pub const NO_FIRE_MISSION: u16 = 0x0000;
/// An entity kind that means no kind
pub const NO_KIND: u16 = 0x0000;
/// An object number that means no object
pub const NO_OBJECT: u16 = 0x0000;
/// An antenna pattern value that means no pattern
pub const NO_PATTERN: f32 = 0.0;
/// A reference number that means no reference
pub const NO_REF_NUMBER: u16 = 0x0000;
/// A specific value that means none
pub const NO_SPECIFIC: u16 = 0x0000;
/// A subcategory that means no subcategory
pub const NO_SUBCAT: u16 = 0x0000;
/// A value that means none
pub const NO_VALUE: u16 = 0x0000;

/// Engine power, in percent, for an engine that is off
pub const POWER_ENGINE_OFF: f32 = -100.0;
/// Engine power, in percent, at idle
pub const POWER_IDLE: f32 = 0.0;
/// Engine power, in percent, at maximum afterburner
pub const POWER_MAX_AFTERBURNER: f32 = 100.0;
/// Engine power, in percent, at military power
pub const POWER_MILITARY: f32 = 50.0;
/// Engine power, in percent, at minimum afterburner
pub const POWER_MIN_AFTERBURNER: f32 = 51.0;

/// An entity number that asks the receiver to assign one
pub const RQST_ASSIGN_ID: u16 = 0xFFFE;
