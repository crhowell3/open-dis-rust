# Receiving PDUs

The same dependencies are required as listed in the previous section.

## Receiving and Parsing a PDU

Using the same UDP socket that we created in the previous section, we 
can receive a PDU and deserialize it from the byte array.

```rust
use bytes::BytesMut;
use std::net::SocketAddr;
use std::io;
use tokio::net::UdpSocket;

use open_dis_rust::simulation_management::acknowledge_pdu::AcknowledgePdu;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:3000").await?;
    let remote_addr = "127.0.0.1:3000";

    socket.connect(remote_addr).await?;

    let mut buf = vec![0; 1204];
    let n = socket.recv(&mut buf[..]).await?;
    loop {
        if n > 0 {
          dbg!(AcknowledgePdu::deserialize(BytesMut::from(buf.as_slice())).unwrap());
          break;
        }
    }
    Ok(())
}
```

In most cases, the receiver is going to be listening for a particular PDU or range of
PDUs. Using the above code as an example, if the data received is not able to be
deserialized into an AcknowledgePdu, an error will be returned. This can be further
handled by the user.

There are other ways to determine what type of PDU the data represents, such as reading
the raw byte array and finding the byte that represents the `pdu_type` field in the header.
Then, the user could set up a match tree to attempt to parse the PDU, if they are expected
to handle it, of course.
