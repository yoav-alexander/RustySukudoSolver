mod pointing_set;
mod sub_set;

use crate::board::SudokuBoard;
use crate::possibility_matrix::bit_storage::{ForSize, StorageForSize};
use crate::solver::pointing_set::PointingSetEnforcer;
use crate::solver::sub_set::SubSetEnforcer;
use std::fmt::{Debug, Formatter};

trait SudokuRuleEnforcer<const N: usize> {
    fn name(&self) -> &'static str;
    fn enforce_rule(&mut self, board: &mut SudokuBoard<N>) -> Result<bool, String>
    where
        ForSize<N>: StorageForSize;
}

pub struct SudokuSolver<const N: usize>
where
    ForSize<N>: StorageForSize,
{
    board: SudokuBoard<N>,
    enforcer: Vec<Box<dyn SudokuRuleEnforcer<N>>>,
    pre_solve_error: Option<String>,
}

impl<const N: usize> SudokuSolver<N>
where
    ForSize<N>: StorageForSize,
{
    pub fn new() -> Self {
        Self {
            board: SudokuBoard::<N>::new(),
            enforcer: vec![
                // Box::new(HiddenSetEnforcer::<N>::new()),
                Box::new(SubSetEnforcer::<N>::new()),
                Box::new(PointingSetEnforcer::<N>::new()),
            ],
            pre_solve_error: None,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: usize) {
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
        while !self.board.improved.is_empty() {
            self.board.improved.clear();

            for rule_enforcer in &mut self.enforcer {
                let is_solved = rule_enforcer.enforce_rule(&mut self.board);
                if is_solved.ok().unwrap_or(false) {
                    break;
                }
                let x = rule_enforcer.name();
                println!("iteration: {iteration} solver {x} board:\n{:?}", self.board);
            }
            println!("Improvements: {:?}", self.board.improved);
            iteration += 1;
        }

        Ok(self.board)
    }
}

impl<const N: usize> Debug for SudokuSolver<N>
where
    ForSize<N>: StorageForSize,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.board)
    }
}
