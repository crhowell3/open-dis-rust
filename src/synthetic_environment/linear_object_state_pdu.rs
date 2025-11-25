//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    constants::MAX_PDU_SIZE_OCTETS,
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
            object_id: EntityId::default(),
            referenced_object_id: EntityId::default(),
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
        buf.put_u8(self.number_of_segments);
        self.requester_id.serialize(buf);
        self.receiving_id.serialize(buf);
        self.object_type.serialize(buf);
        for i in 0..self.linear_segment_parameters.len() {
            self.linear_segment_parameters[i].serialize(buf);
        }
        Ok(())
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
    /// use open_dis_rust::synthetic_environment::LinearObjectStatePdu;
    /// let pdu = LinearObjectStatePdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::LinearObjectState;
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
    use crate::common::pdu::Pdu;
    use bytes::BytesMut;

    #[test]
    fn cast_to_any() {
        let pdu = LinearObjectStatePdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<LinearObjectStatePdu>());
    }
    #[test]
    fn serialize_then_deserialize() {
        let mut pdu = LinearObjectStatePdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = serialize_buf.freeze();
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
