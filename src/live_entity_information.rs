//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

//! The Live Entity (LE) Information/Interaction protocol family

use crate::{
    common::{
        GenericHeader, LiveEntityPduHeader, SerializedLength,
        data_types::{
            EulerAngles, LinearVelocity, dead_reckoning_parameters::DeadReckoningParameters,
            entity_id::EntityId,
        },
        enums::{PduType, ProtocolFamily, RepairGroups},
        pdu::Pdu,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §9.4.2.5
    pub struct TimeSpacePositionInformationPdu {
        header: LiveEntityPduHeader,
        pdu_type: PduType::TimeSpacePositionInformation,
        protocol_family: ProtocolFamily::LiveEntityInformationInteraction,
        fields: {
            pub live_entity_id: EntityId,
            pub tpsi_flag: u8,
            pub entity_location: WorldCoordinate, // TODO(@crhowell3): Replace with RelativeWorldCoordinates
            pub entity_linear_velocity: LinearVelocity,
            pub entity_orientation: EulerAngles,
            pub position_error: PositionError,
            pub orientation_error: OrientationError,
            pub dead_reckoning_parameters: DeadReckoningParameters,  // TODO(@crhowell3): Change to LE DeadReckoningParameters type
            pub measured_speed: u16, // TODO(@crhowell3): Replace with 16-bit fixed binary(?)
            pub system_specific_data_length: u8,
            pub system_specific_data: Vec<u8>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §9.4.3.5
    pub struct AppearancePdu {
        header: LiveEntityPduHeader,
        pdu_type: PduType::Appearance,
        protocol_family: ProtocolFamily::LiveEntityInformationInteraction,
        fields: {
            pub receiving_entity_id: EntityId,
            pub repairing_entity_id: EntityId,
            pub repair: RepairGroups,
            padding: u16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    mod time_space_position_information_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = TimeSpacePositionInformationPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<TimeSpacePositionInformationPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = TimeSpacePositionInformationPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = TimeSpacePositionInformationPdu::deserialize(&mut deserialize_buf)
                .unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 224 / BITS_PER_BYTE;
            let pdu = TimeSpacePositionInformationPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }
}
