//! Provides a generic ID interface.
//!
//! IDs are used as keys for component storages. They can be continuous or
//! sparse, while only the latter one allows to delete arbitrary ids after
//! creation.

pub use self::checked::CheckedId;

use std::{borrow::Cow, fmt::Debug, hash::Hash};

use crate::{allocator::Allocator, bit_set::BitSet, error::InvalidIdError};

mod checked;

/// A trait that marks an ID as continuous. The following properties are
/// required to hold:
///
/// If `x` is a valid ID, `x - 1` is a valid ID unless `x == ZERO`.
/// `allocator.num_valid() - 1` is a valid ID unless `self.num_valid() == ZERO`.
///
/// # Memory safety
///
/// A storage may not rely on this property for memory safety reasons. Instead,
/// it may panic in case this contract is violated.
pub trait Continuous: Id<Key = usize> {}

/// Top-level trait that all IDs must implement.
pub trait Id: Clone + Debug + Sized {
    /// The allocator which manages IDs of this type.
    type Allocator: Allocator<Self> + ?Sized;

    /// The unique key of this ID.
    type Key: Clone + Debug + Hash + Eq + Sized;

    /// Returns the key if this ID is valid, or an `InvalidIdError` otherwise.
    fn try_as_key(
        &self,
        allocator: &Self::Allocator,
    ) -> Result<Cow<Self::Key>, InvalidIdError<Self>>;

    /// Returns the inner key without checking whether or not the `Id` is valid.
    fn as_key_unchecked(&self) -> Cow<Self::Key>;
}

/// An ID that will only turn invalid when merged.
pub trait MergingDeletion: Id {
    /// The merger for this ID. You should make sure that the merger cannot be
    /// duplicated. It's usually a good idea to use
    /// `nitric_component::allocator::Merger` for this.
    type Merger;

    /// Checks if this ID is valid and returns a `CheckedId` which implements
    /// `ValidId`, or an error in case `self` is invalid.
    ///
    /// This is useful if
    ///
    /// * a function requires the `ValidId` trait and your ID may be invalid
    /// * you need to pass your ID to several functions
    fn checked<'merger>(
        self,
        allocator: &Self::Allocator,
        merger: &'merger Self::Merger,
    ) -> Result<CheckedId<'merger, Self>, InvalidIdError<Self>> {
        self.try_as_key(allocator)
            .map(|u| u.into_owned())
            .map(|u: Self::Key| CheckedId::new_from_fields(self, u, merger))
    }
}

/// An ID that can be used for sparse storages and behaves somewhat linear,
/// meaning it starts at zero and does not use higher numbers than necessary.
///
/// # Associated `Key`
///
/// `SparseLinear` requires a `usize` key which  is expected to be linear; if
/// this returns random / very big / very sparse numbers it will cause the
/// storage to grow a lot!
///
/// In short: it is expected to be produced by some sort of counter starting at
/// zero.
pub trait SparseLinear: Id<Key = usize> {
    /// This is the bit set type that can be used by the storage. A storage
    /// which maps this ID type stores a field of type `Self::BitSet`, which
    /// will be used to check if a component exists for a particular ID.
    type BitSet: BitSet;

    /// Convenience method for retrieving the `usize` key of this ID.
    /// **This does not guarantee validity and is equivalent to
    /// `as_key_unchecked`.**
    fn as_usize(&self) -> usize {
        self.as_key_unchecked().into_owned()
    }
}

/// Represents an ID that is guaranteed to be valid.
///
/// If you have an ID that may be valid or not, consider using `.checked()` to
/// retrieve a `Checked` instance which implements `ValidId`.
///
/// # Generics
///
/// * `O`: "Original" ID; this might be `Self` if the ID is always valid, or the
///   wrapped ID (this is the case for `CheckedId`)
///
/// # Contract
///
/// An ID implementing this type must be valid in terms of `Allocator::is_valid`
/// for as long as it can be accessed.
pub trait ValidId<O: Id>: Id<Key = O::Key> {
    /// Returns a reference to the original ID.
    fn as_inner(&self) -> &O;

    /// Consumes `self` and returns the owned, original ID.
    fn into_inner(self) -> O;

    /// Returns the key of this ID.
    ///
    /// This operation cannot fail and is only available for IDs that have this
    /// property ensured at the type-level. For a dynamic version see
    /// `Id::try_as_key`.
    fn as_key(&self) -> Cow<Self::Key>;
}

/// An ID that wraps another ID. It mirrors all basic ID property the wrapped ID
/// has, through respective blanket implementations.
///
/// # References
///
/// This is used for
///
/// * `ValidId`
pub trait WrapperId: Id {
    /// The wrapped, original ID type.
    type Original: Id<Key = Self::Key>;

    /// Returns an immutable reference to the inner ID.
    fn as_inner(&self) -> &Self::Original;

    /// Consumes this ID, returning the inner ID.
    fn into_inner(self) -> Self::Original;
}

impl<T> Continuous for T
where
    T: Id<Key = usize> + WrapperId,
    T::Original: Continuous,
{
}

impl<T> MergingDeletion for T
where
    T: WrapperId,
    T::Original: Id<Key = T::Key> + MergingDeletion,
{
    type Merger = <T::Original as MergingDeletion>::Merger;
}

impl<T> SparseLinear for T
where
    T: Id<Key = usize> + WrapperId,
    T::Original: SparseLinear,
{
    type BitSet = <T::Original as SparseLinear>::BitSet;

    fn as_usize(&self) -> usize {
        self.as_inner().as_usize()
    }
}
