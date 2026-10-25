//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::data_types::{EntityId, attribute_record::AttributeRecord};

#[derive(Clone, Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(
    endian = "endian",
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Big"
)]
pub struct AttributeRecordSet {
    pub entity_id: EntityId,
    #[deku(
        update = "crate::common::wire::update_each(&mut self.attribute_records)?",
        assert = "crate::common::wire::fits_in_pdu(*number_of_attribute_records)"
    )]
    pub number_of_attribute_records: u16,
    #[deku(count = "number_of_attribute_records")]
    pub attribute_records: Vec<AttributeRecord>,
}

impl AttributeRecordSet {
    #[must_use]
    pub fn new() -> Self {
        Self {
            entity_id: EntityId::default(),
            number_of_attribute_records: 0u16,
            attribute_records: vec![],
        }
    }
}
