use std::io::{Read, Write};

pub struct EntityId {
    site: u16,
    application: u16,
    entity: u16,
}

impl EntityId {
    pub fn new() -> Self {
        EntityId {
            site: 0,
            application: 0,
            entity: 0,
        }
    }

    pub fn marshal(&self) {}

    pub fn unmarshal() {}

    pub fn get_site(&self) -> u16 {
        self.site
    }

    pub fn set_site(&mut self, x: u16) {
        self.site = x;
    }

    pub fn get_application(&self) -> u16 {
        self.application
    }

    pub fn set_application(&mut self, x: u16) {
        self.application = x;
    }

    pub fn get_entity(&self) -> u16 {
        self.entity
    }

    pub fn set_entity(&mut self, x: u16) {
        self.entity = x;
    }

    pub fn get_marshalled_size(&self) -> usize {
        6
    }

    pub fn equal(&self, rhs: &EntityId) -> bool {
        self.site == rhs.site && self.application == rhs.application && self.entity == rhs.entity
    }
}
