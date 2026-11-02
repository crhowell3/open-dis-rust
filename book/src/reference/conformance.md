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
