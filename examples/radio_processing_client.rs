//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Jack Laverty
//
//     Licensed under the BSD 2-Clause License

#![warn(rust_2018_idioms)]
use std::error::Error;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:3000";
    let addr = addr.parse::<SocketAddr>()?;
    udp::connect(&addr).await?;
    Ok(())
}

mod udp {
    use bytes::BytesMut;
    use open_dis_rust::{
        common::Pdu,
        radio_communications::{
            IntercomControlPdu, IntercomSignalPdu, ReceiverPdu, SignalPdu, TransmitterPdu,
        },
    };
    use std::error::Error;
    use std::io;
    use std::net::SocketAddr;
    use tokio::net::UdpSocket;

    pub async fn connect(addr: &SocketAddr) -> Result<(), Box<dyn Error>> {
        let bind_addr = if addr.ip().is_ipv4() {
            "0.0.0.0:0"
        } else {
            "[::]:0"
        };

        let socket = UdpSocket::bind(&bind_addr).await?;
        socket.connect(addr).await?;
        send(&socket).await?;
        Ok(())
    }

    async fn send_pdu(writer: &UdpSocket, pdu: &mut impl Pdu) -> Result<(), io::Error> {
        let mut bytes = BytesMut::new();
        let _ = pdu.serialize(&mut bytes).map_err(|_| io::ErrorKind::InvalidData);
        writer.send(&bytes[..]).await?;
        Ok(())
    }

    async fn send(writer: &UdpSocket) -> Result<(), io::Error> {
        send_pdu(writer, &mut TransmitterPdu::new()).await?;
        println!("Sent TransmitterPdu");

        send_pdu(writer, &mut SignalPdu::new()).await?;
        println!("Sent SignalPdu");

        send_pdu(writer, &mut ReceiverPdu::new()).await?;
        println!("Sent ReceiverPdu");

        send_pdu(writer, &mut IntercomSignalPdu::new()).await?;
        println!("Sent IntercomSignalPdu");

        send_pdu(writer, &mut IntercomControlPdu::new()).await?;
        println!("Sent IntercomControlPdu");

        Ok(())
    }
}