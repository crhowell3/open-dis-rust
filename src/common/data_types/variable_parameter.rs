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
pub struct VariableParameter {
    pub record_type: u8,
    pub variable_parameter_field1: f64,
    pub variable_parameter_field2: u32,
    pub variable_parameter_field3: u16,
    pub variable_parameter_field4: u8,
}

impl VariableParameter {
    #[must_use]
    pub const fn new(
        record_type: u8,
        variable_parameter_field1: f64,
        variable_parameter_field2: u32,
        variable_parameter_field3: u16,
        variable_parameter_field4: u8,
    ) -> Self {
        Self {
            record_type,
            variable_parameter_field1,
            variable_parameter_field2,
            variable_parameter_field3,
            variable_parameter_field4,
        }
    }
}
