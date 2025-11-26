//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{entity_id::EntityId, event_id::EventId},
        enums::{PduType, ProtocolFamily, UAPassiveParameterIndex, UAStateChangeUpdateIndicator},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

use super::data_types::{
    acoustic_emitter_system::AcousticEmitterSystem, apa_data::ApaData, shaft_rpms::ShaftRPMs,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.6.4
    pub struct UnderwaterAcousticPdu {
        header: PduHeader,
        pdu_type: PduType::UnderwaterAcoustic,
        protocol_family: ProtocolFamily::DistributedEmissionRegeneration,
        fields: {
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
        let new_pdu = UnderwaterAcousticPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
        assert_eq!(new_pdu.header, pdu.header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
        let pdu = UnderwaterAcousticPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
