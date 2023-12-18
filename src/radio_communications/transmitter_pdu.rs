//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    vector3_double::Vector3Double,
    vector3_float::Vector3Float,
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use super::{modulation_type::ModulationType, radio_entity_type::RadioEntityType};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §5.8.3
pub struct TransmitterPdu {
    pub pdu_header: PduHeader,
    pub entity_id: EntityId,
    pub radio_id: u16,
    pub radio_entity_type: RadioEntityType,
    pub transmit_state: u8,
    pub input_source: u8,
    pub padding1: u16,
    pub antenna_location: Vector3Double,
    pub relative_antenna_location: Vector3Float,
    pub antenna_pattern_type: u16,
    pub antenna_pattern_count: u16,
    pub frequency: u64,
    pub transmit_frequency_bandwidth: f32,
    pub power: f32,
    pub modulation_type: ModulationType,
    pub crypto_system: u16,
    pub crypto_key_id: u16,
    pub modulation_parameter_count: u8,
    pub padding2: u16,
    pub padding3: u8,
    pub modulation_parameter_list: Vec<Vector3Float>,
    pub antenna_pattern_list: Vec<Vector3Float>,
}

impl Default for TransmitterPdu {
    /// Creates a default Transmitter PDU with arbitrary originating and receiving
    /// entity IDs
    ///
    /// # Examples
    ///
    /// Initializing a Transmitter PDU:
    /// ```
    /// use open_dis_rust::radio_communications::transmitter_pdu::TransmitterPdu;
    /// let transmitter_pdu = TransmitterPdu::default();
    /// ```
    ///
    fn default() -> Self {
        TransmitterPdu {
            pdu_header: PduHeader::default(
                PduType::Transmitter,
                ProtocolFamily::RadioCommunications,
                128,
            ),
            entity_id: EntityId::default(1),
            radio_id: 0,
            radio_entity_type: RadioEntityType::default(),
            transmit_state: 0,
            input_source: 0,
            padding1: 0,
            antenna_location: Vector3Double::default(),
            relative_antenna_location: Vector3Float::default(),
            antenna_pattern_type: 0,
            antenna_pattern_count: 0,
            frequency: 0,
            transmit_frequency_bandwidth: 0.0,
            power: 0.0,
            modulation_type: ModulationType::default(),
            crypto_system: 0,
            crypto_key_id: 0,
            modulation_parameter_count: 0,
            padding2: 0,
            padding3: 0,
            modulation_parameter_list: vec![Vector3Float::default()],
            antenna_pattern_list: vec![Vector3Float::default()],
        }
    }
}

impl Pdu for TransmitterPdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.entity_id.serialize(buf);
        buf.put_u16(self.radio_id);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::Transmitter {
            let entity_id = EntityId::decode(&mut buffer);
            let radio_id = buffer.get_u16();
            let radio_entity_type = RadioEntityType::decode(&mut buffer);
            let transmit_state = buffer.get_u8();
            let input_source = buffer.get_u8();
            let padding1 = buffer.get_u16();
            let antenna_location = Vector3Double::decode(&mut buffer);
            let relative_antenna_location = Vector3Float::decode(&mut buffer);
            let antenna_pattern_type = buffer.get_u16();
            let antenna_pattern_count = buffer.get_u16();
            let frequency = buffer.get_u64();
            let transmit_frequency_bandwidth = buffer.get_f32();
            let power = buffer.get_f32();
            let modulation_type = ModulationType::decode(&mut buffer);
            let crypto_system = buffer.get_u16();
            let crypto_key_id = buffer.get_u16();
            let modulation_parameter_count = buffer.get_u8();
            let padding2 = buffer.get_u16();
            let padding3 = buffer.get_u8();
            let mut modulation_parameter_list: Vec<Vector3Float> = vec![];
            for _i in 0..modulation_parameter_count {
                modulation_parameter_list.push(Vector3Float::decode(&mut buffer));
            }
            let mut antenna_pattern_list: Vec<Vector3Float> = vec![];
            for _i in 0..antenna_pattern_count {
                antenna_pattern_list.push(Vector3Float::decode(&mut buffer));
            }
            Ok(TransmitterPdu {
                pdu_header,
                entity_id,
                radio_id,
                radio_entity_type,
                transmit_state,
                input_source,
                padding1,
                antenna_location,
                relative_antenna_location,
                antenna_pattern_type,
                antenna_pattern_count,
                frequency,
                transmit_frequency_bandwidth,
                power,
                modulation_type,
                crypto_system,
                crypto_key_id,
                modulation_parameter_count,
                padding2,
                padding3,
                modulation_parameter_list,
                antenna_pattern_list,
            })
        } else {
            Err(DISError::InvalidDISHeader)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn deserialize_without_header(
        mut buffer: BytesMut,
        pdu_header: PduHeader,
    ) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let entity_id = EntityId::decode(&mut buffer);
        let radio_id = buffer.get_u16();
        let radio_entity_type = RadioEntityType::decode(&mut buffer);
        let transmit_state = buffer.get_u8();
        let input_source = buffer.get_u8();
        let padding1 = buffer.get_u16();
        let antenna_location = Vector3Double::decode(&mut buffer);
        let relative_antenna_location = Vector3Float::decode(&mut buffer);
        let antenna_pattern_type = buffer.get_u16();
        let antenna_pattern_count = buffer.get_u16();
        let frequency = buffer.get_u64();
        let transmit_frequency_bandwidth = buffer.get_f32();
        let power = buffer.get_f32();
        let modulation_type = ModulationType::decode(&mut buffer);
        let crypto_system = buffer.get_u16();
        let crypto_key_id = buffer.get_u16();
        let modulation_parameter_count = buffer.get_u8();
        let padding2 = buffer.get_u16();
        let padding3 = buffer.get_u8();
        let mut modulation_parameter_list: Vec<Vector3Float> = vec![];
        for _i in 0..modulation_parameter_count {
            modulation_parameter_list.push(Vector3Float::decode(&mut buffer));
        }
        let mut antenna_pattern_list: Vec<Vector3Float> = vec![];
        for _i in 0..antenna_pattern_count {
            antenna_pattern_list.push(Vector3Float::decode(&mut buffer));
        }
        Ok(TransmitterPdu {
            pdu_header,
            entity_id,
            radio_id,
            radio_entity_type,
            transmit_state,
            input_source,
            padding1,
            antenna_location,
            relative_antenna_location,
            antenna_pattern_type,
            antenna_pattern_count,
            frequency,
            transmit_frequency_bandwidth,
            power,
            modulation_type,
            crypto_system,
            crypto_key_id,
            modulation_parameter_count,
            padding2,
            padding3,
            modulation_parameter_list,
            antenna_pattern_list,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::TransmitterPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let transmitter_pdu = TransmitterPdu::default();
        let pdu_header = PduHeader::default(
            PduType::Transmitter,
            ProtocolFamily::RadioCommunications,
            1024 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            transmitter_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            transmitter_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, transmitter_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            transmitter_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, transmitter_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, transmitter_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let transmitter_pdu = TransmitterPdu::default();
        let mut buffer = BytesMut::new();
        transmitter_pdu.serialize(&mut buffer);

        let new_transmitter_pdu = TransmitterPdu::deserialize(buffer).unwrap();
        assert_eq!(new_transmitter_pdu.pdu_header, transmitter_pdu.pdu_header);
    }
}
