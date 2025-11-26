//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::generic_header::GenericHeader;
use crate::{
    common::{
        SerializedLength,
        data_types::entity_id::EntityId,
        enums::{AcknowledgeFlag, AcknowledgeResponseFlag, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.5.6
    pub struct AcknowledgePdu {
        header: PduHeader,
        pdu_type: PduType::Acknowledge,
        protocol_family: ProtocolFamily::SimulationManagement,
        fields: {
            pub originating_entity_id: EntityId,
            pub receiving_entity_id: EntityId,
            pub acknowledge_flag: AcknowledgeFlag,
            pub response_flag: AcknowledgeResponseFlag,
            pub request_id: u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AcknowledgePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = AcknowledgePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<AcknowledgePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = AcknowledgePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = AcknowledgePdu::deserialize(&mut deserialize_buf).expect("Some");
        assert_eq!(new_pdu.header, pdu.header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
        let pdu = AcknowledgePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
