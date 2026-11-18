# Receiving PDUs

This guide receives and decodes PDUs using `tokio` for networking. The same dependencies
are used as in [Sending PDUs](sending-pdus.md).

## Decode a PDU

`deserialize` decodes one PDU from the front of a byte slice. It takes the slice by
mutable reference and advances it past the PDU it read:

```rust,ignore
let mut datagram: &[u8] = &buf[..n];
let ack = AcknowledgePdu::deserialize(&mut datagram)?;
```

If the bytes are not a valid Acknowledge PDU, it returns a `DISError` instead.
[Errors](../reference/errors.md) describes each type of error.

## Receive and decode

A receiver usually handles several PDU types and ignores the rest.
`PduHeader::get_pdu_type` reads just the 12-octet header, so you can decide what to do
with a datagram before decoding it:

```rust,ignore
use std::io;
use tokio::net::UdpSocket;

use open_dis_rust::{
    entity_information::EntityStatePdu, prelude::*, simulation_management::AcknowledgePdu,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:3000").await?;
    let mut buf = vec![0u8; MAX_PDU_SIZE_OCTETS];

    loop {
        let (n, from) = socket.recv_from(&mut buf).await?;
        let mut datagram = buf.get(..n).unwrap_or_default();

        match PduHeader::get_pdu_type(datagram) {
            Some(PduType::Acknowledge) => match AcknowledgePdu::deserialize(&mut datagram) {
                Ok(ack) => println!("{from}: acknowledged request {}", ack.request_id),
                Err(e) => eprintln!("{from}: bad Acknowledge PDU: {e}"),
            },
            Some(PduType::EntityState) => match EntityStatePdu::deserialize(&mut datagram) {
                Ok(pdu) => println!("{from}: entity {:?}", pdu.entity_id),
                Err(e) => eprintln!("{from}: bad Entity State PDU: {e}"),
            },
            Some(other) => println!("{from}: ignoring {other:?} PDU"),
            None => eprintln!("{from}: {n} bytes is too short to be a DIS PDU"),
        }
    }
}
```

A few things to note:

- **Size the buffer for the largest PDU.** `MAX_PDU_SIZE_OCTETS` is 8192. A smaller
  buffer could truncate larger PDUs, and attempting to decode a truncated PDU results
  in `DISError::BufferUnderflow`.
- **Decode only the bytes you received.** `buf.get(..n)` limits the slice to the
  datagram, so leftover bytes from an earlier, longer datagram cannot be read.
- **Expect errors.** On a shared network or distributed simulation environment, some
  traffic could naturally fail to decode. Log it and keep receiving rather than hard
  stopping your program.

## Several PDUs in a single datagram

Because `deserialize` advances the slice, you can decode a datagram that holds more than
one PDU by calling it in a loop until the slice is empty.
[Wire Format Conventions](../reference/wire-format.md#several-pdus-in-one-datagram) shows how.
