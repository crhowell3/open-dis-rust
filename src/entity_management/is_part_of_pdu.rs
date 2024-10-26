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
    entity_type::EntityType,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
    vector3_float::Vector3Float,
};

use super::data_types::{named_location::NamedLocation, relationship::Relationship};

#[derive(Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.8.5
pub struct IsPartOfPdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub relationship: Relationship,
    pub part_location: Vector3Float,
    pub named_location_id: NamedLocation,
    pub part_entity_type: EntityType,
}

impl Default for IsPartOfPdu {
    /// Creates a default Is Part Of PDU with arbitrary aggregate ID
    ///
    /// # Examples
    ///
    /// Initializing an Is Part Of PDU:
    /// ```
    /// use open_dis_rust::entity_management::is_part_of_pdu::IsPartOfPdu;
    /// let is_part_of_pdu = IsPartOfPdu::default();
    /// ```
    ///
    fn default() -> Self {
        IsPartOfPdu {
            pdu_header: PduHeader::default(PduType::IsPartOf, ProtocolFamily::EntityManagement, 56),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            relationship: Relationship::default(),
            part_location: Vector3Float::default(),
            named_location_id: NamedLocation::default(),
            part_entity_type: EntityType::default(),
        }
    }
}

impl Pdu for IsPartOfPdu {
    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        self.relationship.serialize(buf);
        self.part_location.serialize(buf);
        self.named_location_id.serialize(buf);
        self.part_entity_type.serialize(buf);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::IsPartOf {
            let originating_entity_id = EntityId::decode(&mut buffer);
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let relationship = Relationship::decode(&mut buffer);
            let part_location = Vector3Float::decode(&mut buffer);
            let named_location_id = NamedLocation::decode(&mut buffer);
            let part_entity_type = EntityType::decode(&mut buffer);
            Ok(IsPartOfPdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                relationship,
                part_location,
                named_location_id,
                part_entity_type,
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
        let originating_entity_id = EntityId::decode(&mut buffer);
        let receiving_entity_id = EntityId::decode(&mut buffer);
        let relationship = Relationship::decode(&mut buffer);
        let part_location = Vector3Float::decode(&mut buffer);
        let named_location_id = NamedLocation::decode(&mut buffer);
        let part_entity_type = EntityType::decode(&mut buffer);
        Ok(IsPartOfPdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            relationship,
            part_location,
            named_location_id,
            part_entity_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IsPartOfPdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let is_part_of_pdu = IsPartOfPdu::default();
        let pdu_header =
            PduHeader::default(PduType::IsPartOf, ProtocolFamily::EntityManagement, 448 / 8);

        assert_eq!(
            pdu_header.protocol_version,
            is_part_of_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            is_part_of_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, is_part_of_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            is_part_of_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, is_part_of_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, is_part_of_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let mut is_part_of_pdu = IsPartOfPdu::default();
        let mut buffer = BytesMut::new();
        is_part_of_pdu.serialize(&mut buffer);

        let new_is_part_of_pdu = IsPartOfPdu::deserialize(buffer).unwrap();
        assert_eq!(new_is_part_of_pdu.pdu_header, is_part_of_pdu.pdu_header);
    }
}
