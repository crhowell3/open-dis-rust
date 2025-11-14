//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::{
    common::{
        WorldCoordinate,
        dis_error::DISError,
        entity_id::EntityId,
        enums::{ForceId, ObjectStateAppearanceGeneral},
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
        simulation_address::SimulationAddress,
    },
    synthetic_environment::data_types::object_type::ObjectType,
};

#[derive(Clone, Debug)]
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

impl Default for ArealObjectStatePdu {
    fn default() -> Self {
        ArealObjectStatePdu {
            pdu_header: PduHeader::default(),
            object_id: EntityId::default(1),
            referenced_object_id: EntityId::default(2),
            update_number: 0,
            force_id: ForceId::default(),
            modifications: 0,
            object_type: ObjectType::default(),
            specific_object_appearance: 0,
            general_object_appearance: ObjectStateAppearanceGeneral::default(),
            number_of_points: 0,
            requester_id: SimulationAddress::default(),
            receiving_id: SimulationAddress::default(),
            object_location: vec![],
        }
    }
}

impl Pdu for ArealObjectStatePdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>() * 2
            + std::mem::size_of::<u16>()
            + std::mem::size_of::<ForceId>()
            + std::mem::size_of::<u8>()
            + std::mem::size_of::<ObjectType>()
            + std::mem::size_of::<u32>()
            + std::mem::size_of::<ObjectStateAppearanceGeneral>()
            + std::mem::size_of::<u16>()
            + std::mem::size_of::<SimulationAddress>() * 2;

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
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
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::ArealObjectState {
            let object_id = EntityId::deserialize(&mut buffer);
            let referenced_object_id = EntityId::deserialize(&mut buffer);
            let update_number = buffer.get_u16();
            let force_id = ForceId::deserialize(&mut buffer);
            let modifications = buffer.get_u8();
            let object_type = ObjectType::deserialize(&mut buffer);
            let specific_object_appearance = buffer.get_u32();
            let general_object_appearance =
                ObjectStateAppearanceGeneral::from_u16(buffer.get_u16()).unwrap();
            let number_of_points = buffer.get_u16();
            let requester_id = SimulationAddress::deserialize(&mut buffer);
            let receiving_id = SimulationAddress::deserialize(&mut buffer);
            let mut object_location: Vec<WorldCoordinate> = vec![];
            for _i in 0..number_of_points {
                object_location.push(WorldCoordinate::deserialize(&mut buffer));
            }
            Ok(ArealObjectStatePdu {
                pdu_header,
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
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type ArealObjectState, got {:?}",
                    pdu_header.pdu_type
                ),
                None,
            ))
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn deserialize_without_header(
        mut buffer: BytesMut,
        pdu_header: PduHeader,
    ) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let object_id = EntityId::deserialize(&mut buffer);
        let referenced_object_id = EntityId::deserialize(&mut buffer);
        let update_number = buffer.get_u16();
        let force_id = ForceId::deserialize(&mut buffer);
        let modifications = buffer.get_u8();
        let object_type = ObjectType::deserialize(&mut buffer);
        let specific_object_appearance = buffer.get_u32();
        let general_object_appearance =
            ObjectStateAppearanceGeneral::from_u16(buffer.get_u16()).unwrap();
        let number_of_points = buffer.get_u16();
        let requester_id = SimulationAddress::deserialize(&mut buffer);
        let receiving_id = SimulationAddress::deserialize(&mut buffer);
        let mut object_location: Vec<WorldCoordinate> = vec![];
        for _i in 0..number_of_points {
            object_location.push(WorldCoordinate::deserialize(&mut buffer));
        }
        Ok(ArealObjectStatePdu {
            pdu_header,
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
        })
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
}

#[cfg(test)]
mod tests {
    use super::ArealObjectStatePdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let pdu = ArealObjectStatePdu::new();
        let pdu_header = PduHeader::default();

        assert_eq!(pdu_header.protocol_version, pdu.pdu_header.protocol_version);
        assert_eq!(pdu_header.exercise_id, pdu.pdu_header.exercise_id);
        assert_eq!(pdu_header.pdu_type, pdu.pdu_header.pdu_type);
        assert_eq!(pdu_header.protocol_family, pdu.pdu_header.protocol_family);
        assert_eq!(pdu_header.length, pdu.pdu_header.length);
        assert_eq!(pdu_header.status_record, pdu.pdu_header.status_record);
    }

    #[test]
    fn cast_to_any() {
        let pdu = ArealObjectStatePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<ArealObjectStatePdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut areal_object_state_pdu = ArealObjectStatePdu::new();
        let mut buffer = BytesMut::new();
        areal_object_state_pdu.serialize(&mut buffer);

        let new_areal_object_state_pdu = ArealObjectStatePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_areal_object_state_pdu.pdu_header,
            areal_object_state_pdu.pdu_header
        );
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 384 / 8;
        let pdu = ArealObjectStatePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
