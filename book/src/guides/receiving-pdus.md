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

A receiver usually handles several PDU types and ignores the rest. `AnyPdu::deserialize`
reads the PDU type from the header and decodes the PDU as that type. `AnyPdu` has one
variant per PDU type, named after its `PduType`, so you `match` on the ones you handle:

```rust,ignore
use std::io;
use tokio::net::UdpSocket;

use open_dis_rust::prelude::*;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:3000").await?;
    let mut buf = vec![0u8; MAX_PDU_SIZE_OCTETS];

    loop {
        let (n, from) = socket.recv_from(&mut buf).await?;
        let mut datagram = buf.get(..n).unwrap_or_default();

        match AnyPdu::deserialize(&mut datagram) {
            Ok(AnyPdu::Acknowledge(ack)) => {
                println!("{from}: acknowledged request {}", ack.request_id);
            }
            Ok(AnyPdu::EntityState(pdu)) => println!("{from}: entity {:?}"), pdu.entity_id),
            Ok(other) => println!("{from}: ignoring {:?} PDU", other.pdu_type()),
            Err(e) => eprintln!("{from}: undecodable datagram: {e}"),
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

If you only handle one or two PDU types, you can skip decoding the rest:
`PduHeader::get_pdu_type` reads just the 12-octet header, and then you can call
`deserialize` on the concrete type, such as `EntityStatePdu::deserialize`.

## Several PDUs in a single datagram

A datagram can hold more than one PDU. `AnyPdu::deserialize_all` decodes each of them in
turn. [Wire Format Conventions](../reference/wire-format.md#several-pdus-in-one-datagram)
shows how.
