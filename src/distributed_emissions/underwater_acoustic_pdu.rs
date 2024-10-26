//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    event_id::EventId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

use super::data_types::{
    acoustic_emitter_system::AcousticEmitterSystem, apa_data::ApaData, shaft_rpms::ShaftRPMs,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.6.4
pub struct UnderwaterAcousticPdu {
    pub pdu_header: PduHeader,
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub state_change_indicator: i8,
    pub pad: i8,
    pub passive_parameter_index: u16,
    pub propulsion_plant_configuration: u8,
    pub number_of_shafts: u8,
    pub number_of_apas: u8,
    pub number_of_ua_emitter_systems: u8,
    pub shaft_rpms: Vec<ShaftRPMs>,
    pub apa_data: Vec<ApaData>,
    pub emitter_systems: Vec<AcousticEmitterSystem>,
}

impl Default for UnderwaterAcousticPdu {
    /// Creates a default-initialized Underwater Acoustic PDU
    ///
    /// # Examples
    ///
    /// Initializing an Underwater Acoustic PDU:
    /// ```
    /// use open_dis_rust::distributed_emissions::underwater_acoustic_pdu::UnderwaterAcousticPdu;
    /// let mut underwater_acoustic_pdu = UnderwaterAcousticPdu::default();
    /// ```
    ///
    fn default() -> Self {
        UnderwaterAcousticPdu {
            pdu_header: PduHeader::default(
                PduType::UnderwaterAcoustic,
                ProtocolFamily::DistributedEmissionRegeneration,
                56,
            ),
            emitting_entity_id: EntityId::default(1),
            event_id: EventId::default(1),
            state_change_indicator: 0,
            pad: 0,
            passive_parameter_index: 0,
            propulsion_plant_configuration: 0,
            number_of_shafts: 0,
            number_of_apas: 0,
            number_of_ua_emitter_systems: 0,
            shaft_rpms: vec![],
            apa_data: vec![],
            emitter_systems: vec![],
        }
    }
}

impl Pdu for UnderwaterAcousticPdu {
    /// Serialize contents of UnderwaterAcousticPdu into BytesMut buffer
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.emitting_entity_id.serialize(buf);
        self.event_id.serialize(buf);
        buf.put_i8(self.state_change_indicator);
        buf.put_i8(self.pad);
        buf.put_u16(self.passive_parameter_index);
        buf.put_u8(self.propulsion_plant_configuration);
        buf.put_u8(self.number_of_shafts);
        buf.put_u8(self.number_of_apas);
        buf.put_u8(self.number_of_ua_emitter_systems);
        for i in 0..self.shaft_rpms.len() {
            self.shaft_rpms[i].serialize(buf);
        }
        for i in 0..self.apa_data.len() {
            self.apa_data[i].serialize(buf);
        }
        for i in 0..self.emitter_systems.len() {
            self.emitter_systems[i].serialize(buf);
        }
    }

    /// Deserialize bytes from BytesMut buffer and interpret as UnderwaterAcousticPdu
    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::UnderwaterAcoustic {
            let emitting_entity_id = EntityId::decode(&mut buffer);
            let event_id = EventId::decode(&mut buffer);
            let state_change_indicator = buffer.get_i8();
            let pad = buffer.get_i8();
            let passive_parameter_index = buffer.get_u16();
            let propulsion_plant_configuration = buffer.get_u8();
            let number_of_shafts = buffer.get_u8();
            let number_of_apas = buffer.get_u8();
            let number_of_ua_emitter_systems = buffer.get_u8();
            let mut shaft_rpms: Vec<ShaftRPMs> = vec![];
            for _i in 0..number_of_shafts {
                shaft_rpms.push(ShaftRPMs::decode(&mut buffer));
            }
            let mut apa_data: Vec<ApaData> = vec![];
            for _i in 0..number_of_apas {
                apa_data.push(ApaData::decode(&mut buffer));
            }
            let mut emitter_systems: Vec<AcousticEmitterSystem> = vec![];
            for _i in 0..number_of_ua_emitter_systems {
                emitter_systems.push(AcousticEmitterSystem::decode(&mut buffer));
            }
            Ok(UnderwaterAcousticPdu {
                pdu_header,
                emitting_entity_id,
                event_id,
                state_change_indicator,
                pad,
                passive_parameter_index,
                propulsion_plant_configuration,
                number_of_shafts,
                number_of_apas,
                number_of_ua_emitter_systems,
                shaft_rpms,
                apa_data,
                emitter_systems,
            })
        } else {
            Err(DISError::InvalidDISHeader)
        }
    }

    /// Treat UnderwaterAcousticPdu as Any type
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Deserialize bytes from BytesMut buffer, but assume PDU header exists already
    fn deserialize_without_header(
        mut buffer: BytesMut,
        pdu_header: PduHeader,
    ) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let emitting_entity_id = EntityId::decode(&mut buffer);
        let event_id = EventId::decode(&mut buffer);
        let state_change_indicator = buffer.get_i8();
        let pad = buffer.get_i8();
        let passive_parameter_index = buffer.get_u16();
        let propulsion_plant_configuration = buffer.get_u8();
        let number_of_shafts = buffer.get_u8();
        let number_of_apas = buffer.get_u8();
        let number_of_ua_emitter_systems = buffer.get_u8();
        let mut shaft_rpms: Vec<ShaftRPMs> = vec![];
        for _i in 0..number_of_shafts {
            shaft_rpms.push(ShaftRPMs::decode(&mut buffer));
        }
        let mut apa_data: Vec<ApaData> = vec![];
        for _i in 0..number_of_apas {
            apa_data.push(ApaData::decode(&mut buffer));
        }
        let mut emitter_systems: Vec<AcousticEmitterSystem> = vec![];
        for _i in 0..number_of_ua_emitter_systems {
            emitter_systems.push(AcousticEmitterSystem::decode(&mut buffer));
        }
        Ok(UnderwaterAcousticPdu {
            pdu_header,
            emitting_entity_id,
            event_id,
            state_change_indicator,
            pad,
            passive_parameter_index,
            propulsion_plant_configuration,
            number_of_shafts,
            number_of_apas,
            number_of_ua_emitter_systems,
            shaft_rpms,
            apa_data,
            emitter_systems,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::UnderwaterAcousticPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let supplemental_emission_pdu = UnderwaterAcousticPdu::default();
        let pdu_header = PduHeader::default(
            PduType::UnderwaterAcoustic,
            ProtocolFamily::DistributedEmissionRegeneration,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            supplemental_emission_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            supplemental_emission_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            supplemental_emission_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            supplemental_emission_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            supplemental_emission_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.padding,
            supplemental_emission_pdu.pdu_header.padding
        );
    }

    #[test]
    fn cast_to_any() {
        let udnerwater_acoustic_pdu = UnderwaterAcousticPdu::default();
        let any_pdu = udnerwater_acoustic_pdu.as_any();

        assert!(any_pdu.is::<UnderwaterAcousticPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut supplemental_emission_pdu = UnderwaterAcousticPdu::default();
        let mut buffer = BytesMut::new();
        supplemental_emission_pdu.serialize(&mut buffer);

        let new_supplemental_emission_pdu = UnderwaterAcousticPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_supplemental_emission_pdu.pdu_header,
            supplemental_emission_pdu.pdu_header
        );
    }
}
