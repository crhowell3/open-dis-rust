//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Synthetic Environment protocol family

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            ClockTime, EntityId, EntityType, EulerAngles, WorldCoordinate,
            environment::Environment, grid_axis_descriptor::GridAxisDescriptor,
            grid_data_record::GridDataRecord, linear_segment_parameter::LinearSegmentParameter,
            object_identifier::ObjectIdentifier, object_type::ObjectType,
            simulation_address::SimulationAddress,
        },
        enums::{
            ForceId, GriddedDataConstantGrid, GriddedDataCoordinateSystem,
            ObjectStateAppearanceGeneral, PduType, ProtocolFamily,
        },
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.10.2
    pub struct EnvironmentalProcessPdu {
        header: PduHeader,
        pdu_type: PduType::EnvironmentalProcess,
        protocol_family: ProtocolFamily::SyntheticEnvironment,
        fields: {
            pub environmental_process_id: EntityId,
            pub environment_type: EntityType,
            pub model_type: u8,
            pub environment_status: u8,
            pub number_of_environment_records: u16,
            pub sequence_number: u16,
            pub environment_records: Vec<Environment>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.10.3
    pub struct GriddedDataPdu {
        header: PduHeader,
        pdu_type: PduType::GriddedData,
        protocol_family: ProtocolFamily::SyntheticEnvironment,
        fields: {
            pub environmental_simulation_id: EntityId,
            pub field_number: u16,
            pub pdu_number: u16,
            pub pdu_total: u16,
            pub coordinate_system: GriddedDataCoordinateSystem,
            pub number_of_grid_axes: u8,
            pub constant_grid: GriddedDataConstantGrid,
            pub environment_type: EntityType,
            pub orientation: EulerAngles,
            pub sample_time: ClockTime,
            pub total_values: u32,
            pub vector_dimension: u8,
            padding: u8,
            padding2: u16,
            pub grid_axis_descriptors: Vec<GridAxisDescriptor>,
            pub grid_data_list: Vec<GridDataRecord>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.10.4
    pub struct PointObjectStatePdu {
        header: PduHeader,
        pdu_type: PduType::PointObjectState,
        protocol_family: ProtocolFamily::SyntheticEnvironment,
        fields: {
            pub object_id: EntityId,
            pub referenced_object_id: EntityId,
            pub update_number: u16,
            pub force_id: ForceId,
            pub modifications: u8,
            pub object_type: ObjectType,
            pub object_location: WorldCoordinate,
            pub object_orientation: EulerAngles,
            pub specific_object_appearance: u32,
            pub general_object_appearance: u16,
            padding: u16,
            pub requester_id: SimulationAddress,
            pub receiving_id: SimulationAddress,
            padding2: u32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.10.5
    pub struct LinearObjectStatePdu {
        header: PduHeader,
        pdu_type: PduType::LinearObjectState,
        protocol_family: ProtocolFamily::SyntheticEnvironment,
        fields: {
            pub object_id: ObjectIdentifier,
            pub referenced_object_id: ObjectIdentifier,
            pub update_number: u16,
            pub force_id: ForceId,
            pub number_of_segments: u8,
            pub requester_id: SimulationAddress,
            pub receiving_id: SimulationAddress,
            pub object_type: ObjectType,
            pub linear_segment_parameters: Vec<LinearSegmentParameter>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.10.6
    pub struct ArealObjectStatePdu {
        header: PduHeader,
        pdu_type: PduType::ArealObjectState,
        protocol_family: ProtocolFamily::SyntheticEnvironment,
        fields: {
            pub object_id: ObjectIdentifier,
            pub referenced_object_id: ObjectIdentifier,
            pub update_number: u16,
            pub force_id: ForceId,
            pub modifications: u8, // TODO(@anyone) Replace with Modifications UID 242
            pub object_type: ObjectType,
            pub specific_object_appearance: u32, // TODO(@anyone) Implement Specific Object Appearance
            pub general_object_appearance: ObjectStateAppearanceGeneral,
            pub number_of_points: u16,
            pub requester_id: SimulationAddress,
            pub receiving_id: SimulationAddress,
            pub object_location: Vec<WorldCoordinate>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    mod environmental_process_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = EnvironmentalProcessPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<EnvironmentalProcessPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = EnvironmentalProcessPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                EnvironmentalProcessPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 256 / BITS_PER_BYTE;
            let pdu = EnvironmentalProcessPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod gridded_data_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = GriddedDataPdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<GriddedDataPdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = GriddedDataPdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu = GriddedDataPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 512 / BITS_PER_BYTE;
            let pdu = GriddedDataPdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod point_object_state_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = PointObjectStatePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<PointObjectStatePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = PointObjectStatePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                PointObjectStatePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 704 / BITS_PER_BYTE;
            let pdu = PointObjectStatePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod linear_object_state_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = LinearObjectStatePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<LinearObjectStatePdu>());
        }
        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = LinearObjectStatePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                LinearObjectStatePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 320 / 8;
            let pdu = LinearObjectStatePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }

    mod areal_object_state_pdu_tests {
        use super::*;

        #[test]
        fn cast_to_any() {
            let pdu = ArealObjectStatePdu::new();
            let any_pdu = pdu.as_any();

            assert!(any_pdu.is::<ArealObjectStatePdu>());
        }

        #[test]
        fn serialize_then_deserialize() {
            let mut pdu = ArealObjectStatePdu::new();
            let mut serialize_buf = BytesMut::new();
            let _ = pdu.serialize(&mut serialize_buf);

            let mut deserialize_buf = serialize_buf.freeze();
            let new_pdu =
                ArealObjectStatePdu::deserialize(&mut deserialize_buf).unwrap_or_default();
            assert_eq!(new_pdu.header, pdu.header);
        }

        #[test]
        fn check_default_pdu_length() {
            const DEFAULT_LENGTH: u16 = 384 / BITS_PER_BYTE;
            let pdu = ArealObjectStatePdu::new();
            assert_eq!(pdu.header().length, DEFAULT_LENGTH);
        }
    }
}
