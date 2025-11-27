//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            EntityId, EntityType, EulerAngles, LinearVelocity, VariableParameter, WorldCoordinate,
        },
        enums::{EntityCapabilities, ForceId, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

use super::data_types::{
    dead_reckoning_parameters::DeadReckoningParameters, entity_marking::EntityMarking,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.2.2
    pub struct EntityStatePdu {
        header: PduHeader,
        pdu_type: PduType::EntityState,
        protocol_family: ProtocolFamily::EntityInformation,
        fields: {
            pub entity_id: EntityId,
            pub force_id: ForceId,
            pub number_of_articulation_parameters: u8,
            pub entity_type: EntityType,
            pub alternative_entity_type: EntityType,
            pub entity_linear_velocity: LinearVelocity,
            pub entity_location: WorldCoordinate,
            pub entity_orientation: EulerAngles,
            pub entity_appearance: u32,
            pub dead_reckoning_parameters: DeadReckoningParameters,
            pub entity_marking: EntityMarking,
            pub entity_capabilities: EntityCapabilities,
            pub articulation_parameter: Vec<VariableParameter>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EntityStatePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = EntityStatePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<EntityStatePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = EntityStatePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = EntityStatePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
        assert_eq!(new_pdu.header, pdu.header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 1152 / BITS_PER_BYTE;
        let pdu = EntityStatePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
