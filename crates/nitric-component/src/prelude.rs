//! Prelude module, exporting commonly used types. Meant to be imported using a wildcard import
//! (`use nitric_component::prelude::*`).

pub use crate::allocator::{Allocator, Create, CreateChecked, Delete, MergeDeleted, Merger};
pub use crate::id::{Id, MergingDeletion};
pub use crate::storage::{Get, GetMut, Insert, Remove};
