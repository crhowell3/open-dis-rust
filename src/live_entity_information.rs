//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
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
            LEDeadReckoningParameters, LEEntityId, LEEulerAngles, LELinearVelocity,
            LiveEntityOrientation16, LiveEventIdentifier, OrientationError, PositionError,
            RelativeWorldCoordinates,
        },
        pdu::Pdu,
    },
    define_pdu, impl_record_fields, impl_u8_flag_fields,
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

    pub const fn set_data(&mut self, value: u32) {
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
    use crate::{common::pdu::Pdu, pdu_tests};
    use bytes::{Buf, Bytes};

    pdu_tests!(
        time_space_position_information_pdu_tests, TimeSpacePositionInformationPdu,
        pdu_type: PduType::TimeSpacePositionInformation,
        default_bits: 432,
        wrong_type: AppearancePdu,
    );
    pdu_tests!(
        appearance_pdu_tests, AppearancePdu,
        pdu_type: PduType::Appearance,
        default_bits: 136,
        wrong_type: TimeSpacePositionInformationPdu,
    );
    pdu_tests!(
        articulated_parts_pdu_tests, ArticulatedPartsPdu,
        pdu_type: PduType::ArticulatedParts,
        default_bits: 136,
        wrong_type: AppearancePdu,
    );
    pdu_tests!(
        live_entity_fire_pdu_tests, LiveEntityFirePdu,
        pdu_type: PduType::LiveEntityFire,
        default_bits: 496,
        wrong_type: LiveEntityDetonationPdu,
    );
    pdu_tests!(
        live_entity_detonation_pdu_tests, LiveEntityDetonationPdu,
        pdu_type: PduType::LiveEntityDetonation,
        default_bits: 592,
        wrong_type: LiveEntityFirePdu,
    );

    fn to_bytes<P: Pdu>(pdu: &mut P) -> Bytes {
        let mut buf = BytesMut::new();
        pdu.serialize(&mut buf).expect("serialize failed");
        buf.freeze()
    }

    mod live_entity_fire_pdu_layout {
        use super::*;

        fn populated() -> LiveEntityFirePdu {
            let mut pdu = LiveEntityFirePdu::new();
            pdu.flags = 0b0000_0111;
            pdu.event_id = LiveEventIdentifier {
                site_number: 0x01,
                application_number: 0x02,
                event_number: 0x0304,
            };
            pdu.range = 0xBEEF;
            pdu
        }

        #[test]
        fn fields_land_at_spec_offsets() {
            let bytes = to_bytes(&mut populated());

            assert_eq!(bytes.len(), 62);
            assert_eq!(bytes[16], 0b0000_0111, "flags");
            assert_eq!(bytes[17], 0, "padding");
            assert_eq!(&bytes[26..30], &[0x01, 0x02, 0x03, 0x04], "event id");
            assert_eq!(&bytes[60..62], &[0xBE, 0xEF], "range");
        }

        #[test]
        fn populated_round_trip() {
            let mut pdu = populated();
            let original = to_bytes(&mut pdu);

            let mut buf = original.clone();
            let mut decoded = LiveEntityFirePdu::deserialize(&mut buf).expect("deserialize failed");

            assert_eq!(buf.remaining(), 0);
            assert_eq!(decoded.flags, pdu.flags);
            assert_eq!(decoded.event_id, pdu.event_id);
            assert_eq!(decoded.range, pdu.range);
            assert_eq!(to_bytes(&mut decoded), original);
        }
    }

    mod live_entity_detonation_pdu_layout {
        use super::*;

        fn populated() -> LiveEntityDetonationPdu {
            let mut pdu = LiveEntityDetonationPdu::new();
            pdu.detonation_flag_1 = 0b0001_0001;
            pdu.detonation_flag_2 = 0b1000_0000;
            pdu.event_id = LiveEventIdentifier {
                site_number: 0x0A,
                application_number: 0x0B,
                event_number: 0x0C0D,
            };
            pdu.munition_orientation.theta = 0x1234;
            pdu.location_in_entity_coordinates.z = -2;
            pdu.detonation_result = 5;
            pdu
        }

        #[test]
        fn fields_land_at_spec_offsets() {
            let bytes = to_bytes(&mut populated());

            assert_eq!(bytes.len(), 74);
            assert_eq!(bytes[16], 0b0001_0001, "detonation flag 1");
            assert_eq!(bytes[17], 0b1000_0000, "detonation flag 2");
            assert_eq!(&bytes[26..30], &[0x0A, 0x0B, 0x0C, 0x0D], "event id");
            assert_eq!(&bytes[46..48], &[0x12, 0x34], "munition orientation theta");
            assert_eq!(&bytes[70..72], &(-2i16).to_be_bytes(), "entity location z");
            assert_eq!(bytes[72], 5, "detonation result");
            assert_eq!(bytes[73], 0, "padding");
        }

        #[test]
        fn populated_round_trip() {
            let mut pdu = populated();
            let original = to_bytes(&mut pdu);

            let mut buf = original.clone();
            let mut decoded =
                LiveEntityDetonationPdu::deserialize(&mut buf).expect("deserialize failed");

            assert_eq!(buf.remaining(), 0);
            assert_eq!(decoded.detonation_flag_1, pdu.detonation_flag_1);
            assert_eq!(decoded.detonation_flag_2, pdu.detonation_flag_2);
            assert_eq!(decoded.event_id, pdu.event_id);
            assert_eq!(decoded.munition_orientation, pdu.munition_orientation);
            assert_eq!(
                decoded.location_in_entity_coordinates,
                pdu.location_in_entity_coordinates
            );
            assert_eq!(decoded.detonation_result, pdu.detonation_result);
            assert_eq!(to_bytes(&mut decoded), original);
        }
    }

    mod variable_length_round_trips {
        use super::*;

        fn assert_round_trip<P: Pdu>(pdu: &mut P) {
            let original = to_bytes(pdu);
            let mut buf = original.clone();
            let mut decoded = P::deserialize(&mut buf).expect("deserialize failed");

            assert_eq!(buf.remaining(), 0, "deserialize left bytes unread");
            assert_eq!(to_bytes(&mut decoded), original);
        }

        #[test]
        #[ignore = "Option fields always deserialize as None; needs flag-driven deserialization"]
        fn appearance_with_optional_fields() {
            let mut pdu = AppearancePdu::new();
            pdu.appearance_flags1 =
                AppearanceFlags1::VISUAL_INCLUDED | AppearanceFlags1::FLAG2_INCLUDED;
            pdu.appearance_flags2 = Some(AppearanceFlags2::EM_INCLUDED);
            pdu.appearance_visual = Some(EntityAppearance::new(0xDEAD_BEEF));
            pdu.appearance_em = Some(EntityAppearance::new(0x0102_0304));
            assert_round_trip(&mut pdu);
        }

        #[test]
        #[ignore = "Vec fields always deserialize as empty; needs count-driven deserialization"]
        fn articulated_parts_with_records() {
            let mut pdu = ArticulatedPartsPdu::new();
            pdu.number_of_variable_parameter_records = 2;
            pdu.variable_parameter_records = vec![VariableParameter::default(); 2];
            assert_round_trip(&mut pdu);
        }

        #[test]
        #[ignore = "Vec fields always deserialize as empty; needs length-driven deserialization"]
        fn tspi_with_system_specific_data() {
            let mut pdu = TimeSpacePositionInformationPdu::new();
            pdu.system_specific_data_length = 3;
            pdu.system_specific_data = vec![0xAA, 0xBB, 0xCC];
            assert_round_trip(&mut pdu);
        }
    }
}
