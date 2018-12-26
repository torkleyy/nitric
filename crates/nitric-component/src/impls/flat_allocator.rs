use crate::{
    allocator::{Allocator, Create, CreateChecked, Delete},
    bit_set::BitSet,
    error::{InvalidIdError, OomError},
    id::{AsUsize, CheckedId, Id, ValidId},
    impls::FlatBitSet,
};

/// A simple, non-atomic allocator that tries to return a free `usize`, bumps the counter otherwise.
#[derive(Default)]
pub struct FlatAllocator {
    /// Valid IDs
    alive: FlatBitSet,
    counter: usize,
    killed: Vec<usize>,
    /// IDs flagged for deletion
    flagged: FlatBitSet,
}

impl FlatAllocator {
    /// Creates a fresh allocator.
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    fn checked_inc(&mut self) -> Option<usize> {
        match self.counter.checked_add(1) {
            Some(new) => {
                let old = self.counter;
                self.counter = new;

                Some(old)
            }
            None => None,
        }
    }
}

impl FlatAllocator {
    /// Mirrors `Allocator::is_valid`
    #[inline]
    pub fn is_valid(&self, id: usize) -> bool {
        self.alive.contains(id)
    }

    /// Mirrors `Allocator::num_valid`
    pub fn num_valid(&self) -> usize {
        self.alive.count()
    }

    /// Mirrors `Allocator::num_valid_hint`
    #[inline]
    pub fn num_valid_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.counter))
    }

    /// Mirrors `Create::create`
    #[inline]
    pub fn create(&mut self) -> Result<usize, OomError> {
        self.killed
            .pop()
            .or_else(|| self.checked_inc())
            .ok_or(OomError)
    }

    /// Mirrors `Delete::delete`
    #[inline]
    pub fn delete_valid(&mut self, id: usize) {
        debug_assert!(self.alive.contains(id));

        if !self.flagged.add(id) {
            self.killed.push(id);
        }
    }

    /// Mirrors `Delete::try_delete`
    #[inline]
    pub fn try_delete(&mut self, id: usize) -> Result<(), InvalidIdError<usize>> {
        match self.is_valid(id) {
            true => self.delete_valid(id),
            false => return Err(InvalidIdError(id)),
        }

        Ok(())
    }
}

unsafe impl<ID> Allocator<ID> for FlatAllocator
where
    ID: Id<Allocator = Self> + From<usize> + Into<usize>,
{
    fn is_valid(&self, id: &ID) -> bool {
        self.is_valid(id.clone().into())
    }

    fn num_valid(&self) -> usize {
        self.num_valid()
    }

    fn num_valid_hint(&self) -> (usize, Option<usize>) {
        self.num_valid_hint()
    }
}

impl<ID> Create<ID> for FlatAllocator
where
    ID: Id<Allocator = Self> + From<usize> + Into<usize>,
{
    fn create(&mut self) -> Result<ID, OomError> {
        self.create().map(From::from)
    }
}

impl<ID> CreateChecked<ID> for FlatAllocator
where
    ID: Id<Allocator = Self> + AsUsize + From<usize> + Into<usize>,
{
    fn create_checked(&mut self) -> Result<CheckedId<'_, ID>, OomError> {
        <FlatAllocator as Create<ID>>::create(self)
            .map(move |id| unsafe { CheckedId::new_from_fields(id.clone(), id.into(), &*self) })
    }
}

impl<ID> Delete<ID> for FlatAllocator
where
    ID: Id<Allocator = Self> + AsUsize + From<usize> + Into<usize>,
{
    fn delete<V>(&mut self, id: &V)
    where
        V: ValidId<ID>,
    {
        self.delete_valid(id.as_usize())
    }

    fn try_delete(&mut self, id: &ID) -> Result<(), InvalidIdError<ID>> {
        self.try_delete(id.clone().into())
            .map_err(|e| InvalidIdError(From::from(e.0)))
    }
}

