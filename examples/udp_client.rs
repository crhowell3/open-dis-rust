//! An example of hooking up stdin/stdout to either a TCP or UDP stream.
//!
//! This example will connect to a socket address specified in the argument list
//! and then forward all data read on stdin to the server, printing out all data
//! received on stdout. An optional `--udp` argument can be passed to specify
//! that the connection should be made over UDP instead of TCP, translating each
//! line entered on stdin to a UDP packet to be sent to the remote address.
//!
//! Note that this is not currently optimized for performance, especially
//! around buffer management. Rather it's intended to show an example of
//! working with a client.
//!
//! This example can be quite useful when interacting with the other examples in
//! this repository! Many of them recommend running this as a simple "hook up
//! stdin/stdout to a server" to get up and running.

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
        let mut ack_pdu = AcknowledgePdu::default();
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
                dbg!(AcknowledgePdu::deserialize(&mut BytesMut::from(buf.as_slice())).unwrap());
                break;
            }
        }
        Ok(())
    }
}
