//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    ClockTime, EntityCoordinateVector, EntityType, EventId, SerializedLength,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{DEFirePulseShape, PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
};

use super::data_types::directed_energy_damage::DirectedEnergyDamage;

#[derive(Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.3.4
pub struct DirectedEnergyFirePdu {
    pdu_header: PduHeader,
    pub firing_entity_id: EntityId,
    pub event_id: EventId,
    pub munition_type: EntityType,
    pub shot_start_time: ClockTime,
    pub cumulative_shot_time: f32,
    pub aperture_emitter_location: EntityCoordinateVector,
    pub aperture_diameter: f32,
    pub wavelength: f32,
    padding: u32,
    pub pulse_repetition_frequency: f32,
    pub pulse_width: f32,
    pub flags: u16,
    pub pulse_shape: DEFirePulseShape,
    padding2: u8,
    padding3: u32,
    padding4: u16,
    pub number_of_de_records: u16,
    pub damage_descriptions: Vec<DirectedEnergyDamage>,
}

impl Pdu for DirectedEnergyFirePdu {
    fn calculate_length(&self) -> Result<u16, DISError> {
        let length = PduHeader::LENGTH
            + EntityId::LENGTH
            + EventId::LENGTH
            + EntityType::LENGTH
            + ClockTime::LENGTH
            + 4
            + EntityCoordinateVector::LENGTH
            + 4
            + 4
            + 4
            + 4
            + 4
            + 2
            + 1
            + 1
            + 4
            + 2
            + 2;

        u16::try_from(length).map_err(|_| DISError::PduSizeExceeded {
            size: length,
            max_size: MAX_PDU_SIZE_OCTETS,
        })
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    /// Serialize contents of `DirectedEnergyFirePdu` into `BytesMut` buffer
    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let length = self.calculate_length()?;
        self.pdu_header.set_length(length);
        self.pdu_header.serialize(buf);
        self.firing_entity_id.serialize(buf);
        self.event_id.serialize(buf);
        self.munition_type.serialize(buf);
        self.shot_start_time.serialize(buf);
        buf.put_f32(self.cumulative_shot_time);
        self.aperture_emitter_location.serialize(buf);
        buf.put_f32(self.aperture_diameter);
        buf.put_f32(self.wavelength);
        buf.put_u32(self.padding);
        buf.put_f32(self.pulse_repetition_frequency);
        buf.put_f32(self.pulse_width);
        buf.put_u16(self.flags);
        buf.put_u8(self.pulse_shape as u8);
        buf.put_u8(self.padding2);
        buf.put_u32(self.padding3);
        buf.put_u16(self.padding4);
        buf.put_u16(self.number_of_de_records);
        for i in 0..self.damage_descriptions.len() {
            self.damage_descriptions[i].serialize(buf);
        }
        Ok(())
    }

    /// Deserialize bytes from `BytesMut` buffer and interpret as `DirectedEnergyFirePdu`
    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::DirectedEnergyFire {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type DirectedEnergyFire, got {:?}",
                    header.pdu_type
                ),
                None,
            ));
        }
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }

    /// Treat `DirectedEnergyFirePdu` as Any type
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

impl DirectedEnergyFirePdu {
    #[must_use]
    /// Creates a new Entity Damage Status PDU
    ///
    /// # Examples
    ///
    /// Initializing an Entity Damage Status PDU:
    /// ```
    /// use open_dis_rust::warfare::DirectedEnergyFirePdu;
    /// let pdu = DirectedEnergyFirePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::DirectedEnergyFire;
        pdu.pdu_header.protocol_family = ProtocolFamily::Warfare;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let firing_entity_id = EntityId::deserialize(buf);
        let event_id = EventId::deserialize(buf);
        let munition_type = EntityType::deserialize(buf);
        let shot_start_time = ClockTime::deserialize(buf);
        let cumulative_shot_time = buf.get_f32();
        let aperture_emitter_location = EntityCoordinateVector::deserialize(buf);
        let aperture_diameter = buf.get_f32();
        let wavelength = buf.get_f32();
        let padding = buf.get_u32();
        let pulse_repetition_frequency = buf.get_f32();
        let pulse_width = buf.get_f32();
        let flags = buf.get_u16();
        let pulse_shape = DEFirePulseShape::deserialize(buf);
        let padding2 = buf.get_u8();
        let padding3 = buf.get_u32();
        let padding4 = buf.get_u16();
        let number_of_de_records = buf.get_u16();
        let mut damage_descriptions: Vec<DirectedEnergyDamage> = vec![];
        for _ in 0..number_of_de_records {
            damage_descriptions.push(DirectedEnergyDamage::deserialize(buf));
        }

        DirectedEnergyFirePdu {
            pdu_header: PduHeader::default(),
            firing_entity_id,
            event_id,
            munition_type,
            shot_start_time,
            cumulative_shot_time,
            aperture_emitter_location,
            aperture_diameter,
            wavelength,
            padding,
            pulse_repetition_frequency,
            pulse_width,
            flags,
            pulse_shape,
            padding2,
            padding3,
            padding4,
            number_of_de_records,
            damage_descriptions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DirectedEnergyFirePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = DirectedEnergyFirePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<DirectedEnergyFirePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = DirectedEnergyFirePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = DirectedEnergyFirePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 704 / BITS_PER_BYTE;
        let pdu = DirectedEnergyFirePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
