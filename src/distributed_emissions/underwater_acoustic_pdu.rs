//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    GenericHeader, SerializedLength,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily, UAPassiveParameterIndex, UAStateChangeUpdateIndicator},
    event_id::EventId,
    pdu::Pdu,
    pdu_header::PduHeader,
};

use super::data_types::{
    acoustic_emitter_system::AcousticEmitterSystem, apa_data::ApaData, shaft_rpms::ShaftRPMs,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.6.4
pub struct UnderwaterAcousticPdu {
    pdu_header: PduHeader,
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub state_change_update_indicator: UAStateChangeUpdateIndicator,
    padding: u8,
    pub passive_parameter_index: UAPassiveParameterIndex,
    pub propulsion_plant_configuration: u8,
    pub number_of_shafts: u8,
    pub number_of_apas: u8,
    pub number_of_ua_emitter_systems: u8,
    pub shaft_rpms: Vec<ShaftRPMs>,
    pub apa_data: Vec<ApaData>,
    pub emitter_systems: Vec<AcousticEmitterSystem>,
}

impl Default for UnderwaterAcousticPdu {
    fn default() -> Self {
        UnderwaterAcousticPdu {
            pdu_header: PduHeader::default(),
            emitting_entity_id: EntityId::default(),
            event_id: EventId::default(1),
            state_change_update_indicator: UAStateChangeUpdateIndicator::default(),
            padding: 0u8,
            passive_parameter_index: UAPassiveParameterIndex::default(),
            propulsion_plant_configuration: 0u8,
            number_of_shafts: 0u8,
            number_of_apas: 0u8,
            number_of_ua_emitter_systems: 0u8,
            shaft_rpms: vec![],
            apa_data: vec![],
            emitter_systems: vec![],
        }
    }
}

impl Pdu for UnderwaterAcousticPdu {
    type Header = PduHeader;

    fn calculate_length(&self) -> Result<u16, DISError> {
        let length = PduHeader::LENGTH
            + EntityId::LENGTH
            + EventId::LENGTH
            + std::mem::size_of::<u8>() * 6
            + std::mem::size_of::<u16>();

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

    /// Serialize contents of `UnderwaterAcousticPdu` into `BytesMut` buf
    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let length = self.calculate_length()?;
        self.pdu_header.set_length(length);
        self.pdu_header.serialize(buf);
        self.emitting_entity_id.serialize(buf);
        self.event_id.serialize(buf);
        buf.put_u8(self.state_change_update_indicator as u8);
        buf.put_u8(self.padding);
        buf.put_u16(self.passive_parameter_index as u16);
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
        Ok(())
    }

    /// Deserialize bytes from `BytesMut` buf and interpret as `UnderwaterAcousticPdu`
    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::UnderwaterAcoustic {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type UnderwaterAcoustic, got {:?}",
                    header.pdu_type
                ),
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

impl UnderwaterAcousticPdu {
    #[must_use]
    /// Creates a new `UnderwaterAcousticPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `UnderwaterAcousticPdu`:
    /// ```
    /// use open_dis_rust::distributed_emissions::UnderwaterAcousticPdu;
    /// let pdu = UnderwaterAcousticPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::UnderwaterAcoustic;
        pdu.pdu_header.protocol_family = ProtocolFamily::DistributedEmissionRegeneration;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let emitting_entity_id = EntityId::deserialize(buf);
        let event_id = EventId::deserialize(buf);
        let state_change_update_indicator = UAStateChangeUpdateIndicator::deserialize(buf);
        let padding = buf.get_u8();
        let passive_parameter_index = UAPassiveParameterIndex::deserialize(buf);
        let propulsion_plant_configuration = buf.get_u8();
        let number_of_shafts = buf.get_u8();
        let number_of_apas = buf.get_u8();
        let number_of_ua_emitter_systems = buf.get_u8();
        let mut shaft_rpms: Vec<ShaftRPMs> = vec![];
        for _i in 0..number_of_shafts {
            shaft_rpms.push(ShaftRPMs::deserialize(buf));
        }
        let mut apa_data: Vec<ApaData> = vec![];
        for _i in 0..number_of_apas {
            apa_data.push(ApaData::deserialize(buf));
        }
        let mut emitter_systems: Vec<AcousticEmitterSystem> = vec![];
        for _i in 0..number_of_ua_emitter_systems {
            emitter_systems.push(AcousticEmitterSystem::deserialize(buf));
        }
        UnderwaterAcousticPdu {
            pdu_header: PduHeader::default(),
            emitting_entity_id,
            event_id,
            state_change_update_indicator,
            padding,
            passive_parameter_index,
            propulsion_plant_configuration,
            number_of_shafts,
            number_of_apas,
            number_of_ua_emitter_systems,
            shaft_rpms,
            apa_data,
            emitter_systems,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UnderwaterAcousticPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = UnderwaterAcousticPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<UnderwaterAcousticPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = UnderwaterAcousticPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = UnderwaterAcousticPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
        let pdu = UnderwaterAcousticPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
