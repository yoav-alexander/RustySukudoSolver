mod hidden_set;
mod naked_set;
mod pointing_set;

use crate::all_equal::AllEqual;
use crate::board::Board;
use crate::join::IteratorDebugJoin;
use crate::region::RegionType;
use crate::subset::Subset;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

enum ExcludedPos<'a> {
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
            ExcludedPos::Single(erow, ecol) => *ecol == col && *erow == row,
            ExcludedPos::Group(points) => points
                .iter()
                .any(|(erow, ecol)| *ecol == col && *erow == row),
        }
    }
}

pub struct SudokuSolver<const N: usize> {
    board: Board<N>,
    known_hidden_sets: HashSet<Subset>,
    known_naked_sets: HashSet<Subset>,
    known_pointing_sets: HashSet<Subset>,
    improved: Vec<(usize, usize)>,
}

impl<const N: usize> SudokuSolver<N> {
    pub fn new() -> Self {
        Self {
            board: Board::<N>::new(),
            known_hidden_sets: HashSet::new(),
            known_naked_sets: HashSet::new(),
            known_pointing_sets: HashSet::new(),
            improved: Vec::new(),
        }
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
        self.known_hidden_sets
            .insert(Subset::new(vec![value], vec![(row, col)]));
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

    fn apply_external_subset(
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

    pub fn solve(mut self) -> Result<Board<N>, String> {
        if self.board.is_board_resolved() {
            return Ok(self.board);
        }

        let mut iteration = 1;
        while self.improved.len() > 0 {
            println!("doing subset regions: {iteration}");
            println!("{:?}", self.board);
            iteration += 1;
            self.improved = Vec::new();

            let is_solved = self.enforce_pointing_regions()?;
            if is_solved {
                break;
            }
            let is_solved = self.enforce_hidden_regions()?;
            if is_solved {
                break;
            }
            let is_solved = self.enforce_pointing_regions()?;
            if is_solved {
                break;
            }
            println!("Improvements: {:?}", self.improved);
        }
        Ok(self.board)
    }
}

impl<const N: usize> Debug for SudokuSolver<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.board)
    }
}
