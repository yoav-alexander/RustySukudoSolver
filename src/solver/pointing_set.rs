use crate::region::{get_all_boxes, RegionType};
use crate::solver::SudokuSolver;
use crate::subset::Subset;
use std::ops::Sub;

fn diff<T>(a: &[T], b: &[T]) -> Vec<T::Output>
where
    T: Sub + Copy,
{
    a.iter().zip(b).map(|(&v1, &v2)| v1 - v2).collect()
}

impl<const N: usize> SudokuSolver<N> {
    fn infer_pointing_sets_in_region(
        pos_lines: Vec<Vec<(usize, usize)>>,
        val_lines: Vec<Vec<u16>>,
        val_total: &Vec<u16>,
    ) -> Vec<Subset> {
        let mut pointing_sets = Vec::new();
        for (line_values, line_positions) in val_lines.into_iter().zip(pos_lines) {
            let only_in_line: Vec<u16> = diff(&val_total, &line_values)
                .into_iter()
                .enumerate()
                .filter(|&(i, d)| d == 0 && val_total[i] != 1)
                .map(|(i, _)| i as u16 + 1)
                .collect();
            if only_in_line.len() > 0 {
                pointing_sets.push(Subset::new(only_in_line, line_positions))
            }
        }
        pointing_sets
    }

    fn get_pointing_sets_in_region(
        &self,
        region: Vec<(usize, usize)>,
    ) -> Vec<(RegionType, Subset)> {
        let size = self.board.size();
        let block_size = self.board.block_size();
        let mut pos_rows = vec![Vec::new(); block_size];
        let mut pos_cols = vec![Vec::new(); block_size];
        let mut val_rows = vec![vec![0; size]; block_size];
        let mut val_cols = vec![vec![0; size]; block_size];
        let mut val_total = vec![0; size];

        for (row, col) in region {
            let possible_values = self.board.get_possible_values(row, col);
            pos_rows[row % block_size].push((row, col));
            pos_cols[col % block_size].push((row, col));

            for val in possible_values {
                val_rows[row % block_size][val as usize - 1] += 1;
                val_cols[col % block_size][val as usize - 1] += 1;
                val_total[val as usize - 1] += 1;
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

    pub fn enforce_pointing_regions(&mut self) -> Result<bool, String> {
        let boxes = get_all_boxes(self.board.size());

        for box_ in boxes {
            let pointing_sets = self.get_pointing_sets_in_region(box_);
            for (rtype, subset) in pointing_sets {
                if self.known_pointing_sets.contains(&subset) {
                    continue;
                }
                let is_solved = self.apply_external_subset(rtype, &subset)?;
                if is_solved {
                    return Ok(true);
                }
                self.known_pointing_sets.insert(subset);
            }
        }

        Ok(false)
    }
}
