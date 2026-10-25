//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct IntercomCommunicationsParameters {
    pub record_type: u16,
    pub record_length: u16,
    pub record_specific_field: u32,
}

impl IntercomCommunicationsParameters {
    #[must_use]
    pub const fn new(record_type: u16, record_length: u16, record_specific_field: u32) -> Self {
        Self {
            record_type,
            record_length,
            record_specific_field,
        }
    }
}
