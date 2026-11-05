# Conformance Notes

This page records how closely open-dis-rust follows IEEE 1278.1-2012. This includes
what has been checked, how the crate behaves at the edges of the standard, and
where it is currently known to differ. This page will be updated as issues
are found and eventually fixed.

If you find a difference that is not listed here, please [report it](../get-in-touch.md#encoding-and-interoperability-problems),
ideally with a packet capture.

## How the crate is checked

- **Review against the standard.** PDU and record layouts have been manually reviewed
  field by field against the IEEE 1278.1-2012 standard document.
- **Round-trip tests.** Every PDU has tests that encode it, decode the result, and
  check that the bytes match. These tests catch bugs where encoding and decoding
  may differ, but it fails to catch cases where both follow the same wrong layout.

## What has been checked and matches

- **PDU header.** The 12-octet layout, field order, and sizes are correct for both the
  standard header and the Live Entity header.
- **PDU Status record.** The bit assignments are correct, including the per-PDU meanings
  of bits 4 and 5.
- **Timestamps.** Encoded in units of 3600/2³¹ seconds past the hour, with the low
  bit set for absolute time.
- **Byte order.** Every field is big-endian, and floating-point values are IEEE 754.
- **Size limit.** PDUs larger than 8192 octets are rejected when encoding.
- **Fixed-size fields.** For nearly all PDUs, the fixed part of the layout is correct.

## Decoding behavior

These choices are stricter than some other DIS implementations. They matter most when
you receive traffic from simulators you do not control.

- **Undefined enumeration values are rejected.** If any enumerated field holds a value
  that is not in the crate's SISO-REF-010 tables, such as a newly assigned country code,
  the whole PDU fails to decode with a `WireFormat` error. Newer simulators may send
  values the crate does not know about yet.
- **Extra octets are rejected.** If a PDU's length field is longer than the fields the
  crate reads, decoding fails with a `DeserializationError`.
- **The protocol version is not checked.** Any version from 1 to 7 is accepted, and the
  PDU is decoded with the 2012 layout. Most DIS 6 (1998) PDUs decode correctly this way
  because their layouts did not change in the 2012 version.
- **The protocol family is not checked** against the PDU type.
- **Padding is preserved.** Padding fields are read and kept, so a decoded PDU re-encodes
  to identical bytes. PDUs you build yourself always send zero padding.

## Known differences

Each of these produces bytes that differ from the standard in some way. The eventual goal
is to have this list be completely empty; that is, all known differences have been addressed.

### Underwater Acoustic PDU
 
- The emitter system data length and beam data length fields aren't computed, so
  they're always sent as 0.
- **To verify:** Shaft RPM Rate of Change is encoded as a 32-bit integer; it may need
  to be a 32-bit float.

### Electromagnetic Emission PDU
 
The system data length and beam data length fields aren't computed, so they're
always sent as 0. Receivers that use these lengths to step through the records can't
parse the PDU.
 
### Linear Object State PDU
 
The general segment appearance in each Linear Segment Parameter record is 32 bits
instead of 16, which makes each segment 66 octets instead of 64.
 
### Gridded Data PDU
 
Grid data records that use representation type 0 are padded to a 16-bit boundary
instead of a 32-bit one. Types 1 and 2 are padded correctly.
 
### Data Query PDU
 
The PDU carries full fixed and variable datum records. The standard calls for
32-bit datum IDs only. The reliable version, Data Query-R, is correct.
 
### Minefield Response NACK PDU
 
Missing PDU sequence numbers are encoded as 64-bit values with no padding. The
standard calls for 8-bit values padded to a 32-bit boundary.
 
### Directed Energy Fire PDU
 
- The PDU carries Damage Description records (record type 4500). It should carry DE
  Precision Aimpoint (4000) or DE Area Aimpoint (4001) records.
- **To verify:** the 32-bit field after Wavelength is treated as padding; it may be
  Peak Irradiance, a 32-bit float.

## Simplified records
 
These PDUs encode correctly when the affected records are empty, but the records
are modeled more simply than the standard defines them.
 
### Aggregate State PDU
 
The silent aggregate and silent entity lists are encoded as plain entity types.
The standard defines Silent Aggregate System records (12 octets each) and Silent
Entity System records (variable length, with appearance records).
 
### Is Group Of PDU
 
Grouped entity descriptions are encoded as 64-bit values. In the standard, their
size and contents depend on the grouped entity category.
 
### Environmental Process PDU
 
Environment records include only their 8-octet header. The geometry and state data
that follows the header isn't represented.
 
### Intercom Control PDU
 
Intercom communications parameter records have a fixed 32-bit body, although the
standard allows them to be any length given by their record length field.
 
### IFF PDU
 
Layer 1 and layer 2 are decoded into fields. Any further layers, such as Mode 5 and
Mode S data, are kept as raw octets in `IffLayers::other_layers`.
 
## Still to verify
 
These may differ from the standard. They need checking against the text of
IEEE 1278.1-2012 or against captures.
 
### Record Specification record
 
Used by the Transfer Ownership and Set Record-R PDUs. The crate treats the record
length as a number of octets and doesn't pad the record values. The standard may
define the length in bits, require a 32-bit padding field after the record set
serial number, and pad the values to a 64-bit boundary.
 
### Live Entity Fire and Detonation PDUs
 
The flag fields are kept as plain integers, and every optional field is always
encoded. The standard may use the flags to mark some fields as present or absent.
 
## Fields kept as raw values
 
These fields encode correctly, but the crate doesn't interpret them for you. You
read and write their bits yourself, using the SISO-REF-010 enumerations.
 
- Entity appearance and entity capabilities, in the Entity State and Entity State
  Update PDUs.
- Variable parameter records. These are kept as four generic fields rather than as
  typed articulated or attached part records.
- Modulation parameters and antenna patterns in the Transmitter PDU.
- Signal and Intercom Signal data.
- Specific object appearance in the object state PDUs.

The Damage Description record defaults to a record type and length of 0. Set them to
4500 and 40 when you build one yourself.
