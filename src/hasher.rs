use std::{
    any::TypeId,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::entity::EntityId;

#[derive(Clone, Copy)]
pub struct NoHasher<T> {
    state: u64,
    _marker: PhantomData<T>,
}

impl<T: EnableNoHasher> Default for NoHasher<T> {
    fn default() -> Self {
        Self {
            state: 0,
            _marker: PhantomData,
        }
    }
}

impl<T: EnableNoHasher> Hasher for NoHasher<T> {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("can't hash bytes");
    }

    fn write_u8(&mut self, i: u8) {
        self.state = i as u64;
    }

    fn write_u16(&mut self, i: u16) {
        self.state = i as u64;
    }

    fn write_u32(&mut self, i: u32) {
        self.state = i as u64;
    }

    fn write_u64(&mut self, i: u64) {
        self.state = i;
    }

    fn write_u128(&mut self, i: u128) {
        self.state = i as u64;
    }

    fn write_usize(&mut self, i: usize) {
        self.state = i as u64;
    }

    fn write_i8(&mut self, i: i8) {
        self.state = i as u64;
    }

    fn write_i16(&mut self, i: i16) {
        self.state = i as u64;
    }

    fn write_i32(&mut self, i: i32) {
        self.state = i as u64;
    }

    fn write_i64(&mut self, i: i64) {
        self.state = i as u64;
    }

    fn write_i128(&mut self, i: i128) {
        self.state = i as u64;
    }

    fn write_isize(&mut self, i: isize) {
        self.state = i as u64;
    }
}

#[derive(Clone, Copy)]
pub struct BuildNoHasher<T> {
    _marker: PhantomData<T>,
}

impl<T: EnableNoHasher> Default for BuildNoHasher<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: EnableNoHasher> std::hash::BuildHasher for BuildNoHasher<T> {
    type Hasher = NoHasher<T>;

    fn build_hasher(&self) -> NoHasher<T> {
        NoHasher {
            state: 0,
            _marker: PhantomData,
        }
    }
}

pub trait EnableNoHasher: Hash {}

impl EnableNoHasher for u8 {}
impl EnableNoHasher for u16 {}
impl EnableNoHasher for u32 {}
impl EnableNoHasher for u64 {}
impl EnableNoHasher for u128 {}
impl EnableNoHasher for usize {}
impl EnableNoHasher for i8 {}
impl EnableNoHasher for i16 {}
impl EnableNoHasher for i32 {}
impl EnableNoHasher for i64 {}
impl EnableNoHasher for i128 {}
impl EnableNoHasher for isize {}

impl EnableNoHasher for TypeId {}
impl EnableNoHasher for EntityId {}
