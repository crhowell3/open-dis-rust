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

use super::data_types::supply_quantity::SupplyQuantity;

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.4.4
pub struct ResupplyReceivedPdu {
    pub pdu_header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub supplying_entity_id: EntityId,
    pub number_of_supply_types: u8,
    pub padding1: i16,
    pub padding2: i8,
    pub supplies: Vec<SupplyQuantity>,
}

impl Default for ResupplyReceivedPdu {
    /// Creates a default Resupply Received PDU with arbitrary receiving and supplying entity IDs
    ///
    /// # Examples
    ///
    /// Initializing a Resupply Received PDU:
    /// ```
    /// use open_dis_rust::logistics::resupply_received_pdu::ResupplyReceivedPdu;
    /// let resupply_received_pdu = ResupplyReceivedPdu::default();
    /// ```
    ///
    fn default() -> Self {
        ResupplyReceivedPdu {
            pdu_header: PduHeader::default(
                PduType::ResupplyReceived,
                ProtocolFamily::Logistics,
                56,
            ),
            receiving_entity_id: EntityId::default(1),
            supplying_entity_id: EntityId::default(2),
            number_of_supply_types: 0,
            padding1: 0,
            padding2: 0,
            supplies: vec![],
        }
    }
}

impl Pdu for ResupplyReceivedPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        self.supplying_entity_id.serialize(buf);
        buf.put_u8(self.number_of_supply_types);
        buf.put_i16(self.padding1);
        buf.put_i8(self.padding2);
        for i in 0..self.supplies.len() {
            self.supplies[i].serialize(buf);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::ResupplyReceived {
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let supplying_entity_id = EntityId::decode(&mut buffer);
            let number_of_supply_types = buffer.get_u8();
            let padding1 = buffer.get_i16();
            let padding2 = buffer.get_i8();
            let mut supplies: Vec<SupplyQuantity> = vec![];
            for _i in 0..number_of_supply_types {
                supplies.push(SupplyQuantity::decode(&mut buffer));
            }

            Ok(ResupplyReceivedPdu {
                pdu_header,
                receiving_entity_id,
                supplying_entity_id,
                number_of_supply_types,
                padding1,
                padding2,
                supplies,
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
        let number_of_supply_types = buffer.get_u8();
        let padding1 = buffer.get_i16();
        let padding2 = buffer.get_i8();
        let mut supplies: Vec<SupplyQuantity> = vec![];
        for _i in 0..number_of_supply_types {
            supplies.push(SupplyQuantity::decode(&mut buffer));
        }

        Ok(ResupplyReceivedPdu {
            pdu_header,
            receiving_entity_id,
            supplying_entity_id,
            number_of_supply_types,
            padding1,
            padding2,
            supplies,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ResupplyReceivedPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let resupply_received_pdu = ResupplyReceivedPdu::default();
        let pdu_header = PduHeader::default(
            PduType::ResupplyReceived,
            ProtocolFamily::Logistics,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            resupply_received_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            resupply_received_pdu.pdu_header.exercise_id
        );
        assert_eq!(
            pdu_header.pdu_type,
            resupply_received_pdu.pdu_header.pdu_type
        );
        assert_eq!(
            pdu_header.protocol_family,
            resupply_received_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, resupply_received_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, resupply_received_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut resupply_received_pdu = ResupplyReceivedPdu::default();
        let mut buffer = BytesMut::new();
        resupply_received_pdu.serialize(&mut buffer);

        let new_resupply_received_pdu = ResupplyReceivedPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_resupply_received_pdu.pdu_header,
            resupply_received_pdu.pdu_header
        );
    }
}
