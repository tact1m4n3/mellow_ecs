use std::collections::HashMap;

use crate::{hasher::BuildNoHasher, tables::TableId};

#[derive(Default)]
pub struct Entities {
    next_id: usize,
    tables: HashMap<EntityId, TableId, BuildNoHasher<EntityId>>,
}

impl Entities {
    pub fn alloc(&mut self) -> EntityId {
        assert!(self.next_id < usize::MAX);
        let id = EntityId(self.next_id);
        self.next_id += 1;
        id
    }

    pub fn del(&mut self, id: EntityId) {
        self.tables.remove(&id);
    }

    pub fn set_table_id(&mut self, id: EntityId, table_id: TableId) {
        self.tables.insert(id, table_id);
    }

    pub fn table_id(&self, id: EntityId) -> Option<TableId> {
        self.tables.get(&id).copied()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(usize);
