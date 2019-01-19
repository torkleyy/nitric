use crate::{
    allocator::{Allocator, Create, CreateChecked, Delete, MergeDeleted, Merger},
    error::{InvalidIdError, OomError},
    id::{CheckedId, SparseLinear, ValidId},
    impls::{FlatUsize, UsizeAllocator},
    util::Reference,
};

/// Wrapper for `UsizeAllocator` returning `FlatUsize`.
#[derive(Debug)]
pub struct FlatAllocator {
    inner: UsizeAllocator,
    merger: Reference,
}

impl FlatAllocator {
    /// Creates a fresh allocator and its associated merger for deleting IDs.
    pub fn new() -> (Self, Merger<Self>) {
        let merger = Merger::new();

        let alloc = FlatAllocator {
            inner: Default::default(),
            merger: merger.instance_id().reference(),
        };

        (alloc, merger)
    }
}

impl Allocator<FlatUsize> for FlatAllocator {
    fn is_valid(&self, id: &FlatUsize) -> bool {
        self.inner.is_valid(id.clone().into())
    }

    fn num_valid(&self) -> usize {
        self.inner.num_valid()
    }

    fn num_valid_hint(&self) -> (usize, Option<usize>) {
        self.inner.num_valid_hint()
    }
}

impl Create<FlatUsize> for FlatAllocator {
    #[inline]
    fn create(&mut self) -> Result<FlatUsize, OomError> {
        self.inner.create().map(From::from)
    }
}

impl CreateChecked<FlatUsize> for FlatAllocator {
    fn create_checked<'merger>(
        &mut self,
        merger: &'merger Merger<Self>,
    ) -> Result<CheckedId<'merger, FlatUsize>, OomError> {
        self.inner
            .create()
            .map(move |id| CheckedId::new_from_fields(id.into(), id, merger))
    }
}

impl Delete<FlatUsize> for FlatAllocator {
    #[inline]
    fn is_flagged<V>(&self, id: &V) -> bool
    where
        V: ValidId<FlatUsize>,
    {
        self.inner.is_flagged(id.as_inner().as_usize())
    }

    fn delete<V>(&mut self, id: &V)
    where
        V: ValidId<FlatUsize>,
    {
        self.inner.delete_valid(id.as_inner().as_usize())
    }

    fn try_delete(&mut self, id: &FlatUsize) -> Result<(), InvalidIdError<FlatUsize>> {
        self.inner
            .try_delete(id.clone().into())
            .map_err(|e| InvalidIdError(From::from(e.0)))
    }
}

impl MergeDeleted<FlatUsize> for FlatAllocator {
    fn merge_deleted(&mut self, merger: &mut Merger<Self>) -> Vec<FlatUsize> {
        merger.instance_id().assert_eq(&self.merger);

        self.inner
            .merge_deleted()
            .iter()
            .cloned()
            .map(Into::into)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn assert_deleted() {
        let (mut alloc, mut merger) = FlatAllocator::new();

        let a = alloc.create().unwrap();
        alloc.assert_deleted(&a);

        assert!(alloc.is_valid(&a));

        alloc.merge_deleted(&mut merger);

        assert!(!alloc.is_valid(&a));
        alloc.assert_deleted(&a);
        assert!(!alloc.is_valid(&a));
    }

    #[test]
    fn checked() {
        let (mut alloc, mut merger) = FlatAllocator::new();

        let a = alloc.create().unwrap();
        let b = alloc.create_checked(&merger).unwrap();
        let c = alloc.create_checked(&merger).unwrap();

        println!("{}, {}", b.as_usize(), c.as_usize());

        if let Ok(a) = a.try_as_key(&alloc) {
            println!("a: {}", a);
        }

        alloc.merge_deleted(&mut merger);

        // println!("{}, {}", b.as_usize(), c.as_usize()); <-- would fail since we cannot hold
        //                                                     `merger` until here
    }

    #[test]
    fn try_delete() {
        let (mut alloc, mut merger) = FlatAllocator::new();

        let a = alloc.create().unwrap();
        let b = alloc.create_checked(&merger).unwrap();

        alloc.try_delete(&a).unwrap();
        alloc.try_delete(b.id()).unwrap();
        alloc.merge_deleted(&mut merger);

        assert_eq!(alloc.num_valid(), 0);
        assert_eq!(alloc.num_valid_hint().1, Some(2));
    }
}
