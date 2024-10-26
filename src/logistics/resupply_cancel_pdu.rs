//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use bytes::BytesMut;
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.4.5
pub struct ResupplyCancelPdu {
    pub pdu_header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub supplying_entity_id: EntityId,
}

impl Default for ResupplyCancelPdu {
    /// Creates a default Resupply Cancel PDU with arbitrary receiving and repairing entity IDs
    ///
    /// # Examples
    ///
    /// Initializing a Resupply Cancel PDU:
    /// ```
    /// use open_dis_rust::logistics::resupply_cancel_pdu::ResupplyCancelPdu;
    /// let resupply_cancel_pdu = ResupplyCancelPdu::default();
    /// ```
    ///
    fn default() -> Self {
        ResupplyCancelPdu {
            pdu_header: PduHeader::default(PduType::ResupplyCancel, ProtocolFamily::Logistics, 56),
            receiving_entity_id: EntityId::default(1),
            supplying_entity_id: EntityId::default(2),
        }
    }
}

impl Pdu for ResupplyCancelPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        self.supplying_entity_id.serialize(buf);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::ResupplyCancel {
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let supplying_entity_id = EntityId::decode(&mut buffer);

            Ok(ResupplyCancelPdu {
                pdu_header,
                receiving_entity_id,
                supplying_entity_id,
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
        let receiving_entity_id = EntityId::decode(&mut buffer);
        let supplying_entity_id = EntityId::decode(&mut buffer);

        Ok(ResupplyCancelPdu {
            pdu_header,
            receiving_entity_id,
            supplying_entity_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ResupplyCancelPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let resupply_cancel_pdu = ResupplyCancelPdu::default();
        let pdu_header =
            PduHeader::default(PduType::ResupplyCancel, ProtocolFamily::Logistics, 448 / 8);

        assert_eq!(
            pdu_header.protocol_version,
            resupply_cancel_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            resupply_cancel_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, resupply_cancel_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            resupply_cancel_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, resupply_cancel_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, resupply_cancel_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut resupply_cancel_pdu = ResupplyCancelPdu::default();
        let mut buffer = BytesMut::new();
        resupply_cancel_pdu.serialize(&mut buffer);

        let new_resupply_cancel_pdu = ResupplyCancelPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_resupply_cancel_pdu.pdu_header,
            resupply_cancel_pdu.pdu_header
        );
    }
}
