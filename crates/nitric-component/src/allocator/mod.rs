//! Module for defining the allocator interface and its traits.
//!
//! This type actually has a top-level trait (`Allocator`), but most of its functionality is still
//! split up into traits.
//!

pub use self::phantom::PhantomAllocator;

use crate::id::ValidId;
use crate::{
    error::{InvalidIdError, OomError},
    id::Id,
};
use crate::id::AsUsize;
use crate::id::CheckedId;

mod phantom;

/// Generic allocator for IDs of type `Self::Id`.
///
/// # Unsafety
///
/// This is unsafe to implement because it is required to uphold the contract of `is_valid`.
/// Breaking it is allowed exhibit undefined behavior.
pub unsafe trait Allocator {
    /// The ID type this allocator manages.
    type Id: Id<Allocator = Self>;

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
    fn is_valid(&self, id: &Self::Id) -> bool;

    /// Retrieves the number of valid IDs. This may or may not be expensive to calculate.
    ///
    /// If you're looking for this for optimization purposes, use `num_valid_hint` instead.
    fn num_valid(&self) -> usize;

    /// Retrieves the number of valid IDs if it is cheap to do so. Returns `None` in case this
    /// operation is non-trivial.
    fn num_valid_hint(&self) -> Option<usize>;
}

/// Trait implemented by allocators that can create new IDs, atomically and without additional
/// arguments.
pub trait Create: Allocator {
    /// Creates a new ID of type `<Self as Allocator>::Id`.
    ///
    /// In case your allocator supports atomic ID creation, you should implement this for `&Self`,
    /// too.
    fn create(&mut self) -> Result<Self::Id, OomError>;
}

/// Trait implemented by allocators that can create new IDs, atomically and without additional
/// arguments.
pub trait CreateChecked: Allocator + Create
where
    Self::Id: AsUsize,
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
    fn create_checked(&mut self) -> Result<CheckedId<'_, Self::Id>, OomError> {
        let id = self.create()?;
        let checked = id.checked(&*self).expect("The ID was just created, it cannot be invalid");

        Ok(checked)
    }
}

/// Trait implemented by allocators that can delete IDs, atomically and without additional
/// arguments.
pub trait Delete: Allocator {
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
    fn delete(&mut self, id: &Self::Id)
    where
        Self::Id: ValidId;

    /// Deletes a previously created ID, failing if the ID is invalid.
    /// See `Delete::delete`.
    fn try_delete(&mut self, id: &Self::Id) -> Result<(), InvalidIdError<Self::Id>>;

    /// Makes sure `id` is deleted, ignoring the case where deletion fails due to an invalid ID.
    /// See `Delete::delete`.
    fn assert_deleted(&mut self, id: &Self::Id) {
        let _ = self.try_delete(id);
    }
}
