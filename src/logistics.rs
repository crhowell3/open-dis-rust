//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
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
    use crate::{common::pdu::Pdu, pdu_tests};

    pdu_tests!(
        service_request_pdu_tests, ServiceRequestPdu,
        pdu_type: PduType::ServiceRequest,
        default_bits: 224,
        wrong_type: ResupplyOfferPdu,
    );
    pdu_tests!(
        resupply_offer_pdu_tests, ResupplyOfferPdu,
        pdu_type: PduType::ResupplyOffer,
        default_bits: 224,
        wrong_type: ResupplyReceivedPdu,
    );
    pdu_tests!(
        resupply_received_pdu_tests, ResupplyReceivedPdu,
        pdu_type: PduType::ResupplyReceived,
        default_bits: 224,
        wrong_type: ResupplyCancelPdu,
    );
    pdu_tests!(
        resupply_cancel_pdu_tests, ResupplyCancelPdu,
        pdu_type: PduType::ResupplyCancel,
        default_bits: 192,
        wrong_type: RepairCompletePdu,
    );
    pdu_tests!(
        repair_complete_pdu_tests, RepairCompletePdu,
        pdu_type: PduType::RepairComplete,
        default_bits: 224,
        wrong_type: RepairResponsePdu,
    );
    pdu_tests!(
        repair_response_pdu_tests, RepairResponsePdu,
        pdu_type: PduType::RepairResponse,
        default_bits: 224,
        wrong_type: ResupplyOfferPdu,
    );
}
