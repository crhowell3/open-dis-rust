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
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Copy, Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.5.7
pub struct ActionRequestPdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub action_id: u32,
    pub number_of_fixed_datum_records: u32,
    pub number_of_variable_datum_records: u32,
    pub fixed_datum_records: u64,
    pub variable_datum_records: u64,
}

impl Default for ActionRequestPdu {
    /// Creates a default-initialized Action Request PDU
    ///
    /// # Examples
    ///
    /// Initializing an Action Request PDU:
    /// ```
    /// use open_dis_rust::simulation_management::action_request_pdu::ActionRequestPdu;
    /// let mut action_request_pdu = ActionRequestPdu::default();
    /// ```
    ///
    fn default() -> Self {
        ActionRequestPdu {
            pdu_header: PduHeader::default(
                PduType::ActionRequest,
                ProtocolFamily::SimulationManagement,
                56,
            ),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            request_id: 0,
            action_id: 0,
            number_of_fixed_datum_records: 0,
            number_of_variable_datum_records: 0,
            fixed_datum_records: 0,
            variable_datum_records: 0,
        }
    }
}

impl Pdu for ActionRequestPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u32(self.request_id);
        buf.put_u32(self.action_id);
        buf.put_u32(self.number_of_fixed_datum_records);
        buf.put_u32(self.number_of_variable_datum_records);
        buf.put_u64(self.fixed_datum_records);
        buf.put_u64(self.variable_datum_records);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::ActionRequest {
            let originating_entity_id = EntityId::deserialize(&mut buffer);
            let receiving_entity_id = EntityId::deserialize(&mut buffer);
            let request_id = buffer.get_u32();
            let action_id = buffer.get_u32();
            let number_of_fixed_datum_records = buffer.get_u32();
            let number_of_variable_datum_records = buffer.get_u32();
            let mut fixed_datum_records: u64 = 0;
            for _record in 0..number_of_fixed_datum_records as usize {
                fixed_datum_records += buffer.get_u64();
            }
            let mut variable_datum_records: u64 = 0;
            for _record in 0..number_of_variable_datum_records as usize {
                variable_datum_records += buffer.get_u64();
            }

            Ok(ActionRequestPdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                request_id,
                action_id,
                number_of_fixed_datum_records,
                number_of_variable_datum_records,
                fixed_datum_records,
                variable_datum_records,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type ActionRequest, got {:?}",
                    pdu_header.pdu_type
                ),
                None,
            ))
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
        let originating_entity_id = EntityId::deserialize(&mut buffer);
        let receiving_entity_id = EntityId::deserialize(&mut buffer);
        let request_id = buffer.get_u32();
        let action_id = buffer.get_u32();
        let number_of_fixed_datum_records = buffer.get_u32();
        let number_of_variable_datum_records = buffer.get_u32();
        let mut fixed_datum_records: u64 = 0;
        for _record in 0..number_of_fixed_datum_records as usize {
            fixed_datum_records += buffer.get_u64();
        }
        let mut variable_datum_records: u64 = 0;
        for _record in 0..number_of_variable_datum_records as usize {
            variable_datum_records += buffer.get_u64();
        }

        Ok(ActionRequestPdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            request_id,
            action_id,
            number_of_fixed_datum_records,
            number_of_variable_datum_records,
            fixed_datum_records,
            variable_datum_records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ActionRequestPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let action_request_pdu = ActionRequestPdu::default();
        let pdu_header = PduHeader::default(
            PduType::ActionRequest,
            ProtocolFamily::SimulationManagement,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            action_request_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            action_request_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, action_request_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            action_request_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, action_request_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.status_record,
            action_request_pdu.pdu_header.status_record
        );
    }

    #[test]
    fn cast_to_any() {
        let action_request_pdu = ActionRequestPdu::default();
        let any_pdu = action_request_pdu.as_any();

        assert!(any_pdu.is::<ActionRequestPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut action_request_pdu = ActionRequestPdu::default();
        let mut buffer = BytesMut::new();
        action_request_pdu.serialize(&mut buffer);

        let new_action_request_pdu = ActionRequestPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_action_request_pdu.pdu_header,
            action_request_pdu.pdu_header
        );
    }
}
