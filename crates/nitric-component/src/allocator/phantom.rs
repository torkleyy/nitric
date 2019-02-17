use crate::{allocator::Allocator, id::Id};

/// Phantom allocator type required to implement `Id` for wrapper IDs that
/// uphold additional guarantees. This can never be instantiated and is only
/// meant to be used for the associated `Self::Allocator` type.
pub struct PhantomAllocator {
    never: Never,
}

impl<ID> Allocator<ID> for PhantomAllocator
where
    ID: Id<Allocator = Self>,
{
    fn is_valid(&self, _: &ID) -> bool {
        match self.never {}
    }

    fn num_valid(&self) -> usize {
        match self.never {}
    }

    fn num_valid_hint(&self) -> (usize, Option<usize>) {
        match self.never {}
    }
}

enum Never {}
