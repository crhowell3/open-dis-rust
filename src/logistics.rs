//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

//! The Logistics protocol family

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{entity_id::EntityId, supply_quantity::SupplyQuantity},
        enums::{
            PduType, ProtocolFamily, RepairCompleteRepair, RepairResponseRepairResult,
            ServiceRequestServiceTypeRequested,
        },
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.4.2
    pub struct ServiceRequestPdu {
        header: PduHeader,
        pdu_type: PduType::ServiceRequest,
        protocol_family: ProtocolFamily::Logistics,
        fields: {
            pub receiving_entity_id: EntityId,
            pub servicing_entity_id: EntityId,
            pub service_type_requested: ServiceRequestServiceTypeRequested,
            pub number_of_supply_types: u8,
            padding: u16,
            pub supplies: Vec<SupplyQuantity>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.4.3
    pub struct ResupplyOfferPdu {
        header: PduHeader,
        pdu_type: PduType::ResupplyOffer,
        protocol_family: ProtocolFamily::Logistics,
        fields: {
            pub receiving_entity_id: EntityId,
            pub supplying_entity_id: EntityId,
            pub number_of_supply_types: u8,
            padding: u8,
            padding2: u16,
            pub supplies: Vec<SupplyQuantity>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.4.4
    pub struct ResupplyReceivedPdu {
        header: PduHeader,
        pdu_type: PduType::ResupplyReceived,
        protocol_family: ProtocolFamily::Logistics,
        fields: {
            pub receiving_entity_id: EntityId,
            pub supplying_entity_id: EntityId,
            pub number_of_supply_types: u8,
            padding: u8,
            padding2: u16,
            pub supplies: Vec<SupplyQuantity>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.4.5
    pub struct ResupplyCancelPdu {
        header: PduHeader,
        pdu_type: PduType::ResupplyCancel,
        protocol_family: ProtocolFamily::Logistics,
        fields: {
            pub receiving_entity_id: EntityId,
            pub supplying_entity_id: EntityId,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.4.6
    pub struct RepairCompletePdu {
        header: PduHeader,
        pdu_type: PduType::RepairComplete,
        protocol_family: ProtocolFamily::Logistics,
        fields: {
            pub receiving_entity_id: EntityId,
            pub repairing_entity_id: EntityId,
            pub repair: RepairCompleteRepair,
            padding: u16,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.4.7
    pub struct RepairResponsePdu {
        header: PduHeader,
        pdu_type: PduType::RepairResponse,
        protocol_family: ProtocolFamily::Logistics,
        fields: {
            pub receiving_entity_id: EntityId,
            pub repairing_entity_id: EntityId,
            pub repair_result: RepairResponseRepairResult,
            padding: u8,
            padding2: u16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    mod service_request_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = ServiceRequestPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<ServiceRequestPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = ServiceRequestPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = ServiceRequestPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
            let pdu = ServiceRequestPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod resupply_offer_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = ResupplyOfferPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<ResupplyOfferPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = ResupplyOfferPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = ResupplyOfferPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
            let pdu = ResupplyOfferPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod resupply_received_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = ResupplyReceivedPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<ResupplyReceivedPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = ResupplyReceivedPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                ResupplyReceivedPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
            let pdu = ResupplyReceivedPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod resupply_cancel_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = ResupplyCancelPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<ResupplyCancelPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = ResupplyCancelPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = ResupplyCancelPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 192 / BITS_PER_BYTE;
            let pdu = ResupplyCancelPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod repair_complete_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = RepairCompletePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<RepairCompletePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = RepairCompletePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = RepairCompletePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
            let pdu = RepairCompletePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod repair_response_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = RepairResponsePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<RepairResponsePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = RepairResponsePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = RepairResponsePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
            let pdu = RepairResponsePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }
}
