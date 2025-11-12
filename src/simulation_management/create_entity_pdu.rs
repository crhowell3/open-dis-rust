//     open-dis-rust - Rust implementation of the IEEE 1278.1-2012 Distributed Interactive
//                     Simulation (DIS) application protocol
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD 2-Clause License

use crate::common::{
    ProtocolFamily,
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType},
};
use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

#[derive(Copy, Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §7.5.2
pub struct CreateEntityPdu {
    pub pdu_header: PduHeader,
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
}

impl Default for CreateEntityPdu {
    fn default() -> Self {
        CreateEntityPdu {
            pdu_header: PduHeader::default(),
            originating_entity_id: EntityId::default(1),
            receiving_entity_id: EntityId::default(2),
            request_id: 0,
        }
    }
}

impl Pdu for CreateEntityPdu {
    fn length(&self) -> u16 {
        let length = std::mem::size_of::<PduHeader>()
            + std::mem::size_of::<EntityId>() * 2
            + std::mem::size_of::<u32>();

        length as u16
    }

    fn header(&self) -> &PduHeader {
        &self.pdu_header
    }

    fn header_mut(&mut self) -> &mut PduHeader {
        &mut self.pdu_header
    }

    fn serialize(&mut self, buf: &mut BytesMut) {
        self.pdu_header.length = u16::try_from(std::mem::size_of_val(self))
            .expect("The length of the PDU should fit in a u16.");
        self.pdu_header.serialize(buf);
        self.originating_entity_id.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        buf.put_u32(self.request_id);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::deserialize(&mut buffer);
        if pdu_header.pdu_type == PduType::CreateEntity {
            let originating_entity_id = EntityId::deserialize(&mut buffer);
            let receiving_entity_id = EntityId::deserialize(&mut buffer);
            let request_id = buffer.get_u32();

            Ok(CreateEntityPdu {
                pdu_header,
                originating_entity_id,
                receiving_entity_id,
                request_id,
            })
        } else {
            Err(DISError::invalid_header(
                format!(
                    "Expected PDU type CreateEntity, got {:?}",
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

        Ok(CreateEntityPdu {
            pdu_header,
            originating_entity_id,
            receiving_entity_id,
            request_id,
        })
    }
}

impl CreateEntityPdu {
    /// Creates a CreateEntity PDU
    ///
    /// # Examples
    ///
    /// Initializing an CreateEntity PDU:
    /// ```
    /// use open_dis_rust::simulation_management::CreateEntityPdu;
    /// let mut create_entity_pdu = CreateEntityPdu::new();
    /// ```
    ///
    pub fn new() -> Self {
        let mut pdu = Self::default();
        pdu.pdu_header.pdu_type = PduType::CreateEntity;
        pdu.pdu_header.protocol_family = ProtocolFamily::SimulationManagement;
        pdu.finalize();
        pdu
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::CreateEntityPdu;
    use crate::common::{Pdu, PduType, pdu_header::PduHeader};

    #[test]
    fn create_header() {
        let create_entity_pdu = CreateEntityPdu::default();
        let pdu_header = PduHeader::default();

        assert_eq!(
            pdu_header.protocol_version,
            create_entity_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            create_entity_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, create_entity_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            create_entity_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, create_entity_pdu.pdu_header.length);
        assert_eq!(
            pdu_header.status_record,
            create_entity_pdu.pdu_header.status_record
        );
    }

    #[test]
    fn cast_to_any() {
        let create_entity_pdu = CreateEntityPdu::default();
        let any_pdu = create_entity_pdu.as_any();

        assert!(any_pdu.is::<CreateEntityPdu>());
    }

    #[test]
    fn deserialize_header() {
        let mut create_entity_pdu = CreateEntityPdu::default();
        let mut buffer = BytesMut::new();
        create_entity_pdu.serialize(&mut buffer);

        let new_create_entity_pdu = CreateEntityPdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_create_entity_pdu.pdu_header,
            create_entity_pdu.pdu_header
        );
    }

    #[test]
    fn create_new_pdu() {
        let create_entity_pdu = CreateEntityPdu::new();

        assert_eq!(create_entity_pdu.header().pdu_type, PduType::CreateEntity);
    }
}
