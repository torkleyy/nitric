use std::marker::PhantomData;

use derivative::Derivative;

use crate::{
    allocator::{Merger, PhantomAllocator},
    error::InvalidIdError,
    id::{AsUsize, Id, ValidId},
};

/// Represents an ID that is guaranteed to be valid.
///
/// Implements `ValidId` to allow calling methods that expect `ValidId`s, and can thus skip validity
/// checks. This means we can ensure at type-level that a function cannot fail.
#[derive(Derivative)]
#[derivative(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CheckedId<'merger, ID: Id> {
    /// The wrapped ID which can be extracted using this field or moved out using `into_inner`.
    pub id: ID,
    u_repr: usize,
    _merger: PhantomData<&'merger Merger<ID::Allocator>>,
}

impl<'merger, ID> CheckedId<'merger, ID>
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
        _merger: &'merger Merger<ID::Allocator>,
    ) -> Self {
        CheckedId {
            id,
            u_repr,
            _merger: PhantomData,
        }
    }
}

/// Implementation of `AsUsize` for checked ID, which is only required for generic programming.
/// If you have the concrete type, use `ValidId::as_usize` instead.
///
/// `try_as_usize` always returns `Ok`.
impl<'merger, ID> AsUsize for CheckedId<'merger, ID>
where
    ID: Id + 'merger,
{
    fn try_as_usize(&self, _: &<Self as Id>::Allocator) -> Result<usize, InvalidIdError<Self>> {
        Ok(self.as_usize())
    }
}

impl<'merger, ID> Id for CheckedId<'merger, ID>
where
    ID: Id + 'merger,
{
    type Allocator = PhantomAllocator;
}

unsafe impl<'merger, ID> ValidId<ID> for CheckedId<'merger, ID>
where
    ID: Id + 'merger,
{
    fn as_usize(&self) -> usize {
        self.u_repr
    }

    fn as_inner(&self) -> &ID {
        &self.id
    }

    fn into_inner(self) -> ID {
        self.id
    }
}
