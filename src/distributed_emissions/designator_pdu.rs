//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#![allow(deprecated)]

use crate::{
    common::{
        GenericHeader, SerializedLength,
        data_types::{EntityCoordinateVector, EntityId, LinearAcceleration, WorldCoordinate},
        enums::{
            DeadReckoningAlgorithm, DesignatorCode, DesignatorSystemName, PduType, ProtocolFamily,
        },
        pdu::Pdu,
        pdu_header::PduHeader,
    },
    define_pdu,
};

define_pdu! {
    #[derive(Debug)]
    /// Implemented according to IEEE 1278.1-2012 §7.6.3
    pub struct DesignatorPdu {
        header: PduHeader,
        pdu_type: PduType::Designator,
        protocol_family: ProtocolFamily::DistributedEmissionRegeneration,
        fields: {
            pub designating_entity_id: EntityId,
            pub code_name: DesignatorSystemName,
            pub designated_entity_id: EntityId,
            pub designator_code: DesignatorCode,
            pub designator_power: f32,
            pub designator_wavelength: f32,
            pub designator_spot_wrt_designated: EntityCoordinateVector,
            pub designator_spot_location: WorldCoordinate,
            pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
            padding: u8,
            padding2: u16,
            pub entity_linear_acceleration: LinearAcceleration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DesignatorPdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = DesignatorPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<DesignatorPdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = DesignatorPdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = DesignatorPdu::deserialize(&mut deserialize_buf).unwrap_or_default();
        assert_eq!(new_pdu.header, pdu.header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 704 / BITS_PER_BYTE;
        let pdu = DesignatorPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
