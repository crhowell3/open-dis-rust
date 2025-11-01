use open_dis_rust::common::pdu_header::ProtocolFamily;

#[test]
fn create_acknowledge_pdu() {
    use open_dis_rust::simulation_management::AcknowledgePdu;
    let ack_pdu = AcknowledgePdu::default();
    assert_eq!(
        ack_pdu.pdu_header.protocol_family,
        ProtocolFamily::SimulationManagement
    );
}

#[test]
fn create_acknowledge_reliable_pdu() {
    use open_dis_rust::simulation_management_with_reliability::AcknowledgeReliablePdu;
    let ack_reliable_pdu = AcknowledgeReliablePdu::default();
    assert_eq!(
        ack_reliable_pdu.pdu_header.protocol_family,
        ProtocolFamily::SimulationManagementWithReliability
    );
}

#[test]
fn create_entity_state_pdu() {
    use open_dis_rust::entity_information::EntityStatePdu;
    let entity_state_pdu = EntityStatePdu::default();
    assert_eq!(
        entity_state_pdu.pdu_header.protocol_family,
        ProtocolFamily::EntityInformation
    );
}
