use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

use super::{
    propulsion_system_data::PropulsionSystemData,
    vectoring_nozzle_system_data::VectoringNozzleSystemData,
};

#[derive(Clone, Debug)]
pub struct SupplementalEmissionPdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub infrared_signature_representation_index: u16,
    pub acoustic_signature_representation_index: u16,
    pub radar_cross_section_signature_representation_index: u16,
    pub number_of_propulsion_systems: u16,
    pub number_of_vectoring_nozzle_systems: u16,
    pub propulsion_system_data: Vec<PropulsionSystemData>,
    pub vectoring_nozzle_system_data: Vec<VectoringNozzleSystemData>,
}

impl Default for SupplementalEmissionPdu {
    fn default() -> Self {
        SupplementalEmissionPdu {
            pdu_header: PduHeader::default(
                PduType::SupplementalEmission,
                ProtocolFamily::DistributedEmissionRegeneration,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            infrared_signature_representation_index: 0,
            acoustic_signature_representation_index: 0,
            radar_cross_section_signature_representation_index: 0,
            number_of_propulsion_systems: 0,
            number_of_vectoring_nozzle_systems: 0,
            propulsion_system_data: vec![],
            vectoring_nozzle_system_data: vec![],
        }
    }
}

impl Pdu for SupplementalEmissionPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        buf.put_u16(self.infrared_signature_representation_index);
        buf.put_u16(self.acoustic_signature_representation_index);
        buf.put_u16(self.radar_cross_section_signature_representation_index);
        buf.put_u16(self.number_of_propulsion_systems);
        buf.put_u16(self.number_of_vectoring_nozzle_systems);
        for i in 0..self.propulsion_system_data.len() {
            self.propulsion_system_data[i].serialize(buf);
        }
        for i in 0..self.vectoring_nozzle_system_data.len() {
            self.vectoring_nozzle_system_data[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::SupplementalEmission {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let infrared_signature_representation_index = buffer.get_u16();
            let acoustic_signature_representation_index = buffer.get_u16();
            let radar_cross_section_signature_representation_index = buffer.get_u16();
            let number_of_propulsion_systems = buffer.get_u16();
            let number_of_vectoring_nozzle_systems = buffer.get_u16();
            let mut propulsion_system_data: Vec<PropulsionSystemData> = vec![];
            for _i in 0..number_of_propulsion_systems {
                propulsion_system_data.push(PropulsionSystemData::decode(&mut buffer));
            }
            let mut vectoring_nozzle_system_data: Vec<VectoringNozzleSystemData> = vec![];
            for _i in 0..number_of_vectoring_nozzle_systems {
                vectoring_nozzle_system_data.push(VectoringNozzleSystemData::decode(&mut buffer));
            }
            Ok(SupplementalEmissionPdu {
                pdu_header,
                originating_entity_id,
                infrared_signature_representation_index,
                acoustic_signature_representation_index,
                radar_cross_section_signature_representation_index,
                number_of_propulsion_systems,
                number_of_vectoring_nozzle_systems,
                propulsion_system_data,
                vectoring_nozzle_system_data,
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
        let originating_entity_id = EntityId::decode(&mut buffer);
        let infrared_signature_representation_index = buffer.get_u16();
        let acoustic_signature_representation_index = buffer.get_u16();
        let radar_cross_section_signature_representation_index = buffer.get_u16();
        let number_of_propulsion_systems = buffer.get_u16();
        let number_of_vectoring_nozzle_systems = buffer.get_u16();
        let mut propulsion_system_data: Vec<PropulsionSystemData> = vec![];
        for _i in 0..number_of_propulsion_systems {
            propulsion_system_data.push(PropulsionSystemData::decode(&mut buffer));
        }
        let mut vectoring_nozzle_system_data: Vec<VectoringNozzleSystemData> = vec![];
        for _i in 0..number_of_vectoring_nozzle_systems {
            vectoring_nozzle_system_data.push(VectoringNozzleSystemData::decode(&mut buffer));
        }
        Ok(SupplementalEmissionPdu {
            pdu_header,
            originating_entity_id,
            infrared_signature_representation_index,
            acoustic_signature_representation_index,
            radar_cross_section_signature_representation_index,
            number_of_propulsion_systems,
            number_of_vectoring_nozzle_systems,
            propulsion_system_data,
            vectoring_nozzle_system_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SupplementalEmissionPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let supplemental_emission_pdu = SupplementalEmissionPdu::default();
        let pdu_header = PduHeader::default(
            PduType::SupplementalEmission,
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
    fn deserialize_header() {
        let mut supplemental_emission_pdu = SupplementalEmissionPdu::default();
        let mut buffer = BytesMut::new();
        supplemental_emission_pdu.serialize(&mut buffer);

        let new_supplemental_emission_pdu = SupplementalEmissionPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_supplemental_emission_pdu.pdu_header,
            supplemental_emission_pdu.pdu_header
        );
    }
}
