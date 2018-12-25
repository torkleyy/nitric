//! Module defining the `BitSet` trait.

/// `BitSet` trait which may or may not be hierarchical. This structure is used as storage mask
/// to determine if components exist for certain IDs.
pub unsafe trait BitSet: Sized + Default {
    /// Creates a `BitSet` with no bits set.
    fn empty_bit_set() -> Self {
        Default::default()
    }

    /// Adds a bit to the bit set, returning the previous value.
    ///
    /// Does nothing (and returns `true`) if the bit was set already.
    fn add(&mut self, bit: usize) -> bool;
    /// Removes a bit from the bit set, returning the previous value.
    ///
    /// Does nothing (and returns `false`) if the bit was zero already.
    fn remove(&mut self, bit: usize) -> bool;

    /// Checks if `bit` is set.
    fn contains(&self, bit: usize) -> bool;
}
