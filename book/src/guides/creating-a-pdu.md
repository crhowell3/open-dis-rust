# Creating a PDU

The IEEE 1278.1-2012 standard defines 72 PDUs, all of which have been
implemented in this crate. While the topics and contents of the PDUs
differ, each PDU can be easily instantiated.

### Importing Modules

The PDUs are divided into separate protocol families, which are groups
of PDUs that serve a similar purpose. To begin creating a PDU, first
import the PDU's module, or, if using multiple PDUs from the same family,
import the entire protocol family module.

```rust
// Imports just the Acknowledge PDU module
use open_dis_rust::simulation_management::acknowledge_pdu::AcknowledgePdu;

// Imports the entire SIMAN protocol family, including the associated data types
use open_dis_rust::simulation_management::*;
```

### Creating a PDU

To instantiate a PDU, use the PDU's `default` method to pre-populate all its
fields with data.

```rust
let mut acknowledge_pdu = AcknowledgePdu::default();
```

It is advised to use the `mut` keyword so that the default values within the
PDU's fields can be manipulated before sending it over UDP.

### Editing the PDU Fields

Each PDU has different fields; no two PDUs have the exact same fields. Every
PDU is guaranteed to have a `pdu_header` field that is of type `PduHeader`.
See the crate documentation for more information on this type. It is generally
advised that this field and its subfields should not be directly edited because
it contains the header information that a receiver would need to correctly
parse and interpret the data. Upon calling `default`, this field is pre-populated
with the correct metadata for the PDU, and it should not be edited afterwards.

All other fields within a PDU can be safely edited so long as the data being
assigned to them is of the correct type and length. For most fields, editing
the value is just a simple assignment, especially with primitive data types.
However, there are other types that require a little more work to construct
before assigning. Type information can be found within the crate documentation.

The following is an example for editing a PDU field that is a primitive type.

```rust
// Instantiate a default Acknowledge PDU
let mut acknowledge_pdu = AcknowledgePdu::default();

// Valid assignment of the request_id field
acknowledge_pdu.request_id = 14;
```

The following is an example for editing a PDU field that is of a special type.

```rust
// Instantiate a default Acknowledge PDU
let mut acknowledge_pdu = AcknowledgePdu::default();

// Assignment occurring field-wise for the originating_entity_id field
acknowledge_pdu.originating_entity_id.simulation_address.application_id = 1;
acknowledge_pdu.originating_entity_id.simulation_address.site_id = 3;
acknowledge_pdu.originating_entity_id.entity_id = 12;

// Assignment can also occur by creating an EntityId and assigning
let mut origin_id = EntityId::default(1);
origin_id.simulation_address.application_id = 1;
origin_id.simulation_address.site_id = 4;
origin_id.entity_id = 15;
acknowledge_pdu.originating_entity_id = origin_id;
```
