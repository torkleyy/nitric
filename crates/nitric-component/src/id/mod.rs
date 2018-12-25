//! Provides a generic ID interface.
//!
//! IDs are used as keys for component storages. They can be continuous or sparse, while only
//! the latter one allows to delete arbitrary ids after creation.

pub use self::checked::CheckedId;

use std::{fmt::Debug, hash::Hash};

use crate::{allocator::Allocator, bit_set::BitSet, error::InvalidIdError};

mod checked;

/// An ID that has a unique `usize` representation as long as it is valid.
pub trait AsUsize: Id {
    /// Checks if this ID is valid and returns a `CheckedId` which implementes `ValidId`, or an
    /// error in case `self` is invalid.
    ///
    /// This is useful if
    ///
    /// * a function requires the `ValidId` trait and your ID may be invalid
    /// * you need to pass your ID to several functions
    fn checked(
        self,
        allocator: &Self::Allocator,
    ) -> Result<CheckedId<'_, Self>, InvalidIdError<Self>> {
        // Unsafety: this is safe because the allocator is guaranteed to keep IDs valid for as long
        // as it is borrowed immutably. We ensure it's valid
        self.try_as_usize(allocator)
            .map(|u| unsafe { CheckedId::new_from_fields(self, u, allocator) })
    }

    /// Returns the `usize` representation of this ID, failing if the ID is invalid.
    ///
    /// See `ValidId::as_usize` for a version that cannot fail.
    ///
    /// # Contract
    ///
    /// Returning an `Ok` value <-> ID is valid.
    fn try_as_usize(&self, allocator: &Self::Allocator) -> Result<usize, InvalidIdError<Self>>;
}

/// A trait that marks an ID as continuous. The following properties are required to hold:
///
/// If `x` is a valid ID, `x - 1` is a valid ID unless `x == ZERO`.
/// `allocator.num_valid() - 1` is a valid ID unless `self.num_valid() == ZERO`.
///
/// # Memory safety
///
/// A storage may not rely on this property for memory safety reasons.
pub trait Continuous: AsUsize {}

/// Top-level trait that all IDs must implement.
pub trait Id: Clone + Debug + Hash + Eq + Sized {
    /// The allocator which manages IDs of this type.
    type Allocator: Allocator<Id = Self> + ?Sized;
}

/// An ID that can be used for sparse storages and behaves somewhat linear, meaning it starts at
/// zero and does not use higher numbers than necessary.
///
/// # `AsUsize` implementation
///
/// Notice that the `AsUsize` representation is expected to be linear; if this returns random /
/// very big / very sparse numbers it will cause the storage to grow a lot!
///
/// In short: it is expected to be produced by some sort of counter starting at zero.
pub trait SparseLinear: AsUsize + Id {
    /// This is the bit set type that will be used by the storage. A storage which maps this ID
    /// type stores a field of type `Self::BitSet`, which will be used to check if a component
    /// exists for a particular ID.
    type BitSet: BitSet;
}

/// Represents an ID that is guaranteed to be valid.
///
/// If you have an ID that may be valid or not, consider using `.checked()` to retrieve a `Checked`
/// instance which implements `ValidId`.
///
/// # Generics
///
/// * `O`: "Original" ID; this might be `Self` if the ID is always valid, or a wrapper ID
///   (like `CheckedId`)
///
/// # Contract
///
/// An ID implementing this type must be valid in terms of `Allocator::is_valid` for as long as it
/// can be accessed.
///
/// # Memory safety
///
/// Implementing this type without meeting the contract may produce memory safety bugs.
pub unsafe trait ValidId<O: Id>: Id {
    /// Returns the `usize` representation of this ID.
    ///
    /// This operation cannot fail and is only available for IDs that have this property ensured
    /// at the type-level. For a dynamic version see `AsUsize::try_as_usize`.
    fn as_usize(&self) -> usize;

    /// Borrows the wrapped ID.
    fn as_inner(&self) -> &O;

    /// Moves the wrapped ID out.
    fn into_inner(self) -> O;
}
