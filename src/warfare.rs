//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

//! The Warfare protocol family

use crate::common::{
    data_types::{
        ClockTime, DirectedEnergyDamage, EntityCoordinateVector, EntityId, EntityType, EventId,
        LinearVelocity, MunitionDescriptor, VariableParameter, WorldCoordinate,
    },
    enums::{DEFirePulseShape, DetonationResult, PduType, ProtocolFamily},
    pdu_header::PduHeader,
};

/// Implemented according to IEEE 1278.1-2012 §7.3.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct FirePdu {
    #[deku(skip)]
    header: PduHeader,
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
crate::impl_pdu!(FirePdu, PduHeader, PduType::Fire, ProtocolFamily::Warfare);

/// Implemented according to IEEE 1278.1-2012 §7.3.3
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct DetonationPdu {
    #[deku(skip)]
    header: PduHeader,
    pub firing_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub exploding_entity_id: EntityId,
    pub event_id: EventId,
    pub velocity: LinearVelocity,
    pub location_in_world_coordinates: WorldCoordinate,
    pub descriptor: MunitionDescriptor,
    pub location_in_entitys_coordinates: EntityCoordinateVector,
    pub detonation_result: DetonationResult,
    #[deku(update = "self.variable_parameters.len()")]
    pub number_of_variable_parameters: u8,
    padding: u16,
    #[deku(count = "number_of_variable_parameters")]
    pub variable_parameters: Vec<VariableParameter>,
}
crate::impl_pdu!(
    DetonationPdu,
    PduHeader,
    PduType::Detonation,
    ProtocolFamily::Warfare
);

/// Implemented according to IEEE 1278.1-2012 §7.3.4
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct DirectedEnergyFirePdu {
    #[deku(skip)]
    header: PduHeader,
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
    #[deku(
        update = "self.damage_descriptions.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_de_records)"
    )]
    pub number_of_de_records: u16,
    #[deku(count = "number_of_de_records")]
    pub damage_descriptions: Vec<DirectedEnergyDamage>,
}
crate::impl_pdu!(
    DirectedEnergyFirePdu,
    PduHeader,
    PduType::DirectedEnergyFire,
    ProtocolFamily::Warfare
);

/// Implemented according to IEEE 1278.1-2012 §7.3.5
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct EntityDamageStatusPdu {
    #[deku(skip)]
    header: PduHeader,
    pub damaged_entity_id: EntityId,
    padding: u16,
    padding2: u16,
    #[deku(
        update = "self.damage_descriptions.len()",
        assert = "crate::common::wire::fits_in_pdu(*number_of_damage_descriptions)"
    )]
    pub number_of_damage_descriptions: u16,
    #[deku(count = "number_of_damage_descriptions")]
    pub damage_descriptions: Vec<DirectedEnergyDamage>,
}
crate::impl_pdu!(
    EntityDamageStatusPdu,
    PduHeader,
    PduType::EntityDamageStatus,
    ProtocolFamily::Warfare
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        common::{
            constants::BITS_PER_BYTE,
            pdu::{Pdu, to_bytes},
        },
        pdu_tests,
    };
    use bytes::Buf;

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
