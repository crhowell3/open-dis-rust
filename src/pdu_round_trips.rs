//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

//! Round trips for every PDU defined with `impl_pdu!`: each list is filled
//! with its own number of entries, so a count wired to the wrong list fails;
//! the hand-written layouts (padding, bit lengths, IFF layers) have their own
//! tests; and random bodies must never panic.

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    clippy::arithmetic_side_effects,
    clippy::too_many_lines,
    clippy::float_cmp
)]

use bytes::{Buf, Bytes, BytesMut};

use crate::common::{
    DISError,
    data_types::{
        ClockTime, EntityCoordinateVector, EntityId, EntityType, EulerAngles, VariableParameter,
        WorldCoordinate,
        acoustic_beam_data::AcousticBeamData,
        acoustic_emitter_system_data::AcousticEmitterSystemData,
        aggregate_id::AggregateId,
        antenna_pattern::AntennaPattern,
        apa_data::ApaData,
        attribute_record::AttributeRecord,
        attribute_record_set::AttributeRecordSet,
        datum_records::{FixedDatumRecord, VariableDatumRecord},
        directed_energy_damage::DirectedEnergyDamage,
        electromagnetic_emission_beam_data::ElectromagneticEmissionBeamData,
        electromagnetic_emission_system_data::ElectromagneticEmissionSystemData,
        environment::Environment,
        grid_axis_descriptor::{GridAxisDescriptor, GridAxisType},
        grid_data_record::{DataRepresentationType, GridDataRecord},
        iff_fundamental_parameter_data::IFFFundamentalParameterData,
        intercom_communications_parameters::IntercomCommunicationsParameters,
        linear_segment_parameter::LinearSegmentParameter,
        modulation_parameters::ModulationParameters,
        point::Point,
        propulsion_system_data::PropulsionSystemData,
        shaft_rpms::ShaftRPMs,
        supply_quantity::SupplyQuantity,
        track_jam_target::TrackJamTarget,
        variable_transmitter_parameters::VariableTransmitterParameters,
        vectoring_nozzle_system_data::VectoringNozzleSystemData,
    },
    enums::{GridAxisDescriptorAxisType, GriddedDataDataRepresentation, VariableRecordTypes},
    pdu::Pdu,
};
use crate::{
    distributed_emissions::*, entity_information::*, entity_management::*, logistics::*,
    minefield::*, radio_communications::*, simulation_management::*,
    simulation_management_with_reliability::*, synthetic_environment::*, warfare::*,
};

fn to_bytes<P: Pdu>(pdu: &mut P) -> Bytes {
    let mut buf = BytesMut::new();
    pdu.serialize(&mut buf).unwrap();
    buf.freeze()
}

/// Serialize, decode, re-serialize: the bytes must match and be fully read.
fn round_trip<P: Pdu>(pdu: &mut P) -> P {
    let original = to_bytes(pdu);
    assert_eq!(
        usize::from(crate::common::GenericHeader::length(pdu.header())),
        original.len(),
        "header length"
    );
    let mut buf = original.clone();
    let mut decoded = P::deserialize(&mut buf).unwrap_or_else(|e| panic!("decode failed: {e}"));
    assert_eq!(buf.remaining(), 0, "bytes left over");
    assert_eq!(to_bytes(&mut decoded), original, "re-encoded bytes differ");
    decoded
}

/// Fill a PDU's lists with `fill`, round-trip it, and check the decoded
/// lists with `check`. Each list gets a different length.
macro_rules! lists {
    ($test:ident, $pdu:ty, |$p:ident| $fill:block, |$d:ident| $check:block) => {
        #[test]
        fn $test() {
            let mut $p = <$pdu>::new();
            $fill
            let $d = round_trip(&mut $p);
            $check
        }
    };
}

// ---------------------------------------------------------------------------
// Distributed emissions
// ---------------------------------------------------------------------------

fn ee_system(beams: usize, targets: usize) -> ElectromagneticEmissionSystemData {
    ElectromagneticEmissionSystemData {
        beam_data_records: vec![
            ElectromagneticEmissionBeamData {
                track_jam_targets: vec![TrackJamTarget::default(); targets],
                ..ElectromagneticEmissionBeamData::default()
            };
            beams
        ],
        ..ElectromagneticEmissionSystemData::default()
    }
}

lists!(
    electromagnetic_emissions,
    ElectromagneticEmissionsPdu,
    |p| {
        p.systems = vec![ee_system(1, 2), ee_system(3, 0)];
    },
    |d| {
        assert_eq!(d.number_of_systems, 2);
        assert_eq!(d.systems[0].number_of_beams, 1);
        assert_eq!(
            d.systems[0].beam_data_records[0].number_of_track_jam_targets,
            2
        );
        assert_eq!(d.systems[1].beam_data_records.len(), 3);
    }
);

lists!(
    underwater_acoustic,
    UnderwaterAcousticPdu,
    |p| {
        p.shaft_rpms = vec![ShaftRPMs::default(); 1];
        p.apa_data = vec![ApaData::default(); 2];
        p.emitter_systems = vec![
            AcousticEmitterSystemData {
                beam_records: vec![AcousticBeamData::default(); 2],
                ..AcousticEmitterSystemData::default()
            };
            3
        ];
    },
    |d| {
        assert_eq!(
            (
                d.shaft_rpms.len(),
                d.apa_data.len(),
                d.emitter_systems.len()
            ),
            (1, 2, 3)
        );
        assert_eq!(d.emitter_systems[2].number_of_beams, 2);
    }
);

lists!(
    supplemental_emission,
    SupplementalEmissionPdu,
    |p| {
        p.propulsion_system_data = vec![PropulsionSystemData::default(); 2];
        p.vectoring_nozzle_system_data = vec![VectoringNozzleSystemData::default(); 3];
    },
    |d| {
        assert_eq!(d.propulsion_system_data.len(), 2);
        assert_eq!(d.vectoring_nozzle_system_data.len(), 3);
    }
);

// ---------------------------------------------------------------------------
// Entity information
// ---------------------------------------------------------------------------

lists!(
    entity_state,
    EntityStatePdu,
    |p| {
        p.articulation_parameter = vec![VariableParameter::default(); 3];
    },
    |d| {
        assert_eq!(d.number_of_articulation_parameters, 3);
    }
);

lists!(
    entity_state_update,
    EntityStateUpdatePdu,
    |p| {
        p.variable_parameter_records = vec![VariableParameter::default(); 2];
    },
    |d| {
        assert_eq!(d.variable_parameter_records.len(), 2);
    }
);

lists!(
    attribute,
    AttributePdu,
    |p| {
        p.attribute_record_sets = vec![
            AttributeRecordSet {
                attribute_records: vec![AttributeRecord {
                    record_specific_fields: vec![1, 2, 3],
                    ..AttributeRecord::default()
                }],
                ..AttributeRecordSet::default()
            };
            2
        ];
    },
    |d| {
        assert_eq!(d.number_of_attribute_record_sets, 2);
        assert_eq!(
            d.attribute_record_sets[1].attribute_records[0].record_length,
            16
        );
    }
);

// ---------------------------------------------------------------------------
// Entity management
// ---------------------------------------------------------------------------

lists!(
    aggregate_state,
    AggregateStatePdu,
    |p| {
        p.aggregate_id_list = vec![AggregateId::default(); 1];
        p.entity_id_list = vec![EntityId::default(); 2];
        p.silent_aggregate_system_list = vec![EntityType::default(); 3];
        p.silent_entity_system_list = vec![EntityType::default(); 4];
        p.variable_datum_list = vec![VariableDatumRecord::default(); 5];
    },
    |d| {
        assert_eq!(d.aggregate_id_list.len(), 1);
        assert_eq!(d.entity_id_list.len(), 2);
        assert_eq!(d.silent_aggregate_system_list.len(), 3);
        assert_eq!(d.silent_entity_system_list.len(), 4);
        assert_eq!(d.variable_datum_list.len(), 5);
    }
);

#[test]
fn aggregate_state_pads_id_lists_to_32_bits() {
    let base = to_bytes(&mut AggregateStatePdu::new()).len();
    for (aggregates, entities, padding) in [(0, 0, 0), (1, 0, 2), (0, 1, 2), (1, 1, 0), (2, 1, 2)] {
        let mut pdu = AggregateStatePdu::new();
        pdu.aggregate_id_list = vec![AggregateId::default(); aggregates];
        pdu.entity_id_list = vec![EntityId::default(); entities];
        let mut decoded = round_trip(&mut pdu);
        let bytes = to_bytes(&mut decoded);
        assert_eq!(
            bytes.len(),
            base + 6 * (aggregates + entities) + padding,
            "{aggregates}+{entities}"
        );
        assert_eq!(bytes.len() % 4, 0, "{aggregates}+{entities}");
    }
}

lists!(
    is_group_of,
    IsGroupOfPdu,
    |p| {
        p.grouped_entity_descriptions = vec![1, 2, 3];
    },
    |d| {
        assert_eq!(d.number_of_grouped_entities, 3);
        assert_eq!(d.grouped_entity_descriptions, [1, 2, 3]);
    }
);

// ---------------------------------------------------------------------------
// Logistics
// ---------------------------------------------------------------------------

lists!(
    service_request,
    ServiceRequestPdu,
    |p| {
        p.supplies = vec![SupplyQuantity::default(); 2];
    },
    |d| {
        assert_eq!(d.number_of_supply_types, 2);
    }
);

lists!(
    resupply_offer,
    ResupplyOfferPdu,
    |p| {
        p.supplies = vec![SupplyQuantity::default(); 3];
    },
    |d| {
        assert_eq!(d.number_of_supply_types, 3);
    }
);

lists!(
    resupply_received,
    ResupplyReceivedPdu,
    |p| {
        p.supplies = vec![SupplyQuantity::default(); 1];
    },
    |d| {
        assert_eq!(d.number_of_supply_types, 1);
    }
);

// ---------------------------------------------------------------------------
// Minefield
// ---------------------------------------------------------------------------

lists!(
    minefield_state,
    MinefieldStatePdu,
    |p| {
        p.perimeter_points = vec![Point::default(); 2];
        p.mine_type = vec![EntityType::default(); 3];
    },
    |d| {
        assert_eq!((d.perimeter_points.len(), d.mine_type.len()), (2, 3));
    }
);

lists!(
    minefield_query,
    MinefieldQueryPdu,
    |p| {
        p.requested_perimeter_points = vec![Point::default(); 3];
        p.sensor_types = vec![0x0101];
    },
    |d| {
        assert_eq!(
            (d.requested_perimeter_points.len(), d.sensor_types.len()),
            (3, 1)
        );
    }
);

lists!(
    minefield_response_nack,
    MinefieldResponseNackPdu,
    |p| {
        p.missing_pdu_sequence_numbers = vec![7, 8];
    },
    |d| {
        assert_eq!(d.number_of_missing_pdus, 2);
    }
);

mod minefield_data {
    use super::*;

    fn point(x: f32) -> EntityCoordinateVector {
        EntityCoordinateVector {
            x_coordinate: x,
            y_coordinate: 0.0,
            z_coordinate: 0.0,
        }
    }

    fn plain_mine(entity_number: u16) -> Mine {
        Mine {
            location: point(f32::from(entity_number)),
            entity_number,
            ..Mine::default()
        }
    }

    fn full_mine(entity_number: u16, wires: Vec<Vec<EntityCoordinateVector>>) -> Mine {
        Mine {
            ground_burial_depth_offset: Some(1.0),
            water_burial_depth_offset: Some(2.0),
            snow_burial_depth_offset: Some(3.0),
            orientation: Some(EulerAngles::default()),
            thermal_contrast: Some(4.0),
            reflectance: Some(5.0),
            emplacement_time: Some(ClockTime::default()),
            fusing: Some(0x0102),
            scalar_detection_coefficients: Some(vec![7, 8, 9]),
            paint_scheme: Some(3),
            trip_wires: Some(wires),
            ..plain_mine(entity_number)
        }
    }

    /// Fixed fields up to and including the mine type (Table 189).
    const FIXED: usize = 44;

    #[test]
    fn mines_without_optional_fields() {
        let mut pdu = MinefieldDataPdu::new();
        pdu.sensor_types = vec![0x0101];
        pdu.mines = vec![plain_mine(10), plain_mine(11), plain_mine(12)];
        let bytes = to_bytes(&mut pdu);
        // sensor type + P1, 3 locations, 3 entity numbers + P2
        assert_eq!(bytes.len(), FIXED + 4 + 36 + 8);
        assert_eq!(pdu.data_filter, MinefieldDataFilter::empty());
        let decoded = round_trip(&mut pdu);
        assert_eq!(decoded.number_of_mines_in_this_pdu, 3);
        assert_eq!(decoded.mines[2].entity_number, 12);
    }

    #[test]
    fn every_optional_field() {
        let mut pdu = MinefieldDataPdu::new();
        pdu.sensor_types = vec![1, 2, 3];
        pdu.mines = vec![
            full_mine(
                1,
                vec![vec![point(1.0)], vec![point(2.0), point(3.0), point(4.0)]],
            ),
            full_mine(2, vec![]),
        ];
        let bytes = to_bytes(&mut pdu);
        assert_eq!(pdu.data_filter.bits(), 0x7FF);
        // 3 sensor types + P1 (8), locations (24), 7 four-octet-aligned
        // fields: 3 depths, orientation, thermal, reflectance, time (80), entity
        // numbers, fusing, 6 coefficients, paint (16, P2 0),
        // wire counts + P3 (4), vertex counts + P4 (4), 4 vertices (48)
        assert_eq!(bytes.len(), FIXED + 8 + 24 + 80 + 16 + 4 + 4 + 48);
        let decoded = round_trip(&mut pdu);
        let wires = decoded.mines[0].trip_wires.as_ref().unwrap();
        assert_eq!(wires.iter().map(Vec::len).collect::<Vec<_>>(), [1, 3]);
        assert_eq!(wires[1][2].x_coordinate, 4.0);
        assert_eq!(decoded.mines[1].trip_wires.as_ref().map(Vec::len), Some(0));
        assert_eq!(
            decoded.mines[1].scalar_detection_coefficients.as_deref(),
            Some(&[7, 8, 9][..])
        );
    }

    #[test]
    fn each_field_is_sent_for_every_mine_before_the_next() {
        let mut pdu = MinefieldDataPdu::new();
        pdu.mines = vec![
            Mine {
                fusing: Some(0xAAAA),
                ..plain_mine(0x1111)
            },
            Mine {
                fusing: Some(0xBBBB),
                ..plain_mine(0x2222)
            },
        ];
        let bytes = to_bytes(&mut pdu);
        let after_locations = FIXED + 24;
        assert_eq!(
            &bytes[after_locations..after_locations + 8],
            &[0x11, 0x11, 0x22, 0x22, 0xAA, 0xAA, 0xBB, 0xBB],
            "both entity numbers, then both fusings"
        );
    }

    #[test]
    fn burial_depths_bring_ground_and_orientation() {
        // Encode ground, water and orientation, then clear the ground and
        // orientation bits: a decoder must still read them (G and R).
        let mut pdu = MinefieldDataPdu::new();
        pdu.mines = vec![Mine {
            ground_burial_depth_offset: Some(1.5),
            water_burial_depth_offset: Some(2.5),
            orientation: Some(EulerAngles::default()),
            ..plain_mine(1)
        }];
        let mut bytes = to_bytes(&mut pdu).to_vec();
        assert_eq!(&bytes[32..36], &[0, 0, 0, 0b1011]);
        bytes[35] = 0b0010;
        let decoded = MinefieldDataPdu::deserialize(&mut Bytes::from(bytes)).unwrap();
        assert_eq!(decoded.mines[0].ground_burial_depth_offset, Some(1.5));
        assert!(decoded.mines[0].orientation.is_some());
    }

    #[test]
    fn water_without_ground_and_orientation_is_an_error() {
        let mut pdu = MinefieldDataPdu::new();
        pdu.mines = vec![Mine {
            water_burial_depth_offset: Some(1.0),
            ..plain_mine(1)
        }];
        assert!(pdu.serialize(&mut BytesMut::new()).is_err());
    }

    #[test]
    fn a_field_on_some_mines_only_is_an_error() {
        let mut pdu = MinefieldDataPdu::new();
        pdu.mines = vec![
            Mine {
                reflectance: Some(1.0),
                ..plain_mine(1)
            },
            plain_mine(2),
        ];
        let err = pdu.serialize(&mut BytesMut::new()).unwrap_err();
        assert!(
            err.to_string()
                .contains("reflectance is set on 1 of 2 mines"),
            "{err}"
        );
    }

    #[test]
    fn coefficients_must_match_the_sensor_types() {
        let mut pdu = MinefieldDataPdu::new();
        pdu.sensor_types = vec![1, 2];
        pdu.mines = vec![Mine {
            scalar_detection_coefficients: Some(vec![1]),
            ..plain_mine(1)
        }];
        assert!(pdu.serialize(&mut BytesMut::new()).is_err());
    }

    #[test]
    fn padding_bits_of_the_filter_are_kept() {
        let mut pdu = MinefieldDataPdu::new();
        pdu.data_filter = MinefieldDataFilter::from_bits_retain(1 << 20);
        let decoded = round_trip(&mut pdu);
        assert_eq!(decoded.data_filter.bits(), 1 << 20);
    }

    #[test]
    fn query_sensor_types_are_16_bit_and_padded() {
        let mut pdu = MinefieldQueryPdu::new();
        let base = to_bytes(&mut pdu).len();
        pdu.sensor_types = vec![0x0102, 0x0304, 0x0506];
        let bytes = to_bytes(&mut pdu);
        assert_eq!(bytes.len(), base + 8, "3 sensor types, padded to 8 octets");
        assert_eq!(&bytes[base..base + 6], &[1, 2, 3, 4, 5, 6]);
        assert_eq!(round_trip(&mut pdu).sensor_types, [0x0102, 0x0304, 0x0506]);
    }
}

mod gridded_data {
    use super::*;

    /// Fixed fields up to and including the padding after Vector Dimension.
    const FIXED: usize = 64;

    fn regular_axis() -> GridAxisDescriptor {
        GridAxisDescriptor {
            data: GridAxisType::FixedSpacing {
                number_of_points_on_x_axis: 10,
                initial_index: 1,
            },
            ..GridAxisDescriptor::default()
        }
    }

    fn irregular_axis(points: usize) -> GridAxisDescriptor {
        GridAxisDescriptor {
            data: GridAxisType::VariableSpacing {
                initial_index: 0,
                coordinate_scale_x: 0.5,
                coordinate_offset_x: 2.0,
                x_values: (0..points).map(|i| i as u16).collect(),
            },
            ..GridAxisDescriptor::default()
        }
    }

    fn record(data: DataRepresentationType) -> GridDataRecord {
        GridDataRecord {
            sample_type: 0x1234,
            data,
            ..GridDataRecord::default()
        }
    }

    #[test]
    fn record_sizes_follow_tables_66_to_70() {
        let cases: [(GridAxisDescriptor, usize); 4] = [
            (regular_axis(), 24),
            (irregular_axis(4), 48),
            (irregular_axis(5), 56),
            (irregular_axis(0), 40),
        ];
        for (axis, size) in cases {
            let mut pdu = GriddedDataPdu::new();
            pdu.grid_axis_descriptors = vec![axis];
            assert_eq!(to_bytes(&mut pdu).len(), FIXED + size);
            round_trip(&mut pdu);
        }

        let records: [(DataRepresentationType, usize); 6] = [
            (
                DataRepresentationType::Type0 {
                    data_values: vec![1, 2, 3],
                },
                10,
            ),
            (
                DataRepresentationType::Type0 {
                    data_values: vec![1, 2],
                },
                8,
            ),
            (
                DataRepresentationType::Type1 {
                    field_scale: 1.0,
                    field_offset: 0.0,
                    data_values: vec![1],
                },
                16,
            ),
            (
                DataRepresentationType::Type1 {
                    field_scale: 1.0,
                    field_offset: 0.0,
                    data_values: vec![1, 2],
                },
                20,
            ),
            (
                DataRepresentationType::Type2 {
                    data_values: vec![],
                },
                8,
            ),
            (
                DataRepresentationType::Type2 {
                    data_values: vec![1.0, 2.0, 3.0],
                },
                20,
            ),
        ];
        for (data, size) in records {
            let mut pdu = GriddedDataPdu::new();
            pdu.grid_data_list = vec![record(data)];
            assert_eq!(to_bytes(&mut pdu).len(), FIXED + size);
            round_trip(&mut pdu);
        }
    }

    #[test]
    fn axes_and_records_round_trip_together() {
        let mut pdu = GriddedDataPdu::new();
        pdu.grid_axis_descriptors = vec![regular_axis(), irregular_axis(3)];
        pdu.grid_data_list = vec![
            record(DataRepresentationType::Type1 {
                field_scale: 2.0,
                field_offset: 1.0,
                data_values: vec![5, 6, 7],
            }),
            record(DataRepresentationType::Type2 {
                data_values: vec![9.5],
            }),
            record(DataRepresentationType::Type0 {
                data_values: vec![1],
            }),
        ];
        let decoded = round_trip(&mut pdu);
        assert_eq!(decoded.number_of_grid_axes, 2);
        assert_eq!(
            decoded.grid_axis_descriptors[1].axis_type,
            GridAxisDescriptorAxisType::IrregularAxis
        );
        assert_eq!(
            decoded.grid_data_list.len(),
            3,
            "records are read to the end of the PDU"
        );
        assert_eq!(decoded.grid_data_list[0].sample_type, 0x1234);
        assert_eq!(
            decoded.grid_data_list[1].data_representation,
            GriddedDataDataRepresentation::Type2
        );
        assert_eq!(
            decoded.grid_data_list[1].data,
            DataRepresentationType::Type2 {
                data_values: vec![9.5]
            }
        );
    }

    #[test]
    fn tags_follow_the_data() {
        let mut pdu = GriddedDataPdu::new();
        let mut axis = irregular_axis(1);
        axis.axis_type = GridAxisDescriptorAxisType::RegularAxis; // stale
        pdu.grid_axis_descriptors = vec![axis];
        let mut grid = record(DataRepresentationType::Type2 {
            data_values: vec![],
        });
        grid.data_representation = GriddedDataDataRepresentation::Type0; // stale
        pdu.grid_data_list = vec![grid];
        let decoded = round_trip(&mut pdu);
        assert_eq!(
            pdu.grid_axis_descriptors[0].axis_type,
            GridAxisDescriptorAxisType::IrregularAxis
        );
        assert_eq!(
            decoded.grid_data_list[0].data_representation,
            GriddedDataDataRepresentation::Type2
        );
    }

    #[test]
    fn undefined_data_representation_is_rejected() {
        let mut pdu = GriddedDataPdu::new();
        pdu.grid_data_list = vec![record(DataRepresentationType::Type2 {
            data_values: vec![],
        })];
        let mut bytes = to_bytes(&mut pdu).to_vec();
        bytes[FIXED + 3] = 7;
        let err = GriddedDataPdu::deserialize(&mut Bytes::from(bytes)).unwrap_err();
        assert!(
            err.to_string().contains("GriddedDataDataRepresentation"),
            "{err}"
        );
    }
}

// ---------------------------------------------------------------------------
// Radio communications
// ---------------------------------------------------------------------------

lists!(
    transmitter,
    TransmitterPdu,
    |p| {
        p.modulation_parameters = Some(ModulationParameters {
            record_specific_fields: vec![1, 2, 3],
        });
        p.antenna_pattern = Some(AntennaPattern {
            record_specific_fields: vec![9; 5],
        });
        p.variable_transmitter_parameters = vec![
            VariableTransmitterParameters {
                record_type: VariableRecordTypes::default(),
                record_specific_fields: vec![4],
                ..VariableTransmitterParameters::default()
            };
            2
        ];
    },
    |d| {
        assert_eq!(d.modulation_parameter_length, 3);
        assert_eq!(d.antenna_pattern_length, 5);
        assert_eq!(d.number_of_variable_transmitter_parameters_records, 2);
        assert_eq!(
            d.modulation_parameters.unwrap().record_specific_fields,
            [1, 2, 3]
        );
        assert_eq!(d.antenna_pattern.unwrap().record_specific_fields.len(), 5);
        assert_eq!(d.variable_transmitter_parameters[0].record_length, 8);
    }
);

#[test]
fn transmitter_without_modulation_or_antenna_data() {
    let decoded = round_trip(&mut TransmitterPdu::new());
    assert!(decoded.modulation_parameters.is_none());
    assert!(decoded.antenna_pattern.is_none());
}

#[test]
fn signal_data_length_is_in_bits_and_data_is_padded_to_32_bits() {
    let mut pdu = SignalPdu::new();
    let base = to_bytes(&mut pdu).len();
    assert_eq!(base % 4, 0);

    pdu.data = vec![0xAB; 5];
    let bytes = to_bytes(&mut pdu);
    assert_eq!(pdu.data_length, 40, "5 octets is 40 bits");
    assert_eq!(bytes.len(), base + 8, "5 octets padded to 8");
    assert!(bytes[base + 5..].iter().all(|&b| b == 0));
    assert_eq!(round_trip(&mut pdu).data, [0xAB; 5]);

    // A bit length that isn't a whole number of octets is kept.
    pdu.data_length = 37;
    let decoded = round_trip(&mut pdu);
    assert_eq!((decoded.data_length, decoded.data.len()), (37, 5));
}

#[test]
fn intercom_signal_data_round_trips() {
    let mut pdu = IntercomSignalPdu::new();
    pdu.data = vec![1, 2, 3, 4, 5, 6, 7];
    let decoded = round_trip(&mut pdu);
    assert_eq!(decoded.data_length, 56);
    assert_eq!(decoded.data, [1, 2, 3, 4, 5, 6, 7]);
}

lists!(
    intercom_control,
    IntercomControlPdu,
    |p| {
        p.intercom_parameters = vec![IntercomCommunicationsParameters::default(); 2];
    },
    |d| {
        assert_eq!(d.intercom_parameters_length, 2);
    }
);

// ---------------------------------------------------------------------------
// Simulation management (with and without reliability)
// ---------------------------------------------------------------------------

macro_rules! datum_lists {
    ($($test:ident: $pdu:ty),* $(,)?) => {$(
        lists!($test, $pdu, |p| {
            p.fixed_datum_records = vec![FixedDatumRecord::default(); 2];
            p.variable_datum_records = vec![
                VariableDatumRecord { length_bits: 12, value: vec![0xA, 0xB], ..VariableDatumRecord::default() };
                3
            ];
        }, |d| {
            assert_eq!(d.number_of_fixed_datum_records, 2);
            assert_eq!(d.number_of_variable_datum_records, 3);
            assert_eq!(d.variable_datum_records[2].value, [0xA, 0xB]);
        });
    )*};
}

datum_lists!(
    action_request: ActionRequestPdu,
    action_response: ActionResponsePdu,
    data_query: DataQueryPdu,
    set_data: SetDataPdu,
    data: DataPdu,
    event_report: EventReportPdu,
    comment: CommentPdu,
    action_request_reliable: ActionRequestReliablePdu,
    action_response_reliable: ActionResponseReliablePdu,
    set_data_reliable: SetDataReliablePdu,
    data_reliable: DataReliablePdu,
    event_report_reliable: EventReportReliablePdu,
    comment_reliable: CommentReliablePdu,
);

lists!(
    data_query_reliable,
    DataQueryReliablePdu,
    |p| {
        p.fixed_datum_ids = vec![VariableRecordTypes::default(); 2];
        p.variable_datum_ids = vec![VariableRecordTypes::default(); 1];
    },
    |d| {
        assert_eq!(
            (
                d.number_of_fixed_datum_records,
                d.number_of_variable_datum_records
            ),
            (2, 1)
        );
    }
);

lists!(
    record_query_reliable,
    RecordQueryReliablePdu,
    |p| {
        p.record_ids = vec![10, 20, 30];
    },
    |d| {
        assert_eq!(d.number_of_records, 3);
        assert_eq!(d.record_ids, [10, 20, 30]);
    }
);

// ---------------------------------------------------------------------------
// Synthetic environment and warfare
// ---------------------------------------------------------------------------

lists!(
    environmental_process,
    EnvironmentalProcessPdu,
    |p| {
        p.environment_records = vec![Environment::default(); 2];
    },
    |d| {
        assert_eq!(d.number_of_environment_records, 2);
    }
);

lists!(
    linear_object_state,
    LinearObjectStatePdu,
    |p| {
        p.linear_segment_parameters = vec![LinearSegmentParameter::default(); 3];
    },
    |d| {
        assert_eq!(d.number_of_segments, 3);
    }
);

lists!(
    areal_object_state,
    ArealObjectStatePdu,
    |p| {
        p.object_location = vec![WorldCoordinate::default(); 4];
    },
    |d| {
        assert_eq!(d.number_of_points, 4);
    }
);

lists!(
    detonation,
    DetonationPdu,
    |p| {
        p.variable_parameters = vec![VariableParameter::default(); 2];
    },
    |d| {
        assert_eq!(d.number_of_variable_parameters, 2);
    }
);

lists!(
    directed_energy_fire,
    DirectedEnergyFirePdu,
    |p| {
        p.damage_descriptions = vec![DirectedEnergyDamage::default(); 2];
    },
    |d| {
        assert_eq!(d.number_of_de_records, 2);
    }
);

lists!(
    entity_damage_status,
    EntityDamageStatusPdu,
    |p| {
        p.damage_descriptions = vec![DirectedEnergyDamage::default(); 3];
    },
    |d| {
        assert_eq!(d.number_of_damage_descriptions, 3);
    }
);

// ---------------------------------------------------------------------------
// IFF layers
// ---------------------------------------------------------------------------

mod iff {
    use super::*;

    fn layer_2(records: usize) -> IffLayer2 {
        IffLayer2 {
            iff_parameters: vec![IFFFundamentalParameterData::default(); records],
            ..IffLayer2::default()
        }
    }

    #[test]
    fn layer_1_only() {
        let decoded = round_trip(&mut IFFPdu::new());
        assert!(decoded.layers.layer_2.is_none());
        assert!(decoded.layers.other_layers.is_empty());
    }

    #[test]
    fn layer_2_counts_and_length_are_set() {
        let mut pdu = IFFPdu::new();
        pdu.layers.layer_2 = Some(layer_2(2));
        let decoded = round_trip(&mut pdu);
        let layer = decoded.layers.layer_2.unwrap();
        assert_eq!(layer.layer_header.layer_number, 2);
        assert_eq!(
            layer
                .secondary_operational_data
                .number_of_iff_fundamental_parameter_records,
            2
        );
        let expected = 4 + 20 + 4 + 2 * 24;
        assert_eq!(usize::from(layer.layer_header.length), expected);
        assert_eq!(layer.iff_parameters.len(), 2);
    }

    #[test]
    fn layers_after_2_are_kept_as_received() {
        let mut pdu = IFFPdu::new();
        pdu.layers.layer_2 = Some(layer_2(0));
        pdu.layers.other_layers = vec![3, 0, 0, 8, 1, 2, 3, 4];
        let decoded = round_trip(&mut pdu);
        assert!(decoded.layers.layer_2.is_some());
        assert_eq!(decoded.layers.other_layers, [3, 0, 0, 8, 1, 2, 3, 4]);
    }

    #[test]
    fn a_layer_3_without_layer_2_is_kept_as_received() {
        let mut pdu = IFFPdu::new();
        pdu.layers.other_layers = vec![3, 0, 0, 8, 9, 9, 9, 9];
        let decoded = round_trip(&mut pdu);
        assert!(decoded.layers.layer_2.is_none());
        assert_eq!(decoded.layers.other_layers.len(), 8);
    }
}

// ---------------------------------------------------------------------------
// Malformed input
// ---------------------------------------------------------------------------

/// Random bodies behind a valid header must decode or fail, never panic.
/// Whatever does decode must encode to bytes that decode and re-encode
/// unchanged. (Not necessarily the input: padding, for one, is re-encoded
/// as zeros.)
fn fuzz<P: Pdu + Default>() {
    let header = to_bytes(&mut P::default()).slice(..12);
    let mut state = 0x9E37_79B9_7F4A_7C15_u64;
    for _ in 0..300 {
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let len = 12 + (next() % 160) as usize;
        let mut bytes = header.to_vec();
        bytes.extend((12..len).map(|_| next() as u8));
        bytes[8..10].copy_from_slice(&(len as u16).to_be_bytes());
        let Ok(mut decoded) = P::deserialize(&mut Bytes::from(bytes)) else {
            continue;
        };
        let Ok(()) = decoded.serialize(&mut BytesMut::new()) else {
            continue; // e.g. a decoded count too large to re-encode
        };
        let canonical = to_bytes(&mut decoded);
        let mut again = P::deserialize(&mut canonical.clone()).unwrap_or_else(|e| {
            panic!(
                "{}: re-encoded bytes don't decode: {e}",
                std::any::type_name::<P>()
            )
        });
        assert_eq!(
            to_bytes(&mut again),
            canonical,
            "{}",
            std::any::type_name::<P>()
        );
    }
}

#[test]
fn random_bodies_never_panic_and_decoded_pdus_re_encode_stably() {
    fuzz::<crate::distributed_emissions::ElectromagneticEmissionsPdu>();
    fuzz::<crate::distributed_emissions::DesignatorPdu>();
    fuzz::<crate::distributed_emissions::UnderwaterAcousticPdu>();
    fuzz::<crate::distributed_emissions::IFFPdu>();
    fuzz::<crate::distributed_emissions::SupplementalEmissionPdu>();
    fuzz::<crate::entity_information::EntityStatePdu>();
    fuzz::<crate::entity_information::CollisionPdu>();
    fuzz::<crate::entity_information::CollisionElasticPdu>();
    fuzz::<crate::entity_information::EntityStateUpdatePdu>();
    fuzz::<crate::entity_information::AttributePdu>();
    fuzz::<crate::entity_management::AggregateStatePdu>();
    fuzz::<crate::entity_management::IsGroupOfPdu>();
    fuzz::<crate::entity_management::TransferOwnershipPdu>();
    fuzz::<crate::entity_management::IsPartOfPdu>();
    fuzz::<crate::information_operations::InformationOperationsReportPdu>();
    fuzz::<crate::information_operations::InformationOperationsActionPdu>();
    fuzz::<crate::live_entity_information::TimeSpacePositionInformationPdu>();
    fuzz::<crate::live_entity_information::AppearancePdu>();
    fuzz::<crate::live_entity_information::ArticulatedPartsPdu>();
    fuzz::<crate::live_entity_information::LiveEntityFirePdu>();
    fuzz::<crate::live_entity_information::LiveEntityDetonationPdu>();
    fuzz::<crate::logistics::ServiceRequestPdu>();
    fuzz::<crate::logistics::ResupplyOfferPdu>();
    fuzz::<crate::logistics::ResupplyReceivedPdu>();
    fuzz::<crate::logistics::ResupplyCancelPdu>();
    fuzz::<crate::logistics::RepairCompletePdu>();
    fuzz::<crate::logistics::RepairResponsePdu>();
    fuzz::<crate::minefield::MinefieldStatePdu>();
    fuzz::<crate::minefield::MinefieldQueryPdu>();
    fuzz::<crate::minefield::MinefieldDataPdu>();
    fuzz::<crate::synthetic_environment::GriddedDataPdu>();
    fuzz::<crate::minefield::MinefieldResponseNackPdu>();
    fuzz::<crate::radio_communications::TransmitterPdu>();
    fuzz::<crate::radio_communications::SignalPdu>();
    fuzz::<crate::radio_communications::ReceiverPdu>();
    fuzz::<crate::radio_communications::IntercomSignalPdu>();
    fuzz::<crate::radio_communications::IntercomControlPdu>();
    fuzz::<crate::simulation_management::CreateEntityPdu>();
    fuzz::<crate::simulation_management::RemoveEntityPdu>();
    fuzz::<crate::simulation_management::StartResumePdu>();
    fuzz::<crate::simulation_management::StopFreezePdu>();
    fuzz::<crate::simulation_management::AcknowledgePdu>();
    fuzz::<crate::simulation_management::ActionRequestPdu>();
    fuzz::<crate::simulation_management::ActionResponsePdu>();
    fuzz::<crate::simulation_management::DataQueryPdu>();
    fuzz::<crate::simulation_management::SetDataPdu>();
    fuzz::<crate::simulation_management::DataPdu>();
    fuzz::<crate::simulation_management::EventReportPdu>();
    fuzz::<crate::simulation_management::CommentPdu>();
    fuzz::<crate::simulation_management_with_reliability::CreateEntityReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::RemoveEntityReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::StartResumeReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::StopFreezeReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::AcknowledgeReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::ActionRequestReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::ActionResponseReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::DataQueryReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::SetDataReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::DataReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::EventReportReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::CommentReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::RecordQueryReliablePdu>();
    fuzz::<crate::simulation_management_with_reliability::SetRecordReliablePdu>();
    fuzz::<crate::synthetic_environment::EnvironmentalProcessPdu>();
    fuzz::<crate::synthetic_environment::PointObjectStatePdu>();
    fuzz::<crate::synthetic_environment::LinearObjectStatePdu>();
    fuzz::<crate::synthetic_environment::ArealObjectStatePdu>();
    fuzz::<crate::warfare::FirePdu>();
    fuzz::<crate::warfare::DetonationPdu>();
    fuzz::<crate::warfare::DirectedEnergyFirePdu>();
    fuzz::<crate::warfare::EntityDamageStatusPdu>();
}

#[test]
fn truncation_is_an_error_for_every_pdu() {
    fn check<P: Pdu + Default>() {
        let bytes = to_bytes(&mut P::default());
        for cut in [0, 11, 12, bytes.len() - 1] {
            assert!(
                P::deserialize(&mut bytes.slice(..cut)).is_err(),
                "{}: {cut} of {} octets",
                std::any::type_name::<P>(),
                bytes.len()
            );
        }
    }
    check::<crate::distributed_emissions::ElectromagneticEmissionsPdu>();
    check::<crate::distributed_emissions::DesignatorPdu>();
    check::<crate::distributed_emissions::UnderwaterAcousticPdu>();
    check::<crate::distributed_emissions::IFFPdu>();
    check::<crate::distributed_emissions::SupplementalEmissionPdu>();
    check::<crate::entity_information::EntityStatePdu>();
    check::<crate::entity_information::CollisionPdu>();
    check::<crate::entity_information::CollisionElasticPdu>();
    check::<crate::entity_information::EntityStateUpdatePdu>();
    check::<crate::entity_information::AttributePdu>();
    check::<crate::entity_management::AggregateStatePdu>();
    check::<crate::entity_management::IsGroupOfPdu>();
    check::<crate::entity_management::TransferOwnershipPdu>();
    check::<crate::entity_management::IsPartOfPdu>();
    check::<crate::information_operations::InformationOperationsReportPdu>();
    check::<crate::information_operations::InformationOperationsActionPdu>();
    check::<crate::live_entity_information::TimeSpacePositionInformationPdu>();
    check::<crate::live_entity_information::AppearancePdu>();
    check::<crate::live_entity_information::ArticulatedPartsPdu>();
    check::<crate::live_entity_information::LiveEntityFirePdu>();
    check::<crate::live_entity_information::LiveEntityDetonationPdu>();
    check::<crate::logistics::ServiceRequestPdu>();
    check::<crate::logistics::ResupplyOfferPdu>();
    check::<crate::logistics::ResupplyReceivedPdu>();
    check::<crate::logistics::ResupplyCancelPdu>();
    check::<crate::logistics::RepairCompletePdu>();
    check::<crate::logistics::RepairResponsePdu>();
    check::<crate::minefield::MinefieldStatePdu>();
    check::<crate::minefield::MinefieldQueryPdu>();
    check::<crate::minefield::MinefieldDataPdu>();
    check::<crate::synthetic_environment::GriddedDataPdu>();
    check::<crate::minefield::MinefieldResponseNackPdu>();
    check::<crate::radio_communications::TransmitterPdu>();
    check::<crate::radio_communications::SignalPdu>();
    check::<crate::radio_communications::ReceiverPdu>();
    check::<crate::radio_communications::IntercomSignalPdu>();
    check::<crate::radio_communications::IntercomControlPdu>();
    check::<crate::simulation_management::CreateEntityPdu>();
    check::<crate::simulation_management::RemoveEntityPdu>();
    check::<crate::simulation_management::StartResumePdu>();
    check::<crate::simulation_management::StopFreezePdu>();
    check::<crate::simulation_management::AcknowledgePdu>();
    check::<crate::simulation_management::ActionRequestPdu>();
    check::<crate::simulation_management::ActionResponsePdu>();
    check::<crate::simulation_management::DataQueryPdu>();
    check::<crate::simulation_management::SetDataPdu>();
    check::<crate::simulation_management::DataPdu>();
    check::<crate::simulation_management::EventReportPdu>();
    check::<crate::simulation_management::CommentPdu>();
    check::<crate::simulation_management_with_reliability::CreateEntityReliablePdu>();
    check::<crate::simulation_management_with_reliability::RemoveEntityReliablePdu>();
    check::<crate::simulation_management_with_reliability::StartResumeReliablePdu>();
    check::<crate::simulation_management_with_reliability::StopFreezeReliablePdu>();
    check::<crate::simulation_management_with_reliability::AcknowledgeReliablePdu>();
    check::<crate::simulation_management_with_reliability::ActionRequestReliablePdu>();
    check::<crate::simulation_management_with_reliability::ActionResponseReliablePdu>();
    check::<crate::simulation_management_with_reliability::DataQueryReliablePdu>();
    check::<crate::simulation_management_with_reliability::SetDataReliablePdu>();
    check::<crate::simulation_management_with_reliability::DataReliablePdu>();
    check::<crate::simulation_management_with_reliability::EventReportReliablePdu>();
    check::<crate::simulation_management_with_reliability::CommentReliablePdu>();
    check::<crate::simulation_management_with_reliability::RecordQueryReliablePdu>();
    check::<crate::simulation_management_with_reliability::SetRecordReliablePdu>();
    check::<crate::synthetic_environment::EnvironmentalProcessPdu>();
    check::<crate::synthetic_environment::PointObjectStatePdu>();
    check::<crate::synthetic_environment::LinearObjectStatePdu>();
    check::<crate::synthetic_environment::ArealObjectStatePdu>();
    check::<crate::warfare::FirePdu>();
    check::<crate::warfare::DetonationPdu>();
    check::<crate::warfare::DirectedEnergyFirePdu>();
    check::<crate::warfare::EntityDamageStatusPdu>();
}

#[test]
fn undefined_enum_values_are_errors_not_defaults() {
    // Setting each octet of a Fire PDU in turn to 200 must, somewhere, hit an
    // enumeration (such as the munition's EntityKind) and fail to decode.
    let mut bytes = to_bytes(&mut FirePdu::new()).to_vec();
    let mut saw_enum_error = false;
    for offset in 12..bytes.len() {
        let original = bytes[offset];
        bytes[offset] = 200;
        if let Err(DISError::WireFormat(e)) = FirePdu::deserialize(&mut Bytes::from(bytes.clone()))
        {
            saw_enum_error |= e.to_string().contains("is not a defined");
        }
        bytes[offset] = original;
    }
    assert!(saw_enum_error);
}
