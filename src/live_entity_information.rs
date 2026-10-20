//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

//! The Live Entity (LE) Information/Interaction protocol family

use crate::{
    common::{
        GenericHeader, LiveEntityPduHeader, SerializedLength,
        data_types::{
            EntityType, VariableParameter, entity_marking::EntityMarking,
            fixed_binary_16::FixedBinary16, munition_descriptor::MunitionDescriptor,
        },
        enums::{EntityCapabilities, ForceId, PduType, ProtocolFamily},
        live_entity_records::{
            le_dead_reckoning_parameters::LEDeadReckoningParameters, le_entity_id::LEEntityId,
            le_euler_angles::LEEulerAngles, le_linear_velocity::LELinearVelocity,
            orientation_error::OrientationError, position_error::PositionError,
            relative_world_coordinates::RelativeWorldCoordinates,
        },
        pdu::Pdu,
    },
    define_pdu, impl_u8_flag_fields,
    pdu_macro::{FieldDeserialize, FieldLen, FieldSerialize},
};

use bitflags::bitflags;
use bytes::{Buf, BufMut, BytesMut};

bitflags! {
    /// Appearance Flags 1 (IEEE 1278.1-2012 §9.4.3.5)
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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
impl_u8_flag_fields!(AppearanceFlags1);

bitflags! {
    /// Appearance Flags 2 (IEEE 1278.1-2012 §9.4.3.5)
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct AppearanceFlags2: u8 {
        const EM_INCLUDED     = 0b0000_0010;
        const AUDIO_INCLUDED  = 0b0000_0100;
    }
}
impl_u8_flag_fields!(AppearanceFlags2);

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct PlatformAppearance: u32 {
        const PaintScheme = 0b0000_0000_0000_0001;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EntityAppearance {
    data: u32,
}

#[allow(dead_code)]
impl EntityAppearance {
    #[must_use]
    pub const fn new(data: u32) -> Self {
        Self { data }
    }

    #[must_use]
    pub const fn data(&self) -> u32 {
        self.data
    }

    pub fn set_data(&mut self, value: u32) {
        self.data = value;
    }

    const fn get_bits(self, shift: u8, mask: u32) -> u32 {
        (self.data >> shift) & mask
    }

    const fn set_bits(&mut self, shift: u8, mask: u32, value: u32) {
        self.data &= !(mask << shift);
        self.data |= (value & mask) << shift;
    }
}

impl FieldSerialize for EntityAppearance {
    fn serialize_field(&self, buf: &mut BytesMut) {
        buf.put_u32(self.data);
    }
}

impl FieldDeserialize for EntityAppearance {
    fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
        Self {
            data: buf.get_u32(),
        }
    }
}

impl FieldLen for EntityAppearance {
    fn field_len(&self) -> usize {
        4
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
            pub live_entity_id: LEEntityId,
            pub tpsi_flag: u8,
            pub entity_location: RelativeWorldCoordinates,
            pub entity_linear_velocity: LELinearVelocity,
            pub entity_orientation: LEEulerAngles,
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
            pub live_entity_id: LEEntityId,
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

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §9.4.4.5
    pub struct ArticulatedPartsPdu {
        header: LiveEntityPduHeader,
        pdu_type: PduType::ArticulatedParts,
        protocol_family: ProtocolFamily::LiveEntityInformationInteraction,
        fields: {
            pub live_entity_id: LEEntityId,
            pub number_of_variable_parameter_records: u8,
            pub variable_parameter_records: Vec<VariableParameter>,
        }
    }
}

macro_rules! impl_record_fields {
    ($name:ident { $($f:ident : $t:ty),* $(,)? }) => {
        impl FieldSerialize for $name {
            fn serialize_field(&self, buf: &mut BytesMut) {
                $( <$t as FieldSerialize>::serialize_field(&self.$f, buf); )*
            }
        }

        impl FieldDeserialize for $name {
            fn deserialize_field<B: Buf>(buf: &mut B) -> Self {
                // Struct-expression fields are evaluated in source order,
                // so this reads the buffer in wire order.
                Self {
                    $( $f: <$t as FieldDeserialize>::deserialize_field(buf), )*
                }
            }
        }

        impl FieldLen for $name {
            fn field_len(&self) -> usize {
                0 $( + <$t as FieldLen>::field_len(&self.$f) )*
            }
        }
    };
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct LiveEventIdentifier {
    pub site_number: u8,
    pub application_number: u8,
    pub event_number: u16,
}
impl_record_fields!(LiveEventIdentifier {
    site_number: u8,
    application_number: u8,
    event_number: u16,
});

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §9.4.5.6
    pub struct LiveEntityFirePdu {
        header: LiveEntityPduHeader,
        pdu_type: PduType::LiveEntityFire,
        protocol_family: ProtocolFamily::LiveEntityInformationInteraction,
        fields: {
            pub firing_live_entity_id: LEEntityId,
            pub flags: u8,
            pub padding: u8,
            pub target_live_entity_id: LEEntityId,
            pub munition_live_entity_id: LEEntityId,
            pub event_id: LiveEventIdentifier,
            pub location: RelativeWorldCoordinates,
            pub munition_descriptor: MunitionDescriptor,
            pub velocity: LELinearVelocity,
            pub range: u16,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct LiveEntityRelativeLocation {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
impl_record_fields!(LiveEntityRelativeLocation {
    x: i16,
    y: i16,
    z: i16,
});

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct LiveMunitionDescriptor {
    pub munition_type: EntityType,
    pub warhead: u16,
    pub fuse: u16,
    pub quantity: u16,
    pub rate: u16,
}
impl_record_fields!(LiveMunitionDescriptor {
    munition_type: EntityType,
    warhead: u16,
    fuse: u16,
    quantity: u16,
    rate: u16,
});

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct LiveEntityOrientation16 {
    pub psi: i16,
    pub theta: i16,
    pub phi: i16,
}
impl_record_fields!(LiveEntityOrientation16 {
    psi: i16,
    theta: i16,
    phi: i16,
});

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §9.4.6.8
    pub struct LiveEntityDetonationPdu {
        header: LiveEntityPduHeader,
        pdu_type: PduType::LiveEntityDetonation,
        protocol_family: ProtocolFamily::LiveEntityInformationInteraction,
        fields: {
            pub firing_live_entity_id: LEEntityId,
            pub detonation_flag_1: u8,
            pub detonation_flag_2: u8,
            pub target_live_entity_id: LEEntityId,
            pub munition_live_entity_id: LEEntityId,
            pub event_id: LiveEventIdentifier,
            pub location_in_world_coordinates: RelativeWorldCoordinates,
            pub velocity: LELinearVelocity,
            pub munition_orientation: LiveEntityOrientation16,
            pub munition_descriptor: LiveMunitionDescriptor,
            pub location_in_entity_coordinates: LiveEntityRelativeLocation,
            pub detonation_result: u8,
            pub padding: u8,
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
            const DEFAULT_LENGTH: u16 = 432 / BITS_PER_BYTE;
            let pdu = TimeSpacePositionInformationPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod appearance_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = AppearancePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<AppearancePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = AppearancePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = AppearancePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 136 / BITS_PER_BYTE;
            let pdu = AppearancePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod articulated_parts_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = ArticulatedPartsPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<ArticulatedPartsPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = ArticulatedPartsPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                ArticulatedPartsPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 136 / BITS_PER_BYTE;
            let pdu = ArticulatedPartsPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod live_entity_fire_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = LiveEntityFirePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<LiveEntityFirePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = LiveEntityFirePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = LiveEntityFirePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 496 / BITS_PER_BYTE;
            let pdu = LiveEntityFirePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod live_entity_detonation_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = LiveEntityDetonationPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<LiveEntityDetonationPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = LiveEntityDetonationPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                LiveEntityDetonationPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 592 / BITS_PER_BYTE;
            let pdu = LiveEntityDetonationPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }
}
