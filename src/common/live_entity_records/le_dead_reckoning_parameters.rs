//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2026 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BytesMut};

use crate::{
    common::{
        enums::DeadReckoningAlgorithm,
        live_entity_records::{
            le_entity_angular_velocity::LEEntityAngularVelocity,
            le_entity_linear_acceleration::LEEntityLinearAcceleration,
        },
    },
    impl_record_fields,
    pdu_macro::{FieldDeserialize, FieldSerialize},
};

#[derive(Copy, Clone, Debug, Default)]
pub struct LEDeadReckoningParameters {
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    pub entity_linear_acceleration: LEEntityLinearAcceleration,
    pub entity_angular_velocity: LEEntityAngularVelocity,
}

impl_record_fields!(LEDeadReckoningParameters {
    dead_reckoning_algorithm: DeadReckoningAlgorithm,
    entity_linear_acceleration: LEEntityLinearAcceleration,
    entity_angular_velocity: LEEntityAngularVelocity,
});

impl LEDeadReckoningParameters {
    #[must_use]
    pub const fn new(
        dead_reckoning_algorithm: DeadReckoningAlgorithm,
        entity_linear_acceleration: LEEntityLinearAcceleration,
        entity_angular_velocity: LEEntityAngularVelocity,
    ) -> Self {
        Self {
            dead_reckoning_algorithm,
            entity_linear_acceleration,
            entity_angular_velocity,
        }
    }

    /// Convenience wrapper around [`FieldSerialize::serialize_field`].
    pub fn serialize(&self, buf: &mut BytesMut) {
        self.serialize_field(buf);
    }

    /// Convenience wrapper around [`FieldDeserialize::deserialize_field`].
    pub fn deserialize<B: Buf>(buf: &mut B) -> Self {
        Self::deserialize_field(buf)
    }
}
