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
    enums::{ForceId, PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
    simulation_address::SimulationAddress,
};

use super::data_types::{
    linear_segment_parameter::LinearSegmentParameter, object_type::ObjectType,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.10.5
pub struct LinearObjectStatePdu {
    pdu_header: PduHeader,
    pub object_id: EntityId,            // TODO(@anyone) Replace with ObjectId
    pub referenced_object_id: EntityId, // TODO(@anyone) Replace with ObjectId
    pub update_number: u16,
    pub force_id: ForceId,
    pub number_of_segments: u8,
    pub requester_id: SimulationAddress,
    pub receiving_id: SimulationAddress,
    pub object_type: ObjectType,
    pub linear_segment_parameters: Vec<LinearSegmentParameter>,
}

impl Default for LinearObjectStatePdu {
    fn default() -> Self {
        LinearObjectStatePdu {
            pdu_header: PduHeader::default(),
            object_id: EntityId::default(1),
            referenced_object_id: EntityId::default(2),
            update_number: 0,
            force_id: ForceId::default(),
            number_of_segments: 0,
            requester_id: SimulationAddress::default(),
            receiving_id: SimulationAddress::default(),
            object_type: ObjectType::default(),
            linear_segment_parameters: vec![],
        }
    }
}

impl Pdu for LinearObjectStatePdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>() * 2
            + std::mem::size_of::<u16>()
            + std::mem::size_of::<ForceId>()
            + std::mem::size_of::<u8>()
            + std::mem::size_of::<SimulationAddress>() * 2
            + std::mem::size_of::<ObjectType>();

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
        buf.put_u8(self.number_of_segments);
        self.requester_id.serialize(buf);
        self.receiving_id.serialize(buf);
        self.object_type.serialize(buf);
        for i in 0..self.linear_segment_parameters.len() {
            self.linear_segment_parameters[i].serialize(buf);
        }
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::LinearObjectState {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type LinearObjectState, got {:?}",
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

impl LinearObjectStatePdu {
    /// Creates a new `LinearObjectStatePdu`
    ///
    /// # Examples
    ///
    /// Initializing an `LinearObjectStatePdu`:
    /// ```
    /// use open_dis_rust::warfare::LinearObjectStatePdu;
    /// let pdu = LinearObjectStatePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::DirectedEnergyFire;
        pdu.pdu_header.protocol_family = ProtocolFamily::SyntheticEnvironment;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let object_id = EntityId::deserialize(buf);
        let referenced_object_id = EntityId::deserialize(buf);
        let update_number = buf.get_u16();
        let force_id = ForceId::deserialize(buf);
        let number_of_segments = buf.get_u8();
        let requester_id = SimulationAddress::deserialize(buf);
        let receiving_id = SimulationAddress::deserialize(buf);
        let object_type = ObjectType::deserialize(buf);
        let mut linear_segment_parameters: Vec<LinearSegmentParameter> = vec![];
        for _i in 0..number_of_segments {
            linear_segment_parameters.push(LinearSegmentParameter::deserialize(buf));
        }

        LinearObjectStatePdu {
            pdu_header: PduHeader::default(),
            object_id,
            referenced_object_id,
            update_number,
            force_id,
            number_of_segments,
            requester_id,
            receiving_id,
            object_type,
            linear_segment_parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinearObjectStatePdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn create_header() {
        let pdu = LinearObjectStatePdu::new();
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
        let pdu = LinearObjectStatePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<LinearObjectStatePdu>());
    }
    #[test]
    fn deserialize_header() {
        let mut pdu = LinearObjectStatePdu::default();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = LinearObjectStatePdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 320 / 8;
        let pdu = LinearObjectStatePdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
