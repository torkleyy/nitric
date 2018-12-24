use crate::id::Id;

/// A component storage implementation, providing a mapping from IDs to components using two `Vec`s.
///
/// (short: "Double Vec Storage")
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
pub struct DvStorage<ID, C>
where
    ID: Id,
{
    data: Vec<C>,
    indices: Vec<usize>,
    mask: ID::Mask,
}

impl<ID, C> DvStorage<ID, C>
where
    ID: Id,
{
    /// Creates a new, empty component storage.
    pub fn new() -> Self {
        Default::default()
    }
}

impl<ID, C> Default for DvStorage<ID, C>
where
    ID: Id,
{
    fn default() -> Self {
        DvStorage {
            data: vec![],
            indices: vec![],
            mask: ID::empty_mask(),
        }
    }
}
