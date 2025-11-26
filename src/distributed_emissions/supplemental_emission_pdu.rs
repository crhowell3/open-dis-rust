//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::{
    common::{
        GenericHeader,
        data_types::entity_id::EntityId,
        enums::{PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
        serialized_length::SerializedLength,
    },
    define_pdu,
};

use super::data_types::{
    propulsion_system_data::PropulsionSystemData,
    vectoring_nozzle_system_data::VectoringNozzleSystemData,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.6.6
    pub struct SupplementalEmissionPdu {
        header: PduHeader,
        pdu_type: PduType::SupplementalEmission,
        protocol_family: ProtocolFamily::DistributedEmissionRegeneration,
        fields: {
            pub originating_entity_id: EntityId,
            pub infrared_signature_representation_index: u16,
            pub acoustic_signature_representation_index: u16,
            pub radar_cross_section_signature_representation_index: u16,
            pub number_of_propulsion_systems: u16,
            pub number_of_vectoring_nozzle_systems: u16,
            pub propulsion_system_data: Vec<PropulsionSystemData>,
            pub vectoring_nozzle_system_data: Vec<VectoringNozzleSystemData>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SupplementalEmissionPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = SupplementalEmissionPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<SupplementalEmissionPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = SupplementalEmissionPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu =
            SupplementalEmissionPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
        assert_eq!(new_pdu.header, pdu.header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
        let pdu = SupplementalEmissionPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
