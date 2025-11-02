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
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    simulation_address::SimulationAddress,
};

use super::data_types::{
    linear_segment_parameter::LinearSegmentParameter, object_type::ObjectType,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.10.5
pub struct LinearObjectStatePdu {
    pub pdu_header: PduHeader,
    pub object_id: EntityId,
    pub referenced_object_id: EntityId,
    pub update_number: u16,
    pub force_id: u8,
    pub number_of_segments: u8,
    pub requester_id: SimulationAddress,
    pub receiving_id: SimulationAddress,
    pub object_type: ObjectType,
    pub linear_segment_parameters: Vec<LinearSegmentParameter>,
}

impl Default for LinearObjectStatePdu {
    /// Creates a default Linear Object State PDU with arbitrary environmental process ID
    ///
    /// # Examples
    ///
    /// Initializing an Linear Object State PDU:
    /// ```
    /// use open_dis_rust::synthetic_environment::linear_object_state_pdu::LinearObjectStatePdu;
    /// let linear_object_state_pdu = LinearObjectStatePdu::default();
    /// ```
    ///
    fn default() -> Self {
        LinearObjectStatePdu {
            pdu_header: PduHeader::default(
                PduType::LinearObjectState,
                ProtocolFamily::SyntheticEnvironment,
                56,
            ),
            object_id: EntityId::default(1),
            referenced_object_id: EntityId::default(2),
            update_number: 0,
            force_id: 0,
            number_of_segments: 0,
            requester_id: SimulationAddress::default(),
            receiving_id: SimulationAddress::default(),
            object_type: ObjectType::default(),
            linear_segment_parameters: vec![],
        }
    }
}

impl Pdu for LinearObjectStatePdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.object_id.serialize(buf);
        self.referenced_object_id.serialize(buf);
        buf.put_u16(self.update_number);
        buf.put_u8(self.force_id);
        buf.put_u8(self.number_of_segments);
        self.requester_id.serialize(buf);
        self.receiving_id.serialize(buf);
        self.object_type.serialize(buf);
        for i in 0..self.linear_segment_parameters.len() {
            self.linear_segment_parameters[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::LinearObjectState {
            let object_id = EntityId::deserialize(&mut buffer);
            let referenced_object_id = EntityId::deserialize(&mut buffer);
            let update_number = buffer.get_u16();
            let force_id = buffer.get_u8();
            let number_of_segments = buffer.get_u8();
            let requester_id = SimulationAddress::deserialize(&mut buffer);
            let receiving_id = SimulationAddress::deserialize(&mut buffer);
            let object_type = ObjectType::deserialize(&mut buffer);
            let mut linear_segment_parameters: Vec<LinearSegmentParameter> = vec![];
            for _i in 0..number_of_segments {
                linear_segment_parameters.push(LinearSegmentParameter::deserialize(&mut buffer));
            }
            Ok(LinearObjectStatePdu {
                pdu_header,
                object_id,
                referenced_object_id,
                update_number,
                force_id,
                number_of_segments,
                requester_id,
                receiving_id,
                object_type,
                linear_segment_parameters,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type LinearObjectState, got {:?}",
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
        let number_of_segments = buffer.get_u8();
        let requester_id = SimulationAddress::deserialize(&mut buffer);
        let receiving_id = SimulationAddress::deserialize(&mut buffer);
        let object_type = ObjectType::deserialize(&mut buffer);
        let mut linear_segment_parameters: Vec<LinearSegmentParameter> = vec![];
        for _i in 0..number_of_segments {
            linear_segment_parameters.push(LinearSegmentParameter::deserialize(&mut buffer));
        }
        Ok(LinearObjectStatePdu {
            pdu_header,
            object_id,
            referenced_object_id,
            update_number,
            force_id,
            number_of_segments,
            requester_id,
            receiving_id,
            object_type,
            linear_segment_parameters,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::LinearObjectStatePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let linear_object_state_pdu = LinearObjectStatePdu::default();
        let pdu_header = PduHeader::default(
            PduType::LinearObjectState,
            ProtocolFamily::SyntheticEnvironment,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            linear_object_state_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            linear_object_state_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            linear_object_state_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            linear_object_state_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, linear_object_state_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.padding,
            linear_object_state_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let mut linear_object_state_pdu = LinearObjectStatePdu::default();
        let mut buffer = BytesMut::new();
        linear_object_state_pdu.serialize(&mut buffer);

        let new_linear_object_state_pdu = LinearObjectStatePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_linear_object_state_pdu.pdu_header,
            linear_object_state_pdu.pdu_header
        );
    }
}
