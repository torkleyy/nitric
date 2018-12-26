use crate::id::Id;
use crate::allocator::Allocator;

pub struct FlatAllocator {

}

impl FlatAllocator {

}

unsafe impl Allocator for FlatAllocator {
    type Id = FlatUsize;

    fn is_valid(&self, id: &<Self as Allocator>::Id) -> bool {
        unimplemented!()
    }

    fn num_valid(&self) -> usize {
        unimplemented!()
    }

    fn num_valid_hint(&self) -> Option<usize> {
        unimplemented!()
    }
}

/// A `usize`-based ID with a sparse allocator and a `FlatBitSet`.
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
    pub fn get(&self) -> usize {
        self.inner
    }
}

impl Id for FlatUsize {
    type Allocator = FlatAllocator;
}
