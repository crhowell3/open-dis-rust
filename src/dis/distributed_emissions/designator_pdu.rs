use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::dis::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

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
    pub dead_reckoning_algorithm: i8,
    pub padding1: u8,
    pub padding2: i8,
    pub entity_linear_acceleration: Vector3Float,
}

impl DesignatorPdu {}

impl Pdu for DesignatorPdu {}
