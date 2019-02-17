//! Implementations of the generic interfaces provided by this crate.

pub use self::{
    flat_allocator::FlatAllocator, flat_bit_set::FlatBitSet, flat_id::FlatUsize,
    usize_allocator::UsizeAllocator,
};

mod flat_allocator;
mod flat_bit_set;
mod flat_id;
mod type_safe;
mod usize_allocator;
