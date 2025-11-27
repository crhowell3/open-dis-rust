//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#![allow(deprecated)]

use crate::common::data_types::SimulationAddress;
use crate::{
    common::{
        GenericHeader, SerializedLength,
        enums::{DISAttributeActionCode, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
    entity_information::data_types::attribute_record_set::AttributeRecordSet,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.6.3
    pub struct AttributePdu {
        header: PduHeader,
        pdu_type: PduType::Attribute,
        protocol_family: ProtocolFamily::EntityInformation,
        fields: {
            pub originating_simulation_address: SimulationAddress,
            padding: u32,
            padding2: u16,
            pub attribute_record_pdu_type: u8,
            pub attribute_record_protocol_version: u8,
            pub master_attribute_record_type: u32,
            pub action_code: DISAttributeActionCode,
            padding3: u8,
            pub number_of_attribute_record_sets: u16,
            pub attribute_record_sets: Vec<AttributeRecordSet>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AttributePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = AttributePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<AttributePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = AttributePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = AttributePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
        assert_eq!(new_pdu.header, pdu.header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
        let pdu = AttributePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
