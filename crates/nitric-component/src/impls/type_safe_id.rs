use std::{
    cmp::{Ord, Ordering, PartialOrd},
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
};

use derivative::Derivative;

use crate::{
    allocator::{Allocator, Create, CreateChecked, Delete, MergeDeleted, Merger},
    error::{InvalidIdError, OomError},
    id::{AsUsize, CheckedId, Continuous, Id, SparseLinear, ValidId},
};

/// Type-safe allocator for `TypeSafeId`s.
pub struct TypeSafeAllocator<M, ID: Id> {
    _marker: PhantomData<M>,
    alloc: ID::Allocator,
}

impl<M, ID> Allocator<TypeSafeId<M, ID>> for TypeSafeAllocator<M, ID>
where
    ID: Id,
{
    fn is_valid(&self, id: &TypeSafeId<M, ID>) -> bool {
        self.alloc.is_valid(&id.id)
    }

    fn num_valid(&self) -> usize {
        self.alloc.num_valid()
    }

    fn num_valid_hint(&self) -> (usize, Option<usize>) {
        self.alloc.num_valid_hint()
    }
}

impl<M, ID> Create<TypeSafeId<M, ID>> for TypeSafeAllocator<M, ID>
where
    ID: Id,
    ID::Allocator: Create<ID>,
{
    fn create(&mut self) -> Result<TypeSafeId<M, ID>, OomError> {
        self.alloc.create().map(TypeSafeId::from_inner)
    }
}

impl<M, ID> CreateChecked<TypeSafeId<M, ID>> for TypeSafeAllocator<M, ID>
where
    ID: Id,
    ID::Allocator: CreateChecked<ID>,
{
    fn create_checked<'merger>(
        &mut self,
        merger: &'merger Merger<Self>,
    ) -> Result<CheckedId<'merger, TypeSafeId<M, ID>>, OomError> {
        self.alloc
            .create_checked(merger)
            .map(TypeSafeId::from_inner)
    }
}

impl<M, ID> Delete<TypeSafeId<M, ID>> for TypeSafeAllocator<M, ID>
where
    ID: Id,
    ID::Allocator: Delete<ID>,
{
    fn is_flagged<V>(&mut self, id: &V) -> bool
    where
        V: ValidId<TypeSafeId<M, ID>>,
    {
        self.alloc.is_flagged(id)
    }

    fn delete<V>(&mut self, id: &V)
    where
        V: ValidId<TypeSafeId<M, ID>>,
    {
        self.alloc.delete(id)
    }

    fn try_delete(
        &mut self,
        id: &TypeSafeId<M, ID>,
    ) -> Result<(), InvalidIdError<TypeSafeId<M, ID>>> {
        self.alloc.try_delete(id)
    }
}

/// A wrapper around an `ID` to make it type-safe.
#[derive(Derivative)]
#[derivative(
    Clone(bound = "ID: Clone"),
    Copy(bound = "ID: Copy"),
    Debug(bound = "ID: Debug"),
    Eq(bound = "ID: Eq"),
    Hash(bound = "ID: Hash"),
    PartialEq(bound = "ID: PartialEq")
)]
pub struct TypeSafeId<M, ID> {
    id: ID,
    _marker: PhantomData<M>,
}

impl<M, ID> TypeSafeId<M, ID> {
    /// Creates a new `TypeSafeId` from an inner ID.
    pub fn from_inner(id: ID) -> Self {
        TypeSafeId {
            id,
            _marker: PhantomData,
        }
    }

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

impl<M, ID> Id for TypeSafeId<M, ID>
where
    ID: Id,
{
    type Allocator = TypeSafeAllocator<M, ID>;
}

impl<M, ID> Ord for TypeSafeId<M, ID>
where
    ID: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<M, ID> PartialOrd for TypeSafeId<M, ID>
where
    ID: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
