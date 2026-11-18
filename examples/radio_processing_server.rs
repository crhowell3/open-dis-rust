//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Jack Laverty
//
//     Licensed under the BSD 2-Clause License

#![warn(rust_2018_idioms)]
use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;

use open_dis_rust::prelude::*;

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl Server {
    async fn run(self) -> Result<(), io::Error> {
        let Self {
            socket,
            mut buf,
            mut to_send,
        } = self;

        loop {
            if let Some((size, peer)) = to_send {
                match AnyPdu::deserialize(&mut buf.get(..size).unwrap_or_default()) {
                    Ok(AnyPdu::Transmitter(p)) => {
                        println!(
                            "Transmitter from {peer}: entity={:?} freq={} state={:?}",
                            p.entity_id, p.frequency, p.transmit_state
                        );
                    }
                    Ok(AnyPdu::Signal(p)) => {
                        println!(
                            "Signal from {peer}: entity={:?} samples={} data_length={}",
                            p.entity_id, p.samples, p.data_length
                        );
                    }
                    Ok(AnyPdu::Receiver(p)) => {
                        println!(
                            "Receiver from {peer}: entity={:?} state={:?} power={}",
                            p.entity_id, p.receiver_state, p.received_power
                        );
                    }
                    Ok(AnyPdu::IntercomSignal(p)) => {
                        println!(
                            "IntercomSignal from {peer}: intercom={:?} samples={}",
                            p.intercom_reference_id, p.samples
                        );
                    }
                    Ok(AnyPdu::IntercomControl(p)) => {
                        println!(
                            "IntercomControl from {peer}: intercom={:?} command={:?} transmit_state={:?}",
                            p.source_intercom_reference_id, p.command, p.transmit_line_state
                        );
                    }
                    Ok(other) => {
                        println!("Non-radio {:?} PDU from {peer}", other.pdu_type());
                    }
                    Err(e) => {
                        println!("Undecodable datagram ({size} bytes) from {peer}: {e}")
                    }
                }
            }

            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:3000".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
    };

    server.run().await?;
    Ok(())
}
