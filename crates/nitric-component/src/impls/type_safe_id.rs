#![cfg(ignore)]

use std::marker::PhantomData;

use crate::id::{AsUsize, Continuous, Id, SparseLinear, ValidId};

// TODO

pub struct TypeSafeAllocator<ID: Id> {
    alloc: ID::Allocator,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeSafeId<M, ID> {
    id: ID,
    _marker: PhantomData<M>,
}

impl<M, ID> TypeSafeId<M, ID> {
    /// Returns a reference to the wrapped ID.
    pub fn inner(&self) -> &ID {
        &self.id
    }

    /// Returns a mutable reference to the wrapped ID.
    pub fn inner_mut(&mut self) -> &mut ID {
        &mut self.id
    }

    /// Moves out the wrapped ID.
    pub fn into_inner(self) -> ID {
        self.id
    }
}
