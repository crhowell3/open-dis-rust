//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Synthetic Environment protocol family

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            ClockTime, EntityId, EntityType, Environment, EulerAngles, GridAxisDescriptor,
            GridDataRecord, LinearSegmentParameter, ObjectIdentifier, ObjectType,
            SimulationAddress, WorldCoordinate,
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
    use crate::{common::pdu::Pdu, pdu_tests};

    pdu_tests!(
        environmental_process_pdu_tests, EnvironmentalProcessPdu,
        pdu_type: PduType::EnvironmentalProcess,
        default_bits: 256,
        wrong_type: GriddedDataPdu,
    );
    pdu_tests!(
        gridded_data_pdu_tests, GriddedDataPdu,
        pdu_type: PduType::GriddedData,
        default_bits: 512,
        wrong_type: PointObjectStatePdu,
    );
    pdu_tests!(
        point_object_state_pdu_tests, PointObjectStatePdu,
        pdu_type: PduType::PointObjectState,
        default_bits: 704,
        wrong_type: LinearObjectStatePdu,
    );
    pdu_tests!(
        linear_object_state_pdu_tests, LinearObjectStatePdu,
        pdu_type: PduType::LinearObjectState,
        default_bits: 320,
        wrong_type: ArealObjectStatePdu,
    );
    pdu_tests!(
        areal_object_state_pdu_tests, ArealObjectStatePdu,
        pdu_type: PduType::ArealObjectState,
        default_bits: 384,
        wrong_type: GriddedDataPdu,
    );
}
