//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{EntityCoordinateVector, entity_id::EntityId, event_id::EventId},
        enums::{PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

use super::data_types::{
    beam_data::BeamData, fundamental_operational_data::FundamentalOperationalData,
    iff_fundamental_parameter_data::IFFFundamentalParameterData, layer_header::LayerHeader,
    secondary_operational_data::SecondaryOperationalData, system_id::SystemId,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.6.5
    pub struct IFFPdu {
        header: PduHeader,
        pdu_type: PduType::IFF,
        protocol_family: ProtocolFamily::DistributedEmissionRegeneration,
        fields: {
            pub emitting_entity_id: EntityId,
            pub event_id: EventId,
            pub relative_antenna_location: EntityCoordinateVector,
            pub system_id: SystemId,
            pub system_designator: u8,
            pub system_specific_data: u8,
            pub fundamental_operational_data: FundamentalOperationalData,
            pub layer_header: LayerHeader,
            pub beam_data: BeamData,
            pub secondary_operational_data: SecondaryOperationalData,
            pub iff_parameters: Vec<IFFFundamentalParameterData>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IFFPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = IFFPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<IFFPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = IFFPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = IFFPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
        assert_eq!(new_pdu.header, pdu.header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 704 / BITS_PER_BYTE;
        let pdu = IFFPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
