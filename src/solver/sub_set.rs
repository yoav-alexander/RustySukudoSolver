use crate::board::SudokuBoard;
use crate::possibility_matrix::bit_storage::{ForSize, StorageForSize};
use crate::region::get_all_regions;
use crate::solver::SudokuRuleEnforcer;
use crate::subset::Subset;
use itertools::Itertools;
use std::collections::HashSet;

pub struct SubSetEnforcer<const N: usize> {
    known_sub_sets: HashSet<Subset>,
}

type PositionCombination = Vec<((usize, usize), Vec<usize>)>;

pub fn get_values_set(vec: &[Vec<usize>], size: usize) -> Vec<usize> {
    let mut common_values = vec![0; size + 1];
    for &value in vec.iter().flatten() {
        common_values[value] += 1;
    }
    common_values
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| if v > 0 { Some(i) } else { None })
        .collect()
}

impl<const N: usize> SubSetEnforcer<N>
where
    ForSize<N>: StorageForSize,
{
    pub fn new() -> Self {
        Self {
            known_sub_sets: HashSet::default(),
        }
    }

    fn get_possible_combinations_in_region(
        board: &SudokuBoard<N>,
        region: Vec<(usize, usize)>,
    ) -> Vec<PositionCombination> {
        let max_sub_set_size = board.size() / 2;
        let possible_positions: Vec<_> = region
            .into_iter()
            .map(|p| (p, board.get_possible_values(p.0, p.1).collect::<Vec<_>>()))
            .filter(|(_, pv)| pv.len() > 1 && pv.len() < max_sub_set_size)
            .collect();

        let mut possible_combinations = Vec::new();
        for size in 0..=max_sub_set_size {
            possible_combinations.extend(possible_positions.iter().cloned().combinations(size));
        }

        possible_combinations
    }

    fn get_sub_sets_in_region(board: &SudokuBoard<N>, region: Vec<(usize, usize)>) -> Vec<Subset> {
        let possible_combinations = Self::get_possible_combinations_in_region(board, region);

        let mut sub_sets = Vec::new();
        for possible_combination in possible_combinations {
            let (positions, values): (Vec<_>, Vec<_>) = possible_combination.into_iter().unzip();
            let values_set = get_values_set(&values, board.size());
            if positions.len() == values_set.len() {
                sub_sets.push(Subset::new(values_set, positions));
            }
        }

        sub_sets
    }
}

impl<const N: usize> SudokuRuleEnforcer<N> for SubSetEnforcer<N>
where
    ForSize<N>: StorageForSize,
{
    fn name(&self) -> &'static str {
        "SubSetEnforcer"
    }

    fn enforce_rule(&mut self, board: &mut SudokuBoard<N>) -> Result<bool, String> {
        let regions = get_all_regions(board.size());

        for (region_type, region) in regions {
            let subsets = Self::get_sub_sets_in_region(board, region);
            for subset in subsets {
                if self.known_sub_sets.contains(&subset) {
                    continue;
                }
                let is_solved = board.apply_external_subset(region_type, &subset)?;
                if is_solved {
                    return Ok(true);
                }
                let is_solved = board.apply_internal_subset(&subset)?;
                if is_solved {
                    return Ok(true);
                }
                self.known_sub_sets.insert(subset);
            }
        }

        Ok(false)
    }
}
