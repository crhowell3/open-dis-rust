use crate::dis6::entity_id::EntityId;
use crate::dis6::families::simulation_management::SimulationManagementFamilyPdu;

pub struct DataQueryPdu {
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub time_interval: u32,
    pub fixed_datum_records: Vec<VariableRecordType>,
    pub variable_datum_records: Vec<VariableRecordType>,    
}

impl Default for DataQueryPdu {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationManagementFamilyPdu for DataQueryPdu {
    fn get_originating_entity_id(&self) -> EntityId {
        self.originating_entity_id.as_ref()
    }

    fn set_originating_entity_id(&mut self, x: EntityId) {
        self.originating_entity_id = x;
    }

    fn get_receiving_entity_id(&self) -> EntityId {
        self.receiving_entity_id.as_ref()
    }

    fn set_receiving_entity_id(&mut self, x: EntityId) {
        self.receiving_entity_id = x;
    }
}

impl DataQueryPdu {
    pub fn new() -> Self {
        Self {
            originating_entity_id: Default::default(),
            receiving_entity_id: Default::default(),
            request_id: 0,
            time_interval: 0,
            fixed_datum_records: vec![],
            variable_datum_records: vec![],
        }
    }
}
