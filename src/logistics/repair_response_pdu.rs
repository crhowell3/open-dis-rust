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
    enums::RepairResponseRepairResult,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Copy, Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.4.7
pub struct RepairResponsePdu {
    pub pdu_header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub repairing_entity_id: EntityId,
    pub repair_result: RepairResponseRepairResult,
    _padding: u8,
    _padding2: u16,
}

impl Default for RepairResponsePdu {
    /// Creates a default Repair Response PDU with arbitrary receiving and repairing entity IDs
    ///
    /// # Examples
    ///
    /// Initializing a Repair Response PDU:
    /// ```
    /// use open_dis_rust::logistics::repair_response_pdu::RepairResponsePdu;
    /// let repair_response_pdu = RepairResponsePdu::default();
    /// ```
    ///
    fn default() -> Self {
        RepairResponsePdu {
            pdu_header: PduHeader::default(PduType::RepairResponse, ProtocolFamily::Logistics, 28),
            receiving_entity_id: EntityId::default(1),
            repairing_entity_id: EntityId::default(2),
            repair_result: RepairResponseRepairResult::default(),
            _padding: 0_u8,
            _padding2: 0_u16,
        }
    }
}

impl Pdu for RepairResponsePdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        self.repairing_entity_id.serialize(buf);
        buf.put_u8(self.repair_result as u8);
        buf.put_u8(self._padding);
        buf.put_u16(self._padding2);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::RepairResponse {
            let receiving_entity_id = EntityId::deserialize(&mut buffer);
            let repairing_entity_id = EntityId::deserialize(&mut buffer);
            let repair_result = RepairResponseRepairResult::deserialize(&mut buffer);
            let _padding = buffer.get_u8();
            let _padding2 = buffer.get_u16();

            Ok(RepairResponsePdu {
                pdu_header,
                receiving_entity_id,
                repairing_entity_id,
                repair_result,
                _padding,
                _padding2,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type RepairResponse, got {:?}",
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
        let receiving_entity_id = EntityId::deserialize(&mut buffer);
        let repairing_entity_id = EntityId::deserialize(&mut buffer);
        let repair_result = RepairResponseRepairResult::deserialize(&mut buffer);
        let _padding = buffer.get_u8();
        let _padding2 = buffer.get_u16();

        Ok(RepairResponsePdu {
            pdu_header,
            receiving_entity_id,
            repairing_entity_id,
            repair_result,
            _padding,
            _padding2,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::RepairResponsePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let repair_response_pdu = RepairResponsePdu::default();
        let pdu_header = PduHeader::default(
            PduType::RepairResponse,
            ProtocolFamily::Logistics,
            u16::try_from(std::mem::size_of_val(&repair_response_pdu)).expect("size of pdu"),
        );

        assert_eq!(
            pdu_header.protocol_version,
            repair_response_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            repair_response_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, repair_response_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            repair_response_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, repair_response_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.status_record,
            repair_response_pdu.pdu_header.status_record
        );
    }

    #[test]
    fn deserialize_header() {
        let mut repair_response_pdu = RepairResponsePdu::default();
        let mut buffer = BytesMut::new();
        repair_response_pdu.serialize(&mut buffer);

        let new_repair_response_pdu = RepairResponsePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_repair_response_pdu.pdu_header,
            repair_response_pdu.pdu_header
        );
    }
}
