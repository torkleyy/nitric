//! Module for defining the allocator interface and its traits.
//!
//! This type actually has a top-level trait (`Allocator`), but most of its functionality is still
//! split up into traits.
//!

use crate::{
    error::{InvalidIdError, OomError},
    id::Id,
};

/// Generic allocator for IDs of type `Self::Id`.
pub trait Allocator {
    /// The ID type this allocator manages.
    type Id: Id<Allocator = Self>;

    /// Checks if `id` is a valid ID.
    ///
    /// This can return `false` for example if the ID has been deleted.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if `id` was allocated by a different allocator.
    fn is_valid(&self, id: &Self::Id) -> bool;
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

/// Trait implemented by allocators that can delete IDs, atomically and without additional
/// arguments.
pub trait Delete: Allocator {
    /// Deletes a previously created ID.
    ///
    /// In case your allocator supports atomic ID creation, you should implement this for `&Self`,
    /// too.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if `id` was allocated by a different allocator.
    fn delete(&mut self, id: &Self::Id) -> Result<(), InvalidIdError<Self::Id>>;
}
