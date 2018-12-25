//! Provides a generic interface for storages.

use std::borrow::Borrow;

use crate::id::{Id, ValidId};

/// Interface for getting a component based on an ID immutably.
pub trait Get: Storage {
    /// Retrieves the component associated with `id`.
    fn get<V>(&self, id: &V) -> Option<&Self::Component>
    where
        V: ValidId<Self::Id>;
}

/// Interface for getting a component based on an ID mutably.
pub trait GetMut: Storage {
    /// Retrieves the component associated with `id`.
    fn get_mut<V>(&mut self, id: &V) -> Option<&Self::Component>
    where
        V: ValidId<Self::Id>;
}

/// Interface for inserting (ID, component) pairs.
pub trait Insert: Storage {
    /// Inserts `component` and associates it with `id`.
    ///
    /// Returns the previous component that was associated with `id` if there was any.
    fn insert<V>(&mut self, id: V, component: Self::Component) -> Option<Self::Component> // TODO: this is not safe for continuous IDs (storage may leave holes)
    where
        V: ValidId<Self::Id>;
}

pub trait Storage {
    type Id: Id;
    type Component;
}
