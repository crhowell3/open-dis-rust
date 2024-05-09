//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    clock_time::ClockTime,
    dis_error::DISError,
    entity_id::EntityId,
    entity_type::EntityType,
    event_id::EventId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    vector3_float::Vector3Float,
};

use super::data_types::standard_variable_specification::StandardVariableSpecification;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §5.4.4
pub struct DirectedEnergyFirePdu {
    pub pdu_header: PduHeader,
    pub firing_entity_id: EntityId,
    pub event_id: EventId,
    pub munition_type: EntityType,
    pub shot_start_time: ClockTime,
    pub cumulative_shot_time: f32,
    pub aperture_emitter_location: Vector3Float,
    pub aperture_diameter: f32,
    pub wavelength: f32,
    pub padding: u32,
    pub pulse_repetition_frequency: f32,
    pub pulse_width: f32,
    pub flags: u16,
    pub pulse_shape: u8,
    pub padding1: u8,
    pub padding2: u32,
    pub padding3: u16,
    pub number_of_de_records: u16,
    pub de_records: Vec<StandardVariableSpecification>,
}

impl Default for DirectedEnergyFirePdu {
    /// Creates a default Directed Energy Fire PDU with arbitrary firing entity ID and target entity
    /// ID
    ///
    /// # Examples
    ///
    /// Initializing a Directed Energy Fire PDU:
    /// ```
    /// use open_dis_rust::warfare::directed_energy_fire_pdu::DirectedEnergyFirePdu;
    /// let directed_enery_fire_pdu = DirectedEnergyFirePdu::default();
    /// ```
    ///
    fn default() -> Self {
        DirectedEnergyFirePdu {
            pdu_header: PduHeader::default(
                PduType::DirectedEnergyFire,
                ProtocolFamily::Warfare,
                56,
            ),
            firing_entity_id: EntityId::default(1),
            event_id: EventId::default(1),
            munition_type: EntityType::default(),
            shot_start_time: ClockTime::default(),
            cumulative_shot_time: 0.0,
            aperture_emitter_location: Vector3Float::default(),
            aperture_diameter: 0.0,
            wavelength: 0.0,
            padding: 0,
            pulse_repetition_frequency: 0.0,
            pulse_width: 0.0,
            flags: 0,
            pulse_shape: 0,
            padding1: 0,
            padding2: 0,
            padding3: 0,
            number_of_de_records: 0,
            de_records: vec![],
        }
    }
}

impl Pdu for DirectedEnergyFirePdu {
    fn serialize(&self, buf: &mut BytesMut) {
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
        buf.put_u8(self.pulse_shape);
        buf.put_u8(self.padding1);
        buf.put_u32(self.padding2);
        buf.put_u16(self.padding3);
        buf.put_u16(self.number_of_de_records);
        for i in 0..self.de_records.len() {
            self.de_records[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::DirectedEnergyFire {
            let firing_entity_id = EntityId::decode(&mut buffer);
            let event_id = EventId::decode(&mut buffer);
            let munition_type = EntityType::decode(&mut buffer);
            let shot_start_time = ClockTime::decode(&mut buffer);
            let cumulative_shot_time = buffer.get_f32();
            let aperture_emitter_location = Vector3Float::decode(&mut buffer);
            let aperture_diameter = buffer.get_f32();
            let wavelength = buffer.get_f32();
            let padding = buffer.get_u32();
            let pulse_repetition_frequency = buffer.get_f32();
            let pulse_width = buffer.get_f32();
            let flags = buffer.get_u16();
            let pulse_shape = buffer.get_u8();
            let padding1 = buffer.get_u8();
            let padding2 = buffer.get_u32();
            let padding3 = buffer.get_u16();
            let number_of_de_records = buffer.get_u16();
            let mut de_records: Vec<StandardVariableSpecification> = vec![];
            for _i in 0..number_of_de_records {
                de_records.push(StandardVariableSpecification::decode(&mut buffer));
            }
            Ok(DirectedEnergyFirePdu {
                pdu_header,
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
                padding1,
                padding2,
                padding3,
                number_of_de_records,
                de_records,
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
        let firing_entity_id = EntityId::decode(&mut buffer);
        let event_id = EventId::decode(&mut buffer);
        let munition_type = EntityType::decode(&mut buffer);
        let shot_start_time = ClockTime::decode(&mut buffer);
        let cumulative_shot_time = buffer.get_f32();
        let aperture_emitter_location = Vector3Float::decode(&mut buffer);
        let aperture_diameter = buffer.get_f32();
        let wavelength = buffer.get_f32();
        let padding = buffer.get_u32();
        let pulse_repetition_frequency = buffer.get_f32();
        let pulse_width = buffer.get_f32();
        let flags = buffer.get_u16();
        let pulse_shape = buffer.get_u8();
        let padding1 = buffer.get_u8();
        let padding2 = buffer.get_u32();
        let padding3 = buffer.get_u16();
        let number_of_de_records = buffer.get_u16();
        let mut de_records: Vec<StandardVariableSpecification> = vec![];
        for _i in 0..number_of_de_records {
            de_records.push(StandardVariableSpecification::decode(&mut buffer));
        }
        Ok(DirectedEnergyFirePdu {
            pdu_header,
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
            padding1,
            padding2,
            padding3,
            number_of_de_records,
            de_records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::DirectedEnergyFirePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let directed_enery_fire_pdu = DirectedEnergyFirePdu::default();
        let pdu_header = PduHeader::default(
            PduType::DirectedEnergyFire,
            ProtocolFamily::Warfare,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            directed_enery_fire_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            directed_enery_fire_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            directed_enery_fire_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            directed_enery_fire_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, directed_enery_fire_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.padding,
            directed_enery_fire_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let directed_enery_fire_pdu = DirectedEnergyFirePdu::default();
        let mut buffer = BytesMut::new();
        directed_enery_fire_pdu.serialize(&mut buffer);

        let new_directed_enery_fire_pdu = DirectedEnergyFirePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_directed_enery_fire_pdu.pdu_header,
            directed_enery_fire_pdu.pdu_header
        );
    }
}
