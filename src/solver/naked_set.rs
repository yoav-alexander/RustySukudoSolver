use crate::all_equal::AllEqual;
use crate::region::get_all_regions;
use crate::solver::SudokuSolver;
use crate::subset::Subset;
use itertools::Itertools;

// -=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=-
//                             naked regions implementations
// -=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=-

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

impl<const N: usize> SudokuSolver<N> {
    fn get_possible_combinations_in_region(
        &self,
        region: Vec<(usize, usize)>,
    ) -> Vec<Vec<((usize, usize), Vec<u16>)>> {
        let max_naked_set_size = self.board.size() / 2;
        let possible_positions: Vec<_> = region
            .into_iter()
            .map(|p| (p, self.board.get_possible_values(p.0, p.1)))
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

    fn get_naked_sets_in_region(&mut self, region: Vec<(usize, usize)>) -> Vec<Subset> {
        let possible_combinations = self.get_possible_combinations_in_region(region);

        let mut naked_sets = Vec::new();
        for possible_combination in possible_combinations {
            let (positions, values): (Vec<_>, Vec<_>) = possible_combination.into_iter().unzip();
            let values_set = get_values_set(&values, self.board.size());
            if positions.len() == values_set.len() {
                naked_sets.push(Subset::new(values_set, positions));
            }
        }

        naked_sets
    }

    pub fn enforce_naked_regions(&mut self) -> Result<bool, String> {
        let regions = get_all_regions(self.board.size());

        for (region_type, region) in regions {
            let subsets = self.get_naked_sets_in_region(region);
            for subset in subsets {
                if self.known_naked_sets.contains(&subset) {
                    continue;
                }
                let is_solved = self.apply_external_subset(region_type, &subset)?;
                if is_solved {
                    return Ok(true);
                }
                self.known_naked_sets.insert(subset);
            }
        }

        Ok(false)
    }
}
