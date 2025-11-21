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
    entity_type::EntityType,
    enums::{PduType, ProtocolFamily},
    pdu::Pdu,
    pdu_header::PduHeader,
};

use super::data_types::environment::Environment;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.10.2
pub struct EnvironmentalProcessPdu {
    pdu_header: PduHeader,
    pub environmental_process_id: EntityId,
    pub environment_type: EntityType,
    pub model_type: u8,
    pub environment_status: u8,
    pub number_of_environment_records: u8,
    pub sequence_number: u8,
    pub environment_records: Vec<Environment>,
}

impl Default for EnvironmentalProcessPdu {
    fn default() -> Self {
        EnvironmentalProcessPdu {
            pdu_header: PduHeader::default(),
            environmental_process_id: EntityId::default(1),
            environment_type: EntityType::default(),
            model_type: 0,
            environment_status: 0,
            number_of_environment_records: 0,
            sequence_number: 0,
            environment_records: vec![],
        }
    }
}

impl Pdu for EnvironmentalProcessPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>()
            + std::mem::size_of::<EntityType>()
            + std::mem::size_of::<u8>() * 4;

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
        self.environmental_process_id.serialize(buf);
        self.environment_type.serialize(buf);
        buf.put_u8(self.model_type);
        buf.put_u8(self.environment_status);
        buf.put_u8(self.number_of_environment_records);
        buf.put_u8(self.sequence_number);
        for i in 0..self.environment_records.len() {
            self.environment_records[i].serialize(buf);
        }
        Ok(())
    }

    fn deserialize<B: Buf>(buf: &mut B) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let header: PduHeader = PduHeader::deserialize(buf);
        if header.pdu_type != PduType::DirectedEnergyFire {
            return Err(DISError::invalid_header(
                format!(
                    "Expected PDU type DirectedEnergyFire, got {:?}",
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

impl EnvironmentalProcessPdu {
    /// Creates a new `EnvironmentalProcessPdu`
    ///
    /// # Examples
    ///
    /// Initializing an `EnvironmentalProcessPdu`:
    /// ```
    /// use open_dis_rust::synthetic_environment::EnvironmentalProcessPdu;
    /// let pdu = EnvironmentalProcessPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::EnvironmentalProcess;
        pdu.pdu_header.protocol_family = ProtocolFamily::SyntheticEnvironment;
        pdu.finalize();
        pdu
    }

    fn deserialize_body<B: Buf>(buf: &mut B) -> Self {
        let environmental_process_id = EntityId::deserialize(buf);
        let environment_type = EntityType::deserialize(buf);
        let model_type = buf.get_u8();
        let environment_status = buf.get_u8();
        let number_of_environment_records = buf.get_u8();
        let sequence_number = buf.get_u8();
        let mut environment_records: Vec<Environment> = vec![];
        for _i in 0..number_of_environment_records {
            environment_records.push(Environment::deserialize(buf));
        }

        EnvironmentalProcessPdu {
            pdu_header: PduHeader::default(),
            environmental_process_id,
            environment_type,
            model_type,
            environment_status,
            number_of_environment_records,
            sequence_number,
            environment_records,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EnvironmentalProcessPdu;
    use crate::common::{pdu::Pdu, pdu_header::PduHeader};
    use bytes::{Bytes, BytesMut};

    #[test]
    fn cast_to_any() {
        let pdu = EnvironmentalProcessPdu::new();
        let any_pdu = pdu.as_any();

        assert!(any_pdu.is::<EnvironmentalProcessPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut pdu = EnvironmentalProcessPdu::new();
        let mut serialize_buffer = BytesMut::new();
        pdu.serialize(&mut serialize_buffer);

        let mut deserialize_buffer = Bytes::new();
        let new_pdu = EnvironmentalProcessPdu::deserialize(&mut deserialize_buffer).unwrap();
        assert_eq!(new_pdu.pdu_header, pdu.pdu_header);
    }

    #[test]
    fn check_default_pdu_length() {
        const DEFAULT_LENGTH: u16 = 256 / 8;
        let pdu = EnvironmentalProcessPdu::new();
        assert_eq!(pdu.header().length, DEFAULT_LENGTH);
    }
}
