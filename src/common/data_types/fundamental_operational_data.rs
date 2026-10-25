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
pub struct FundamentalOperationalData {
    pub system_status: u8,
    pub data_field1: u8,
    pub information_layers: u8,
    pub data_field2: u8,
    pub parameter1: u16,
    pub parameter2: u16,
    pub parameter3: u16,
    pub parameter4: u16,
    pub parameter5: u16,
    pub parameter6: u16,
}

impl FundamentalOperationalData {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        system_status: u8,
        data_field1: u8,
        information_layers: u8,
        data_field2: u8,
        parameter1: u16,
        parameter2: u16,
        parameter3: u16,
        parameter4: u16,
        parameter5: u16,
        parameter6: u16,
    ) -> Self {
        Self {
            system_status,
            data_field1,
            information_layers,
            data_field2,
            parameter1,
            parameter2,
            parameter3,
            parameter4,
            parameter5,
            parameter6,
        }
    }
}
