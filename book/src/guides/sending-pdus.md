# Sending PDUs

While the network protocol is not required to be UDP, this crate has been validated
against UDP topology, and the example code provided uses UDP as the protocol.

## Dependencies

This demonstration utilizes the `open_dis_rust` crate as well as the `bytes` and
`tokio` crates.

## Create and Serialize the PDU

As shown in the previous section, we can create a PDU as such:

```rust
use open_dis_rust::simulation_management::AcknowledgePdu;

let mut ack_pdu = AcknowledgePdu::new();
```

Then, we need to create an empty, mutable byte array into which we will serialize
the PDU:

```rust
use bytes::BytesMut;

let mut bytes = BytesMut::new();
```

Now, we simply call the `serialize` function from the PDU and pass in a mutable
reference to the byte array:

```rust
ack_pdu.serialize(&mut bytes);
```

The PDU has now been serialized into the byte array and is ready to be sent.

## Initializing a UDP Socket

We need to select an address and a port to send the message. For this exercise,
we will assume the address is `localhost (127.0.0.1)`, and we will use port `3000`.
To create the socket, we use the `tokio` crate:

```rust
use bytes::BytesMut;
use std::net::SocketAddr;
use std::io;
use tokio::net::UdpSocket;

use open_dis_rust::simulation_management::AcknowledgePdu;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:3000").await?;
    let remote_addr = "127.0.0.1:3000";

    socket.connect(remote_addr).await?;

    let mut ack_pdu = AcknowledgePdu::new();
    let mut bytes = BytesMut::new();

    ack_pdu.serialize(&mut bytes);
    socket.send(&bytes[..]).await?;
}
```

We create a UDP socket and bind it to all network interfaces, specifically on port
`3000`. We then establish a one-to-one connection to `127.0.0.0:3000`. Afterwards,
we take our serialized PDU and send it via the socket's `send()` function. Anything
listening on 127.0.0.1:3000 will be able to receive the message.

A tutorial on receiving and parsing a PDU is available in the next section.
