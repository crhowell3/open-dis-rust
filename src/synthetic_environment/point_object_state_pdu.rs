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
    euler_angles::EulerAngles,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    simulation_address::SimulationAddress,
    vector3_double::Vector3Double,
};

use super::data_types::object_type::ObjectType;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.10.4
pub struct PointObjectStatePdu {
    pub pdu_header: PduHeader,
    pub object_id: EntityId,
    pub referenced_object_id: EntityId,
    pub update_number: u16,
    pub force_id: u8,
    pub modifications: u8,
    pub object_type: ObjectType,
    pub object_location: Vector3Double,
    pub object_orientation: EulerAngles,
    pub object_appearance: f64,
    pub requester_id: SimulationAddress,
    pub receiving_id: SimulationAddress,
    pub pad2: u32,
}

impl Default for PointObjectStatePdu {
    /// Creates a default Point Object State PDU with arbitrary environmental process ID
    ///
    /// # Examples
    ///
    /// Initializing a Point Object State PDU:
    /// ```
    /// use open_dis_rust::synthetic_environment::point_object_state_pdu::PointObjectStatePdu;
    /// let point_object_state_pdu = PointObjectStatePdu::default();
    /// ```
    ///
    fn default() -> Self {
        PointObjectStatePdu {
            pdu_header: PduHeader::default(
                PduType::PointObjectState,
                ProtocolFamily::SyntheticEnvironment,
                56,
            ),
            object_id: EntityId::default(1),
            referenced_object_id: EntityId::default(2),
            update_number: 0,
            force_id: 0,
            modifications: 0,
            object_type: ObjectType::default(),
            object_location: Vector3Double::default(),
            object_orientation: EulerAngles::default(),
            object_appearance: 0.0,
            requester_id: SimulationAddress::default(),
            receiving_id: SimulationAddress::default(),
            pad2: 0,
        }
    }
}

impl Pdu for PointObjectStatePdu {
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
        self.object_location.serialize(buf);
        self.object_orientation.serialize(buf);
        buf.put_f64(self.object_appearance);
        self.requester_id.serialize(buf);
        self.receiving_id.serialize(buf);
        buf.put_u32(self.pad2);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::PointObjectState {
            let object_id = EntityId::deserialize(&mut buffer);
            let referenced_object_id = EntityId::deserialize(&mut buffer);
            let update_number = buffer.get_u16();
            let force_id = buffer.get_u8();
            let modifications = buffer.get_u8();
            let object_type = ObjectType::deserialize(&mut buffer);
            let object_location = Vector3Double::deserialize(&mut buffer);
            let object_orientation = EulerAngles::deserialize(&mut buffer);
            let object_appearance = buffer.get_f64();
            let requester_id = SimulationAddress::deserialize(&mut buffer);
            let receiving_id = SimulationAddress::deserialize(&mut buffer);
            let pad2 = buffer.get_u32();
            Ok(PointObjectStatePdu {
                pdu_header,
                object_id,
                referenced_object_id,
                update_number,
                force_id,
                modifications,
                object_type,
                object_location,
                object_orientation,
                object_appearance,
                requester_id,
                receiving_id,
                pad2,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type PointObjectState, got {:?}",
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
        let object_type = ObjectType::deserialize(&mut buffer);
        let object_location = Vector3Double::deserialize(&mut buffer);
        let object_orientation = EulerAngles::deserialize(&mut buffer);
        let object_appearance = buffer.get_f64();
        let requester_id = SimulationAddress::deserialize(&mut buffer);
        let receiving_id = SimulationAddress::deserialize(&mut buffer);
        let pad2 = buffer.get_u32();
        Ok(PointObjectStatePdu {
            pdu_header,
            object_id,
            referenced_object_id,
            update_number,
            force_id,
            modifications,
            object_type,
            object_location,
            object_orientation,
            object_appearance,
            requester_id,
            receiving_id,
            pad2,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PointObjectStatePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let point_object_state_pdu = PointObjectStatePdu::default();
        let pdu_header = PduHeader::default(
            PduType::PointObjectState,
            ProtocolFamily::SyntheticEnvironment,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            point_object_state_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            point_object_state_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            point_object_state_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            point_object_state_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, point_object_state_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.padding,
            point_object_state_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let mut point_object_state_pdu = PointObjectStatePdu::default();
        let mut buffer = BytesMut::new();
        point_object_state_pdu.serialize(&mut buffer);

        let new_point_object_state_pdu = PointObjectStatePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_point_object_state_pdu.pdu_header,
            point_object_state_pdu.pdu_header
        );
    }
}
