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
pub struct AggregateId {
    pub site: u16,
    pub application: u16,
    pub aggregate_id: u16,
}

impl AggregateId {
    #[must_use]
    pub const fn new(site: u16, application: u16, aggregate_id: u16) -> Self {
        Self {
            site,
            application,
            aggregate_id,
        }
    }
}

