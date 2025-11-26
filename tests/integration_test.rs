use open_dis_rust::common::{
    Pdu,
    enums::{PduType, ProtocolFamily},
};

#[test]
fn create_acknowledge_pdu() {
    use open_dis_rust::simulation_management::AcknowledgePdu;
    let pdu = AcknowledgePdu::new();

    assert_eq!(pdu.header().pdu_type, PduType::Acknowledge);
    assert_eq!(
        pdu.header().protocol_family,
        ProtocolFamily::SimulationManagement
    );
}

#[test]
fn create_information_operations_action_pdu() {
    use open_dis_rust::information_operations::InformationOperationsActionPdu;
    let pdu = InformationOperationsActionPdu::new();

    assert_eq!(pdu.header().pdu_type, PduType::InformationOperationsAction);
    assert_eq!(
        pdu.header().protocol_family,
        ProtocolFamily::InformationOperations
    );
}

#[test]
fn create_information_operations_report_pdu() {
    use open_dis_rust::information_operations::InformationOperationsReportPdu;
    let pdu = InformationOperationsReportPdu::new();

    assert_eq!(pdu.header().pdu_type, PduType::InformationOperationsReport);
    assert_eq!(
        pdu.header().protocol_family,
        ProtocolFamily::InformationOperations
    );
}

// #[test]
// fn create_acknowledge_reliable_pdu() {
//     use open_dis_rust::simulation_management_with_reliability::AcknowledgeReliablePdu;
//     let ack_reliable_pdu = AcknowledgeReliablePdu::new();
//     assert_eq!(
//         ack_reliable_pdu.header().protocol_family,
//         ProtocolFamily::SimulationManagementWithReliability
//     );
// }

// #[test]
// fn create_entity_state_pdu() {
//     use open_dis_rust::entity_information::EntityStatePdu;
//     let entity_state_pdu = EntityStatePdu::new();
//     assert_eq!(
//         entity_state_pdu.header().protocol_family,
//         ProtocolFamily::EntityInformation
//     );
// }
