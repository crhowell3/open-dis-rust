//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2025 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#![allow(deprecated)]

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    EntityCoordinateVector, GenericHeader, LinearAcceleration, PduBody, SerializedLength,
    WorldCoordinate,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{
        DeadReckoningAlgorithm, DesignatorCode, DesignatorSystemName, PduType, ProtocolFamily,
    },
    pdu::Pdu,
    pdu_header::PduHeader,
};

#[derive(Copy, Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.6.3
pub struct DesignatorPdu {
    pdu_header: PduHeader,
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

impl Default for DesignatorPdu {
    fn default() -> Self {
        DesignatorPdu {
            pdu_header: PduHeader::default(),
            designating_entity_id: EntityId::default(),
            code_name: DesignatorSystemName::default(),
            designated_entity_id: EntityId::default(),
            designator_code: DesignatorCode::default(),
            designator_power: 0.0,
            designator_wavelength: 0.0,
            designator_spot_wrt_designated: EntityCoordinateVector::new(0.0, 0.0, 0.0),
            designator_spot_location: WorldCoordinate::new(0.0, 0.0, 0.0),
            dead_reckoning_algorithm: DeadReckoningAlgorithm::default(),
            padding: 0,
            padding2: 0,
            entity_linear_acceleration: LinearAcceleration::new(0.0, 0.0, 0.0),
        }
    }
}

impl Pdu for DesignatorPdu {
    type Header = PduHeader;

    fn calculate_length(&self) -> Result<u16, DISError> {
        let length = PduHeader::LENGTH
            + EntityId::LENGTH * 2
            + std::mem::size_of::<u8>() * 2
            + std::mem::size_of::<u16>() * 3
            + std::mem::size_of::<f32>() * 2
            + EntityCoordinateVector::LENGTH
            + WorldCoordinate::LENGTH
            + LinearAcceleration::LENGTH;

        u16::try_from(length).map_err(|_| DISError::PduSizeExceeded {
            size: length,
            max_size: MAX_PDU_SIZE_OCTETS,
        })
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    /// Serialize contents of `DesignatorPdu` into `BytesMut` buf
    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let length = self.calculate_length()?;
        self.pdu_header.set_length(length);
        self.pdu_header.serialize(buf);
        self.designating_entity_id.serialize(buf);
        buf.put_u16(self.designator_code as u16);
        self.designated_entity_id.serialize(buf);
        buf.put_u16(self.designator_code as u16);
        buf.put_f32(self.designator_power);
        buf.put_f32(self.designator_wavelength);
        self.designator_spot_wrt_designated.serialize(buf);
        self.designator_spot_location.serialize(buf);
        buf.put_u8(self.dead_reckoning_algorithm as u8);
        buf.put_u8(self.padding);
        buf.put_u16(self.padding2);
        self.entity_linear_acceleration.serialize(buf);
        Ok(())
    }

    /// Deserialize bytes from `BytesMut` buf and interpret as `DesignatorPdu`
    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: Self::Header = Self::Header::deserialize(buf);
        if header.pdu_type != PduType::Designator {
            return Err(DISError::invalid_header(
                format!("Expected PDU type Designator, got {:?}", header.pdu_type),
                None,
            ));
        }
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn deserialize_without_header<B: Buf>(buf: &mut B, header: PduHeader) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let mut body = Self::deserialize_body(buf);
        body.pdu_header = header;
        Ok(body)
    }
}

impl PduBody for DesignatorPdu {
    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let designating_entity_id = EntityId::deserialize(buf);
        let code_name = DesignatorSystemName::deserialize(buf);
        let designated_entity_id = EntityId::deserialize(buf);
        let designator_code = DesignatorCode::deserialize(buf);
        let designator_power = buf.get_f32();
        let designator_wavelength = buf.get_f32();
        let designator_spot_wrt_designated = EntityCoordinateVector::deserialize(buf);
        let designator_spot_location = WorldCoordinate::deserialize(buf);
        let dead_reckoning_algorithm = DeadReckoningAlgorithm::deserialize(buf);
        let padding = buf.get_u8();
        let padding2 = buf.get_u16();
        let entity_linear_acceleration = LinearAcceleration::deserialize(buf);

        DesignatorPdu {
            pdu_header: <Self as Pdu>::Header::default(),
            designating_entity_id,
            code_name,
            designated_entity_id,
            designator_code,
            designator_power,
            designator_wavelength,
            designator_spot_wrt_designated,
            designator_spot_location,
            dead_reckoning_algorithm,
            padding,
            padding2,
            entity_linear_acceleration,
        }
    }
}

impl DesignatorPdu {
    #[must_use]
    /// Creates a new `DesignatorPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `DesignatorPdu`:
    /// ```
    /// use open_dis_rust::distributed_emissions::DesignatorPdu;
    /// let pdu = DesignatorPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::Designator;
        pdu.pdu_header.protocol_family = ProtocolFamily::DistributedEmissionRegeneration;
        pdu.finalize();
        pdu
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
        let new_pdu = DesignatorPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 704 / BITS_PER_BYTE;
        let pdu = DesignatorPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
