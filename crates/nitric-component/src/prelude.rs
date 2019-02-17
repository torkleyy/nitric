//! Prelude module, exporting commonly used types. Meant to be imported using a
//! wildcard import (`use nitric_component::prelude::*`).

pub use crate::{
    allocator::{Allocator, Create, CreateChecked, Delete, MergeDeleted, Merger},
    id::{Id, MergingDeletion},
    storage::Storage,
};
