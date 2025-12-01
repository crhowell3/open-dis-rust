//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Minefield protocol family

use crate::{
    common::{
        GenericHeader, Pdu, PduHeader, SerializedLength,
        data_types::{
            ClockTime, EntityCoordinateVector, EntityId, EntityType, EulerAngles, WorldCoordinate,
            minefield_identifier::MinefieldIdentifier, point::Point,
        },
        enums::{
            ForceId, MinefieldSensorTypes, MinefieldStateProtocolMode, PduType, ProtocolFamily,
        },
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.9.2
    pub struct MinefieldStatePdu {
        header: PduHeader,
        pdu_type: PduType::MinefieldState,
        protocol_family: ProtocolFamily::Minefield,
        fields: {
            pub minefield_id: MinefieldIdentifier,
            pub minefield_sequence: u16,
            pub force_id: ForceId,
            pub number_of_perimeter_points: u8,
            pub minefield_type: EntityType,
            pub number_of_mine_types: u16,
            pub minefield_location: WorldCoordinate,
            pub minefield_orientation: EulerAngles,
            pub appearance: u16,
            pub protocol_mode: MinefieldStateProtocolMode,
            pub perimeter_points: Vec<Point>,
            pub mine_type: Vec<EntityType>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.9.3
    pub struct MinefieldQueryPdu {
        header: PduHeader,
        pdu_type: PduType::MinefieldQuery,
        protocol_family: ProtocolFamily::Minefield,
        fields: {
            pub minefield_id: MinefieldIdentifier,
            pub requesting_entity_id: EntityId,
            pub request_id: u8,
            pub number_of_perimeter_points: u8,
            padding: u8,
            pub number_of_sensor_types: u8,
            pub data_filter: u32,
            pub requested_mine_type: EntityType,
            pub requested_perimeter_points: Vec<Point>,
            pub sensor_types: Vec<MinefieldSensorTypes>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.9.4
    pub struct MinefieldDataPdu {
        header: PduHeader,
        pdu_type: PduType::MinefieldData,
        protocol_family: ProtocolFamily::Minefield,
        fields: {
            pub minefield_id: MinefieldIdentifier,
            pub requesting_entity_id: EntityId,
            pub minefield_sequence_number: u16,
            pub request_id: u8,
            pub pdu_sequence_number: u8,
            pub number_of_pdus: u8,
            pub number_of_mines_in_this_pdu: u8,
            pub number_of_sensor_types: u8,
            padding: u8,
            pub data_filter: u32,
            pub mine_type: EntityType,
            pub sensor_types: Vec<MinefieldSensorTypes>,
            pub mine_location: Vec<EntityCoordinateVector>,
            pub ground_burial_depth_offset: Vec<Option<f32>>,
            pub water_burial_depth_offset: Vec<Option<f32>>,
            pub snow_burial_depth_offset: Vec<Option<f32>>,
            pub mine_orientation: Vec<Option<EulerAngles>>,
            pub thermal_contrast: Vec<Option<f32>>,
            pub reflectance: Vec<Option<f32>>,
            pub mine_emplacement_time: Vec<Option<ClockTime>>,
            pub mine_entity_id: Vec<Option<u16>>,
            pub fusing: Vec<Option<u16>>,
            pub scalar_detection_coefficient: Vec<Option<u8>>,
            pub paint_scheme: Vec<Option<u8>>,
            pub number_of_trip_wires: Vec<Option<u8>>,
            pub number_of_vertices: Vec<Option<u8>>,
            pub vertices: Vec<Option<Vec<EntityCoordinateVector>>>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.9.5
    pub struct MinefieldResponseNackPdu {
        header: PduHeader,
        pdu_type: PduType::MinefieldResponseNack,
        protocol_family: ProtocolFamily::Minefield,
        fields: {
            pub minefield_id: EntityId,
            pub requesting_entity_id: EntityId,
            pub request_id: u8,
            pub number_of_missing_pdus: u8,
            pub missing_pdu_sequence_numbers: Vec<u64>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    mod minefield_state_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = MinefieldStatePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<MinefieldStatePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = MinefieldStatePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = MinefieldStatePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 576 / BITS_PER_BYTE;
            let pdu = MinefieldStatePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod minefield_query_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = MinefieldQueryPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<MinefieldQueryPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = MinefieldQueryPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = MinefieldQueryPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / BITS_PER_BYTE;
            let pdu = MinefieldQueryPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod minefield_data_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = MinefieldDataPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<MinefieldDataPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = MinefieldDataPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = MinefieldDataPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 352 / BITS_PER_BYTE;
            let pdu = MinefieldDataPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod minefield_response_nack_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = MinefieldResponseNackPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<MinefieldResponseNackPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = MinefieldResponseNackPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                MinefieldResponseNackPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 208 / BITS_PER_BYTE;
            let pdu = MinefieldResponseNackPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }
}
