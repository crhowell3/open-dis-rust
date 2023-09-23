use crate::dis6::entity_id::EntityId;

pub struct SimulationManagementFamilyPdu {
    originating_entity_id: Option<EntityId>,
    receiving_entity_id: Option<EntityId>,
}

impl SimulationManagementFamilyPdu {
    pub fn new() -> Self {
        SimulationManagementFamilyPdu {
            originating_entity_id: None,
            receiving_entity_id: None,
        }
    }

    pub fn get_originating_entity_id(&self) -> Option<&EntityId> {
        self.originating_entity_id.as_ref()
    }

    pub fn set_originating_entity_id(&mut self, x: EntityId) {
        self.originating_entity_id = Some(x);
    }

    pub fn get_receiving_entity_id(&self) -> Option<&EntityId> {
        self.receiving_entity_id.as_ref()
    }

    pub fn set_receiving_entity_id(&mut self, x: EntityId) {
        self.receiving_entity_id = Some(x);
    }
}
