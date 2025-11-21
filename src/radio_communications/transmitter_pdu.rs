//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::{
    common::{
        EntityCoordinateVector, WorldCoordinate,
        constants::MAX_PDU_SIZE_OCTETS,
        dis_error::DISError,
        entity_id::EntityId,
        enums::{
            PduType, ProtocolFamily, TransmitterAntennaPatternType, TransmitterCryptoSystem,
            TransmitterInputSource, TransmitterTransmitState,
        },
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    radio_communications::data_types::{
        antenna_pattern::AntennaPattern, modulation_parameters::ModulationParameters,
        variable_transmitter_parameters::VariableTransmitterParameters,
    },
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use super::data_types::{modulation_type::ModulationType, radio_entity_type::RadioEntityType};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.7.2
pub struct TransmitterPdu {
    pdu_header: PduHeader,
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
    _padding: u8,
    _padding2: u16,
    pub modulation_parameters: Option<ModulationParameters>,
    pub antenna_pattern: Option<AntennaPattern>,
    pub variable_transmitter_parameters: Vec<VariableTransmitterParameters>,
}

impl Default for TransmitterPdu {
    fn default() -> Self {
        TransmitterPdu {
            pdu_header: PduHeader::default(),
            entity_id: EntityId::default(1),
            radio_id: 0u16,
            radio_entity_type: RadioEntityType::default(),
            transmit_state: TransmitterTransmitState::default(),
            input_source: TransmitterInputSource::default(),
            number_of_variable_transmitter_parameters_records: 0u16,
            antenna_location: WorldCoordinate::default(),
            relative_antenna_location: EntityCoordinateVector::default(),
            antenna_pattern_type: TransmitterAntennaPatternType::default(),
            antenna_pattern_length: 0u16,
            frequency: 0u64,
            transmit_frequency_bandwidth: 0.0,
            power: 0.0,
            modulation_type: ModulationType::default(),
            crypto_system: TransmitterCryptoSystem::default(),
            crypto_key_id: 0u16,
            modulation_parameter_length: 0u8,
            _padding: 0u8,
            _padding2: 0u16,
            modulation_parameters: None,
            antenna_pattern: None,
            variable_transmitter_parameters: vec![],
        }
    }
}

impl Pdu for TransmitterPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>()
            + std::mem::size_of::<u16>() * 5
            + std::mem::size_of::<RadioEntityType>()
            + std::mem::size_of::<TransmitterTransmitState>()
            + std::mem::size_of::<TransmitterInputSource>()
            + std::mem::size_of::<WorldCoordinate>()
            + std::mem::size_of::<EntityCoordinateVector>()
            + std::mem::size_of::<TransmitterAntennaPatternType>()
            + std::mem::size_of::<u64>()
            + std::mem::size_of::<f32>() * 2
            + std::mem::size_of::<ModulationType>()
            + std::mem::size_of::<TransmitterCryptoSystem>()
            + std::mem::size_of::<u8>() * 2
            + std::mem::size_of::<Option<ModulationParameters>>()
            + std::mem::size_of::<Option<AntennaPattern>>();

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let size = std::mem::size_of_val(self);
        self.pdu_header.length = u16::try_from(size).map_err(|_| DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })?;
        self.pdu_header.serialize(buf);
        self.entity_id.serialize(buf);
        buf.put_u16(self.radio_id);
        self.radio_entity_type.serialize(buf);
        buf.put_u8(self.transmit_state as u8);
        buf.put_u8(self.input_source as u8);
        buf.put_u16(self.number_of_variable_transmitter_parameters_records);
        self.antenna_location.serialize(buf);
        self.relative_antenna_location.serialize(buf);
        buf.put_u16(self.antenna_pattern_type as u16);
        buf.put_u16(self.antenna_pattern_length);
        buf.put_u64(self.frequency);
        buf.put_f32(self.transmit_frequency_bandwidth);
        buf.put_f32(self.power);
        self.modulation_type.serialize(buf);
        buf.put_u16(self.crypto_system as u16);
        buf.put_u16(self.crypto_key_id);
        buf.put_u8(self.modulation_parameter_length);
        buf.put_u8(self._padding);
        buf.put_u16(self._padding2);
        if let Some(modulation_parameters) = &self.modulation_parameters {
            modulation_parameters.serialize(buf);
        }
        if let Some(antenna_pattern) = &self.antenna_pattern {
            antenna_pattern.serialize(buf);
        }
        for i in 0..self.number_of_variable_transmitter_parameters_records as usize {
            self.variable_transmitter_parameters[i].serialize(buf);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::Transmitter {
            return Err(DISError::invalid_header(
                format!("Expected PDU type Transmitter, got {:?}", header.pdu_type),
                None,
            ));
        }
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn deserialize_without_header<B: Buf>(buf: &mut B, header: PduHeader) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }
}

impl TransmitterPdu {
    /// Creates a new `TransmitterPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `TransmitterPdu`:
    /// ```
    /// use open_dis_rust::radio_communications::TransmitterPdu;
    /// let pdu = TransmitterPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::Transmitter;
        pdu.pdu_header.protocol_family = ProtocolFamily::RadioCommunications;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let entity_id = EntityId::deserialize(buf);
        let radio_id = buf.get_u16();
        let radio_entity_type = RadioEntityType::deserialize(buf);
        let transmit_state = TransmitterTransmitState::deserialize(buf);
        let input_source = TransmitterInputSource::deserialize(buf);
        let number_of_variable_transmitter_parameters_records = buf.get_u16();
        let antenna_location = WorldCoordinate::deserialize(buf);
        let relative_antenna_location = EntityCoordinateVector::deserialize(buf);
        let antenna_pattern_type = TransmitterAntennaPatternType::deserialize(buf);
        let antenna_pattern_length = buf.get_u16();
        let frequency = buf.get_u64();
        let transmit_frequency_bandwidth = buf.get_f32();
        let power = buf.get_f32();
        let modulation_type = ModulationType::deserialize(buf);
        let crypto_system = TransmitterCryptoSystem::deserialize(buf);
        let crypto_key_id = buf.get_u16();
        let modulation_parameter_length = buf.get_u8();
        let _padding = buf.get_u8();
        let _padding2 = buf.get_u16();
        let modulation_parameters =
            ModulationParameters::deserialize(buf, modulation_parameter_length);
        let antenna_pattern = AntennaPattern::deserialize(buf, antenna_pattern_length);
        let mut variable_transmitter_parameters: Vec<VariableTransmitterParameters> = vec![];
        for _ in 0..number_of_variable_transmitter_parameters_records {
            variable_transmitter_parameters.push(VariableTransmitterParameters::deserialize(buf));
        }

        TransmitterPdu {
            pdu_header: PduHeader::default(),
            entity_id,
            radio_id,
            radio_entity_type,
            transmit_state,
            input_source,
            number_of_variable_transmitter_parameters_records,
            antenna_location,
            relative_antenna_location,
            antenna_pattern_type,
            antenna_pattern_length,
            frequency,
            transmit_frequency_bandwidth,
            power,
            modulation_type,
            crypto_system,
            crypto_key_id,
            modulation_parameter_length,
            _padding,
            _padding2,
            modulation_parameters,
            antenna_pattern,
            variable_transmitter_parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TransmitterPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn create_header() {
        let pdu = TransmitterPdu::new();
        let pdu_header = PduHeader::default();

        assert_eq!(pdu_header.protocol_version, pdu.pdu_header.protocol_version);
        assert_eq!(pdu_header.exercise_id, pdu.pdu_header.exercise_id);
        assert_eq!(pdu_header.pdu_type, pdu.pdu_header.pdu_type);
        assert_eq!(pdu_header.protocol_family, pdu.pdu_header.protocol_family);
        assert_eq!(pdu_header.length, pdu.pdu_header.length);
        assert_eq!(pdu_header.status_record, pdu.pdu_header.status_record);
    }

    #[test]
    fn cast_to_any() {
        let pdu = TransmitterPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<TransmitterPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = TransmitterPdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = TransmitterPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 832 / 8;
        let pdu = TransmitterPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
