use std::{error::Error, net::UdpSocket};

use open_dis_rust::prelude::*;
use open_dis_rust::{common::enums::PduType, entity_information::EntityStatePdu};

fn main() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("0.0.0.0:3000")?;
    println!("listening on {}", socket.local_addr()?);

    let mut buf = [0u8; MAX_PDU_SIZE_OCTETS];
    loop {
        let (n, from) = socket.recv_from(&mut buf)?;
        let Some(datagram) = buf.get(..n) else {
            continue;
        };

        // Peek at the 12-octet header to find out which PDU this is.
        match PduHeader::get_pdu_type(datagram) {
            Some(PduType::EntityState) => {
                let mut bytes = datagram;
                match EntityStatePdu::deserialize(&mut bytes) {
                    Ok(pdu) => println!(
                        "{from}: entity {}:{}:{} '{}' at ({:.1}, {:.1}, {:.1})",
                        pdu.entity_id.simulation_address.site_id,
                        pdu.entity_id.simulation_address.application_id,
                        pdu.entity_id.entity_id,
                        pdu.entity_marking.entity_marking_string,
                        pdu.entity_location.x_coordinate,
                        pdu.entity_location.y_coordinate,
                        pdu.entity_location.z_coordinate,
                    ),
                    Err(e) => eprintln!("{from}: bad Entity State PDU: {e}"),
                }
            }
            Some(other) => println!("{from}: ignoring {other:?} PDU"),
            None => eprintln!("{from}: {n} bytes is too short to be a DIS PDU"),
        }
    }
}
