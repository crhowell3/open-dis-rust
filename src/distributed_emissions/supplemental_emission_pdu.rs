//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
};

use super::data_types::{
    propulsion_system_data::PropulsionSystemData,
    vectoring_nozzle_system_data::VectoringNozzleSystemData,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.6.6
pub struct SupplementalEmissionPdu {
    pdu_header: PduHeader,
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
            pdu_header: PduHeader::default(),
            originating_entity_id: EntityId::default(1),
            infrared_signature_representation_index: 0u16,
            acoustic_signature_representation_index: 0u16,
            radar_cross_section_signature_representation_index: 0u16,
            number_of_propulsion_systems: 0u16,
            number_of_vectoring_nozzle_systems: 0u16,
            propulsion_system_data: vec![],
            vectoring_nozzle_system_data: vec![],
        }
    }
}

impl Pdu for SupplementalEmissionPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>()
            + std::mem::size_of::<u16>() * 5;

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    /// Serialize contents of `SupplementalEmissionPdu` into `BytesMut` buf
    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let size = std::mem::size_of_val(self);
        self.pdu_header.length = u16::try_from(size).map_err(|_| DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })?;
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
        Ok(())
    }

    /// Deserialize bytes from `BytesMut` buf and interpret as `SupplementalEmissionPdu`
    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::SupplementalEmission {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type SupplementalEmission, got {:?}",
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

impl SupplementalEmissionPdu {
    /// Creates a new `SupplementalEmissionPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `SupplementalEmissionPdu`:
    /// ```
    /// use open_dis_rust::distributed_emissions::SupplementalEmissionPdu;
    /// let pdu = SupplementalEmissionPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::SupplementalEmission;
        pdu.pdu_header.protocol_family = ProtocolFamily::DistributedEmissionRegeneration;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let originating_entity_id = EntityId::deserialize(buf);
        let infrared_signature_representation_index = buf.get_u16();
        let acoustic_signature_representation_index = buf.get_u16();
        let radar_cross_section_signature_representation_index = buf.get_u16();
        let number_of_propulsion_systems = buf.get_u16();
        let number_of_vectoring_nozzle_systems = buf.get_u16();
        let mut propulsion_system_data: Vec<PropulsionSystemData> = vec![];
        for _i in 0..number_of_propulsion_systems {
            propulsion_system_data.push(PropulsionSystemData::deserialize(buf));
        }
        let mut vectoring_nozzle_system_data: Vec<VectoringNozzleSystemData> = vec![];
        for _i in 0..number_of_vectoring_nozzle_systems {
            vectoring_nozzle_system_data.push(VectoringNozzleSystemData::deserialize(buf));
        }
        SupplementalEmissionPdu {
            pdu_header: PduHeader::default(),
            originating_entity_id,
            infrared_signature_representation_index,
            acoustic_signature_representation_index,
            radar_cross_section_signature_representation_index,
            number_of_propulsion_systems,
            number_of_vectoring_nozzle_systems,
            propulsion_system_data,
            vectoring_nozzle_system_data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SupplementalEmissionPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let pdu = SupplementalEmissionPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<SupplementalEmissionPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = SupplementalEmissionPdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = SupplementalEmissionPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 224 / 8;
        let pdu = SupplementalEmissionPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
