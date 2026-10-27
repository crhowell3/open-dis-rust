//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Radio Communications protocol family

use deku::{
    DekuError, DekuReader, DekuWriter,
    no_std_io::{Read, Seek, Write},
    reader::Reader,
    writer::Writer,
};

use crate::common::{
    data_types::{
        AntennaPattern, EntityCoordinateVector, EntityId, IntercomCommunicationsParameters,
        ModulationParameters, ModulationType, RadioEntityType, VariableTransmitterParameters,
        WorldCoordinate,
    },
    enums::{
        IntercomControlCommand, IntercomControlControlType, IntercomControlTransmitLineState,
        PduType, ProtocolFamily, ReceiverReceiverState, SignalTDLType,
        TransmitterAntennaPatternType, TransmitterCryptoSystem, TransmitterInputSource,
        TransmitterTransmitState,
    },
    pdu_header::PduHeader,
};

/// Implemented according to IEEE 1278.1-2012 §7.7.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct TransmitterPdu {
    #[deku(skip)]
    header: PduHeader,
    pub entity_id: EntityId,
    pub radio_id: u16,
    pub radio_entity_type: RadioEntityType,
    pub transmit_state: TransmitterTransmitState,
    pub input_source: TransmitterInputSource,
    #[deku(
        update = "crate::common::wire::update_each(&mut self.variable_transmitter_parameters)?",
        assert = "crate::common::wire::fits_in_pdu(*number_of_variable_transmitter_parameters_records)"
    )]
    pub number_of_variable_transmitter_parameters_records: u16,
    pub antenna_location: WorldCoordinate,
    pub relative_antenna_location: EntityCoordinateVector,
    pub antenna_pattern_type: TransmitterAntennaPatternType,
    /// Octets of `antenna_pattern`. Set by `serialize`.
    #[deku(update = "blob_len(self.antenna_pattern.as_ref())")]
    pub antenna_pattern_length: u16,
    pub frequency: u64,
    pub transmit_frequency_bandwidth: f32,
    pub power: f32,
    pub modulation_type: ModulationType,
    pub crypto_system: TransmitterCryptoSystem,
    pub crypto_key_id: u16,
    /// Octets of `modulation_parameters`. Set by `serialize`.
    #[deku(update = "blob_len(self.modulation_parameters.as_ref())")]
    pub modulation_parameter_length: u8,
    padding: u8,
    padding2: u16,
    #[deku(
        reader = "read_blob(deku::reader, usize::from(*modulation_parameter_length)).map(|f| f.map(|record_specific_fields| ModulationParameters { record_specific_fields }))",
        writer = "write_blob(deku::writer, modulation_parameters.as_ref().map(|m| m.record_specific_fields.as_slice()))"
    )]
    pub modulation_parameters: Option<ModulationParameters>,
    #[deku(
        reader = "read_blob(deku::reader, usize::from(*antenna_pattern_length)).map(|f| f.map(|record_specific_fields| AntennaPattern { record_specific_fields }))",
        writer = "write_blob(deku::writer, antenna_pattern.as_ref().map(|a| a.record_specific_fields.as_slice()))"
    )]
    pub antenna_pattern: Option<AntennaPattern>,
    #[deku(count = "number_of_variable_transmitter_parameters_records")]
    pub variable_transmitter_parameters: Vec<VariableTransmitterParameters>,
}
crate::impl_pdu!(
    TransmitterPdu,
    PduHeader,
    PduType::Transmitter,
    ProtocolFamily::RadioCommunications
);

/// Implemented according to IEEE 1278.1-2012 §7.7.3
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct SignalPdu {
    #[deku(skip)]
    header: PduHeader,
    pub entity_id: EntityId,
    pub radio_id: u16,
    pub encoding_scheme: u16,
    pub tdl_type: SignalTDLType,
    pub sample_rate: u32,
    /// Length of `data` in bits. Kept as set when it agrees with `data`;
    /// otherwise `serialize` sets it to `data.len() * 8`.
    #[deku(update = "signal_data_length(self.data_length, self.data.len())?")]
    pub data_length: u16,
    pub samples: u16,
    /// `ceil(data_length / 8)` octets, padded with zeros to a 32-bit
    /// boundary on the wire.
    #[deku(
        reader = "read_signal_data(deku::reader, *data_length)",
        writer = "write_signal_data(deku::writer, data)"
    )]
    pub data: Vec<u8>,
}
crate::impl_pdu!(
    SignalPdu,
    PduHeader,
    PduType::Signal,
    ProtocolFamily::RadioCommunications
);

/// Implemented according to IEEE 1278.1-2012 §7.7.4
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct ReceiverPdu {
    #[deku(skip)]
    header: PduHeader,
    pub entity_id: EntityId,
    pub radio_id: u16,
    pub receiver_state: ReceiverReceiverState,
    padding: u16,
    pub received_power: f32,
    pub transmitter_radio_reference_id: EntityId,
    pub transmitter_radio_id: u16,
}
crate::impl_pdu!(
    ReceiverPdu,
    PduHeader,
    PduType::Receiver,
    ProtocolFamily::RadioCommunications
);

/// Implemented according to IEEE 1278.1-2012 §7.7.5
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct IntercomSignalPdu {
    #[deku(skip)]
    header: PduHeader,
    pub intercom_reference_id: EntityId,
    pub intercom_number: u16,
    pub encoding_scheme: u16,
    pub tdl_type: SignalTDLType,
    pub sample_rate: u32,
    /// Length of `data` in bits. Kept as set when it agrees with `data`;
    /// otherwise `serialize` sets it to `data.len() * 8`.
    #[deku(update = "signal_data_length(self.data_length, self.data.len())?")]
    pub data_length: u16,
    pub samples: u16,
    /// `ceil(data_length / 8)` octets, padded with zeros to a 32-bit
    /// boundary on the wire.
    #[deku(
        reader = "read_signal_data(deku::reader, *data_length)",
        writer = "write_signal_data(deku::writer, data)"
    )]
    pub data: Vec<u8>,
}
crate::impl_pdu!(
    IntercomSignalPdu,
    PduHeader,
    PduType::IntercomSignal,
    ProtocolFamily::RadioCommunications
);

/// Implemented according to IEEE 1278.1-2012 §7.7.6
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct IntercomControlPdu {
    #[deku(skip)]
    header: PduHeader,
    pub control_type: IntercomControlControlType,
    pub communications_channel_type: u8,
    pub source_intercom_reference_id: EntityId,
    pub source_intercom_number: u16,
    pub source_line_id: u8,
    pub transmit_priority: u8,
    pub transmit_line_state: IntercomControlTransmitLineState,
    pub command: IntercomControlCommand,
    pub master_intercom_reference_id: EntityId,
    pub master_intercom_number: u16,
    pub master_channel_id: u16,
    #[deku(
        update = "self.intercom_parameters.len()",
        assert = "crate::common::wire::fits_in_pdu(*intercom_parameters_length)"
    )]
    pub intercom_parameters_length: u32,
    #[deku(count = "intercom_parameters_length")]
    pub intercom_parameters: Vec<IntercomCommunicationsParameters>,
}
crate::impl_pdu!(
    IntercomControlPdu,
    PduHeader,
    PduType::IntercomControl,
    ProtocolFamily::RadioCommunications
);

// ---------------------------------------------------------------------------
// Wire helpers
// ---------------------------------------------------------------------------

/// Anything holding a run of octets whose length a PDU field gives.
trait Blob {
    fn octets(&self) -> usize;
}

impl Blob for ModulationParameters {
    fn octets(&self) -> usize {
        self.record_specific_fields.len()
    }
}

impl Blob for AntennaPattern {
    fn octets(&self) -> usize {
        self.record_specific_fields.len()
    }
}

/// The length field for an optional run of octets: 0 when absent.
fn blob_len<T: Blob>(blob: Option<&T>) -> usize {
    blob.map_or(0, Blob::octets)
}

/// Read `len` octets, or nothing when `len` is 0.
fn read_blob<R: Read + Seek>(
    reader: &mut Reader<R>,
    len: usize,
) -> Result<Option<Vec<u8>>, DekuError> {
    if len == 0 {
        return Ok(None);
    }
    let mut octets = vec![0; len];
    for octet in &mut octets {
        *octet = u8::from_reader_with_ctx(reader, ())?;
    }
    Ok(Some(octets))
}

fn write_blob<W: Write + Seek>(
    writer: &mut Writer<W>,
    octets: Option<&[u8]>,
) -> Result<(), DekuError> {
    octets.map_or(Ok(()), |octets| octets.to_writer(writer, ()))
}

/// Octets of signal data for `data_length` bits.
fn signal_data_octets(data_length: u16) -> usize {
    usize::from(data_length.div_ceil(8))
}

/// Zero octets after `octets` octets of signal data, to a 32-bit boundary.
const fn signal_padding(octets: usize) -> usize {
    octets.next_multiple_of(4).saturating_sub(octets)
}

/// The data length to send: kept when it agrees with `octets`, otherwise
/// `octets * 8`.
fn signal_data_length(current: u16, octets: usize) -> Result<u16, DekuError> {
    if signal_data_octets(current) == octets {
        return Ok(current);
    }
    octets
        .checked_mul(8)
        .and_then(|bits| u16::try_from(bits).ok())
        .ok_or_else(|| DekuError::Assertion("signal data is longer than 8191 octets".into()))
}

fn read_signal_data<R: Read + Seek>(
    reader: &mut Reader<R>,
    data_length: u16,
) -> Result<Vec<u8>, DekuError> {
    let octets = signal_data_octets(data_length);
    let data = read_blob(reader, octets)?.unwrap_or_default();
    for _ in 0..signal_padding(octets) {
        u8::from_reader_with_ctx(reader, ())?;
    }
    Ok(data)
}

fn write_signal_data<W: Write + Seek>(
    writer: &mut Writer<W>,
    data: &[u8],
) -> Result<(), DekuError> {
    data.to_writer(writer, ())?;
    crate::common::wire::write_zeros(writer, signal_padding(data.len()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::{constants::BITS_PER_BYTE, pdu::Pdu},
        pdu_tests,
    };
    use bytes::{Buf, Bytes, BytesMut};

    /// Serialize a PDU, failing the test if serialization returns an error.
    pub fn to_bytes<P: Pdu>(pdu: &mut P) -> Bytes {
        let mut buf = BytesMut::new();
        pdu.serialize(&mut buf).expect("serialize failed");
        buf.freeze()
    }

    pdu_tests!(
        transmitter_pdu_tests, TransmitterPdu,
        pdu_type: PduType::Transmitter,
        default_bits: 832,
        wrong_type: SignalPdu,
    );
    pdu_tests!(
        signal_pdu_tests, SignalPdu,
        pdu_type: PduType::Signal,
        default_bits: 256,
        wrong_type: ReceiverPdu,
    );
    pdu_tests!(
        receiver_pdu_tests, ReceiverPdu,
        pdu_type: PduType::Receiver,
        default_bits: 288,
        wrong_type: IntercomSignalPdu,
    );
    pdu_tests!(
        intercom_signal_pdu_tests, IntercomSignalPdu,
        pdu_type: PduType::IntercomSignal,
        default_bits: 256,
        wrong_type: IntercomControlPdu,
    );
    pdu_tests!(
        intercom_control_pdu_tests, IntercomControlPdu,
        pdu_type: PduType::IntercomControl,
        default_bits: 320,
        wrong_type: SignalPdu,
    );
}
