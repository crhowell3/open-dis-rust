use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Copy, Clone, Debug)]
pub struct EnvironmentalProcessPdu {
    pub pdu_header: PduHeader,
    pub environmental_process_id: EntityId,
    pub environment_type: EntityType,
    pub model_type: u8,
    pub environment_status: u8,
    pub number_of_environment_records: u8,
    pub sequence_number: u8,
    pub environment_records: Vec<Environment>,
}
