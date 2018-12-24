use std::fmt::Debug;

use crate::allocator::Allocator;

// TODO: this is not compatible with all use cases / too complicated for some
// TODO: consider removing trait

/// Generic ID interface.
///
/// IDs are used as keys for component storages. They can be continuous or sparse, while only
/// the latter one allows to delete ids after creation.
pub trait Id: Clone + Debug + Sized {
    /// The allocator which manages IDs of this type.
    type Allocator: Allocator<Id = Self> + ?Sized;

    /// This is an associated type that will be used by the storage. A storage which maps this ID
    /// type stores a field of type `Self::Mask`, which will be used to check if a component exists
    /// for a particular ID.
    ///
    /// ## Examples
    ///
    /// * length (`usize`) if components are inserted continuously
    /// * a bit set for sparsely stored components
    type Mask: Sized;

    /// Creates an empty mask, which is the initial state for every `Storage`.
    fn empty_mask() -> Self::Mask;

    /// Returns a `usize` which represents this ID. Notice that this representation is expected to
    /// be linear; if this returns random / very big / very sparse numbers it will cause the
    /// `Storage` to grow a lot!
    ///
    /// In short: this is expected to be produced by some sort of counter starting at zero.
    fn as_usize(&self, allocator: &Self::Allocator) -> usize; // TODO: return Result
}
