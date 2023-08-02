use crate::{
    bundle::Bundle,
    entity::{Entities, EntityId},
    query::{EntityQuery, Fetch, Query},
    store::Stores,
    tables::Tables,
};

#[derive(Default)]
pub struct World {
    entities: Entities,
    stores: Stores,
    tables: Tables,
}

impl World {
    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityId {
        let entity_id = self.entities.alloc();

        let table_id = if let Some(table_id) = self.tables.with_type(B::type_id()) {
            table_id
        } else {
            let table_id = self.tables.create(B::type_id());
            B::for_each_type(|typ| {
                let table = self.tables.get_mut(table_id);
                let store_id = self.stores.create(*typ);
                table.add_column(store_id, *typ);
            });
            table_id
        };

        let table = self.tables.get_mut(table_id);
        let column_idx = table.push(entity_id);

        bundle.get_components(|ptr, typ| {
            if let Some(store_id) = table.column(typ) {
                let store = self.stores.get_mut(store_id);
                store.set_capacity(table.len());
                unsafe {
                    store
                        .get_unchecked(column_idx)
                        .copy_from(ptr.as_ptr(), typ.layout.size())
                }
            }
        });

        self.entities.set_table_id(entity_id, table_id);

        entity_id
    }

    pub fn del(&mut self, entity_id: EntityId) {
        if let Some(table_id) = self.entities.table_id(entity_id) {
            let table = self.tables.get_mut(table_id);
            let column_idx = table.entity_index(entity_id).unwrap();

            table.columns().iter().for_each(|store_id| {
                let store = self.stores.get_mut(*store_id);
                unsafe { store.remove(column_idx) }
            });
            table.remove(column_idx);

            self.entities.del(entity_id);
        }
    }

    pub fn query<F: Fetch>(&self) -> Query<F> {
        Query::new(&self.stores, &self.tables)
    }

    pub fn query_entity<F: Fetch>(&self, entity_id: EntityId) -> EntityQuery<F> {
        EntityQuery::new(&self.stores, &self.tables, &self.entities, entity_id)
    }
}
