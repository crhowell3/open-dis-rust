use std::{error::Error, net::UdpSocket, thread, time::Duration};

use bytes::BytesMut;
use open_dis_rust::prelude::*;
use open_dis_rust::{
    common::{
        data_types::{EntityId, EntityMarking, EntityType, LinearVelocity, WorldCoordinate},
        enums::{Country, DeadReckoningAlgorithm, EntityKind, EntityMarkingCharacterSet, ForceId},
    },
    entity_information::EntityStatePdu,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Bind to any free local port and send to a receiver on port 3000.
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("127.0.0.1:3000")?;

    // Build an Entity State PDU describing one tank.
    let mut pdu = EntityStatePdu::new();
    pdu.header_mut().exercise_id = 1;
    pdu.entity_id = EntityId::new(1, 1, 1); // site 1, application 1, entity 1
    pdu.force_id = ForceId::Friendly;
    // 1:1:225:1:1:3:0 is the SISO-REF-010 entity type for an M1A2 Abrams.
    pdu.entity_type = EntityType::new(
        EntityKind::Platform,
        1,
        Country::UnitedStatesofAmerica,
        1,
        1,
        3,
        0,
    );
    pdu.entity_marking = EntityMarking::new(EntityMarkingCharacterSet::ASCII, "TANK01".to_string());
    pdu.entity_linear_velocity = LinearVelocity::new(10.0, 0.0, 0.0); // m/s, world frame
    pdu.dead_reckoning_parameters.dead_reckoning_algorithm =
        DeadReckoningAlgorithm::DRMConstantVelocityLowAccelerationLinearMotionEntity;

    // Location is geocentric (ECEF) meters. This point is on the equator at 0 degrees longitude.
    let mut x = 6_378_137.0;

    for _ in 0..10 {
        pdu.entity_location = WorldCoordinate::new(x, 0.0, 0.0);
        pdu.header_mut().timestamp = PduHeader::calculate_dis_timestamp();

        // serialize() fills in the PDU type, family, length and all counts.
        let mut bytes = BytesMut::new();
        pdu.serialize(&mut bytes)?;
        socket.send(&bytes)?;
        println!("sent {}-byte Entity State PDU, x = {x:.1}", bytes.len());

        x += 10.0;
        thread::sleep(Duration::from_secs(1));
    }

    Ok(())
}
