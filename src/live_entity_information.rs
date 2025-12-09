//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

//! The Live Entity (LE) Information/Interaction protocol family

use crate::{
    common::{
        GenericHeader, LiveEntityPduHeader, SerializedLength,
        data_types::{
            EntityType, EulerAngles, LinearVelocity, entity_id::EntityId,
            entity_marking::EntityMarking, fixed_binary_16::FixedBinary16,
        },
        enums::{ForceId, PduType, ProtocolFamily},
        live_entity_records::{
            le_dead_reckoning_parameters::LEDeadReckoningParameters,
            orientation_error::OrientationError, position_error::PositionError,
            relative_world_coordinates::RelativeWorldCoordinates,
        },
        pdu::Pdu,
    },
    define_pdu,
};

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct AppearanceFlags1: u8 {
        const FORCE_ID_INCLUDED          = 0b0000_0001;
        const ENTITY_TYPE_INCLUDED       = 0b0000_0010;
        const ALT_ENTITY_TYPE_INCLUDED   = 0b0000_0100;
        const ENTITY_MARKING_INCLUDED    = 0b0000_1000;
        const CAPABILITIES_INCLUDED      = 0b0001_0000;
        const VISUAL_INCLUDED            = 0b0010_0000;
        const IR_INCLUDED                = 0b0100_0000;
        const FLAG2_INCLUDED             = 0b1000_0000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct AppearanceFlags2: u8 {
        const EM_INCLUDED     = 0b0000_0010;
        const AUDIO_INCLUDED  = 0b0000_0100;
    }
}

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
            pub entity_location: RelativeWorldCoordinates,
            pub entity_linear_velocity: LinearVelocity,
            pub entity_orientation: EulerAngles,  // TODO(@anyone): Convert to LEEulerAngles record
            pub position_error: PositionError,
            pub orientation_error: OrientationError,
            pub dead_reckoning_parameters: LEDeadReckoningParameters,
            pub measured_speed: FixedBinary16,
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
            pub live_entity_id: EntityId,
            pub appearance_flags1: AppearanceFlags1,
            pub appearance_flags2: Option<AppearanceFlags2>,
            pub force_id: Option<ForceId>,
            pub entity_type: Option<EntityType>,
            pub alternate_entity_type: Option<EntityType>,
            pub entity_marking: Option<EntityMarking>,
            pub capabilities: Option<EntityCapabilities>,  // UID 55
            pub appearance_visual: Option<EntityAppearance>,  // UID 31-43
            pub appearance_ir: Option<EntityAppearance>,  // UID 31-43
            pub appearance_em: Option<EntityAppearance>,  // UID 31-43
            pub appearance_audio: Option<EntityAppearance>,  // UID 31-43
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
