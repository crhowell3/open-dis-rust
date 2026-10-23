//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

//! The Logistics protocol family

use crate::common::{
    data_types::{EntityId, SupplyQuantity},
    enums::{
        PduType, ProtocolFamily, RepairCompleteRepair, RepairResponseRepairResult,
        ServiceRequestServiceTypeRequested,
    },
    pdu_header::PduHeader,
};

/// Implemented according to IEEE 1278.1-2012 §7.4.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct ServiceRequestPdu {
    #[deku(skip)]
    header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub servicing_entity_id: EntityId,
    pub service_type_requested: ServiceRequestServiceTypeRequested,
    #[deku(update = "self.supplies.len()")]
    pub number_of_supply_types: u8,
    padding: u16,
    #[deku(count = "number_of_supply_types")]
    pub supplies: Vec<SupplyQuantity>,
}
crate::impl_pdu!(
    ServiceRequestPdu,
    PduHeader,
    PduType::ServiceRequest,
    ProtocolFamily::Logistics
);

/// Implemented according to IEEE 1278.1-2012 §7.4.3
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct ResupplyOfferPdu {
    #[deku(skip)]
    header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub supplying_entity_id: EntityId,
    #[deku(update = "self.supplies.len()")]
    pub number_of_supply_types: u8,
    padding: u8,
    padding2: u16,
    #[deku(count = "number_of_supply_types")]
    pub supplies: Vec<SupplyQuantity>,
}
crate::impl_pdu!(
    ResupplyOfferPdu,
    PduHeader,
    PduType::ResupplyOffer,
    ProtocolFamily::Logistics
);

/// Implemented according to IEEE 1278.1-2012 §7.4.4
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct ResupplyReceivedPdu {
    #[deku(skip)]
    header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub supplying_entity_id: EntityId,
    #[deku(update = "self.supplies.len()")]
    pub number_of_supply_types: u8,
    padding: u8,
    padding2: u16,
    #[deku(count = "number_of_supply_types")]
    pub supplies: Vec<SupplyQuantity>,
}
crate::impl_pdu!(
    ResupplyReceivedPdu,
    PduHeader,
    PduType::ResupplyReceived,
    ProtocolFamily::Logistics
);

/// Implemented according to IEEE 1278.1-2012 §7.4.5
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct ResupplyCancelPdu {
    #[deku(skip)]
    header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub supplying_entity_id: EntityId,
}
crate::impl_pdu!(
    ResupplyCancelPdu,
    PduHeader,
    PduType::ResupplyCancel,
    ProtocolFamily::Logistics
);

/// Implemented according to IEEE 1278.1-2012 §7.4.6
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct RepairCompletePdu {
    #[deku(skip)]
    header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub repairing_entity_id: EntityId,
    pub repair: RepairCompleteRepair,
    padding: u16,
}
crate::impl_pdu!(
    RepairCompletePdu,
    PduHeader,
    PduType::RepairComplete,
    ProtocolFamily::Logistics
);

/// Implemented according to IEEE 1278.1-2012 §7.4.7
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct RepairResponsePdu {
    #[deku(skip)]
    header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub repairing_entity_id: EntityId,
    pub repair_result: RepairResponseRepairResult,
    padding: u8,
    padding2: u16,
}
crate::impl_pdu!(
    RepairResponsePdu,
    PduHeader,
    PduType::RepairResponse,
    ProtocolFamily::Logistics
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::{
            constants::BITS_PER_BYTE,
            pdu::{Pdu, to_bytes},
        },
        pdu_tests,
    };
    use bytes::Buf;

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
