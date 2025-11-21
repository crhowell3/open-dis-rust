//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//                     (DIS) application protocol v6 and v7
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::{
    common::{
        SerializedLength,
        constants::MAX_PDU_SIZE_OCTETS,
        dis_error::DISError,
        entity_id::EntityId,
        enums::{PduType, ProtocolFamily},
        euler_angles::EulerAngles,
        linear_velocity::LinearVelocity,
        pdu::Pdu,
        pdu_header::PduHeader,
        world_coordinate::WorldCoordinate,
    },
    warfare::data_types::variable_parameter::VariableParameter,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.2.5
pub struct EntityStateUpdatePdu {
    pdu_header: PduHeader,
    pub entity_id: EntityId,
    _padding: u8,
    pub number_of_variable_parameters: u8,
    pub entity_linear_velocity: LinearVelocity,
    pub entity_location: WorldCoordinate,
    pub entity_orientation: EulerAngles,
    pub entity_appearance: u32,
    pub variable_parameter_records: Vec<VariableParameter>,
}

impl Default for EntityStateUpdatePdu {
    fn default() -> Self {
        EntityStateUpdatePdu {
            pdu_header: PduHeader::default(),
            entity_id: EntityId::default(1),
            _padding: 0,
            number_of_variable_parameters: 0,
            entity_linear_velocity: LinearVelocity::default(),
            entity_location: WorldCoordinate::default(),
            entity_orientation: EulerAngles::default(),
            entity_appearance: 0,
            variable_parameter_records: vec![],
        }
    }
}

impl Pdu for EntityStateUpdatePdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH
            + EntityId::LENGTH
            + 1
            + 1
            + LinearVelocity::LENGTH
            + WorldCoordinate::LENGTH
            + EulerAngles::LENGTH
            + 4;

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let size = std::mem::size_of_val(self);
        self.pdu_header.length = u16::try_from(size).map_err(|_| DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })?;
        self.pdu_header.serialize(buf);
        self.entity_id.serialize(buf);
        buf.put_u8(self._padding);
        buf.put_u8(self.number_of_variable_parameters);
        self.entity_linear_velocity.serialize(buf);
        self.entity_location.serialize(buf);
        self.entity_orientation.serialize(buf);
        buf.put_u32(self.entity_appearance);
        for i in 0..self.variable_parameter_records.len() {
            self.variable_parameter_records[i].serialize(buf);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::EntityStateUpdate {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type EntityStateUpdate, got {:?}",
                    header.pdu_type
                ),
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

impl EntityStateUpdatePdu {
    /// Creates a new `EntityStateUpdatePdu`
    ///
    /// # Examples
    ///
    /// Initializing an `EntityStateUpdatePdu`:
    /// ```
    /// use open_dis_rust::entity_information::EntityStateUpdatePdu;
    /// let pdu = EntityStateUpdatePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::EntityStateUpdate;
        pdu.pdu_header.protocol_family = ProtocolFamily::EntityInformation;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let entity_id = EntityId::deserialize(buf);
        let _padding = buf.get_u8();
        let number_of_variable_parameters = buf.get_u8();
        let entity_linear_velocity = LinearVelocity::deserialize(buf);
        let entity_location = WorldCoordinate::deserialize(buf);
        let entity_orientation = EulerAngles::deserialize(buf);
        let entity_appearance = buf.get_u32();
        let mut variable_parameter_records: Vec<VariableParameter> = vec![];
        for _i in 0..number_of_variable_parameters {
            variable_parameter_records.push(VariableParameter::deserialize(buf));
        }
        EntityStateUpdatePdu {
            pdu_header: PduHeader::default(),
            entity_id,
            _padding,
            number_of_variable_parameters,
            entity_linear_velocity,
            entity_location,
            entity_orientation,
            entity_appearance,
            variable_parameter_records,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EntityStateUpdatePdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let pdu = EntityStateUpdatePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<EntityStateUpdatePdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = EntityStateUpdatePdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = EntityStateUpdatePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 576 / 8;
        let pdu = EntityStateUpdatePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
