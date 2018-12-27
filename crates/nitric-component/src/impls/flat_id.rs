use crate::{
    allocator::Allocator,
    error::InvalidIdError,
    id::{AsUsize, Id, SparseLinear},
    impls::{FlatAllocator, FlatBitSet},
};

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
    pub fn get_inner(&self) -> usize {
        self.inner
    }
}

impl AsUsize for FlatUsize {
    fn try_as_usize(&self, allocator: &FlatAllocator) -> Result<usize, InvalidIdError<Self>> {
        if allocator.is_valid(&self) {
            Ok(self.inner)
        } else {
            Err(InvalidIdError(*self))
        }
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

        assert_eq!(id.try_as_usize(&alloc), Ok(id.get_inner()));

        alloc.delete(&id.checked(&alloc, &merger).unwrap());
        alloc.merge_deleted(&mut merger);

        assert_eq!(id.try_as_usize(&alloc), Err(InvalidIdError(id)));
    }
}
