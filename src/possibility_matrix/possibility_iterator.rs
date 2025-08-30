use crate::possibility_matrix::bit_storage::{ForSize, StorageForSize};
use num_traits::{One, PrimInt, Zero};

#[derive(Clone)]
pub struct PossibilityIterator<const N: usize, S: StorageForSize = ForSize<N>> {
    mask: S::SType,
    size: usize,
}

impl<const N: usize, S: StorageForSize> PossibilityIterator<N, S> {
    pub const fn new(mask: S::SType, size: usize) -> Self {
        Self { mask, size }
    }
}

impl<const N: usize, S: StorageForSize> Iterator for PossibilityIterator<N, S> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let bit_pos = self.mask.trailing_zeros() as usize;
        if self.mask > S::SType::zero() && bit_pos < self.size {
            // Clear the lowest set bit
            // e.g., 0b0110 & 0b0101 = 0b0100
            self.mask &= self.mask - S::SType::one();
            #[allow(clippy::cast_possible_truncation)]
            return Some(bit_pos + 1);
        }
        None
    }
}
