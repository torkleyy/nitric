use crate::{
    allocator::{
        MergeDeleted,
        Allocator,
        Create,
        CreateChecked,
        Delete
    },
    error::{InvalidIdError, OomError},
    id::{CheckedId, ValidId},
    impls::{
        FlatUsize,
        UsizeAllocator
    }
};

/// Wrapper for `UsizeAllocator` returning `FlatUsize`.
#[derive(Default)]
pub struct FlatAllocator {
    inner: UsizeAllocator,
}

impl FlatAllocator {
    /// Creates a fresh allocator.
    pub fn new() -> Self {
        Default::default()
    }
}

unsafe impl Allocator<FlatUsize> for FlatAllocator {
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
    #[inline]
    fn create_checked(&mut self) -> Result<CheckedId<'_, FlatUsize>, OomError> {
        self.inner
            .create()
            .map(move |id| unsafe { CheckedId::new_from_fields(id.into(), id, &*self) })
    }
}

impl Delete<FlatUsize> for FlatAllocator {
    #[inline]
    fn is_flagged<V>(&mut self, id: &V) -> bool
    where
        V: ValidId<FlatUsize>,
    {
        self.inner.is_flagged(id.as_usize())
    }

    fn delete<V>(&mut self, id: &V)
    where
        V: ValidId<FlatUsize>,
    {
        self.inner.delete_valid(id.as_usize())
    }

    fn try_delete(&mut self, id: &FlatUsize) -> Result<(), InvalidIdError<FlatUsize>> {
        self.inner
            .try_delete(id.clone().into())
            .map_err(|e| InvalidIdError(From::from(e.0)))
    }
}

impl MergeDeleted<FlatUsize> for FlatAllocator {
    fn merge_deleted(&mut self) -> Vec<FlatUsize> {
        self.inner
            .merge_deleted()
            .into_iter()
            .cloned()
            .map(Into::into)
            .collect()
    }
}
