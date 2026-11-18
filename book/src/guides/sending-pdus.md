# Sending PDUs

While the network protocol is not required to be UDP, this crate has been validated
against UDP topology, and the example code provided uses UDP as the protocol.

## Dependencies

This demonstration utilizes the `open_dis_rust` crate and `tokio`:

```shell
cargo add open-dis-rust
cargo add tokio --features full
```

## Create and Serialize the PDU

Create the PDU and fill it its fields as described in [Creating a PDU](creating-a-pdu.md):

```rust,ignore
use open_dis_rust::{prelude::*, simulation_management::AcknowledgePdu};

let mut ack = AcknowledgePdu::new();
ack.request_id = 42;
```

Then, serialize it. `to_bytes` returns a new `Vec<u8>` holding the encoded PDU:

```rust,ignore
let bytes: Vec<u8> = ack.to_bytes()?;
```

The PDU has now been serialized into a byte array and is ready to be sent.

## Initializing a UDP Socket

We need to select an address and a port to send the message. For this exercise,
we will assume the address is `localhost (127.0.0.1)`, and we will use port `3000`.
To create the socket, we use the `tokio` crate:

```rust,ignore
use std::io;
use tokio::net::UdpSocket;

use open_dis_rust::{prelude::*, simulation_management::AcknowledgePdu};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Let the operating system pick the local port, and send to port 3000.
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect("127.0.0.1:3000").await?;

    let mut ack = AcknowledgePdu::new();
    ack.request_id = 42;

    let bytes = ack
        .to_bytes()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    socket.send(&bytes).await?;

    Ok(())
}
```

`DISError` implements `std::error::Error`, so it converts into an `io::Error`, or a
`Box<dyn Error>`, for use with `?`.

Anything listening on port 3000 of the local machine receives the PDU. The next guide
shows how to receive and decode it.

## Using the `bytes` crate

If your application already uses `bytes`, for example through a `tokio` codec, encode
into a `Vec<u8>` and convert it. `Bytes::from` takes over the vector's memory without
copying:

```rust,ignore
let bytes = bytes::Bytes::from(ack.to_bytes()?);
```

When receiving, a `Bytes` or `BytesMut` dereferences to `&[u8]`, so you can pass
`&mut &buf[..]` straight to `deserialize`.
