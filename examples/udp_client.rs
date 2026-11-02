//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#![warn(rust_2018_idioms)]

use std::error::Error;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse what address we're going to connect to
    let addr = "127.0.0.1:3000";
    let addr = addr.parse::<SocketAddr>()?;

    udp::connect(&addr).await?;

    Ok(())
}

mod udp {
    use std::error::Error;
    use std::io;
    use std::net::SocketAddr;

    use bytes::BytesMut;
    use tokio::net::UdpSocket;

    use open_dis_rust::common::data_types::EntityId;
    use open_dis_rust::common::enums::{AcknowledgeFlag, AcknowledgeResponseFlag};
    use open_dis_rust::prelude::*;
    use open_dis_rust::simulation_management::AcknowledgePdu;

    pub async fn connect(addr: &SocketAddr) -> Result<(), Box<dyn Error>> {
        // We'll bind our UDP socket to a local IP/port, but for now we
        // basically let the OS pick both of those.
        let bind_addr = if addr.ip().is_ipv4() {
            "0.0.0.0:0"
        } else {
            "[::]:0"
        };

        let socket = UdpSocket::bind(&bind_addr).await?;
        socket.connect(addr).await?;

        tokio::try_join!(send(&socket), recv(&socket))?;

        Ok(())
    }

    async fn send(writer: &UdpSocket) -> Result<(), io::Error> {
        let mut ack = AcknowledgePdu::new();

        ack.originating_entity_id = EntityId::new(1, 1, 0);
        ack.receiving_entity_id = EntityId::new(1, 2, 0);
        ack.acknowledge_flag = AcknowledgeFlag::CreateEntity;
        ack.response_flag = AcknowledgeResponseFlag::AbleToComply;
        ack.request_id = 42;

        let header = ack.header_mut();
        header.exercise_id = 1;
        header.timestamp = PduHeader::calculate_dis_timestamp();

        let mut bytes = BytesMut::with_capacity(MAX_PDU_SIZE_OCTETS);
        let _ = ack
            .serialize(&mut bytes)
            .map_err(|_| io::ErrorKind::InvalidData);

        writer.send(&bytes).await?;

        Ok(())
    }

    async fn recv(reader: &UdpSocket) -> Result<(), io::Error> {
        loop {
            let mut buf = vec![0; 1024];
            let n = reader.recv(&mut buf[..]).await?;

            if n > 0 {
                dbg!(
                    AcknowledgePdu::deserialize(&mut BytesMut::from(buf.as_slice()))
                        .unwrap_or_default()
                );
                break;
            }
        }
        Ok(())
    }
}
