//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use super::data_types::electromagnetic_emission_system_data::ElectromagneticEmissionSystemData;

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{EntityId, EventId},
        enums::{EEAttributeStateIndicator, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.6.2
    pub struct ElectromagneticEmissionsPdu {
        header: PduHeader,
        pdu_type: PduType::ElectromagneticEmission,
        protocol_family: ProtocolFamily::DistributedEmissionRegeneration,
        fields: {
            pub emitting_entity_id: EntityId,
            pub event_id: EventId,
            pub state_update_indicator: EEAttributeStateIndicator,
            pub number_of_systems: u8,
            padding: u16,
            pub systems: Vec<ElectromagneticEmissionSystemData>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ElectromagneticEmissionsPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = ElectromagneticEmissionsPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<ElectromagneticEmissionsPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = ElectromagneticEmissionsPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu =
            ElectromagneticEmissionsPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
        assert_eq!(new_pdu.header, pdu.header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
        let pdu = ElectromagneticEmissionsPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
