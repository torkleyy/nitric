use std::marker::PhantomData;

use derivative::Derivative;

use crate::allocator::PhantomAllocator;
use crate::id::Id;
use crate::id::ValidId;
use crate::id::AsUsize;
use crate::error::InvalidIdError;

/// Represents an ID that is guaranteed to be valid.
///
/// Implements `ValidId` to allow calling methods that expect `ValidId`s, and can thus skip validity
/// checks. This means we can ensure at type-level that a function cannot fail.
#[derive(Derivative)]
#[derivative(Clone, Debug, Eq, Hash, PartialEq)]
pub struct CheckedId<'allocator, ID: Id> {
    /// The wrapped ID which can be extracted using this field or moved out using `into_inner`.
    pub id: ID,
    u_repr: usize,
    _allocator: PhantomData<&'allocator ID::Allocator>,
}

impl<'allocator, ID> CheckedId<'allocator, ID>
where
    ID: Id,
{
    /// Creates a new checked ID from an `id`, the `usize` representation and a reference to the
    /// allocator.
    ///
    /// # Contract
    ///
    /// * `id` must be valid for as long as `allocator` is borrowed
    pub unsafe fn new_from_fields(
        id: ID,
        u_repr: usize,
        _allocator: &'allocator ID::Allocator,
    ) -> Self {
        CheckedId {
            id,
            u_repr,
            _allocator: PhantomData,
        }
    }

    /// Moves out the inner ID.
    pub fn into_inner(self) -> ID {
        self.id
    }
}

/// Implementation of `AsUsize` for checked ID, which is only required for generic programming.
/// If you have the concrete type, use `ValidId::as_usize` instead.
///
/// `try_as_usize` always returns `Ok`.
impl<'allocator, ID> AsUsize for CheckedId<'allocator, ID>
    where
        ID: Id + 'allocator,
{
    fn try_as_usize(&self, _: &<Self as Id>::Allocator) -> Result<usize, InvalidIdError<Self>> {
        Ok(self.as_usize())
    }
}

impl<'allocator, ID> Id for CheckedId<'allocator, ID>
where
    ID: Id + 'allocator,
{
    type Allocator = PhantomAllocator<'allocator, Self, ID>;
}

unsafe impl<'allocator, ID> ValidId for CheckedId<'allocator, ID>
where
    ID: Id + 'allocator,
{
    fn as_usize(&self) -> usize {
        self.u_repr
    }
}
