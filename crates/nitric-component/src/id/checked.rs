use std::{borrow::Cow, marker::PhantomData};

use derivative::Derivative;

use crate::{
    allocator::PhantomAllocator,
    error::InvalidIdError,
    id::{Id, MergingDeletion, ValidId, WrapperId},
};

/// Represents an ID that is guaranteed to be valid.
///
/// Implements `ValidId` to allow calling methods that expect `ValidId`s, and
/// can thus skip validity checks. This means we can ensure at type-level that a
/// function cannot fail.
#[derive(Derivative)]
#[derivative(
    Clone,
    Copy(bound = "ID: Copy, ID::Key: Copy"),
    Debug,
    Eq,
    Hash,
    PartialEq
)]
pub struct CheckedId<'merger, ID: Id + MergingDeletion> {
    /// The wrapped ID which can be extracted using this field or moved out
    /// using `into_inner`.
    id: ID,
    key: ID::Key,
    _merger: PhantomData<&'merger ID::Merger>,
}

impl<'merger, ID> CheckedId<'merger, ID>
where
    ID: Id + MergingDeletion,
{
    /// Creates a new checked ID from an `id`, the `usize` representation and a
    /// reference to the merger of the `Id`'s allocator.
    ///
    /// # Contract
    ///
    /// * `id` must be valid for as long as `allocator` is borrowed
    pub fn new_from_fields(id: ID, key: ID::Key, _merger: &'merger ID::Merger) -> Self {
        CheckedId {
            id,
            key,
            _merger: PhantomData,
        }
    }

    /// Returns a reference to the wrapped ID.
    pub fn id(&self) -> &ID {
        &self.id
    }
}

impl<'merger, ID> Id for CheckedId<'merger, ID>
where
    ID: Id + MergingDeletion + 'merger,
{
    type Allocator = PhantomAllocator;
    type Key = ID::Key;

    fn try_as_key(
        &self,
        _allocator: &Self::Allocator,
    ) -> Result<Cow<'_, Self::Key>, InvalidIdError<Self>> {
        Ok(self.as_key_unchecked())
    }

    fn as_key_unchecked(&self) -> Cow<'_, Self::Key> {
        Cow::Borrowed(&self.key)
    }
}

impl<'merger, ID> ValidId<ID> for CheckedId<'merger, ID>
where
    ID: Id + MergingDeletion + 'merger,
{
    fn as_inner(&self) -> &ID {
        &self.id
    }

    fn into_inner(self) -> ID {
        self.id
    }

    fn as_key(&self) -> Cow<'_, Self::Key> {
        self.as_key_unchecked()
    }
}

impl<'merger, ID> WrapperId for CheckedId<'merger, ID>
where
    ID: MergingDeletion + Id + 'merger,
{
    type Original = ID;

    fn as_inner(&self) -> &Self::Original {
        &self.id
    }

    fn into_inner(self) -> Self::Original {
        self.id
    }
}
