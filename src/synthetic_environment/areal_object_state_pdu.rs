//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    entity_type::EntityType,
    enums::ObjectStateAppearanceGeneral,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    simulation_address::SimulationAddress,
    vector3_double::Vector3Double,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.10.6
pub struct ArealObjectStatePdu {
    pub pdu_header: PduHeader,
    pub object_id: EntityId,
    pub referenced_object_id: EntityId,
    pub update_number: u16,
    pub force_id: u8,
    pub modifications: u8,
    pub object_type: EntityType,
    pub specific_object_appearance: u32, // TODO(@anyone) Implement Specific Object Appearance
    pub general_object_appearance: ObjectStateAppearanceGeneral,
    pub number_of_points: u16,
    pub requester_id: SimulationAddress,
    pub receiving_id: SimulationAddress,
    pub object_location: Vec<Vector3Double>,
}

impl Default for ArealObjectStatePdu {
    /// Creates a default Areal Object State PDU with arbitrary environmental process ID
    ///
    /// # Examples
    ///
    /// Initializing an Areal Object State PDU:
    /// ```
    /// use open_dis_rust::synthetic_environment::areal_object_state_pdu::ArealObjectStatePdu;
    /// let areal_object_state_pdu = ArealObjectStatePdu::default();
    /// ```
    ///
    fn default() -> Self {
        ArealObjectStatePdu {
            pdu_header: PduHeader::default(
                PduType::ArealObjectState,
                ProtocolFamily::SyntheticEnvironment,
                56,
            ),
            object_id: EntityId::default(1),
            referenced_object_id: EntityId::default(2),
            update_number: 0,
            force_id: 0,
            modifications: 0,
            object_type: EntityType::default(),
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
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.object_id.serialize(buf);
        self.referenced_object_id.serialize(buf);
        buf.put_u16(self.update_number);
        buf.put_u8(self.force_id);
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
            let force_id = buffer.get_u8();
            let modifications = buffer.get_u8();
            let object_type = EntityType::deserialize(&mut buffer);
            let specific_object_appearance = buffer.get_u32();
            let general_object_appearance =
                ObjectStateAppearanceGeneral::from_u16(buffer.get_u16()).unwrap();
            let number_of_points = buffer.get_u16();
            let requester_id = SimulationAddress::deserialize(&mut buffer);
            let receiving_id = SimulationAddress::deserialize(&mut buffer);
            let mut object_location: Vec<Vector3Double> = vec![];
            for _i in 0..number_of_points {
                object_location.push(Vector3Double::deserialize(&mut buffer));
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
        let force_id = buffer.get_u8();
        let modifications = buffer.get_u8();
        let object_type = EntityType::deserialize(&mut buffer);
        let specific_object_appearance = buffer.get_u32();
        let general_object_appearance =
            ObjectStateAppearanceGeneral::from_u16(buffer.get_u16()).unwrap();
        let number_of_points = buffer.get_u16();
        let requester_id = SimulationAddress::deserialize(&mut buffer);
        let receiving_id = SimulationAddress::deserialize(&mut buffer);
        let mut object_location: Vec<Vector3Double> = vec![];
        for _i in 0..number_of_points {
            object_location.push(Vector3Double::deserialize(&mut buffer));
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

#[cfg(test)]
mod tests {
    use super::ArealObjectStatePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let areal_object_state_pdu = ArealObjectStatePdu::default();
        let pdu_header = PduHeader::default(
            PduType::ArealObjectState,
            ProtocolFamily::SyntheticEnvironment,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            areal_object_state_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            areal_object_state_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            areal_object_state_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            areal_object_state_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, areal_object_state_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.padding,
            areal_object_state_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let mut areal_object_state_pdu = ArealObjectStatePdu::default();
        let mut buffer = BytesMut::new();
        areal_object_state_pdu.serialize(&mut buffer);

        let new_areal_object_state_pdu = ArealObjectStatePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_areal_object_state_pdu.pdu_header,
            areal_object_state_pdu.pdu_header
        );
    }
}
