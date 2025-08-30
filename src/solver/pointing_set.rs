use crate::board::SudokuBoard;
use crate::possibility_matrix::bit_storage::{ForSize, StorageForSize};
use crate::region::{get_all_boxes, RegionType};
use crate::solver::SudokuRuleEnforcer;
use crate::subset::Subset;
use std::collections::HashSet;
use std::ops::Sub;

pub struct PointingSetEnforcer<const N: usize>
where
    ForSize<N>: StorageForSize,
{
    known_pointing_sets: HashSet<Subset>,
}

fn diff<T>(a: &[T], b: &[T]) -> Vec<T::Output>
where
    T: Sub + Copy,
{
    a.iter().zip(b).map(|(&v1, &v2)| v1 - v2).collect()
}

impl<const N: usize> PointingSetEnforcer<N>
where
    ForSize<N>: StorageForSize,
{
    pub fn new() -> Self {
        Self {
            known_pointing_sets: HashSet::default(),
        }
    }
    fn infer_pointing_sets_in_region(
        pos_lines: Vec<Vec<(usize, usize)>>,
        val_lines: Vec<Vec<usize>>,
        val_total: &[usize],
    ) -> Vec<Subset> {
        let mut pointing_sets = Vec::new();
        for (line_values, line_positions) in val_lines.into_iter().zip(pos_lines) {
            let only_in_line: Vec<_> = diff(val_total, &line_values)
                .into_iter()
                .enumerate()
                .filter(|&(i, d)| d == 0 && val_total[i] != 1)
                .map(|(i, _)| i + 1)
                .collect();
            if !only_in_line.is_empty() {
                pointing_sets.push(Subset::new(only_in_line, line_positions));
            }
        }
        pointing_sets
    }

    fn get_pointing_sets_in_region(
        board: &SudokuBoard<N>,
        region: Vec<(usize, usize)>,
    ) -> Vec<(RegionType, Subset)> {
        let size = board.size();
        let block_size = board.block_size();
        let mut pos_rows = vec![Vec::with_capacity(block_size); block_size];
        let mut pos_cols = vec![Vec::with_capacity(block_size); block_size];
        let mut val_rows = vec![vec![0; size]; block_size];
        let mut val_cols = vec![vec![0; size]; block_size];
        let mut val_total = vec![0; size];

        for (row, col) in region {
            let possible_values = board.get_possible_values(row, col);
            pos_rows[row % block_size].push((row, col));
            pos_cols[col % block_size].push((row, col));

            for val in possible_values {
                val_rows[row % block_size][val - 1] += 1;
                val_cols[col % block_size][val - 1] += 1;
                val_total[val - 1] += 1;
            }
        }

        let row_pointing_sets = Self::infer_pointing_sets_in_region(pos_rows, val_rows, &val_total)
            .into_iter()
            .map(|s| (RegionType::Row, s));
        let col_pointing_sets = Self::infer_pointing_sets_in_region(pos_cols, val_cols, &val_total)
            .into_iter()
            .map(|s| (RegionType::Col, s));

        row_pointing_sets.chain(col_pointing_sets).collect()
    }
}

impl<const N: usize> SudokuRuleEnforcer<N> for PointingSetEnforcer<N>
where
    ForSize<N>: StorageForSize,
{
    fn name(&self) -> &'static str {
        "PointingSetEnforcer"
    }
    fn enforce_rule(&mut self, board: &mut SudokuBoard<N>) -> Result<bool, String> {
        let boxes = get_all_boxes(board.size());

        for box_ in boxes {
            let pointing_sets = Self::get_pointing_sets_in_region(board, box_);
            for (region_type, subset) in pointing_sets {
                if self.known_pointing_sets.contains(&subset) {
                    continue;
                }
                let is_solved = board.apply_external_subset(region_type, &subset)?;
                if is_solved {
                    return Ok(true);
                }
                self.known_pointing_sets.insert(subset);
            }
        }

        Ok(false)
    }
}
