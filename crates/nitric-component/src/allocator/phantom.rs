use crate::allocator::Allocator;
use crate::id::Id;
use std::marker::PhantomData;

/// Phantom allocator type required to implement `Id` for wrapper IDs that uphold additional
/// guarantees. This can never be instantiated and is only meant to be used for the associated
/// `Self::Allocator` type.
///
/// # Generics
///
/// * `ID`: wrapper ID
/// * `O`: original ID
pub struct PhantomAllocator<'a, ID, O> {
    _marker1: PhantomData<&'a ID>,
    _marker2: PhantomData<&'a O>,
    never: Never,
}

// Unsafety: `is_valid` holds the contract (as it can never be called).
unsafe impl<'a, ID, O> Allocator for PhantomAllocator<'a, ID, O>
where
    ID: Id<Allocator = Self> + 'a,
    O: Id + 'a,
{
    type Id = ID;

    fn is_valid(&self, _: &<Self as Allocator>::Id) -> bool {
        match self.never {}
    }

    fn num_valid(&self) -> usize {
        match self.never {}
    }

    fn num_valid_hint(&self) -> Option<usize> {
        match self.never {}
    }
}

enum Never {}
