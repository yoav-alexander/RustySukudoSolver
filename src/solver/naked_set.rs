use crate::all_equal::AllEqual;
use crate::board::SudokuBoard;
use crate::region::get_all_regions;
use crate::solver::SudokuRuleEnforcer;
use crate::subset::Subset;
use itertools::Itertools;
use std::collections::HashSet;
// -=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=-
//                             naked regions implementations
// -=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=-

pub struct NakedSetEnforcer<const N: usize> {
    known_naked_sets: HashSet<Subset>,
}

pub fn get_values_set(vec: &Vec<Vec<u16>>, size: usize) -> Vec<u16> {
    let mut common_values = vec![0; size + 1];
    for value in vec.iter().flatten() {
        common_values[*value as usize] += 1;
    }
    common_values
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| if v > 0 { Some(i as u16) } else { None })
        .collect()
}

impl<const N: usize> NakedSetEnforcer<N> {
    pub fn new() -> Self {
        Self {
            known_naked_sets: Default::default(),
        }
    }
    fn get_possible_combinations_in_region(
        &self,
        board: &mut SudokuBoard<N>,
        region: Vec<(usize, usize)>,
    ) -> Vec<Vec<((usize, usize), Vec<u16>)>> {
        let max_naked_set_size = board.size() / 2;
        let possible_positions: Vec<_> = region
            .into_iter()
            .map(|p| (p, board.get_possible_values(p.0, p.1)))
            .filter(|(_, pv)| pv.len() > 1 && pv.len() < max_naked_set_size)
            .collect();

        let mut possible_combinations = Vec::new();
        for size in 2..=max_naked_set_size {
            possible_combinations.extend(
                possible_positions
                    .iter()
                    .cloned()
                    .into_iter()
                    .combinations(size),
            );
        }

        possible_combinations
    }

    fn get_naked_sets_in_region(
        &mut self,
        board: &mut SudokuBoard<N>,
        region: Vec<(usize, usize)>,
    ) -> Vec<Subset> {
        let possible_combinations = self.get_possible_combinations_in_region(board, region);

        let mut naked_sets = Vec::new();
        for possible_combination in possible_combinations {
            let (positions, values): (Vec<_>, Vec<_>) = possible_combination.into_iter().unzip();
            let values_set = get_values_set(&values, board.size());
            if positions.len() == values_set.len() {
                naked_sets.push(Subset::new(values_set, positions));
            }
        }

        naked_sets
    }
}

impl<const N: usize> SudokuRuleEnforcer<N> for NakedSetEnforcer<N> {
    fn name(&self) -> &'static str {
        "NakedSetEnforcer"
    }
    fn enforce_rule(&mut self, board: &mut SudokuBoard<N>) -> Result<bool, String> {
        let regions = get_all_regions(board.size());

        for (region_type, region) in regions {
            let subsets = self.get_naked_sets_in_region(board, region);
            for subset in subsets {
                if self.known_naked_sets.contains(&subset) {
                    continue;
                }
                let is_solved = board.apply_external_subset(region_type, &subset)?;
                if is_solved {
                    return Ok(true);
                }
                self.known_naked_sets.insert(subset);
            }
        }

        Ok(false)
    }
}
