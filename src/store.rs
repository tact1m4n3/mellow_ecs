use std::{
    alloc::{self, Layout},
    any::TypeId,
    collections::HashMap,
    hint,
    ptr::{self, NonNull},
    sync::atomic::{AtomicU32, Ordering},
};

use crate::hasher::BuildNoHasher;

#[derive(Default)]
pub struct Stores {
    stores: Vec<Store>,
    locks: HashMap<TypeId, Lock, BuildNoHasher<TypeId>>,
}

impl Stores {
    pub fn create(&mut self, typ: ItemType) -> StoreId {
        let id = self.stores.len();
        self.stores.push(Store::new(typ));
        StoreId(id)
    }

    pub fn drop(&mut self, id: StoreId) {
        self.stores.remove(id.0);
    }

    pub fn get(&self, id: StoreId) -> &Store {
        &self.stores[id.0]
    }

    pub fn get_mut(&mut self, id: StoreId) -> &mut Store {
        &mut self.stores[id.0]
    }

    pub fn acquire_read(&self, type_id: TypeId) {
        if let Some(lock) = self.locks.get(&type_id) {
            lock.acquire_read()
        }
    }

    pub fn release_read(&self, type_id: TypeId) {
        if let Some(lock) = self.locks.get(&type_id) {
            lock.release_read()
        }
    }

    pub fn acquire_write(&self, type_id: TypeId) {
        if let Some(lock) = self.locks.get(&type_id) {
            lock.acquire_write()
        }
    }

    pub fn release_write(&self, type_id: TypeId) {
        if let Some(lock) = self.locks.get(&type_id) {
            lock.release_write()
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreId(usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemType {
    pub id: TypeId,
    pub layout: Layout,
    pub drop: unsafe fn(*mut u8),
}

impl ItemType {
    pub fn of<T: 'static + Send + Sync>() -> Self {
        unsafe fn drop_ptr<T>(x: *mut u8) {
            x.cast::<T>().drop_in_place()
        }

        Self {
            id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),
            drop: drop_ptr::<T>,
        }
    }
}

pub struct Store {
    cap: usize,
    typ: ItemType,
    ptr: NonNull<u8>,
}

impl Store {
    pub fn new(typ: ItemType) -> Self {
        Self {
            cap: 0,
            typ,
            ptr: NonNull::dangling(),
        }
    }

    pub fn set_capacity(&mut self, cap: usize) {
        if cap <= self.cap {
            return;
        }

        let new_cap = cap;

        let new_layout = unsafe {
            Layout::from_size_align_unchecked(
                new_cap * self.typ.layout.size(),
                self.typ.layout.align(),
            )
        };

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            unsafe {
                alloc::realloc(
                    self.ptr.as_ptr(),
                    Layout::from_size_align_unchecked(
                        self.cap * self.typ.layout.size(),
                        self.typ.layout.align(),
                    ),
                    new_cap * self.typ.layout.size(),
                )
            }
        };

        self.cap = new_cap;
        self.ptr = match NonNull::new(new_ptr) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(new_layout),
        }
    }

    pub unsafe fn get_unchecked(&self, idx: usize) -> *mut u8 {
        self.ptr.as_ptr().add(idx * self.typ.layout.size())
    }

    pub fn get(&self, idx: usize) -> Option<*mut u8> {
        if idx < self.cap {
            Some(unsafe { self.get_unchecked(idx) })
        } else {
            None
        }
    }

    pub unsafe fn remove(&mut self, idx: usize) {
        if idx < self.cap {
            unsafe { (self.typ.drop)(self.get_unchecked(idx)) }
            if idx < self.cap - 1 {
                unsafe {
                    ptr::copy(
                        self.ptr.as_ptr().add(idx * self.typ.layout.size()),
                        self.ptr.as_ptr().add((idx + 1) * self.typ.layout.size()),
                        (self.cap - idx - 1) * self.typ.layout.size(),
                    );
                }
            }
        }
    }
}

#[derive(Default)]
struct Lock {
    state: AtomicU32,
}

impl Lock {
    fn is_read(&self) -> bool {
        self.state.load(Ordering::Acquire) & !0x8000_0000 != 0
    }

    fn is_written(&self) -> bool {
        self.state.load(Ordering::Acquire) & 0x8000_0000 != 0
    }

    fn acquire_read(&self) {
        while self.is_written() {
            hint::spin_loop();
        }

        self.state
            .store(self.state.load(Ordering::Acquire) - 1, Ordering::Release);
    }

    fn release_read(&self) {
        if self.is_read() {
            panic!("no read acquired");
        }

        self.state
            .store(self.state.load(Ordering::Acquire) - 1, Ordering::Release);
    }

    fn acquire_write(&self) {
        while self.is_read() {
            hint::spin_loop();
        }

        self.state.store(
            self.state.load(Ordering::Acquire) | 0x8000_0000,
            Ordering::Release,
        );
    }

    fn release_write(&self) {
        if self.is_written() {
            panic!("no write acquired");
        }

        self.state.store(
            self.state.load(Ordering::Acquire) & !0x8000_0000,
            Ordering::Release,
        );
    }
}
