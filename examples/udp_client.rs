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
    use bytes::BytesMut;
    use open_dis_rust::{common::Pdu, simulation_management::AcknowledgePdu};
    use std::error::Error;
    use std::io;
    use std::net::SocketAddr;
    use tokio::net::UdpSocket;

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
        let mut bytes = BytesMut::new();
        let mut ack_pdu = AcknowledgePdu::new();
        let _ = ack_pdu
            .serialize(&mut bytes)
            .map_err(|_| io::ErrorKind::InvalidData);
        writer.send(&bytes[..]).await?;

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
