//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::{
    common::{
        SerializedLength, WorldCoordinate,
        constants::MAX_PDU_SIZE_OCTETS,
        dis_error::DISError,
        entity_id::EntityId,
        enums::{ForceId, ObjectStateAppearanceGeneral, PduType, ProtocolFamily},
        pdu::Pdu,
        pdu_header::PduHeader,
        simulation_address::SimulationAddress,
    },
    synthetic_environment::data_types::object_type::ObjectType,
};

#[derive(Clone, Debug, Default)]
/// Implemented according to IEEE 1278.1-2012 §7.10.6
pub struct ArealObjectStatePdu {
    pdu_header: PduHeader,
    pub object_id: EntityId, // TODO(@anyone) Create an ObjectId type and replace this
    pub referenced_object_id: EntityId, // TODO(@anyone) Same as above
    pub update_number: u16,
    pub force_id: ForceId,
    pub modifications: u8, // TODO(@anyone) Replace with Modifications UID 242
    pub object_type: ObjectType,
    pub specific_object_appearance: u32, // TODO(@anyone) Implement Specific Object Appearance
    pub general_object_appearance: ObjectStateAppearanceGeneral,
    pub number_of_points: u16,
    pub requester_id: SimulationAddress,
    pub receiving_id: SimulationAddress,
    pub object_location: Vec<WorldCoordinate>,
}

impl Pdu for ArealObjectStatePdu {
    fn length(&self) -> u16 {
        let length = PduHeader::LENGTH
            + EntityId::LENGTH * 2
            + std::mem::size_of::<u16>()
            + std::mem::size_of::<ForceId>()
            + std::mem::size_of::<u8>()
            + std::mem::size_of::<ObjectType>()
            + std::mem::size_of::<u32>()
            + std::mem::size_of::<ObjectStateAppearanceGeneral>()
            + std::mem::size_of::<u16>()
            + SimulationAddress::LENGTH * 2;

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
        buf.put_u32(self.specific_object_appearance);
        buf.put_u16(self.general_object_appearance.as_u16());
        buf.put_u16(self.number_of_points);
        self.requester_id.serialize(buf);
        self.receiving_id.serialize(buf);
        for i in 0..self.object_location.len() {
            self.object_location[i].serialize(buf);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::ArealObjectState {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type ArealObjectState, got {:?}",
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

impl ArealObjectStatePdu {
    /// Creates a new `ArealObjectStatePdu`
    ///
    /// # Examples
    ///
    /// Initializing an `ArealObjectStatePdu`:
    /// ```
    /// use open_dis_rust::synthetic_environment::ArealObjectStatePdu;
    /// let mut pdu = ArealObjectStatePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::ArealObjectState;
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
        let specific_object_appearance = buf.get_u32();
        let general_object_appearance =
            ObjectStateAppearanceGeneral::from_u16(buf.get_u16()).unwrap();
        let number_of_points = buf.get_u16();
        let requester_id = SimulationAddress::deserialize(buf);
        let receiving_id = SimulationAddress::deserialize(buf);
        let mut object_location: Vec<WorldCoordinate> = vec![];
        for _i in 0..number_of_points {
            object_location.push(WorldCoordinate::deserialize(buf));
        }

        ArealObjectStatePdu {
            pdu_header: PduHeader::default(),
            object_id,
            referenced_object_id,
            update_number,
            force_id,
            modifications,
            object_type,
            specific_object_appearance,
            general_object_appearance,
            number_of_points,
            requester_id,
            receiving_id,
            object_location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ArealObjectStatePdu;
    use crate::common::pdu::Pdu;
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = ArealObjectStatePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<ArealObjectStatePdu>());
    }

    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = ArealObjectStatePdu::new();
        let mut serialize_buf = BytesMut::new();
        let _ = pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
        let new_pdu = ArealObjectStatePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 384 / 8;
        let pdu = ArealObjectStatePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
