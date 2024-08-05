use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    event_id::EventId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    vector3_float::Vector3Float,
};

use super::data_types::{
    beam_data::BeamData, fundamental_operational_data::FundamentalOperationalData,
    iff_fundamental_parameter_data::IFFFundamentalParameterData, layer_header::LayerHeader,
    secondary_operational_data::SecondaryOperationalData, system_id::SystemId,
};

#[derive(Clone, Debug)]
pub struct IFFPdu {
    pub pdu_header: PduHeader,
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub relative_antenna_location: Vector3Float,
    pub number_of_iff_parameters: u32,
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
            pdu_header: PduHeader::default(
                PduType::IFF,
                ProtocolFamily::DistributedEmissionRegeneration,
                56,
            ),
            emitting_entity_id: EntityId::default(1),
            event_id: EventId::default(1),
            relative_antenna_location: Vector3Float::default(),
            number_of_iff_parameters: 0,
            system_id: SystemId::default(),
            system_designator: 0,
            system_specific_data: 0,
            fundamental_operational_data: FundamentalOperationalData::default(),
            layer_header: LayerHeader::default(),
            beam_data: BeamData::default(),
            secondary_operational_data: SecondaryOperationalData::default(),
            iff_parameters: vec![],
        }
    }
}

impl Pdu for IFFPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.emitting_entity_id.serialize(buf);
        self.event_id.serialize(buf);
        self.relative_antenna_location.serialize(buf);
        buf.put_u32(self.number_of_iff_parameters);
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
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::IFF {
            let emitting_entity_id = EntityId::decode(&mut buffer);
            let event_id = EventId::decode(&mut buffer);
            let relative_antenna_location = Vector3Float::decode(&mut buffer);
            let number_of_iff_parameters = buffer.get_u32();
            let system_id = SystemId::decode(&mut buffer);
            let system_designator = buffer.get_u8();
            let system_specific_data = buffer.get_u8();
            let fundamental_operational_data = FundamentalOperationalData::decode(&mut buffer);
            let layer_header = LayerHeader::decode(&mut buffer);
            let beam_data = BeamData::decode(&mut buffer);
            let secondary_operational_data = SecondaryOperationalData::decode(&mut buffer);
            let mut iff_parameters: Vec<IFFFundamentalParameterData> = vec![];
            for _i in 0..number_of_iff_parameters {
                iff_parameters.push(IFFFundamentalParameterData::decode(&mut buffer));
            }
            Ok(IFFPdu {
                pdu_header,
                emitting_entity_id,
                event_id,
                relative_antenna_location,
                number_of_iff_parameters,
                system_id,
                system_designator,
                system_specific_data,
                fundamental_operational_data,
                layer_header,
                beam_data,
                secondary_operational_data,
                iff_parameters,
            })
        } else {
            Err(DISError::InvalidDISHeader)
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
        let emitting_entity_id = EntityId::decode(&mut buffer);
        let event_id = EventId::decode(&mut buffer);
        let relative_antenna_location = Vector3Float::decode(&mut buffer);
        let number_of_iff_parameters = buffer.get_u32();
        let system_id = SystemId::decode(&mut buffer);
        let system_designator = buffer.get_u8();
        let system_specific_data = buffer.get_u8();
        let fundamental_operational_data = FundamentalOperationalData::decode(&mut buffer);
        let layer_header = LayerHeader::decode(&mut buffer);
        let beam_data = BeamData::decode(&mut buffer);
        let secondary_operational_data = SecondaryOperationalData::decode(&mut buffer);
        let mut iff_parameters: Vec<IFFFundamentalParameterData> = vec![];
        for _i in 0..number_of_iff_parameters {
            iff_parameters.push(IFFFundamentalParameterData::decode(&mut buffer));
        }
        Ok(IFFPdu {
            pdu_header,
            emitting_entity_id,
            event_id,
            relative_antenna_location,
            number_of_iff_parameters,
            system_id,
            system_designator,
            system_specific_data,
            fundamental_operational_data,
            layer_header,
            beam_data,
            secondary_operational_data,
            iff_parameters,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IFFPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let designator_pdu = IFFPdu::default();
        let pdu_header = PduHeader::default(
            PduType::IFF,
            ProtocolFamily::DistributedEmissionRegeneration,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            designator_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            designator_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, designator_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            designator_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, designator_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, designator_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut designator_pdu = IFFPdu::default();
        let mut buffer = BytesMut::new();
        designator_pdu.serialize(&mut buffer);

        let new_designator_pdu = IFFPdu::deserialize(buffer).unwrap();
        assert_eq!(new_designator_pdu.pdu_header, designator_pdu.pdu_header);
    }
}
