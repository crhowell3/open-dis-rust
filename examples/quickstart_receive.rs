//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use std::{error::Error, net::UdpSocket};

use open_dis_rust::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:3000")?;
    println!("listening on {}", socket.local_addr()?);

    let mut buf = [0u8; MAX_PDU_SIZE_OCTETS];
    loop {
        let (n, from) = socket.recv_from(&mut buf)?;
        let Some(mut datagram) = buf.get(..n) else {
            continue;
        };

        // The header says which PDU it is, and AnyPdu decodes it as that type
        match AnyPdu::deserialize(&mut datagram) {
            Ok(AnyPdu::EntityState(pdu)) => println!(
                "{from}: entity {}:{}:{} '{}' at ({:.1}, {:.1}, {:.1})",
                pdu.entity_id.simulation_address.site_id,
                pdu.entity_id.simulation_address.application_id,
                pdu.entity_id.entity_id,
                pdu.entity_marking.entity_marking_string,
                pdu.entity_location.x_coordinate,
                pdu.entity_location.y_coordinate,
                pdu.entity_location.z_coordinate,
            ),
            Ok(other) => println!("{from}: ignoring {:?} PDU", other.pdu_type()),
            Err(e) => eprintln!("{from}: undecodable datagram: {e}"),
        }
    }
}
