use std::{any::TypeId, collections::HashMap};

use crate::{
    entity::EntityId,
    hasher::BuildNoHasher,
    store::{ItemType, StoreId},
};

#[derive(Default)]
pub struct Tables {
    type_ids: HashMap<TypeId, TableId, BuildNoHasher<TypeId>>,
    tables: Vec<Table>,
}

impl Tables {
    pub fn create(&mut self, type_id: TypeId) -> TableId {
        let id = TableId(self.tables.len());
        self.tables.push(Table::default());
        self.type_ids.insert(type_id, id);
        id
    }

    pub fn drop(&mut self, id: TableId) {
        self.type_ids.retain(|_, v| *v == id);
        self.tables.remove(id.0);
    }

    pub fn with_type(&self, type_id: TypeId) -> Option<TableId> {
        self.type_ids.get(&type_id).copied()
    }

    pub fn get(&self, id: TableId) -> &Table {
        &self.tables[id.0]
    }

    pub fn get_mut(&mut self, id: TableId) -> &mut Table {
        &mut self.tables[id.0]
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Table> {
        self.tables.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Table> {
        self.tables.iter_mut()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TableId(usize);

#[derive(Default)]
pub struct Table {
    types: Vec<ItemType>,
    store_ids: Vec<StoreId>,
    entities: Vec<EntityId>,
}

impl Table {
    pub fn add_column(&mut self, store_id: StoreId, typ: ItemType) {
        self.store_ids.push(store_id);
        self.types.push(typ);
    }

    pub fn column(&self, typ: &ItemType) -> Option<StoreId> {
        self.types
            .iter()
            .zip(self.store_ids.iter())
            .find(|(comp, _)| comp.id == typ.id)
            .map(|(_, store_id)| *store_id)
    }

    pub fn has_column(&self, typ: &ItemType) -> bool {
        self.column(typ).is_some()
    }

    pub fn columns(&self) -> &[StoreId] {
        &self.store_ids
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, entity_id: EntityId) -> usize {
        let idx = self.entities.len();
        self.entities.push(entity_id);
        idx
    }

    pub fn entity_index(&self, entity_id: EntityId) -> Option<usize> {
        self.entities.iter().position(|id| *id == entity_id)
    }

    pub fn get(&self, idx: usize) -> Option<EntityId> {
        self.entities.get(idx).copied()
    }

    pub fn remove(&mut self, idx: usize) {
        self.entities.remove(idx);
    }
}
