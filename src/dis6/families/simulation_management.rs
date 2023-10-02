use crate::dis6::entity_id::EntityId;

pub trait SimulationManagementFamilyPdu {
    fn get_originating_entity_id(&self) -> EntityId;
    fn set_originating_entity_id(&mut self, x: EntityId);
    fn get_receiving_entity_id(&self) -> EntityId;
    fn set_receiving_entity_id(&mut self, x: EntityId);
}
