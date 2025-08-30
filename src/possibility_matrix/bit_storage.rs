use num_traits::PrimInt;
use std::fmt::Debug;
use std::ops::{BitAndAssign, BitOrAssign, Sub};

pub trait StorageForSize {
    type SType: Copy
        + std::ops::BitAnd<Output = Self::SType>
        + std::ops::BitOr<Output = Self::SType>
        + std::ops::Not<Output = Self::SType>
        + std::ops::Shl<Output = Self::SType>
        + std::ops::Shr<Output = Self::SType>
        + Sub<Output = Self::SType>
        + BitOrAssign
        + BitAndAssign
        + TryFrom<usize, Error: Debug>
        + From<u8>
        + PrimInt;
}

pub struct ForSize<const N: usize>;

impl StorageForSize for ForSize<9> {
    type SType = u16;
}
impl StorageForSize for ForSize<16> {
    type SType = u16;
}
impl StorageForSize for ForSize<25> {
    type SType = u32;
}
impl StorageForSize for ForSize<36> {
    type SType = u64;
}
