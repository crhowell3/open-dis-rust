use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    vector3_double::Vector3Double,
    vector3_float::Vector3Float,
};

use crate::entity_information::dead_reckoning_parameters::DeadReckoningAlgorithm;

#[derive(Copy, Clone, Debug)]
pub struct DesignatorPdu {
    pub pdu_header: PduHeader,
    pub designating_entity_id: EntityId,
    pub code_name: u8,
    pub designated_entity_id: EntityId,
    pub designator_code: u8,
    pub designator_power: f32,
    pub designator_wavelength: f32,
    pub designator_spot_wrt_designated: Vector3Float,
    pub designator_spot_location: Vector3Double,
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    pub padding1: u8,
    pub padding2: i8,
    pub entity_linear_acceleration: Vector3Float,
}

impl Default for DesignatorPdu {
    fn default() -> Self {
        DesignatorPdu {
            pdu_header: PduHeader::default(
                PduType::Designator,
                ProtocolFamily::DistributedEmissionRegeneration,
                56,
            ),
            designating_entity_id: EntityId::default(1),
            code_name: 0,
            designated_entity_id: EntityId::default(2),
            designator_code: 0,
            designator_power: 0.0,
            designator_wavelength: 0.0,
            designator_spot_wrt_designated: Vector3Float::new(0.0, 0.0, 0.0),
            designator_spot_location: Vector3Double::new(0.0, 0.0, 0.0),
            dead_reckoning_algorithm: DeadReckoningAlgorithm::Other,
            padding1: 0,
            padding2: 0,
            entity_linear_acceleration: Vector3Float::new(0.0, 0.0, 0.0),
        }
    }
}

impl Pdu for DesignatorPdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.designating_entity_id.serialize(buf);
        buf.put_u8(self.code_name);
        self.designated_entity_id.serialize(buf);
        buf.put_u8(self.designator_code);
        buf.put_f32(self.designator_power);
        buf.put_f32(self.designator_wavelength);
        self.designator_spot_wrt_designated.serialize(buf);
        self.designator_spot_location.serialize(buf);
        buf.put_u8(self.dead_reckoning_algorithm as u8);
        buf.put_u8(self.padding1);
        buf.put_i8(self.padding2);
        self.entity_linear_acceleration.serialize(buf);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::Designator {
            let designating_entity_id = EntityId::decode(&mut buffer);
            let code_name = buffer.get_u8();
            let designated_entity_id = EntityId::decode(&mut buffer);
            let designator_code = buffer.get_u8();
            let designator_power = buffer.get_f32();
            let designator_wavelength = buffer.get_f32();
            let designator_spot_wrt_designated = Vector3Float::decode(&mut buffer);
            let designator_spot_location = Vector3Double::decode(&mut buffer);
            let dead_reckoning_algorithm = DeadReckoningAlgorithm::from_u8(buffer.get_u8());
            let padding1 = buffer.get_u8();
            let padding2 = buffer.get_i8();
            let entity_linear_acceleration = Vector3Float::decode(&mut buffer);

            Ok(DesignatorPdu {
                pdu_header,
                designating_entity_id,
                code_name,
                designated_entity_id,
                designator_code,
                designator_power,
                designator_wavelength,
                designator_spot_wrt_designated,
                designator_spot_location,
                dead_reckoning_algorithm,
                padding1,
                padding2,
                entity_linear_acceleration,
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
        let designating_entity_id = EntityId::decode(&mut buffer);
        let code_name = buffer.get_u8();
        let designated_entity_id = EntityId::decode(&mut buffer);
        let designator_code = buffer.get_u8();
        let designator_power = buffer.get_f32();
        let designator_wavelength = buffer.get_f32();
        let designator_spot_wrt_designated = Vector3Float::decode(&mut buffer);
        let designator_spot_location = Vector3Double::decode(&mut buffer);
        let dead_reckoning_algorithm = DeadReckoningAlgorithm::from_u8(buffer.get_u8());
        let padding1 = buffer.get_u8();
        let padding2 = buffer.get_i8();
        let entity_linear_acceleration = Vector3Float::decode(&mut buffer);

        Ok(DesignatorPdu {
            pdu_header,
            designating_entity_id,
            code_name,
            designated_entity_id,
            designator_code,
            designator_power,
            designator_wavelength,
            designator_spot_wrt_designated,
            designator_spot_location,
            dead_reckoning_algorithm,
            padding1,
            padding2,
            entity_linear_acceleration,
        })
    }
}
