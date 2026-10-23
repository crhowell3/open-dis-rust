//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use super::standard_variable_records::StandardVariableRecords;

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct StandardVariableSpecification {
    #[deku(
        update = "crate::common::wire::update_each(&mut self.standard_variable_records)?",
        assert = "crate::common::wire::fits_in_pdu(*number_of_standard_variable_records)"
    )]
    pub number_of_standard_variable_records: u16,
    #[deku(count = "number_of_standard_variable_records")]
    pub standard_variable_records: Vec<StandardVariableRecords>,
}

impl StandardVariableSpecification {
    #[must_use]
    pub const fn new(
        number_of_standard_variable_records: u16,
        standard_variable_records: Vec<StandardVariableRecords>,
    ) -> Self {
        Self {
            number_of_standard_variable_records,
            standard_variable_records,
        }
    }
}
