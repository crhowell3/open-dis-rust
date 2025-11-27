//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{EntityId, EulerAngles, LinearVelocity, VariableParameter, WorldCoordinate},
        enums::{PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.2.5
    pub struct EntityStateUpdatePdu {
        header: PduHeader,
        pdu_type: PduType::EntityStateUpdate,
        protocol_family: ProtocolFamily::EntityInformation,
        fields: {
            pub entity_id: EntityId,
            padding: u8,
            pub number_of_variable_parameters: u8,
            pub entity_linear_velocity: LinearVelocity,
            pub entity_location: WorldCoordinate,
            pub entity_orientation: EulerAngles,
            pub entity_appearance: u32,
            pub variable_parameter_records: Vec<VariableParameter>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EntityStateUpdatePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = EntityStateUpdatePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<EntityStateUpdatePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = EntityStateUpdatePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = EntityStateUpdatePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
        assert_eq!(new_pdu.header, pdu.header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 576 / BITS_PER_BYTE;
        let pdu = EntityStateUpdatePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
