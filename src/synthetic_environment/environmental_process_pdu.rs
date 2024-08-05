//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    entity_type::EntityType,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

use super::environment::Environment;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §5.11.2.2
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

impl Default for EnvironmentalProcessPdu {
    /// Creates a default Environmental Process PDU with arbitrary environmental process ID
    ///
    /// # Examples
    ///
    /// Initializing an Environmental Process PDU:
    /// ```
    /// use open_dis_rust::synthetic_environment::environmental_process_pdu::EnvironmentalProcessPdu;
    /// let environmental_process_pdu = EnvironmentalProcessPdu::default();
    /// ```
    ///
    fn default() -> Self {
        EnvironmentalProcessPdu {
            pdu_header: PduHeader::default(
                PduType::EnvironmentalProcess,
                ProtocolFamily::SyntheticEnvironment,
                56,
            ),
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
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
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
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::EnvironmentalProcess {
            let environmental_process_id = EntityId::decode(&mut buffer);
            let environment_type = EntityType::decode(&mut buffer);
            let model_type = buffer.get_u8();
            let environment_status = buffer.get_u8();
            let number_of_environment_records = buffer.get_u8();
            let sequence_number = buffer.get_u8();
            let mut environment_records: Vec<Environment> = vec![];
            for _i in 0..number_of_environment_records {
                environment_records.push(Environment::decode(&mut buffer));
            }
            Ok(EnvironmentalProcessPdu {
                pdu_header,
                environmental_process_id,
                environment_type,
                model_type,
                environment_status,
                number_of_environment_records,
                sequence_number,
                environment_records,
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
        let environmental_process_id = EntityId::decode(&mut buffer);
        let environment_type = EntityType::decode(&mut buffer);
        let model_type = buffer.get_u8();
        let environment_status = buffer.get_u8();
        let number_of_environment_records = buffer.get_u8();
        let sequence_number = buffer.get_u8();
        let mut environment_records: Vec<Environment> = vec![];
        for _i in 0..number_of_environment_records {
            environment_records.push(Environment::decode(&mut buffer));
        }
        Ok(EnvironmentalProcessPdu {
            pdu_header,
            environmental_process_id,
            environment_type,
            model_type,
            environment_status,
            number_of_environment_records,
            sequence_number,
            environment_records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::EnvironmentalProcessPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let environmental_process_pdu = EnvironmentalProcessPdu::default();
        let pdu_header = PduHeader::default(
            PduType::EnvironmentalProcess,
            ProtocolFamily::SyntheticEnvironment,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            environmental_process_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            environmental_process_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            environmental_process_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            environmental_process_pdu.pdu_header.protocol_family
        );
        assert_eq!(
            pdu_header.length,
            environmental_process_pdu.pdu_header.length
        );
        assert_eq!(
            pdu_header.padding,
            environmental_process_pdu.pdu_header.padding
        );
    }

    #[test]
    fn deserialize_header() {
        let mut environmental_process_pdu = EnvironmentalProcessPdu::default();
        let mut buffer = BytesMut::new();
        environmental_process_pdu.serialize(&mut buffer);

        let new_environmental_process_pdu = EnvironmentalProcessPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_environmental_process_pdu.pdu_header,
            environmental_process_pdu.pdu_header
        );
    }
}
