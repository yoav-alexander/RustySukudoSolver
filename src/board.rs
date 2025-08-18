use crate::possibility_matrix::PossibilityMatrix;
use crate::region::RegionType;
use crate::subset::Subset;
use std::fmt::{Debug, Display, Formatter};

pub enum ExcludedPos<'a> {
    Single(usize, usize),
    Group(&'a Vec<(usize, usize)>),
}

impl ExcludedPos<'_> {
    fn firsts_row(&self) -> usize {
        match self {
            ExcludedPos::Single(row, _) => *row,
            ExcludedPos::Group(values) => values[0].0,
        }
    }

    fn firsts_col(&self) -> usize {
        match self {
            ExcludedPos::Single(_, col) => *col,
            ExcludedPos::Group(values) => values[0].1,
        }
    }

    fn is_excluded(&self, row: usize, col: usize) -> bool {
        match self {
            ExcludedPos::Single(e_row, e_col) => *e_col == col && *e_row == row,
            ExcludedPos::Group(points) => points
                .iter()
                .any(|(e_row, e_col)| *e_col == col && *e_row == row),
        }
    }
}

pub struct SudokuBoard<const N: usize> {
    board: PossibilityMatrix<N>,
    pub improved: Vec<(usize, usize)>,
}

impl<const N: usize> SudokuBoard<N> {
    pub fn new() -> SudokuBoard<N> {
        Self {
            board: PossibilityMatrix::new(),
            improved: Vec::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.board.size()
    }

    pub fn block_size(&self) -> usize {
        self.board.block_size()
    }

    pub fn get_possible_values(&self, row: usize, col: usize) -> Vec<u16> {
        self.board.get_possible_values(row, col)
    }

    pub fn is_solved(&self) -> bool {
        self.board.is_board_resolved()
    }

    pub fn set(&mut self, row: usize, col: usize, value: u16) -> Result<bool, String> {
        if !self.board.is_possible_value(row, col, value) {
            return Err(format!(
                "This board is invalid, Cannot set position ({row},{col}) as {value} \
                because is not one of the possible values {:?}.",
                self.board.get_possible_values(row, col)
            ));
        }
        self.improved.push((row, col));

        self.board.set(row, col, value);
        self.remove_from_row(ExcludedPos::Single(row, col), value)?;
        self.remove_from_col(ExcludedPos::Single(row, col), value)?;
        self.remove_from_box(ExcludedPos::Single(row, col), value)?;
        Ok(self.board.is_board_resolved())
    }

    fn remove_value(&mut self, row: usize, col: usize, value: u16) -> Result<bool, String> {
        if self.board.is_cell_resolved(row, col) {
            if self.board.get_possible_values(row, col)[0] == value {
                return Err(format!(
                    "Invalid Board, at ({row},{col}) removed resolved value {value}."
                ));
            }
            return Ok(false);
        }
        if !self.board.is_possible_value(row, col, value) {
            return Ok(false);
        }
        self.improved.push((row, col));

        self.board.remove_value(row, col, value);
        let mut is_solved = false;

        if self.board.is_cell_resolved(row, col) {
            is_solved = self.set(row, col, self.board.get_possible_values(row, col)[0])?
        }

        Ok(is_solved)
    }

    fn remove_from_row(&mut self, excluded_point: ExcludedPos, value: u16) -> Result<bool, String> {
        let row = excluded_point.firsts_row();

        for i in 0..self.board.size() {
            if excluded_point.is_excluded(row, i) {
                continue;
            }
            let is_solved = self.remove_value(row, i, value)?;
            if is_solved {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn remove_from_col(&mut self, excluded_point: ExcludedPos, value: u16) -> Result<bool, String> {
        let col = excluded_point.firsts_col();

        for i in 0..self.board.size() {
            if excluded_point.is_excluded(i, col) {
                continue;
            }
            let is_solved = self.remove_value(i, col, value)?;
            if is_solved {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn remove_from_box(&mut self, excluded_point: ExcludedPos, value: u16) -> Result<bool, String> {
        let box_row =
            (excluded_point.firsts_row() / self.board.block_size()) * self.board.block_size();
        let box_col =
            (excluded_point.firsts_col() / self.board.block_size()) * self.board.block_size();
        for i in 0..self.board.block_size() {
            for j in 0..self.board.block_size() {
                let current_row = box_row + i;
                let current_col = box_col + j;
                if excluded_point.is_excluded(current_row, current_col) {
                    continue;
                }

                let is_solved = self.remove_value(current_row, current_col, value)?;
                if is_solved {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    pub fn apply_external_subset(
        &mut self,
        region_type: RegionType,
        subset: &Subset,
    ) -> Result<bool, String> {
        let positions = &subset.positions;
        for value in subset.values.iter() {
            let is_solved = match region_type {
                RegionType::Row => self.remove_from_row(ExcludedPos::Group(positions), *value)?,
                RegionType::Col => self.remove_from_col(ExcludedPos::Group(positions), *value)?,
                RegionType::Box => self.remove_from_box(ExcludedPos::Group(positions), *value)?,
            };
            if is_solved {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn apply_internal_subset(&mut self, subset: &Subset) -> Result<bool, String> {
        if subset.size() == 1 {
            // hidden digit - only one possible place for digit in region.
            return self.set(
                subset.positions[0].0,
                subset.positions[0].1,
                subset.values[0],
            );
        }

        if let Err(msg) = self.is_valid_subset(subset) {
            panic!("{}", msg)
        }

        for (row, col) in subset.positions.clone() {
            if self.board.get_possible_values(row, col) == subset.values {
                continue;
            }
            self.improved.push((row, col));
            self.board
                .constrain_possible_values(row, col, &subset.values);
        }

        Ok(false)
    }

    pub fn is_valid_subset(&self, subset: &Subset) -> Result<(), String> {
        for &(row, col) in subset.positions.iter() {
            let possible_values = self.board.get_possible_values(row, col);
            if !possible_values.iter().all(|v| subset.values.contains(v)) {
                return Err(format!(
                    "Can't set position ({row},{col}) as {:?} \
                     because it's not it the valid options: {:?}.",
                    subset.values, possible_values
                ));
            }
        }
        Ok(())
    }
}

impl<const N: usize> Debug for SudokuBoard<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.board, f)
    }
}

impl<const N: usize> Display for SudokuBoard<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.board, f)
    }
}
