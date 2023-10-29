#[test]
fn create_acknowledge_pdu() {
    use open_dis_rust::simulation_management::acknowledge_pdu::AcknowledgePdu;
    let ack_pdu = AcknowledgePdu::default();
    assert_eq!(
        ack_pdu.pdu_header.protocol_family,
        open_dis_rust::common::pdu_header::ProtocolFamily::SimulationManagement
    );
}
