//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

#[derive(Copy, Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct SecondaryOperationalData {
    pub operational_data1: u8,
    pub operational_data2: u8,
    pub number_of_iff_fundamental_parameter_records: u16,
}

impl SecondaryOperationalData {
    #[must_use]
    pub const fn new(
        operational_data1: u8,
        operational_data2: u8,
        number_of_iff_fundamental_parameter_records: u16,
    ) -> Self {
        Self {
            operational_data1,
            operational_data2,
            number_of_iff_fundamental_parameter_records,
        }
    }
}
