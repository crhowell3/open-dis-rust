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

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.8.3
pub struct IsGroupOfPdu {
    pub pdu_header: PduHeader,
    pub group_entity_id: EntityId,
    pub grouped_entity_category: u8,
    pub number_of_grouped_entities: u8,
    pub pad2: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub grouped_entity_descriptions: Vec<u64>,
}

impl Default for IsGroupOfPdu {
    /// Creates a default Is Group Of PDU with arbitrary aggregate ID
    ///
    /// # Examples
    ///
    /// Initializing an Is Group Of PDU:
    /// ```
    /// use open_dis_rust::entity_management::is_group_of_pdu::IsGroupOfPdu;
    /// let is_group_of_pdu = IsGroupOfPdu::default();
    /// ```
    ///
    fn default() -> Self {
        IsGroupOfPdu {
            pdu_header: PduHeader::default(
                PduType::IsGroupOf,
                ProtocolFamily::EntityManagement,
                56,
            ),
            group_entity_id: EntityId::default(1),
            grouped_entity_category: 0,
            number_of_grouped_entities: 0,
            pad2: 0,
            latitude: 0.0,
            longitude: 0.0,
            grouped_entity_descriptions: vec![],
        }
    }
}

impl Pdu for IsGroupOfPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.group_entity_id.serialize(buf);
        buf.put_u8(self.grouped_entity_category);
        buf.put_u8(self.number_of_grouped_entities);
        buf.put_u32(self.pad2);
        buf.put_f64(self.latitude);
        buf.put_f64(self.longitude);
        for i in 0..self.grouped_entity_descriptions.len() {
            buf.put_u64(self.grouped_entity_descriptions[i]);
        }
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::IsGroupOf {
            let group_entity_id = EntityId::deserialize(&mut buffer);
            let grouped_entity_category = buffer.get_u8();
            let number_of_grouped_entities = buffer.get_u8();
            let pad2 = buffer.get_u32();
            let latitude = buffer.get_f64();
            let longitude = buffer.get_f64();
            let mut grouped_entity_descriptions: Vec<u64> = vec![];
            for _i in 0..number_of_grouped_entities {
                grouped_entity_descriptions.push(buffer.get_u64());
            }
            Ok(IsGroupOfPdu {
                pdu_header,
                group_entity_id,
                grouped_entity_category,
                number_of_grouped_entities,
                pad2,
                latitude,
                longitude,
                grouped_entity_descriptions,
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
        let group_entity_id = EntityId::deserialize(&mut buffer);
        let grouped_entity_category = buffer.get_u8();
        let number_of_grouped_entities = buffer.get_u8();
        let pad2 = buffer.get_u32();
        let latitude = buffer.get_f64();
        let longitude = buffer.get_f64();
        let mut grouped_entity_descriptions: Vec<u64> = vec![];
        for _i in 0..number_of_grouped_entities {
            grouped_entity_descriptions.push(buffer.get_u64());
        }
        Ok(IsGroupOfPdu {
            pdu_header,
            group_entity_id,
            grouped_entity_category,
            number_of_grouped_entities,
            pad2,
            latitude,
            longitude,
            grouped_entity_descriptions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IsGroupOfPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let is_group_of_pdu = IsGroupOfPdu::default();
        let pdu_header = PduHeader::default(
            PduType::IsGroupOf,
            ProtocolFamily::EntityManagement,
            448 / 8,
        );

        assert_eq!(
            pdu_header.protocol_version,
            is_group_of_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            is_group_of_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, is_group_of_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            is_group_of_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, is_group_of_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, is_group_of_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut is_group_of_pdu = IsGroupOfPdu::default();
        let mut buffer = BytesMut::new();
        is_group_of_pdu.serialize(&mut buffer);

        let new_is_group_of_pdu = IsGroupOfPdu::deserialize(buffer).unwrap();
        assert_eq!(new_is_group_of_pdu.pdu_header, is_group_of_pdu.pdu_header);
    }
}
