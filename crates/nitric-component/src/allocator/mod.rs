//! Module for defining the allocator interface and its traits.
//!
//! This type actually has a top-level trait (`Allocator`), but most of its functionality is still
//! split up into traits.
//!

pub use self::phantom::PhantomAllocator;

use crate::id::AsUsize;
use crate::id::CheckedId;
use crate::id::ValidId;
use crate::{
    error::{InvalidIdError, OomError},
    id::Id,
};

mod phantom;

/// Generic allocator for IDs of type `ID`.
///
/// # Unsafety
///
/// This is unsafe to implement because it is required to uphold the contract of `is_valid`.
/// Breaking it is allowed exhibit undefined behavior.
pub unsafe trait Allocator<ID>
where
    ID: Id<Allocator = Self>,
{
    /// Checks if `id` is a valid ID.
    ///
    /// This can return `false` for example if the ID has been deleted.
    ///
    /// # Contract
    ///
    /// This method is required to keep returning `true` for as long as the allocator is borrowed
    /// immutably.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if `id` was allocated by a different allocator.
    fn is_valid(&self, id: &ID) -> bool;

    /// Retrieves the number of valid IDs. This may or may not be expensive to calculate.
    ///
    /// If you're looking for this for optimization purposes, use `num_valid_hint` instead.
    fn num_valid(&self) -> usize;

    /// Returns a hint for the number of valid IDs. This should be preferred over `num_valid` when
    /// used for optimization purposes.
    ///
    /// The first element of the tuple is the minimal number of valid IDs, the latter the maximum.
    ///
    /// If the first and the second are equal, this hint must be accurate. If the latter is `None`,
    /// the operation is non-trivial and `num_valid` must be used to retrieve the accurate number.
    fn num_valid_hint(&self) -> (usize, Option<usize>);
}

/// Trait implemented by allocators that can create new IDs, atomically and without additional
/// arguments.
pub trait Create<ID>: Allocator<ID>
where
    ID: Id<Allocator = Self>,
{
    /// Creates a new ID of type `ID`.
    ///
    /// In case your allocator supports atomic ID creation, you should implement this for `&Self`,
    /// too.
    fn create(&mut self) -> Result<ID, OomError>;
}

/// Trait implemented by allocators that can create new IDs, atomically and without additional
/// arguments.
pub trait CreateChecked<ID>: Allocator<ID> + Create<ID>
where
    ID: AsUsize + Id<Allocator = Self>,
{
    /// Creates a new ID of type `<Self as Allocator>::Id` and wraps it in a `CheckedId`.
    /// Also see `Create::create`.
    ///
    /// This is useful because it allows you to perform operations after an ID creation
    /// without the need to `unwrap` impossible errors (since your ID implements `ValidId`.
    ///
    /// Once you're done, you'll want to get the inner ID again using `CheckedId::into_inner`.
    ///
    /// This has a naive default implementation that can be replaced with a custom one if required.
    fn create_checked(&mut self) -> Result<CheckedId<'_, ID>, OomError> {
        let id = self.create()?;
        let checked = id
            .checked(&*self)
            .expect("The ID was just created, it cannot be invalid");

        Ok(checked)
    }
}

/// Trait implemented by allocators that can delete IDs, atomically and without additional
/// arguments.
pub trait Delete<ID>: Allocator<ID>
where
    ID: Id<Allocator = Self>,
{
    /// Deletes a previously created ID that is guaranteed to be valid.
    /// For deleting eventually valid IDs, see `try_delete`.
    ///
    /// In case your allocator supports atomic ID creation, you should implement this for `&Self`,
    /// too.
    ///
    /// # Behavior
    ///
    /// This is not guaranteed to actually delete the ID. This just flags it for deletion;
    /// the allocator is free to require calling further methods for an actual deletion to happen.
    ///
    /// TODO: list common examples here
    ///
    /// # Panics
    ///
    /// Panics in debug mode if `id` was allocated by a different allocator.
    fn delete<V>(&mut self, id: &V)
    where
        V: ValidId<ID>;

    /// Deletes a previously created ID, failing if the ID is invalid.
    /// See `Delete::delete`.
    fn try_delete(&mut self, id: &ID) -> Result<(), InvalidIdError<ID>>;

    /// Makes sure `id` is deleted, ignoring the case where deletion fails due to an invalid ID.
    /// See `Delete::delete`.
    fn assert_deleted(&mut self, id: &ID) {
        let _ = self.try_delete(id);
    }
}
