//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

//! The Warfare protocol family

use crate::common::{
    data_types::MunitionDescriptor,
    enums::{DEFirePulseShape, DetonationResult, PduType, ProtocolFamily},
    pdu_header::PduHeader,
    records::{
        ClockTime, DirectedEnergyDamage, EntityIdentifier, EntityType, EventIdentifier,
        VariableParameter, Vector, WorldCoordinates,
    },
};

/// Implemented according to IEEE 1278.1-2012 §7.3.2
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct FirePdu {
    #[deku(skip)]
    header: PduHeader,
    pub firing_entity_id: EntityIdentifier,
    pub target_entity_id: EntityIdentifier,
    pub munition_expendable_id: EntityIdentifier,
    pub event_id: EventIdentifier,
    pub fire_mission_index: u32,
    pub location_in_world_coordinates: WorldCoordinates,
    pub descriptor: MunitionDescriptor,
    pub velocity: Vector,
    pub range: f32,
}
crate::common::pdu::impl_pdu!(FirePdu, PduHeader, PduType::Fire, ProtocolFamily::Warfare);

/// Implemented according to IEEE 1278.1-2012 §7.3.3
#[derive(Debug, Default, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "big")]
pub struct DetonationPdu {
    #[deku(skip)]
    header: PduHeader,
    pub firing_entity_id: EntityIdentifier,
    pub target_entity_id: EntityIdentifier,
    pub exploding_entity_id: EntityIdentifier,
    pub event_id: EventIdentifier,
    pub velocity: Vector,
    pub location_in_world_coordinates: WorldCoordinates,
    pub descriptor: MunitionDescriptor,
    pub location_in_entitys_coordinates: Vector,
    pub detonation_result: DetonationResult,
    #[deku(update = "self.variable_parameters.len()")]
    pub number_of_variable_parameters: u8,
    padding: u16,
    #[deku(count = "number_of_variable_parameters")]
    pub variable_parameters: Vec<VariableParameter>,
}
crate::common::pdu::impl_pdu!(
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
    pub firing_entity_id: EntityIdentifier,
    pub event_id: EventIdentifier,
    pub munition_type: EntityType,
    pub shot_start_time: ClockTime,
    pub cumulative_shot_time: f32,
    pub aperture_emitter_location: Vector,
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
crate::common::pdu::impl_pdu!(
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
    pub damaged_entity_id: EntityIdentifier,
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
crate::common::pdu::impl_pdu!(
    EntityDamageStatusPdu,
    PduHeader,
    PduType::EntityDamageStatus,
    ProtocolFamily::Warfare
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
