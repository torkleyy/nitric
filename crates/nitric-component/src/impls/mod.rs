//! Implementations of the generic interfaces provided by this crate.

pub use self::flat_allocator::FlatAllocator;
pub use self::flat_bit_set::FlatBitSet;
pub use self::flat_id::FlatUsize;
pub use self::tv_storage::TvStorage;
pub use self::type_safe::{TypeSafeAllocator, TypeSafeId};
pub use self::usize_allocator::UsizeAllocator;

mod flat_allocator;
mod flat_bit_set;
mod flat_id;
mod tv_storage;
mod type_safe;
mod usize_allocator;
