//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    EntityCoordinateVector,
    constants::MAX_PDU_SIZE_OCTETS,
    dis_error::DISError,
    entity_id::EntityId,
    enums::{PduType, ProtocolFamily},
    event_id::EventId,
    pdu::Pdu,
    pdu_header::PduHeader,
};

use super::data_types::{
    beam_data::BeamData, fundamental_operational_data::FundamentalOperationalData,
    iff_fundamental_parameter_data::IFFFundamentalParameterData, layer_header::LayerHeader,
    secondary_operational_data::SecondaryOperationalData, system_id::SystemId,
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.6.5
pub struct IFFPdu {
    pdu_header: PduHeader,
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub relative_antenna_location: EntityCoordinateVector,
    pub system_id: SystemId,
    pub system_designator: u8,
    pub system_specific_data: u8,
    pub fundamental_operational_data: FundamentalOperationalData,
    pub layer_header: LayerHeader,
    pub beam_data: BeamData,
    pub secondary_operational_data: SecondaryOperationalData,
    pub iff_parameters: Vec<IFFFundamentalParameterData>,
}

impl Default for IFFPdu {
    fn default() -> Self {
        IFFPdu {
            pdu_header: PduHeader::default(),
            emitting_entity_id: EntityId::default(1),
            event_id: EventId::default(1),
            relative_antenna_location: EntityCoordinateVector::default(),
            system_id: SystemId::default(),
            system_designator: 0u8,
            system_specific_data: 0u8,
            fundamental_operational_data: FundamentalOperationalData::default(),
            layer_header: LayerHeader::default(),
            beam_data: BeamData::default(),
            secondary_operational_data: SecondaryOperationalData::default(),
            iff_parameters: vec![],
        }
    }
}

impl Pdu for IFFPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>()
            + std::mem::size_of::<EventId>()
            + std::mem::size_of::<EntityCoordinateVector>()
            + std::mem::size_of::<SystemId>()
            + std::mem::size_of::<u8>() * 2
            + std::mem::size_of::<FundamentalOperationalData>()
            + std::mem::size_of::<LayerHeader>()
            + std::mem::size_of::<BeamData>()
            + std::mem::size_of::<SecondaryOperationalData>();

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    /// Serialize contents of `IFFPdu` into `BytesMut` buf
    fn serialize(&mut self, buf: &mut BytesMut) -> Result<(), DISError> {
        let size = std::mem::size_of_val(self);
        self.pdu_header.length = u16::try_from(size).map_err(|_| DISError::PduSizeExceeded {
            size,
            max_size: MAX_PDU_SIZE_OCTETS,
        })?;
        self.pdu_header.serialize(buf);
        self.emitting_entity_id.serialize(buf);
        self.event_id.serialize(buf);
        self.relative_antenna_location.serialize(buf);
        self.system_id.serialize(buf);
        buf.put_u8(self.system_designator);
        buf.put_u8(self.system_specific_data);
        self.fundamental_operational_data.serialize(buf);
        self.layer_header.serialize(buf);
        self.beam_data.serialize(buf);
        self.secondary_operational_data.serialize(buf);
        for i in 0..self.iff_parameters.len() {
            self.iff_parameters[i].serialize(buf);
        }
        Ok(())
    }

    /// Deserialize bytes from `BytesMut` buf and interpret as `IFFPdu`
    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::IFF {
            return Err(DISError::invalid_header(
                format!("Expected PDU type IFF, got {:?}", header.pdu_type),
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

impl IFFPdu {
    /// Creates a new `IFFPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `IFFPdu`:
    /// ```
    /// use open_dis_rust::distributed_emissions::IFFPdu;
    /// let pdu = IFFPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::IFF;
        pdu.pdu_header.protocol_family = ProtocolFamily::DistributedEmissionRegeneration;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let emitting_entity_id = EntityId::deserialize(buf);
        let event_id = EventId::deserialize(buf);
        let relative_antenna_location = EntityCoordinateVector::deserialize(buf);
        let system_id = SystemId::deserialize(buf);
        let system_designator = buf.get_u8();
        let system_specific_data = buf.get_u8();
        let fundamental_operational_data = FundamentalOperationalData::deserialize(buf);
        let layer_header = LayerHeader::deserialize(buf);
        let beam_data = BeamData::deserialize(buf);
        let secondary_operational_data = SecondaryOperationalData::deserialize(buf);
        let mut iff_parameters: Vec<IFFFundamentalParameterData> = vec![];
        for _i in 0..secondary_operational_data.number_of_iff_fundamental_parameter_records {
            iff_parameters.push(IFFFundamentalParameterData::deserialize(buf));
        }

        IFFPdu {
            pdu_header: PduHeader::default(),
            emitting_entity_id,
            event_id,
            relative_antenna_location,
            system_id,
            system_designator,
            system_specific_data,
            fundamental_operational_data,
            layer_header,
            beam_data,
            secondary_operational_data,
            iff_parameters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IFFPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn create_header() {
        let pdu = IFFPdu::new();
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
        let pdu = IFFPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<IFFPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = IFFPdu::new();
        let mut serialize_buf = BytesMut::new();
        pdu.serialize(&mut serialize_buf);

        let mut deserialize_buf = Bytes::new();
        let new_pdu = IFFPdu::deserialize(&mut deserialize_buf).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 604 / 8;
        let pdu = IFFPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
