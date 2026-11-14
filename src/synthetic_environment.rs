//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! The Synthetic Environment protocol family

use crate::common::{
    enums::{
        ForceId, GriddedDataConstantGrid, GriddedDataCoordinateSystem,
        ObjectStateAppearanceGeneral, PduType, ProtocolFamily,
    },
    pdu_header::PduHeader,
    records::{
        ClockTime, EntityIdentifier, EntityType, Environment, EulerAngles, GridAxisDescriptor,
        GridData, LinearSegmentParameter, ObjectIdentifier, ObjectType, SimulationAddress,
        WorldCoordinates,
    },
};

/// Implemented according to IEEE 1278.1-2012 §7.10.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct EnvironmentalProcessPdu {
    #[deku(skip)]
    header: PduHeader,
    pub environmental_process_id: EntityIdentifier,
    pub environment_type: EntityType,
    pub model_type: u8,
    pub environment_status: u8,
    #[deku(
        update = "self.environment_records.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_environment_records)"
    )]
    pub number_of_environment_records: u16,
    pub sequence_number: u16,
    #[deku(count = "number_of_environment_records")]
    pub environment_records: Vec<Environment>,
}
crate::common::pdu::impl_pdu!(
    EnvironmentalProcessPdu,
    PduHeader,
    PduType::EnvironmentalProcess,
    ProtocolFamily::SyntheticEnvironment
);

/// Implemented according to IEEE 1278.1-2012 §7.10.3
///
/// Grid Data records have no count: they fill the rest of the PDU.
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct GriddedDataPdu {
    #[deku(skip)]
    header: PduHeader,
    pub environmental_simulation_id: EntityIdentifier,
    pub field_number: u16,
    pub pdu_number: u16,
    pub pdu_total: u16,
    pub coordinate_system: GriddedDataCoordinateSystem,
    /// Set by `serialize` from `grid_axis_descriptors`.
    #[deku(update = "crate::common::wire::update_each(&mut self.grid_axis_descriptors)?")]
    pub number_of_grid_axes: u8,
    pub constant_grid: GriddedDataConstantGrid,
    pub environment_type: EntityType,
    pub orientation: EulerAngles,
    pub sample_time: ClockTime,
    pub total_values: u32,
    pub vector_dimension: u8,
    padding: u8,
    padding2: u16,
    #[deku(count = "number_of_grid_axes")]
    pub grid_axis_descriptors: Vec<GridAxisDescriptor>,
    #[deku(read_all, update = "updated_grid_data(&self.grid_data_list)?")]
    pub grid_data_list: Vec<GridData>,
}
crate::common::pdu::impl_pdu!(
    GriddedDataPdu,
    PduHeader,
    PduType::GriddedData,
    ProtocolFamily::SyntheticEnvironment
);

/// The grid data records with their own update rules applied.
fn updated_grid_data(records: &[GridData]) -> Result<Vec<GridData>, deku::DekuError> {
    let mut records = records.to_vec();
    crate::common::wire::update_each(&mut records)?;
    Ok(records)
}

/// Implemented according to IEEE 1278.1-2012 §7.10.4
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct PointObjectStatePdu {
    #[deku(skip)]
    header: PduHeader,
    pub object_id: EntityIdentifier,
    pub referenced_object_id: EntityIdentifier,
    pub update_number: u16,
    pub force_id: ForceId,
    pub modifications: u8,
    pub object_type: ObjectType,
    pub object_location: WorldCoordinates,
    pub object_orientation: EulerAngles,
    pub specific_object_appearance: u32,
    pub general_object_appearance: u16,
    padding: u16,
    pub requester_id: SimulationAddress,
    pub receiving_id: SimulationAddress,
    padding2: u32,
}
crate::common::pdu::impl_pdu!(
    PointObjectStatePdu,
    PduHeader,
    PduType::PointObjectState,
    ProtocolFamily::SyntheticEnvironment
);

/// Implemented according to IEEE 1278.1-2012 §7.10.5
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct LinearObjectStatePdu {
    #[deku(skip)]
    header: PduHeader,
    pub object_id: ObjectIdentifier,
    pub referenced_object_id: ObjectIdentifier,
    pub update_number: u16,
    pub force_id: ForceId,
    #[deku(update = "self.linear_segment_parameters.len()")]
    pub number_of_segments: u8,
    pub requester_id: SimulationAddress,
    pub receiving_id: SimulationAddress,
    pub object_type: ObjectType,
    #[deku(count = "number_of_segments")]
    pub linear_segment_parameters: Vec<LinearSegmentParameter>,
}
crate::common::pdu::impl_pdu!(
    LinearObjectStatePdu,
    PduHeader,
    PduType::LinearObjectState,
    ProtocolFamily::SyntheticEnvironment
);

/// Implemented according to IEEE 1278.1-2012 §7.10.6
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct ArealObjectStatePdu {
    #[deku(skip)]
    header: PduHeader,
    pub object_id: ObjectIdentifier,
    pub referenced_object_id: ObjectIdentifier,
    pub update_number: u16,
    pub force_id: ForceId,
    pub modifications: u8, // TODO(@anyone) Replace with Modifications UID 242,
    pub object_type: ObjectType,
    pub specific_object_appearance: u32, // TODO(@anyone) Implement Specific Object Appearance,
    pub general_object_appearance: ObjectStateAppearanceGeneral,
    #[deku(
        update = "self.object_location.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_points)"
    )]
    pub number_of_points: u16,
    pub requester_id: SimulationAddress,
    pub receiving_id: SimulationAddress,
    #[deku(count = "number_of_points")]
    pub object_location: Vec<WorldCoordinates>,
}
crate::common::pdu::impl_pdu!(
    ArealObjectStatePdu,
    PduHeader,
    PduType::ArealObjectState,
    ProtocolFamily::SyntheticEnvironment
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::pdu::pdu_tests,
        common::{constants::BITS_PER_BYTE, pdu::Pdu},
    };

    /// Serialize a PDU, failing the test if serialization returns an error.
    pub fn to_bytes<P: Pdu>(pdu: &mut P) -> Vec<u8> {
        pdu.to_bytes().expect("serialize failed")
    }

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
