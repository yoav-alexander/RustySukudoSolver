pub mod bit_storage;
pub mod possibility_iterator;

use crate::possibility_matrix::bit_storage::{ForSize, StorageForSize};
use crate::possibility_matrix::possibility_iterator::PossibilityIterator;
use num_traits::{Bounded, One, Zero};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

macro_rules! assert_position {
    ($matrix:expr, $row:expr, $col:expr) => {
        assert!(
            $row < $matrix.size && $col < $matrix.size,
            "Invalid position ({:},{:})",
            $row,
            $col
        );
    };
}

macro_rules! assert_value {
    ($matrix:expr, $value:expr) => {
        assert!(
            $value > 0 && $value <= $matrix.size,
            "Invalid value {} expected between 1 and {}",
            $value,
            $matrix.size
        );
    };
}

fn is_one_on_bit<T>(x: T) -> bool
where
    T: Copy + std::ops::BitAnd<Output = T> + std::ops::Sub<Output = T> + PartialEq + From<u8>,
{
    x != T::from(0) && (x & (x - T::from(1))) == T::from(0)
}

pub struct PossibilityMatrix<const N: usize, S: StorageForSize = ForSize<N>>
where
    ForSize<N>: StorageForSize,
{
    size: usize,
    block_size: usize,
    board: [[S::SType; N]; N],
}

impl<const N: usize, S: StorageForSize> PossibilityMatrix<N, S>
where
    ForSize<N>: StorageForSize,
{
    pub fn new() -> Self {
        Self {
            size: N,
            block_size: N.isqrt(),
            board: [[<S::SType>::max_value(); N]; N],
        }
    }

    pub const fn size(&self) -> usize {
        self.size
    }

    pub const fn block_size(&self) -> usize {
        self.block_size
    }

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn value_to_stype(&self, value: usize) -> S::SType {
        assert_value!(self, value);
        value.try_into().unwrap()
    }

    pub fn set(&mut self, row: usize, col: usize, value: usize) {
        assert_position!(self, row, col);
        let value = self.value_to_stype(value);
        self.board[row][col] = S::SType::one() << (value - S::SType::one());
    }

    #[allow(dead_code)]
    pub fn set_possible_values(&mut self, row: usize, col: usize, values: &[usize]) {
        assert_position!(self, row, col);

        self.board[row][col] = S::SType::zero();
        for &value in values {
            let value = self.value_to_stype(value);
            self.board[row][col] |= S::SType::one() << (value - S::SType::one());
        }
    }

    pub fn constrain_possible_values(&mut self, row: usize, col: usize, values: &[usize]) {
        assert_position!(self, row, col);
        assert!(!values.is_empty());

        let mut mask = S::SType::zero();
        for &value in values {
            let value = self.value_to_stype(value);
            mask |= S::SType::one() << (value - S::SType::one());
        }
        self.board[row][col] &= mask;
    }

    pub fn remove_value(&mut self, row: usize, col: usize, value: usize) {
        assert_position!(self, row, col);
        let value = self.value_to_stype(value);
        self.board[row][col] &= !(S::SType::one() << (value - S::SType::one()));
    }

    pub fn get_possible_values(&self, row: usize, col: usize) -> PossibilityIterator<N, S> {
        assert_position!(self, row, col);
        PossibilityIterator::<N, S>::new(self.board[row][col], self.size)
    }

    pub fn is_possible_value(&self, row: usize, col: usize, value: usize) -> bool {
        assert_position!(self, row, col);
        assert_value!(self, value);
        let value = self.value_to_stype(value);
        (self.board[row][col] & (S::SType::one() << (value - S::SType::one()))) != S::SType::zero()
    }

    pub fn is_cell_resolved(&self, row: usize, col: usize) -> bool {
        assert_position!(self, row, col);
        is_one_on_bit(((S::SType::one() << N) - S::SType::one()) & self.board[row][col])
    }

    pub fn is_board_resolved(&self) -> bool {
        (0..self.size).all(|row| (0..self.size).all(|col| self.is_cell_resolved(row, col)))
    }
}

impl<const N: usize, S: StorageForSize> Debug for PossibilityMatrix<N, S>
where
    ForSize<N>: StorageForSize,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cell_width = self.size * 2;
        let line_width = (cell_width + 1) * self.block_size() + 1;

        let write_horizontal_line = |f: &mut Formatter| -> fmt::Result {
            for _ in 0..self.block_size() {
                write!(f, "+")?;
                for _ in 0..line_width {
                    write!(f, "-")?;
                }
            }
            writeln!(f, "+")
        };

        write_horizontal_line(f)?;
        for row in 0..self.size {
            write!(f, "| ")?;
            for col in 0..self.size {
                let cell_possible_values: Vec<_> = self.get_possible_values(row, col).collect();
                let value_string = if cell_possible_values.is_empty() {
                    " ".repeat(cell_width)
                } else {
                    let values: Vec<String> = cell_possible_values
                        .iter()
                        .map(ToString::to_string)
                        .collect();
                    let value_str = values.join(",");
                    format!("{value_str:<cell_width$}")
                };

                write!(f, "{value_string} ")?;
                if (col + 1) % self.block_size() == 0 {
                    write!(f, "| ")?;
                }
            }
            writeln!(f)?;
            if (row + 1) % self.block_size() == 0 {
                write_horizontal_line(f)?;
            }
        }
        Ok(())
    }
}

impl<const N: usize, S: StorageForSize> Display for PossibilityMatrix<N, S>
where
    ForSize<N>: StorageForSize,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let block_size = self.block_size();
        let cell_width = 3;
        let line_width = cell_width * block_size;

        let write_horizontal_line = |f: &mut Formatter| -> fmt::Result {
            for _ in 0..block_size {
                write!(f, "+")?;
                for _ in 0..line_width {
                    write!(f, "-")?;
                }
            }
            writeln!(f, "+")
        };

        write_horizontal_line(f)?;
        for row in 0..self.size {
            write!(f, "|")?;
            for col in 0..self.size {
                let cell_possible_values: Vec<_> = self.get_possible_values(row, col).collect();
                match cell_possible_values.len() {
                    0 => write!(f, " ! ")?,
                    1 => write!(f, " {} ", cell_possible_values[0])?,
                    _ => write!(f, " _ ")?,
                }

                if (col + 1) % block_size == 0 {
                    write!(f, "|")?;
                }
            }
            writeln!(f)?;
            if (row + 1) % block_size == 0 {
                write_horizontal_line(f)?;
            }
        }
        Ok(())
    }
}
