use crate::dis6::entity_id::EntityId;

pub struct DataPdu {
    pub originating_entity_id: EntityId,
    pub receiving_entity_id: EntityId,
    pub request_id: u32,
    pub padding: u32,
    pub fixed_datums: Vec<VariableDatumRecords>,
    pub variable_datums: Vec<VariableDatumRecords>,   
}

impl Default for DataPdu {
    fn default() -> Self {
        Self::new(),
    }
}

impl DataPdu {
    pub fn new() -> Self {
        Self {
            originating_entity_id: Default::default(),
            receiving_entity_id: Default::default(),
            request_id: 0,
            padding: 0,
            fixed_datums: vec![],
            variable_datums: vec![],
        }
    }
}
