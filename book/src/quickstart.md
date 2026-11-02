# Quickstart

In this walkthrough, you'll run two small programs. One broadcasts an Entity State PDU
for a tank once a second. The other listens for PDUs and prints what it hears. This
walkthrough takes about 5 minutes.

Both programs live in the repository's `examples` directory. You can run them from a
clone of the repository, as shown at the end of this page, or copy them into your
own project.

## 1. Send an Entity State PDU

An Entity State PDU is the heartbeat of DIS. Every simulator sends one for each entity
it owns, at regular intervals, so that everyone else knows where that entity is and
what it is doing.

```rust,ignore
{{#include ../../examples/quickstart_send.rs}}
```

A few things to notice:

- **`EntityStatePdu::new()`** gives you a PDU with sensible defaults and the header
  already set up. You only fill in the fields you care about.
- **Entity IDs** consist of three numbers: site, application, and entity IDs. Together
  they must be unique within the exercise. It should be noted that this crate does NOt
  enforce this constraint. That is completely up to the consumer to ensure uniqueness.
- **Entity types** come from the SISO-REF-010 enumerations.
- **Locations** are geocentric (ECEF) coordinates in meters.
- **`serialize()`** fills in the PDU type, protocol family, length, and every count
  field before encoding. You should not have to fill out `header.length` youself.

## 2. Receive and decode PDUs

The receiver binds to UDP port 3000, which is traditionally the port used for DIS
traffic per the standard. For each datagram, it reads the header to see what kind of
PDU arrived, then it attempts to decode it.

```rust,ignore
{{#include ../../examples/quickstart_receive.rs}}
```

`PduHeader::get_pdu_type` reads only the 12-octet header, so it is a cheap way to skip
PDUs you aren't accounting for before fully decoding them. `deserialize` returns an error
instead of panicking when a datagram is truncated or malformed. A receiver on a shared
network might expect to receive some bad PDUs at times.

## 3. Run the examples

In one terminal, start the receiver:

```shell
cargo run --example quickstart_receive
```

In a second terminal, start the sender:

```shell
cargo run --example quickstart_send
```

The sender reports each PDU it sends:

```text
sent 144-byte Entity State PDU, x = 6378137.0
sent 144-byte Entity State PDU, x = 6378147.0
sent 144-byte Entity State PDU, x = 6378157.0
```

The receiver prints the tank moving 10 meters along the x-axis each second:

```text
listening on 0.0.0.0:3000
127.0.0.1:35774: entity 1:1:1 'TANK01' at (6378137.0, 0.0, 0.0)
127.0.0.1:35774: entity 1:1:1 'TANK01' at (6378147.0, 0.0, 0.0)
127.0.0.1:35774: entity 1:1:1 'TANK01' at (6378157.0, 0.0, 0.0)
```

**NOTE:** Source ports will differ as they are chosen at runtime.

## 4. Observe packets in Wireshark (optional)

[Wireshark](https://www.wireshark.org/) includes a dissector for the DIS protocol, which
makes it the easiest way to check what your application is actually putting out on the
wire.

How to use the DIS dissector:

1. Start a capture on the loopback interface.
2. Enter `dis` in the display filter bar.
3. Run the sender again.

Each packet should decode as an Entity State PDU, with the entity ID, entity type, marking,
and location you set. If the dissector does not recognize the traffic, check that you are
sending on the correct port (3000), or right-click a packet and choose **Decode As... -> DIS**.

## Sending to other machines

The sender in this example talks only to `127.0.0.1`. To reach other simulators on your network,
send to your subnet's broadcast address or to a multicast group your exercise agrees on. For
broadcast, enable it on the socket first:

```rust,ignore
let socket = UdpSocket::bind("0.0.0.0:0")?;
socket.set_broadcast(true)?;
socket.connect("192.168.1.255:3000")?;  // replace with your subnet's broadcast address
```

Every participant must also use the same **exercise ID**, which this example sets to `1`. This
rule is not enforced by this crate, but it is specified in the DIS standard. Most DIS applications
ignore PDUs from other exercises.

## Next steps

- [Creating a PDU](guides/creating-a-pdu.md) covers building other PDU types.
- [Sending PDUs](guides/sending-pdus.md) shows sending with `tokio`.
- [Receiving PDUs](guides/receiving-pdus.md) covers receiving in more depth.
