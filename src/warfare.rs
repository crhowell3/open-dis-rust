//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

//! The Warfare protocol family

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{
            ClockTime, EntityCoordinateVector, EntityId, EntityType, EventId, LinearVelocity,
            VariableParameter, WorldCoordinate, directed_energy_damage::DirectedEnergyDamage,
            munition_descriptor::MunitionDescriptor,
        },
        enums::{DEFirePulseShape, DetonationResult, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.3.2
    pub struct FirePdu {
        header: PduHeader,
        pdu_type: PduType::Fire,
        protocol_family: ProtocolFamily::Warfare,
        fields: {
            pub firing_entity_id: EntityId,
            pub target_entity_id: EntityId,
            pub munition_expendable_id: EntityId,
            pub event_id: EventId,
            pub fire_mission_index: u32,
            pub location_in_world_coordinates: WorldCoordinate,
            pub descriptor: MunitionDescriptor,
            pub velocity: LinearVelocity,
            pub range: f32,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.3.3
    pub struct DetonationPdu {
        header: PduHeader,
        pdu_type: PduType::Detonation,
        protocol_family: ProtocolFamily::Warfare,
        fields: {
            pub firing_entity_id: EntityId,
            pub target_entity_id: EntityId,
            pub exploding_entity_id: EntityId,
            pub event_id: EventId,
            pub velocity: LinearVelocity,
            pub location_in_world_coordinates: WorldCoordinate,
            pub descriptor: MunitionDescriptor,
            pub location_in_entitys_coordinates: EntityCoordinateVector,
            pub detonation_result: DetonationResult,
            pub number_of_variable_parameters: u8,
            padding: u16,
            pub variable_parameters: Vec<VariableParameter>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.3.4
    pub struct DirectedEnergyFirePdu {
        header: PduHeader,
        pdu_type: PduType::DirectedEnergyFire,
        protocol_family: ProtocolFamily::Warfare,
        fields: {
            pub firing_entity_id: EntityId,
            pub event_id: EventId,
            pub munition_type: EntityType,
            pub shot_start_time: ClockTime,
            pub cumulative_shot_time: f32,
            pub aperture_emitter_location: EntityCoordinateVector,
            pub aperture_diameter: f32,
            pub wavelength: f32,
            padding: u32,
            pub pulse_repetition_frequency: f32,
            pub pulse_width: f32,
            pub flags: u16,
            pub pulse_shape: DEFirePulseShape,
            padding2: u8,
            padding3: u32,
            padding4: u16,
            pub number_of_de_records: u16,
            pub damage_descriptions: Vec<DirectedEnergyDamage>,
        }
    }
}

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.3.5
    pub struct EntityDamageStatusPdu {
        header: PduHeader,
        pdu_type: PduType::EntityDamageStatus,
        protocol_family: ProtocolFamily::Warfare,
        fields: {
            pub damaged_entity_id: EntityId,
            padding: u16,
            padding2: u16,
            pub number_of_damage_descriptions: u16,
            pub damage_descriptions: Vec<DirectedEnergyDamage>,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{common::pdu::Pdu, pdu_tests};

    pdu_tests!(
        fire_pdu_tests, FirePdu,
        pdu_type: PduType::Fire,
        default_bits: 768,
        wrong_type: DetonationPdu,
    );
    pdu_tests!(
        detonation_pdu_tests, DetonationPdu,
        pdu_type: PduType::Detonation,
        default_bits: 832,
        wrong_type: DirectedEnergyFirePdu,
    );
    pdu_tests!(
        directed_energy_fire_pdu_tests, DirectedEnergyFirePdu,
        pdu_type: PduType::DirectedEnergyFire,
        default_bits: 704,
        wrong_type: EntityDamageStatusPdu,
    );
    pdu_tests!(
        entity_damage_status_pdu_tests, EntityDamageStatusPdu,
        pdu_type: PduType::EntityDamageStatus,
        default_bits: 192,
        wrong_type: DetonationPdu,
    );
}
