//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

#![allow(deprecated)]

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    EntityCoordinateVector, LinearAcceleration, WorldCoordinate,
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
    _padding: u8,
    _padding2: u16,
    pub entity_linear_acceleration: LinearAcceleration,
}

impl Default for DesignatorPdu {
    fn default() -> Self {
        DesignatorPdu {
            pdu_header: PduHeader::default(),
            designating_entity_id: EntityId::default(1),
            code_name: DesignatorSystemName::default(),
            designated_entity_id: EntityId::default(2),
            designator_code: DesignatorCode::default(),
            designator_power: 0.0,
            designator_wavelength: 0.0,
            designator_spot_wrt_designated: EntityCoordinateVector::new(0.0, 0.0, 0.0),
            designator_spot_location: WorldCoordinate::new(0.0, 0.0, 0.0),
            dead_reckoning_algorithm: DeadReckoningAlgorithm::default(),
            _padding: 0,
            _padding2: 0,
            entity_linear_acceleration: LinearAcceleration::new(0.0, 0.0, 0.0),
        }
    }
}

impl Pdu for DesignatorPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>() * 2
            + std::mem::size_of::<DesignatorSystemName>()
            + std::mem::size_of::<DesignatorCode>()
            + std::mem::size_of::<f32>() * 2
            + std::mem::size_of::<EntityCoordinateVector>()
            + std::mem::size_of::<WorldCoordinate>()
            + std::mem::size_of::<DeadReckoningAlgorithm>()
            + std::mem::size_of::<u8>()
            + std::mem::size_of::<u16>()
            + std::mem::size_of::<LinearAcceleration>();

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    /// Serialize contents of `DesignatorPdu` into `BytesMut` buf
    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let size = std::mem::size_of_val(self);
        self.pdu_header.length = u16::try_from(size).map_err(|_| DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })?;
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
        buf.put_u8(self._padding);
        buf.put_u16(self._padding2);
        self.entity_linear_acceleration.serialize(buf);
        Ok(())
    }

    /// Deserialize bytes from `BytesMut` buf and interpret as `DesignatorPdu`
    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
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

impl DesignatorPdu {
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
        let _padding = buf.get_u8();
        let _padding2 = buf.get_u16();
        let entity_linear_acceleration = LinearAcceleration::deserialize(buf);

        DesignatorPdu {
            pdu_header: PduHeader::default(),
            designating_entity_id,
            code_name,
            designated_entity_id,
            designator_code,
            designator_power,
            designator_wavelength,
            designator_spot_wrt_designated,
            designator_spot_location,
            dead_reckoning_algorithm,
            _padding,
            _padding2,
            entity_linear_acceleration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DesignatorPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let pdu = DesignatorPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<DesignatorPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = DesignatorPdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = DesignatorPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 704 / 8;
        let pdu = DesignatorPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
