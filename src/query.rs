use std::{
    iter::{self, Filter, Once, Peekable},
    marker::PhantomData,
    slice::Iter,
};

use crate::{
    entity::{Entities, EntityId},
    store::{ItemType, Stores},
    tables::{Table, Tables},
};

pub trait Fetch {
    fn for_each_type(f: impl FnMut(&ItemType, bool, bool));
    unsafe fn from_components(f: impl FnMut(&ItemType) -> Option<*mut u8>) -> Self;
}

impl<T: 'static + Send + Sync> Fetch for &T {
    fn for_each_type(mut f: impl FnMut(&ItemType, bool, bool)) {
        f(&ItemType::of::<T>(), false, false);
    }

    unsafe fn from_components(mut f: impl FnMut(&ItemType) -> Option<*mut u8>) -> Self {
        &*f(&ItemType::of::<T>()).unwrap().cast::<T>()
    }
}

impl<T: 'static + Send + Sync> Fetch for &mut T {
    fn for_each_type(mut f: impl FnMut(&ItemType, bool, bool)) {
        f(&ItemType::of::<T>(), true, false);
    }

    unsafe fn from_components(mut f: impl FnMut(&ItemType) -> Option<*mut u8>) -> Self {
        &mut *f(&ItemType::of::<T>()).unwrap().cast::<T>()
    }
}

impl<T: 'static + Send + Sync> Fetch for Option<&T> {
    fn for_each_type(mut f: impl FnMut(&ItemType, bool, bool)) {
        f(&ItemType::of::<T>(), false, true);
    }

    unsafe fn from_components(mut f: impl FnMut(&ItemType) -> Option<*mut u8>) -> Self {
        f(&ItemType::of::<T>()).map(|ptr| &*ptr.cast::<T>())
    }
}

impl<T: 'static + Send + Sync> Fetch for Option<&mut T> {
    fn for_each_type(mut f: impl FnMut(&ItemType, bool, bool)) {
        f(&ItemType::of::<T>(), true, true);
    }

    unsafe fn from_components(mut f: impl FnMut(&ItemType) -> Option<*mut u8>) -> Self {
        f(&ItemType::of::<T>()).map(|ptr| &mut *ptr.cast::<T>())
    }
}

macro_rules! tuple_impl {
    ($($name:ident),*) => {
        impl<$($name: Fetch),*> Fetch for ($($name,)*) {
            fn for_each_type(mut f: impl FnMut(&ItemType, bool, bool)) {
                $($name::for_each_type(&mut f);)*
            }

            unsafe fn from_components(mut f: impl FnMut(&ItemType) -> Option<*mut u8>) -> Self {
                ($($name::from_components(&mut f),)*)
            }
        }
    };
}

tuple_impl!(A);
tuple_impl!(A, B);
tuple_impl!(A, B, C);
tuple_impl!(A, B, C, D);
tuple_impl!(A, B, C, D, E);
tuple_impl!(A, B, C, D, E, F);
tuple_impl!(A, B, C, D, E, F, G);
tuple_impl!(A, B, C, D, E, F, G, H);
tuple_impl!(A, B, C, D, E, F, G, H, I);
tuple_impl!(A, B, C, D, E, F, G, H, I, J);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

pub struct Query<'a, F: Fetch> {
    stores: &'a Stores,
    table_iter: Peekable<TableFilter<'a, F, Iter<'a, Table>>>,
    column_idx: usize,
    _lock: QueryLock<'a, F>,
    _marker: PhantomData<F>,
}

impl<'a, F: Fetch> Query<'a, F> {
    pub fn new(stores: &'a Stores, tables: &'a Tables) -> Self {
        Self {
            stores,
            table_iter: TableFilter::new(tables.iter()).peekable(),
            column_idx: 0,
            _lock: QueryLock::new(stores),
            _marker: PhantomData,
        }
    }
}

impl<'a, F: Fetch> Iterator for Query<'a, F> {
    type Item = (EntityId, F);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(table) = self.table_iter.peek() {
            if self.column_idx >= table.len() {
                self.table_iter.next();
                self.column_idx = 0;
            }
        }

        if let Some(table) = self.table_iter.peek() {
            let entity_id = table.get(self.column_idx).unwrap();
            let components = unsafe {
                F::from_components(|typ| {
                    if let Some(store_id) = table.column(typ) {
                        let store = self.stores.get(store_id);
                        store.get(self.column_idx)
                    } else {
                        None
                    }
                })
            };

            self.column_idx += 1;

            Some((entity_id, components))
        } else {
            None
        }
    }
}

pub struct EntityQuery<'a, F: Fetch> {
    stores: &'a Stores,
    table: Option<&'a Table>,
    column_idx: Option<usize>,
    _lock: QueryLock<'a, F>,
    _marker: PhantomData<F>,
}

impl<'a, F: Fetch> EntityQuery<'a, F> {
    pub fn new(
        stores: &'a Stores,
        tables: &'a Tables,
        entities: &'a Entities,
        entity_id: EntityId,
    ) -> Self {
        let table_id = entities.table_id(entity_id);
        let table = table_id.and_then(|table_id| {
            let table = tables.get(table_id);
            TableFilter::<F, Once<&Table>>::new(iter::once(table)).next()
        });
        let column_idx = table.and_then(|table| table.entity_index(entity_id));

        Self {
            stores,
            table,
            column_idx,
            _lock: QueryLock::new(stores),
            _marker: PhantomData,
        }
    }
}

impl<'a, F: Fetch> Iterator for EntityQuery<'a, F> {
    type Item = F;

    fn next(&mut self) -> Option<Self::Item> {
        self.table
            .take()
            .zip(self.column_idx.take())
            .map(|(table, column_idx)| unsafe {
                F::from_components(|typ| {
                    if let Some(store_id) = table.column(typ) {
                        let store = self.stores.get(store_id);
                        store.get(column_idx)
                    } else {
                        None
                    }
                })
            })
    }
}

pub struct TableFilter<'a, F: Fetch, I: Iterator<Item = &'a Table>> {
    iter: Filter<I, fn(&&Table) -> bool>,
    _marker: PhantomData<F>,
}

impl<'a, F: Fetch, I: Iterator<Item = &'a Table>> TableFilter<'a, F, I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.filter(|table| {
                if table.is_empty() {
                    return false;
                }

                let mut ok = true;
                F::for_each_type(|typ, _, is_opt| {
                    if !is_opt && !table.has_column(typ) {
                        ok = false;
                    }
                });
                ok
            }),
            _marker: PhantomData,
        }
    }
}

impl<'a, F: Fetch, I: Iterator<Item = &'a Table>> Iterator for TableFilter<'a, F, I> {
    type Item = &'a Table;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

struct QueryLock<'a, F: Fetch> {
    stores: &'a Stores,
    _marker: PhantomData<F>,
}

impl<'a, F: Fetch> QueryLock<'a, F> {
    pub fn new(stores: &'a Stores) -> Self {
        F::for_each_type(|typ, is_mut, _| {
            if is_mut {
                stores.acquire_write(typ.id);
            } else {
                stores.acquire_read(typ.id);
            }
        });

        Self {
            stores,
            _marker: PhantomData,
        }
    }
}

impl<'a, F: Fetch> Drop for QueryLock<'a, F> {
    fn drop(&mut self) {
        F::for_each_type(|typ, is_mut, _| {
            if is_mut {
                self.stores.release_write(typ.id);
            } else {
                self.stores.release_read(typ.id);
            }
        });
    }
}
