use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    event_id::EventId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Copy, Clone, Debug)]
pub struct ElectromagneticEmissionsPdu {
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub state_update_indicator: u8,
    pub number_of_systems: u8,
    pub padding_for_emissions_pdu: u8,
    pub systems: Vec<ElectromagneticEmissionSystemData>,
}
