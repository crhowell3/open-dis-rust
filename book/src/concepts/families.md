# Protocol Families & PDU Status

## Protocol families

IEEE 1278.1-2012 groups its 72 PDU types into **protocol families** by purpose. The
family is in octet 3 of every PDU header next to the PDU type. You never set it yourself:
`serialize()` fills it in automatically.

In open-dis-rust, each family is a module, and the PDUs for that family are defined in each.

| # | Family | Module | What it's for | PDUs |
| ---: | --- | --- | --- | ---: |
| 1 | Entity Information/Interaction | `entity_information` | Where entities are, what they look like, and collisions | 5 |
| 2 | Warfare | `warfare` | Weapons fire, detonations and damage | 4 |
| 3 | Logistics | `logistics` | Resupply and repair between entities | 6 |
| 4 | Radio Communications | `radio_communications` | Simulated radios and intercoms, including audio | 5 |
| 5 | Simulation Management | `simulation_management` | Starting, stopping and controlling the exercise | 12 |
| 6 | Distributed Emission Regeneration | `distributed_emissions` | Radar, jammers, sonar, IFF and laser designators | 5 |
| 7 | Entity Management | `entity_management` | Groups, aggregates and transferring ownership of entities | 4 |
| 8 | Minefield | `minefield` | Minefields and the mines in them | 4 |
| 9 | Synthetic Environment | `synthetic_environment` | Weather, terrain changes and environmental objects | 5 |
| 10 | Simulation Management with Reliability | `simulation_management_with_reliability` | The same jobs as family 5, with acknowledgements and retries | 15 |
| 11 | Live Entity Information/Interaction | `live_entity_information` | Compact PDUs for live, instrumented entities on a range | 5 |
| 13 | Information Operations | `information_operations` | Cyber and electronic warfare effects | 2 |

Family 0 means "other", and family 12 is reserved for the Non Real-Time service. Neither has any PDUs.

### Which families you need

Most applications need only a few families:

- **Every participant** should send and receive **Entity Information** and responds to
  **Simulation Management**.
- **Combat simulations** may use **Warfare**.
- **Radio and voice simulations** may use **Radio Communications**.
- **Sensor and electronic warfare simulations** use **Distributed Emission Regeneration**.

### Reliable and unreliable simulation management

Families 5 and 10 are leveraged for the same tasks. Family 5 PDUs are sent once, and the sender
cannot tell whether they arrived. Family 10 PDUs carry a **required reliability service** field
that asks the receiver to acknowledge them, and the sender retries until it gets a reply. Use
family 10 when a lost message would cause real problems, i.e., when creating or removing
entities across simulators.

## PDU Status

Octet 10 of the PDU header holds the **PDU Status** record. It has a few one- and two-bit flags
taht tell a receiver how to treat the PDU. A new PDU has all of these flags cleared, which is
correct for most traffic.

You reach the flags through `header.status_record`:

```rust,ignore
use open_dis_rust::common::enums::LVCIndicator;

pdu.header_mut().status_record.set_lvc(LVCIndicator::Virtual);
```

### Status flags

**LVC Indicator.** Whether the PDU comes from a **live** system (real equipment on a
range), a **virtual** one (a person within a simulator) or a **constructive** one
(a computer-controlled simulation, such as a wargame). Many exercises mix all three,
and receivers sometimes treat them differently such as by hiding constructive entities
from a live display. Setting it is optional; the default is `NoStatement`.
 
**Transferred Entity Indicator (TEI).** Set when the entity a PDU describes is now
owned by a different simulation from the one that created it. That happens after
the Transfer Ownership PDU moves an entity from one simulator to another. Receivers
use it to know that the entity is still the same one, even though a different
application is now sending its updates.
 
**Coupled Extension Indicator (CEI).** Set when the PDU has an Attribute PDU
**coupled** to it, carrying extra data that extends this PDU. Receivers that see this
flag should wait for the matching Attribute PDU before acting on the data.
 
**PDU-specific bits.** Bits 4 and 5 mean different things for different PDUs. For
example, in a Fire PDU, bit 4 says whether a munition or an expendable, such as
chaff, was fired. In radio PDUs, bits 4 and 5 say whether the radio is attached to an
entity.
 
[Wire Format Conventions](../reference/wire-format.md#pdu-status) lists the exact bit
positions and the method for reading and writing each flag.

### Live Entity PDUs
 
The five Live Entity PDUs don't have a PDU Status record. Their header,
`LiveEntityPduHeader`, uses the same octet for a **subprotocol number** instead, which
says how the rest of the PDU is encoded.
