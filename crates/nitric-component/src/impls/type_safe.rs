use std::borrow::Cow;
use std::fmt::Debug;
use std::marker::PhantomData;

use derivative::Derivative;

use crate::allocator::Allocator;
use crate::error::InvalidIdError;
use crate::id::{Id, WrapperId};

/// A wrapper around `ID::Allocator` to allow for multiple, opaque ID types with the same
/// backing ID.
///
/// The type `T` is just a dummy type to actually allow for unique IDs; it should ideally
/// just be a non-instantiable enum.
pub struct TypeSafeAllocator<ID: Id, T>
where
    ID::Allocator: Sized,
{
    alloc: ID::Allocator,
    _marker: PhantomData<T>,
}

impl<ID, T> Allocator<TypeSafeId<ID, T>> for TypeSafeAllocator<ID, T>
where
    ID: Id,
    ID::Allocator: Sized,
{
    fn is_valid(&self, id: &TypeSafeId<ID, T>) -> bool {
        self.alloc.is_valid(&id.id)
    }

    fn num_valid(&self) -> usize {
        self.alloc.num_valid()
    }

    fn num_valid_hint(&self) -> (usize, Option<usize>) {
        self.alloc.num_valid_hint()
    }
}

/// A wrapper around an ID to allow for multiple, opaque ID types with the same
/// backing ID.
#[derive(Derivative)]
#[derivative(Clone(bound = "ID: Clone"), Debug(bound = "ID: Debug"))]
pub struct TypeSafeId<ID, T> {
    id: ID,
    _marker: PhantomData<T>,
}

impl<ID, T> From<ID> for TypeSafeId<ID, T> {
    fn from(id: ID) -> Self {
        TypeSafeId {
            id,
            _marker: PhantomData,
        }
    }
}

impl<ID, T> Id for TypeSafeId<ID, T>
where
    ID: Id,
    ID::Allocator: Sized,
{
    type Allocator = TypeSafeAllocator<ID, T>;
    type Key = ID::Key;

    fn try_as_key(
        &self,
        allocator: &Self::Allocator,
    ) -> Result<Cow<'_, Self::Key>, InvalidIdError<Self>> {
        self.id
            .try_as_key(&allocator.alloc)
            .map_err(invalid_err_into)
    }

    fn as_key_unchecked(&self) -> Cow<'_, Self::Key> {
        self.id.as_key_unchecked()
    }
}

impl<ID, T> WrapperId for TypeSafeId<ID, T>
where
    ID: Id,
    ID::Allocator: Sized,
{
    type Original = ID;

    fn as_inner(&self) -> &Self::Original {
        &self.id
    }

    fn into_inner(self) -> Self::Original {
        self.id
    }
}

fn invalid_err_into<ID: Debug, T>(
    InvalidIdError(id): InvalidIdError<ID>,
) -> InvalidIdError<TypeSafeId<ID, T>> {
    InvalidIdError(id.into())
}
