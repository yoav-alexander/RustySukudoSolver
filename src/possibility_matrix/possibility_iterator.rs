use std::fmt::Debug;

#[derive(Clone)]
pub struct PossibilityIterator {
    mask: u16,
    size: usize,
}

impl PossibilityIterator {
    pub fn new(mask: u16, size: usize) -> Self {
        PossibilityIterator { mask, size }
    }
}

impl Iterator for PossibilityIterator {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        let bit_pos = self.mask.trailing_zeros() as u16;
        while self.mask > 0 && bit_pos < self.size as u16 {
            // Clear the lowest set bit
            // e.g., 0b0110 & 0b0101 = 0b0100
            self.mask &= self.mask - 1;
            return Some(bit_pos + 1);
        }
        None
    }
}
