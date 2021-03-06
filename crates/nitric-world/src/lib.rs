#![warn(missing_docs)]

//! Provides a `World` type which allows you to index resources with their type,
//! and an arbitrary key type.

use std::{any::TypeId, borrow::Borrow, hash::Hash};

use derive_new::new;
use hashbrown::HashMap;
use mopa::{mopafy, Any};

/// A collection of resources that can be accessed via their Type and an
/// arbitrary key type `K`.
#[derive(Default, new)]
pub struct World<K>
where
    K: Hash + Eq,
{
    #[new(default)]
    resources: HashMap<TypeId, HashMap<K, Box<Resource>>>,
}

impl<K: Hash + Eq> World<K> {
    /// Adds a resource to the world
    pub fn insert<T: Resource>(&mut self, k: K, v: T) -> Option<T> {
        self.resources
            .entry(TypeId::of::<T>())
            .or_insert_with(|| HashMap::new())
            .insert(k, Box::new(v) as Box<Resource>)
            .map(|b| *b.downcast().ok().expect("Unreachable"))
    }

    /// Retrieves an immutable reference to a resource from the world.
    pub fn get<T: Resource, Q: ?Sized>(&self, k: &Q) -> Option<&T>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|m| m.get(k).and_then(|b| b.downcast_ref()))
    }

    /// Retrieves a mutable reference to a resource from the world.
    pub fn get_mut<T: Resource, Q: ?Sized>(&mut self, k: &Q) -> Option<&mut T>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|m| m.get_mut(k).and_then(|b| b.downcast_mut()))
    }

    /// Removes a resource from the world.
    pub fn remove<T: Resource, Q: ?Sized>(&mut self, k: &Q) -> Option<T>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let mut kill_it = false;
        let ret = self.resources.get_mut(&TypeId::of::<T>()).and_then(|m| {
            let ret = m
                .remove(k)
                .map(|b| *b.downcast().ok().expect("Unreachable"));
            kill_it = m.len() == 0;
            ret
        });
        if kill_it {
            self.resources.remove(&TypeId::of::<T>());
        }
        ret
    }
}

/// A bundle trait automatically implemented for any type that is `Any + Send +
/// Sync`.
pub trait Resource: Any + Send + Sync + 'static {}

mopafy!(Resource);

impl<T> Resource for T where T: Any + Send + Sync {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_get_remove() {
        let mut world = World::new();
        assert_eq!(world.insert::<u32>(String::from("foo"), 2), None);
        assert_eq!(world.insert::<u32>(String::from("foo"), 1), Some(2));
        assert_eq!(world.get::<u32, _>("foo"), Some(&1));
        assert_eq!(world.get_mut::<u32, _>("foo"), Some(&mut 1));
        assert_eq!(world.get::<u32, _>("bar"), None);
        assert_eq!(world.get_mut::<u32, _>("bar"), None);
        assert_eq!(world.get::<i32, _>("foo"), None);
        assert_eq!(world.get_mut::<i32, _>("foo"), None);
        assert_eq!(world.remove::<i32, _>("foo"), None);
        assert_eq!(world.remove::<u32, _>("bar"), None);
        assert_eq!(world.remove::<u32, _>("foo"), Some(1));
        assert_eq!(world.remove::<u32, _>("foo"), None);
    }
}
