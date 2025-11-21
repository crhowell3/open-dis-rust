//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    SerializedLength, WorldCoordinate,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{ForceId, PduType, ProtocolFamily},
    euler_angles::EulerAngles,
    pdu::Pdu,
    pdu_header::PduHeader,
    simulation_address::SimulationAddress,
};

use super::data_types::object_type::ObjectType;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.10.4
pub struct PointObjectStatePdu {
    pdu_header: PduHeader,
    pub object_id: EntityId,
    pub referenced_object_id: EntityId,
    pub update_number: u16,
    pub force_id: ForceId,
    pub modifications: u8,
    pub object_type: ObjectType,
    pub object_location: WorldCoordinate,
    pub object_orientation: EulerAngles,
    pub specific_object_appearance: u32,
    pub general_object_appearance: u16,
    _padding: u16,
    pub requester_id: SimulationAddress,
    pub receiving_id: SimulationAddress,
    _padding2: u32,
}

impl Default for PointObjectStatePdu {
    fn default() -> Self {
        PointObjectStatePdu {
            pdu_header: PduHeader::default(),
            object_id: EntityId::default(1),
            referenced_object_id: EntityId::default(2),
            update_number: 0,
            force_id: ForceId::default(),
            modifications: 0,
            object_type: ObjectType::default(),
            object_location: WorldCoordinate::default(),
            object_orientation: EulerAngles::default(),
            specific_object_appearance: 0u32,
            general_object_appearance: 0u16,
            _padding: 0u16,
            requester_id: SimulationAddress::default(),
            receiving_id: SimulationAddress::default(),
            _padding2: 0u32,
        }
    }
}

impl Pdu for PointObjectStatePdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH
            + EntityId::LENGTH * 2
            + 2
            + 1
            + 1
            + ObjectType::LENGTH
            + WorldCoordinate::LENGTH
            + EulerAngles::LENGTH
            + 4
            + 2
            + 2
            + SimulationAddress::LENGTH * 2
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
        self.object_id.serialize(buf);
        self.referenced_object_id.serialize(buf);
        buf.put_u16(self.update_number);
        buf.put_u8(self.force_id as u8);
        buf.put_u8(self.modifications);
        self.object_type.serialize(buf);
        self.object_location.serialize(buf);
        self.object_orientation.serialize(buf);
        buf.put_u32(self.specific_object_appearance);
        buf.put_u16(self.general_object_appearance);
        buf.put_u16(self._padding);
        self.requester_id.serialize(buf);
        self.receiving_id.serialize(buf);
        buf.put_u32(self._padding2);
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::PointObjectState {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type PointObjectState, got {:?}",
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

impl PointObjectStatePdu {
    /// Creates a new `PointObjectStatePdu`
    ///
    /// # Examples
    ///
    /// Initializing a `PointObjectStatePdu`:
    /// ```
    /// use open_dis_rust::warfare::PointObjectStatePdu;
    /// let pdu = PointObjectStatePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::PointObjectState;
        pdu.pdu_header.protocol_family = ProtocolFamily::SyntheticEnvironment;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let object_id = EntityId::deserialize(buf);
        let referenced_object_id = EntityId::deserialize(buf);
        let update_number = buf.get_u16();
        let force_id = ForceId::deserialize(buf);
        let modifications = buf.get_u8();
        let object_type = ObjectType::deserialize(buf);
        let object_location = WorldCoordinate::deserialize(buf);
        let object_orientation = EulerAngles::deserialize(buf);
        let specific_object_appearance = buf.get_u32();
        let general_object_appearance = buf.get_u16();
        let _padding = buf.get_u16();
        let requester_id = SimulationAddress::deserialize(buf);
        let receiving_id = SimulationAddress::deserialize(buf);
        let _padding2 = buf.get_u32();

        PointObjectStatePdu {
            pdu_header: PduHeader::default(),
            object_id,
            referenced_object_id,
            update_number,
            force_id,
            modifications,
            object_type,
            object_location,
            object_orientation,
            specific_object_appearance,
            general_object_appearance,
            _padding,
            requester_id,
            receiving_id,
            _padding2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PointObjectStatePdu;
    use crate::common::{constants::BITS_PER_BYTE, pdu::Pdu};
    use bytes::BytesMut;

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = PointObjectStatePdu::default();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = PointObjectStatePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 704 / BITS_PER_BYTE;
        let pdu = PointObjectStatePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
