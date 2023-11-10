//     open-dis-rust - Rust implementation of the IEEE-1278.1 Distributed Interactive Simulation
//     Copyright (C) 2023 Cameron Howell
//
//     Licensed under the BSD-2-Clause License

use bytes::{Buf, BufMut, BytesMut};
use std::any::Any;

use crate::common::{
    dis_error::DISError,
    entity_id::EntityId,
    pdu::Pdu,
    pdu_header::{PduHeader, PduType, ProtocolFamily},
};

#[derive(Copy, Clone, Debug)]
/// Implemented according to IEEE 1278.1-2012 §5.5.10
pub struct RepairCompletePdu {
    pub pdu_header: PduHeader,
    pub receiving_entity_id: EntityId,
    pub repairing_entity_id: EntityId,
    pub repair: RepairGroups,
    pub padding2: i8,
}

impl RepairCompletePdu {
    /// Creates a default Repair Complete PDU with arbitrary receiving and repairing entity IDs
    ///
    /// # Examples
    ///
    /// Initializing a Repair Complete PDU:
    /// ```
    /// use open_dis_rust::logistics::repair_complete_pdu::RepairCompletePdu;
    /// let repair_complete_pdu = RepairCompletePdu::default();
    /// ```
    ///
    pub fn default() -> Self {
        RepairCompletePdu {
            pdu_header: PduHeader::default(PduType::RepairComplete, ProtocolFamily::Logistics, 56),
            receiving_entity_id: EntityId::default(1),
            repairing_entity_id: EntityId::default(2),
            repair: RepairGroups::GeneralRepairCodes,
            padding2: 0,
        }
    }
}

impl Pdu for RepairCompletePdu {
    fn serialize(&self, buf: &mut BytesMut) {
        self.pdu_header.serialize(buf);
        self.receiving_entity_id.serialize(buf);
        self.repairing_entity_id.serialize(buf);
        buf.put_u8(self.repair as u8);
        buf.put_i8(self.padding2);
    }

    fn deserialize(mut buffer: BytesMut) -> Result<Self, DISError>
    where
        Self: Sized,
    {
        let pdu_header = PduHeader::decode(&mut buffer);
        if pdu_header.pdu_type == PduType::RepairComplete {
            let receiving_entity_id = EntityId::decode(&mut buffer);
            let repairing_entity_id = EntityId::decode(&mut buffer);
            let repair = RepairGroups::from_u8(buffer.get_u8());
            let padding2 = buffer.get_i8();

            Ok(RepairCompletePdu {
                pdu_header,
                receiving_entity_id,
                repairing_entity_id,
                repair,
                padding2,
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
        let repairing_entity_id = EntityId::decode(&mut buffer);
        let repair = RepairGroups::from_u8(buffer.get_u8());
        let padding2 = buffer.get_i8();

        Ok(RepairCompletePdu {
            pdu_header,
            receiving_entity_id,
            repairing_entity_id,
            repair,
            padding2,
        })
    }
}

#[derive(Copy, Clone, Debug)]
/// Implemented according to SISO-REF-010-2020 UID 272
pub enum RepairGroups {
    GeneralRepairCodes = 0,
    DriveTrain = 1,
    HullAirframeBody = 2,
    InterfacesWithEnvironment = 3,
    Weapons = 4,
    FuelSystems = 5,
    Electronics = 6,
    LifeSupportSystems = 7,
    HydraulicSystemsAndActuators = 8,
    AuxilliaryCraft = 9,
}

impl RepairGroups {
    pub fn from_u8(byte: u8) -> RepairGroups {
        match byte {
            0 => RepairGroups::GeneralRepairCodes,
            1 => RepairGroups::DriveTrain,
            2 => RepairGroups::HullAirframeBody,
            3 => RepairGroups::InterfacesWithEnvironment,
            4 => RepairGroups::Weapons,
            5 => RepairGroups::FuelSystems,
            6 => RepairGroups::Electronics,
            7 => RepairGroups::LifeSupportSystems,
            8 => RepairGroups::HydraulicSystemsAndActuators,
            9 => RepairGroups::AuxilliaryCraft,
            _ => RepairGroups::GeneralRepairCodes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RepairCompletePdu;
    use crate::common::{
        pdu::Pdu,
        pdu_header::{PduHeader, PduType, ProtocolFamily},
    };
    use bytes::BytesMut;

    #[test]
    fn create_header() {
        let repair_complete_pdu = RepairCompletePdu::default();
        let pdu_header =
            PduHeader::default(PduType::RepairComplete, ProtocolFamily::Logistics, 448 / 8);

        assert_eq!(
            pdu_header.protocol_version,
            repair_complete_pdu.pdu_header.protocol_version
        );
        assert_eq!(
            pdu_header.exercise_id,
            repair_complete_pdu.pdu_header.exercise_id
        );
        assert_eq!(pdu_header.pdu_type, repair_complete_pdu.pdu_header.pdu_type);
        assert_eq!(
            pdu_header.protocol_family,
            repair_complete_pdu.pdu_header.protocol_family
        );
        assert_eq!(pdu_header.length, repair_complete_pdu.pdu_header.length);
        assert_eq!(pdu_header.padding, repair_complete_pdu.pdu_header.padding);
    }

    #[test]
    fn deserialize_header() {
        let repair_complete_pdu = RepairCompletePdu::default();
        let mut buffer = BytesMut::new();
        repair_complete_pdu.serialize(&mut buffer);

        let new_repair_complete_pdu = RepairCompletePdu::deserialize(buffer).unwrap();
        assert_eq!(
            new_repair_complete_pdu.pdu_header,
            repair_complete_pdu.pdu_header
        );
    }
}
