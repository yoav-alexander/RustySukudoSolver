mod hidden_set;
mod naked_set;
mod pointing_set;

use crate::all_equal::AllEqual;
use crate::board::SudokuBoard;
use crate::join::IteratorDebugJoin;
use crate::solver::hidden_set::HiddenSetEnforcer;
use crate::solver::naked_set::NakedSetEnforcer;
use crate::solver::pointing_set::PointingSetEnforcer;
use std::fmt::{Debug, Formatter};

trait SudokuRuleEnforcer<const N: usize> {
    fn name(&self) -> &'static str;
    fn enforce_rule(&mut self, board: &mut SudokuBoard<N>) -> Result<bool, String>;
}

pub struct SudokuSolver<const N: usize> {
    board: SudokuBoard<N>,
    improvers: Vec<Box<dyn SudokuRuleEnforcer<N>>>,
    pre_solve_error: Option<String>,
}

impl<const N: usize> SudokuSolver<N> {
    pub fn new() -> Self {
        Self {
            board: SudokuBoard::<N>::new(),
            improvers: vec![
                Box::new(HiddenSetEnforcer::<N>::new()),
                Box::new(NakedSetEnforcer::<N>::new()),
                Box::new(PointingSetEnforcer::<N>::new()),
            ],
            pre_solve_error: None,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: u16) {
        if self.pre_solve_error.is_some() {
            return;
        }
        if let Err(msg) = self.board.set(row, col, value) {
            self.pre_solve_error = Some(msg);
        }
    }

    pub fn solve(mut self) -> Result<SudokuBoard<N>, String> {
        if let Some(error_msg) = self.pre_solve_error {
            return Err(error_msg);
        }
        if self.board.is_solved() {
            return Ok(self.board);
        }
        println!("solving:\n{}\n{:?}", self.board, self.board);
        let mut iteration = 1;
        while self.board.improved.len() > 0 {
            self.board.improved = Vec::new();

            for mut improver in self.improvers.iter_mut() {
                let is_solved = improver.enforce_rule(&mut self.board);
                if is_solved.ok().unwrap_or(false) {
                    break;
                }
                let x = improver.name();
                println!("iteration: {iteration} solver {x} board:\n{:?}", self.board);
            }
            println!("Improvements: {:?}", self.board.improved);
            iteration += 1;
        }

        Ok(self.board)
    }
}

impl<const N: usize> Debug for SudokuSolver<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.board)
    }
}

/*
+---------+---------+---------+
| _  5  _ | _  6  _ | _  3  _ |
| 4  _  8 | 5  _  _ | _  _  _ |
| 3  _  _ | _  _  _ | _  _  8 |
+---------+---------+---------+
| 8  _  7 | 3  _  _ | _  _  _ |
| _  1  _ | _  _  _ | _  _  _ |
| _  _  _ | _  _  _ | 6  8  4 |
+---------+---------+---------+
| 5  6  3 | 1  _  _ | 4  2  7 |
| _  _  _ | _  _  _ | _  9  1 |
| _  9  _ | _  _  4 | _  6  5 |
+---------+---------+---------+

*/
