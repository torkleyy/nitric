use crate::id::MergingDeletion;
use crate::{
    allocator::{Allocator, Merger},
    error::InvalidIdError,
    id::{Id, SparseLinear},
    impls::{FlatAllocator, FlatBitSet},
};
use std::borrow::Cow;

/// A `usize`-based ID using the `FlatAllocator` and a `FlatBitSet`.
///
/// # Examples
///
/// ```
/// use nitric_component::impls::FlatUsize;
///
/// #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// pub struct ClientId(pub FlatUsize);
///
///
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FlatUsize {
    inner: usize,
}

impl FlatUsize {
    /// Returns the inner `usize`.
    pub fn into_usize(self) -> usize {
        self.inner
    }
}

impl From<usize> for FlatUsize {
    fn from(inner: usize) -> Self {
        FlatUsize { inner }
    }
}

impl Into<usize> for FlatUsize {
    fn into(self) -> usize {
        self.inner
    }
}

impl Id for FlatUsize {
    type Allocator = FlatAllocator;
    type Key = usize;

    fn try_as_key(
        &self,
        allocator: &Self::Allocator,
    ) -> Result<Cow<'_, Self::Key>, InvalidIdError<Self>> {
        match allocator.is_valid(self) {
            true => Ok(self.as_key_unchecked()),
            false => Err(InvalidIdError(*self)),
        }
    }

    fn as_key_unchecked(&self) -> Cow<'_, Self::Key> {
        Cow::Borrowed(&self.inner)
    }
}

impl MergingDeletion for FlatUsize {
    type Merger = Merger<Self::Allocator>;
}

impl SparseLinear for FlatUsize {
    type BitSet = FlatBitSet;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::*;

    #[test]
    fn invalid_id() {
        let (mut alloc, mut merger) = FlatAllocator::new();

        let id = alloc.create().unwrap();

        assert_eq!(
            id.try_as_key(&alloc).map(Cow::into_owned),
            Ok(id.into_usize())
        );

        alloc.delete(&id.checked(&alloc, &merger).unwrap());
        alloc.merge_deleted(&mut merger);

        assert_eq!(
            id.try_as_key(&alloc).map(Cow::into_owned),
            Err(InvalidIdError(id))
        );
    }
}
