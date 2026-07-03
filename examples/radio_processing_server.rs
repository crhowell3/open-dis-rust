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
use bytes::BytesMut;
use open_dis_rust::{
    common::{pdu::Pdu, pdu_header::PduHeader, enums::PduType},
    radio_communications::{
        TransmitterPdu, SignalPdu, ReceiverPdu, IntercomSignalPdu, IntercomControlPdu,
    },
};

enum ParsedPdu {
    Transmitter(TransmitterPdu),
    Signal(SignalPdu),
    Receiver(ReceiverPdu),
    IntercomSignal(IntercomSignalPdu),
    IntercomControl(IntercomControlPdu),
}

fn deserialize_pdu(bytes: &[u8]) -> Option<ParsedPdu> {
    let pdu_type = PduHeader::get_pdu_type(bytes)?;
    let mut buf = BytesMut::from(bytes);

    match pdu_type {
        PduType::Transmitter     => TransmitterPdu::deserialize(&mut buf).ok().map(ParsedPdu::Transmitter),
        PduType::Signal          => SignalPdu::deserialize(&mut buf).ok().map(ParsedPdu::Signal),
        PduType::Receiver        => ReceiverPdu::deserialize(&mut buf).ok().map(ParsedPdu::Receiver),
        PduType::IntercomSignal  => IntercomSignalPdu::deserialize(&mut buf).ok().map(ParsedPdu::IntercomSignal),
        PduType::IntercomControl => IntercomControlPdu::deserialize(&mut buf).ok().map(ParsedPdu::IntercomControl),
        _ => None,
    }
}

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
                match deserialize_pdu(&buf[..size]) {
                    Some(ParsedPdu::Transmitter(p)) => {
                        println!("Transmitter from {peer}: entity={:?} freq={} state={:?}",
                            p.entity_id, p.frequency, p.transmit_state);
                    }
                    Some(ParsedPdu::Signal(p)) => {
                        println!("Signal from {peer}: entity={:?} samples={} data_length={}",
                            p.entity_id, p.samples, p.data_length);
                    }
                    Some(ParsedPdu::Receiver(p)) => {
                        println!("Receiver from {peer}: entity={:?} state={:?} power={}",
                            p.entity_id, p.receiver_state, p.received_power);
                    }
                    Some(ParsedPdu::IntercomSignal(p)) => {
                        println!("IntercomSignal from {peer}: intercom={:?} samples={}",
                            p.intercom_reference_id, p.samples);
                    }
                    Some(ParsedPdu::IntercomControl(p)) => {
                        println!("IntercomControl from {peer}: intercom={:?} command={:?} transmit_state={:?}",
                            p.source_intercom_reference_id, p.command, p.transmit_line_state);
                    }
                    None => {
                        println!("Unknown or non-radio PDU ({size} bytes) from {peer}");
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