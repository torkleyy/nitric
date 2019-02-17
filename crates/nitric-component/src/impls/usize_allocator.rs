use crate::{
    bit_set::BitSet,
    error::{InvalidIdError, OomError},
    impls::FlatBitSet,
};

/// A simple, non-atomic allocator that tries to return a free `usize`, bumps
/// the counter otherwise.
#[derive(Debug, Default)]
pub struct UsizeAllocator {
    /// Valid IDs
    alive: FlatBitSet,
    counter: usize,
    killed: Vec<usize>,
    /// IDs flagged for deletion
    flagged: FlatBitSet,
}

impl UsizeAllocator {
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

impl UsizeAllocator {
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
        let id = self
            .killed
            .pop()
            .or_else(|| self.checked_inc())
            .ok_or(OomError)?;

        self.alive.add(id);

        Ok(id)
    }

    /// Mirrors `Delete::is_delete`
    #[inline]
    pub fn is_flagged(&self, id: usize) -> bool {
        self.flagged.contains(id)
    }

    /// Mirrors `Delete::delete`
    #[inline]
    pub fn delete_valid(&mut self, id: usize) {
        debug_assert!(self.alive.contains(id));

        self.flagged.add(id);
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

    /// Mirrors `MergeDeleted::merge_deleted`
    pub fn merge_deleted(&mut self) -> &[usize] {
        let start = self.killed.len();

        while let Some(id) = self.flagged.pop_front() {
            self.alive.remove(id);
            self.killed.push(id);
        }

        &self.killed[start..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let empty = UsizeAllocator::new();

        for i in 0..100 {
            assert_eq!(empty.is_valid(i), false);
        }
    }

    #[test]
    fn checked_inc() {
        let mut empty = UsizeAllocator::new();

        assert_eq!(empty.counter, 0);
        assert_eq!(empty.checked_inc(), Some(0));
        assert_eq!(empty.checked_inc(), Some(1));
        assert_eq!(empty.checked_inc(), Some(2));

        empty.counter = usize::max_value();

        assert_eq!(empty.checked_inc(), None);
    }

    #[test]
    fn is_valid() {
        let mut alloc = UsizeAllocator::new();

        for i in 0..3 {
            alloc.create().unwrap();
            assert_eq!(alloc.is_valid(i), true);
        }

        assert_eq!(alloc.is_valid(3), false);

        for i in 0..3 {
            assert_eq!(alloc.is_valid(i), true);
            alloc.delete_valid(i);
            assert_eq!(alloc.is_valid(i), true);
        }

        alloc.merge_deleted();

        for i in 0..3 {
            assert_eq!(alloc.is_valid(i), false);
        }

        for _ in 0..3 {
            alloc.create().unwrap();
        }

        for i in 0..3 {
            assert_eq!(alloc.is_valid(i), true);
        }

        assert_eq!(alloc.is_valid(3), false);

        for i in 0..3 {
            assert_eq!(alloc.is_valid(i), true);
            alloc.delete_valid(i);
            assert_eq!(alloc.is_valid(i), true);
        }
    }

    #[test]
    fn num_valid() {
        let mut alloc = UsizeAllocator::new();

        for i in 0..3 {
            assert_eq!(alloc.num_valid(), i);
            alloc.create().unwrap();
            assert_eq!(alloc.num_valid(), i + 1);
        }

        assert_eq!(alloc.num_valid(), 3);

        for i in 0..3 {
            alloc.delete_valid(i);
            assert_eq!(alloc.num_valid(), 3);
        }

        alloc.merge_deleted();
        assert_eq!(alloc.num_valid(), 0);

        let a = alloc.create().unwrap();
        assert_eq!(alloc.num_valid(), 1);
        let b = alloc.create().unwrap();
        alloc.delete_valid(b);
        assert_eq!(alloc.num_valid(), 2);
        alloc.merge_deleted();
        assert_eq!(alloc.num_valid(), 1);
        alloc.delete_valid(a);
        alloc.merge_deleted();

        for i in 0..3 {
            assert_eq!(alloc.num_valid(), i);
            alloc.create().unwrap();
            assert_eq!(alloc.num_valid(), i + 1);
        }

        assert_eq!(alloc.num_valid(), 3);

        for i in 0..3 {
            alloc.delete_valid(i);
            assert_eq!(alloc.num_valid(), 3);
        }

        alloc.merge_deleted();
        assert_eq!(alloc.num_valid(), 0);
    }

    #[test]
    fn is_flagged() {
        let mut alloc = UsizeAllocator::new();

        for i in 0..100 {
            assert_eq!(alloc.is_flagged(i), false);
        }

        for i in 0..3 {
            alloc.create().unwrap();
            assert_eq!(alloc.is_flagged(i), false);
        }

        for i in 0..3 {
            assert_eq!(alloc.is_flagged(i), false);
            alloc.delete_valid(i);
            assert_eq!(alloc.is_flagged(i), true);
        }

        alloc.merge_deleted();

        for i in 0..3 {
            assert_eq!(alloc.is_flagged(i), false);
        }

        for _ in 0..3 {
            alloc.create().unwrap();
        }

        for i in 0..3 {
            assert_eq!(alloc.is_flagged(i), false);
        }

        assert_eq!(alloc.is_flagged(3), false);

        for i in 0..3 {
            assert_eq!(alloc.is_flagged(i), false);
            alloc.delete_valid(i);
            assert_eq!(alloc.is_flagged(i), true);
        }
    }

    #[test]
    fn num_valid_hint() {
        let mut alloc = UsizeAllocator::new();

        assert_eq!(alloc.num_valid_hint(), (0, Some(0)));

        alloc.create().unwrap();

        assert_eq!(alloc.num_valid_hint(), (0, Some(1)));

        alloc.create().unwrap();
        alloc.create().unwrap();
        alloc.create().unwrap();

        assert_eq!(alloc.num_valid_hint(), (0, Some(4)));

        alloc.delete_valid(2);

        assert_eq!(alloc.num_valid_hint(), (0, Some(4)));

        alloc.merge_deleted();

        assert_eq!(alloc.num_valid_hint(), (0, Some(4)));
    }

    #[test]
    fn try_delete() {
        let mut alloc = UsizeAllocator::new();

        for i in 0..100 {
            assert_eq!(alloc.try_delete(i), Err(InvalidIdError(i)));
        }

        alloc.create().unwrap();
        alloc.create().unwrap();
        alloc.create().unwrap();

        for i in 0..3 {
            assert_eq!(alloc.try_delete(i), Ok(()));
            assert_eq!(alloc.try_delete(i), Ok(()));
        }

        alloc.merge_deleted();

        for i in 0..3 {
            assert_eq!(alloc.try_delete(i), Err(InvalidIdError(i)));
        }
    }
}
