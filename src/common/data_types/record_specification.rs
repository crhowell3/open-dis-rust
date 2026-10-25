//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use super::record_specification_element::RecordSpecificationElement;

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct RecordSpecification {
    #[deku(
        update = "crate::common::wire::update_each(&mut self.record_sets)?",
        assert = "crate::common::wire::fits_in_pdu(*number_of_record_sets)"
    )]
    pub number_of_record_sets: u32,
    #[deku(count = "number_of_record_sets")]
    pub record_sets: Vec<RecordSpecificationElement>,
}

impl RecordSpecification {
    #[must_use]
    pub const fn new(
        number_of_record_sets: u32,
        record_sets: Vec<RecordSpecificationElement>,
    ) -> Self {
        Self {
            number_of_record_sets,
            record_sets,
        }
    }
}

