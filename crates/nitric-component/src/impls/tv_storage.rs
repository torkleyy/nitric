use crate::{
    bit_set::BitSet,
    id::{SparseLinear, ValidId},
    storage::{Get, GetMut, Insert, Remove, Storage},
};
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

/// A component storage implementation, providing a mapping from IDs to components using two `Vec`s.
///
/// (short: "Triple Vec Storage")
///
/// The first `Vec` is a sparse vector which maps an ID to a data index.
/// The second `Vec` is a dense vector which maps a data index to a component.
///
/// This turned out to be a good default and outperforms other implementations in most benchmarks.
///
/// ## Generics
///
/// * `ID`: The ID, which is used as key.
/// * `C`: The component, which is the value.
pub struct TvStorage<ID, C>
where
    ID: SparseLinear,
{
    data: Vec<C>,
    data_indices: Vec<usize>,
    ids: Vec<usize>,
    mask: ID::BitSet,
}

impl<ID, C> TvStorage<ID, C>
where
    ID: SparseLinear,
{
    /// Creates a new, empty component storage.
    pub fn new() -> Self {
        Default::default()
    }
}

impl<ID, C> Debug for TvStorage<ID, C>
where
    ID: SparseLinear,
    C: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.ids.iter().zip(self.ids.iter()))
            .finish()
    }
}

impl<ID, C> Default for TvStorage<ID, C>
where
    ID: SparseLinear,
{
    fn default() -> Self {
        TvStorage {
            data: vec![],
            data_indices: vec![],
            ids: vec![],
            mask: Default::default(),
        }
    }
}

impl<ID, C> Get for TvStorage<ID, C>
where
    ID: SparseLinear,
{
    #[inline]
    fn get<V>(&self, id: &V) -> Option<&<Self as Storage>::Component>
    where
        V: ValidId<Self::Id>,
    {
        let id: usize = id.as_usize();
        if self.mask.contains(id) {
            let data_index = self.data_indices[id];
            let data = &self.data[data_index];

            Some(data)
        } else {
            None
        }
    }
}

impl<ID, C> GetMut for TvStorage<ID, C>
where
    ID: SparseLinear,
{
    #[inline]
    fn get_mut<V>(&mut self, id: &V) -> Option<&mut <Self as Storage>::Component>
    where
        V: ValidId<Self::Id>,
    {
        let id: usize = id.as_usize();
        if self.mask.contains(id) {
            let data_index = self.data_indices[id];
            let data = &mut self.data[data_index];

            Some(data)
        } else {
            None
        }
    }
}

impl<ID, C> Insert for TvStorage<ID, C>
where
    ID: SparseLinear,
{
    #[inline]
    fn insert<V>(
        &mut self,
        id_orig: V,
        component: <Self as Storage>::Component,
    ) -> Option<<Self as Storage>::Component>
    where
        V: ValidId<Self::Id>,
    {
        use std::iter::repeat;
        use std::mem::replace;

        let id: usize = id_orig.as_usize();

        if self.mask.add(id) {
            Some(replace(self.get_mut(&id_orig).unwrap(), component))
        } else {
            if self.data_indices.len() <= id {
                let delta = id + 1 - self.data_indices.len();
                self.data_indices.extend(repeat(0).take(delta));
            }

            self.data_indices[id] = self.data.len();
            self.ids.push(id);
            self.data.push(component);

            None
        }
    }
}

impl<ID, C> Remove for TvStorage<ID, C>
where
    ID: SparseLinear,
{
    #[inline]
    fn remove<V>(&mut self, id: &V) -> Option<<Self as Storage>::Component>
    where
        V: ValidId<Self::Id>,
    {
        let id = id.as_usize();

        if self.mask.remove(id) {
            let data_index = self.data_indices[id];
            // Grab the usize representation of the ID at the end
            let last_id = *self.ids.last().unwrap();

            // the data for `last_index` will be found under `date_index` now, since we swap
            // the last one with `data_index`.
            self.data_indices[last_id] = data_index;
            debug_assert_eq!(self.ids.swap_remove(data_index), id);

            Some(self.data.swap_remove(data_index))
        } else {
            None
        }
    }
}

impl<ID, C> Storage for TvStorage<ID, C>
where
    ID: SparseLinear,
{
    type Id = ID;
    type Component = C;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        allocator::Create,
        id::AsUsize,
        impls::{FlatAllocator, FlatUsize},
    };

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Comp(u32);

    fn new_storage() -> TvStorage<FlatUsize, Comp> {
        TvStorage::new()
    }

    #[test]
    fn new() {
        let empty = new_storage();
        let mut alloc = FlatAllocator::new();

        let ids = (0..100)
            .map(|_| alloc.create())
            .collect::<Result<Vec<FlatUsize>, _>>()
            .unwrap();

        for id in &ids {
            assert_eq!(empty.get(&id.checked(&alloc).unwrap()), None);
        }
    }

    #[test]
    fn insert() {
        let mut storage = new_storage();
        let mut alloc = FlatAllocator::new();

        let ids = (0..100)
            .map(|_| alloc.create())
            .collect::<Result<Vec<FlatUsize>, _>>()
            .unwrap();

        assert!(storage
            .insert(ids[4].clone().checked(&alloc).unwrap(), Comp(41))
            .is_none());
        assert!(storage
            .insert(ids[8].clone().checked(&alloc).unwrap(), Comp(21))
            .is_none());
        assert!(storage
            .insert(ids[92].clone().checked(&alloc).unwrap(), Comp(17))
            .is_none());

        assert_eq!(
            storage.insert(ids[8].clone().checked(&alloc).unwrap(), Comp(210)),
            Some(Comp(21))
        );
    }

    #[test]
    fn remove() {
        let mut storage = new_storage();
        let mut alloc = FlatAllocator::new();

        let ids = (0..100)
            .map(|_| alloc.create())
            .collect::<Result<Vec<FlatUsize>, _>>()
            .unwrap();
        let checked = ids
            .into_iter()
            .map(|i| i.checked(&alloc))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(storage.remove(&checked[3]).is_none());
        assert!(storage.remove(&checked[11]).is_none());
        assert!(storage.remove(&checked[65]).is_none());

        storage.insert(checked[25].clone(), Comp(25));

        assert_eq!(storage.remove(&checked[25]), Some(Comp(25)));
        assert_eq!(storage.remove(&checked[25]), None);
    }

    #[test]
    fn get() {
        let mut storage = new_storage();
        let mut alloc = FlatAllocator::new();

        let ids = (0..100)
            .map(|_| alloc.create())
            .collect::<Result<Vec<FlatUsize>, _>>()
            .unwrap();
        let checked = ids
            .into_iter()
            .map(|i| i.checked(&alloc))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        storage.insert(checked[12].clone(), Comp(25));

        assert_eq!(storage.get(&checked[12]), Some(&Comp(25)));
        assert_eq!(storage.get(&checked[12]), Some(&Comp(25)));
        assert_eq!(storage.get(&checked[25]), None);
        assert_eq!(storage.get(&checked[25]), None);
    }

    #[test]
    fn get_mut() {
        let mut storage = new_storage();
        let mut alloc = FlatAllocator::new();

        let ids = (0..100)
            .map(|_| alloc.create())
            .collect::<Result<Vec<FlatUsize>, _>>()
            .unwrap();
        let checked = ids
            .into_iter()
            .map(|i| i.checked(&alloc))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        storage.insert(checked[12].clone(), Comp(25));

        assert_eq!(storage.get_mut(&checked[25]), None);
        assert_eq!(storage.get_mut(&checked[12]), Some(&mut Comp(25)));
        *storage.get_mut(&checked[12]).unwrap() = Comp(11);

        assert_eq!(storage.get_mut(&checked[12]), Some(&mut Comp(11)));
    }
}
