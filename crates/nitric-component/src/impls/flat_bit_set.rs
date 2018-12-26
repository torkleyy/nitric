use crate::bit_set::BitSet;

use std::mem::size_of;

/// Single-layer bit set.
#[derive(Clone, Debug, Default)]
pub struct FlatBitSet {
    bits: Vec<usize>,
}

impl FlatBitSet {
    const NUM_BITS: usize = size_of::<usize>() * 8;

    /// Creates a new `FlatBitSet` with all bits set to zero.
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    fn ensure_size(&mut self, size: usize) {
        use std::iter::repeat;

        if self.bits.len() < size {
            let delta = size - self.bits.len();
            self.bits.extend(repeat(0).take(delta));
        }
    }
}

unsafe impl BitSet for FlatBitSet {
    #[inline]
    fn add(&mut self, bit: usize) -> bool {
        let pos = bit / Self::NUM_BITS;
        let shift = bit % Self::NUM_BITS;
        let mask = 0x1 << shift;

        self.ensure_size(pos + 1);

        let num = &mut self.bits[pos];
        let old = (*num & mask) != 0;

        *num |= mask;

        old
    }

    fn remove(&mut self, bit: usize) -> bool {
        let pos = bit / Self::NUM_BITS;
        let shift = bit % Self::NUM_BITS;
        let mask = 0x1 << shift;

        if self.bits.len() > pos {
            let num = &mut self.bits[pos];
            let old = (*num & mask) != 0;

            *num &= !mask;

            old
        } else {
            false
        }
    }

    fn pop_front(&mut self) -> Option<usize> {
        let pos = self.bits.iter().position(|n| *n != 0)?;
        let mpos = pos * Self::NUM_BITS;

        (0..Self::NUM_BITS)
            .into_iter()
            .map(|bpos| self.remove(mpos + bpos))
            .position(|b| b)
            .map(|n| n + mpos)
    }

    fn contains(&self, bit: usize) -> bool {
        let pos = bit / Self::NUM_BITS;
        let shift = bit % Self::NUM_BITS;
        let mask = 0x1 << shift;

        if self.bits.len() > pos {
            (self.bits[pos] & mask) != 0
        } else {
            false
        }
    }

    fn count(&self) -> usize {
        self.bits.iter().map(|n| n.count_ones() as usize).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let empty = FlatBitSet::new();

        for i in 0..100 {
            assert_eq!(empty.contains(i), false);
        }
    }

    #[test]
    fn ensure_size() {
        let mut empty = FlatBitSet::new();

        assert_eq!(empty.bits.len(), 0);
        empty.ensure_size(5);
        assert_eq!(empty.bits.len(), 5);
        empty.ensure_size(3);
        assert_eq!(empty.bits.len(), 5);
    }

    #[test]
    fn add() {
        let mut bitset = FlatBitSet::new();

        bitset.add(0);
        assert_eq!(bitset.bits[0], 0x1);

        for i in 0..64 {
            bitset.add(i);
        }

        assert_eq!(bitset.bits[0], !0);
    }

    #[test]
    fn remove() {
        let mut bitset = FlatBitSet::new();

        bitset.add(5);
        bitset.remove(5);

        assert_eq!(bitset.bits[0], 0);

        bitset.add(555);
        bitset.remove(555);

        assert!(bitset.bits.iter().all(|n| *n == 0));

        for i in 0..64 {
            bitset.add(i);
        }

        bitset.remove(5);

        assert_eq!(bitset.bits[0], !0b100000);
    }

    #[test]
    fn contains() {
        let mut bitset = FlatBitSet::new();

        bitset.add(123);
        assert_eq!(bitset.contains(123), true);
        assert_eq!(bitset.contains(122), false);
        assert_eq!(bitset.contains(124), false);

        bitset.remove(123);
        assert_eq!(bitset.contains(123), false);
    }
}
