//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Radio Communications protocol family

use crate::{
    common::{
        GenericHeader, SerializedLength,
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
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.7.2
    pub struct TransmitterPdu {
        header: PduHeader,
        pdu_type: PduType::Transmitter,
        protocol_family: ProtocolFamily::RadioCommunications,
        fields: {
            pub entity_id: EntityId,
            pub radio_id: u16,
            pub radio_entity_type: RadioEntityType,
            pub transmit_state: TransmitterTransmitState,
            pub input_source: TransmitterInputSource,
            pub number_of_variable_transmitter_parameters_records: u16,
            pub antenna_location: WorldCoordinate,
            pub relative_antenna_location: EntityCoordinateVector,
            pub antenna_pattern_type: TransmitterAntennaPatternType,
            pub antenna_pattern_length: u16,
            pub frequency: u64,
            pub transmit_frequency_bandwidth: f32,
            pub power: f32,
            pub modulation_type: ModulationType,
            pub crypto_system: TransmitterCryptoSystem,
            pub crypto_key_id: u16,
            pub modulation_parameter_length: u8,
            padding: u8,
            padding2: u16,
            #[len = modulation_parameter_length]
            pub modulation_parameters: Option<ModulationParameters>,
            pub antenna_pattern: Option<AntennaPattern>,
            pub variable_transmitter_parameters: Vec<VariableTransmitterParameters>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.7.3
    pub struct SignalPdu {
        header: PduHeader,
        pdu_type: PduType::Signal,
        protocol_family: ProtocolFamily::RadioCommunications,
        fields: {
            pub entity_id: EntityId,
            pub radio_id: u16,
            pub encoding_scheme: u16,
            pub tdl_type: SignalTDLType,
            pub sample_rate: u32,
            pub data_length: u16,
            pub samples: u16,
            pub data: Vec<u8>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.7.4
    pub struct ReceiverPdu {
        header: PduHeader,
        pdu_type: PduType::Receiver,
        protocol_family: ProtocolFamily::RadioCommunications,
        fields: {
            pub entity_id: EntityId,
            pub radio_id: u16,
            pub receiver_state: ReceiverReceiverState,
            padding: u16,
            pub received_power: f32,
            pub transmitter_radio_reference_id: EntityId,
            pub transmitter_radio_id: u16,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.7.5
    pub struct IntercomSignalPdu {
        header: PduHeader,
        pdu_type: PduType::IntercomSignal,
        protocol_family: ProtocolFamily::RadioCommunications,
        fields: {
            pub intercom_reference_id: EntityId,
            pub intercom_number: u16,
            pub encoding_scheme: u16,
            pub tdl_type: SignalTDLType,
            pub sample_rate: u32,
            pub data_length: u16,
            pub samples: u16,
            pub data: Vec<u8>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.7.6
    pub struct IntercomControlPdu {
        header: PduHeader,
        pdu_type: PduType::IntercomControl,
        protocol_family: ProtocolFamily::RadioCommunications,
        fields: {
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
            pub intercom_parameters_length: u32,
            pub intercom_parameters: Vec<IntercomCommunicationsParameters>,
        }
    }
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
    fn to_bytes<P: Pdu>(pdu: &mut P) -> Bytes {
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
