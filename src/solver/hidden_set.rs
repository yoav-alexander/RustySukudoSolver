use crate::all_equal::AllEqual;
use crate::board::SudokuBoard;
use crate::region::get_all_regions;
use crate::solver::SudokuRuleEnforcer;
use crate::subset::Subset;
use std::collections::{HashMap, HashSet};
// -=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=-
//                             hidden regions implementations
// -=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=-

pub struct HiddenSetEnforcer<const N: usize> {
    known_hidden_sets: HashSet<Subset>,
}

impl<const N: usize> HiddenSetEnforcer<N> {
    pub fn new() -> Self {
        Self {
            known_hidden_sets: Default::default(),
        }
    }
    fn get_hidden_sets_in_region(
        &mut self,
        board: &mut SudokuBoard<N>,
        region: Vec<(usize, usize)>,
    ) -> Result<Vec<Subset>, String> {
        let mut digit_position_map = HashMap::new(); // <value, [(row, col)]>
        for &(row, col) in region.iter() {
            let possible_values = board.get_possible_values(row, col);
            for value in possible_values {
                digit_position_map
                    .entry(value)
                    .or_insert(Vec::new())
                    .push((row, col));
            }
        }

        let mut digit_position_count_map = vec![Vec::new(); board.size() + 1];
        for (value, positions) in digit_position_map.into_iter() {
            digit_position_count_map[positions.len()].push((value, positions));
        }

        if digit_position_count_map[0].len() > 0 {
            return Err(format!(
                "This board is invalid, no valid place for {:?} in this region {:?}",
                digit_position_count_map[0]
                    .iter()
                    .map(|(value, _)| { value })
                    .collect::<Vec<_>>(),
                region
            ));
        }

        let mut subsets = Vec::new();
        // if region.contains(&(4, 8)) {
        //     // println!(
        //     //     "digit_position_count_map:\n{:}",
        //     //     digit_position_count_map.iter().debug_join("\n")
        //     // );
        //     // println!("{}", vec![(4, 8)].all_equal())
        // }

        for (index, possible_subset) in digit_position_count_map.into_iter().enumerate().skip(1) {
            // if region.contains(&(4, 8)) {
            //     println!("p0: {:?}", possible_subset);
            //     println!("len: {:?}", possible_subset.len());
            //     println!("index: {:?}", index);
            // }
            if possible_subset.len() != index {
                continue;
            }

            let (values, possible_positions): (Vec<u16>, Vec<_>) =
                possible_subset.into_iter().unzip();
            // if region.contains(&(4, 8)) {
            //     println!("p1: {:?}", possible_positions);
            // }
            if possible_positions.all_equal() {
                // if region.contains(&(4, 8)) {
                //     println!("pp:\n{:?}", possible_positions);
                // }
                let positions = possible_positions.into_iter().next().unwrap();
                subsets.push(Subset::new(values, positions));
            }
        }
        //
        // if region.contains(&(4, 8)) {
        //     println!("subsets:\n{:}", subsets.iter().debug_join("\n"));
        // }

        Ok(subsets)
    }
}

impl<const N: usize> SudokuRuleEnforcer<N> for HiddenSetEnforcer<N> {
    fn name(&self) -> &'static str {
        "HiddenSetEnforcer"
    }
    fn enforce_rule(&mut self, board: &mut SudokuBoard<N>) -> Result<bool, String> {
        let regions = get_all_regions(board.size());

        for (_, region) in regions {
            let subsets = self.get_hidden_sets_in_region(board, region)?;
            for subset in subsets {
                if self.known_hidden_sets.contains(&subset) {
                    continue;
                }
                let is_solved = board.apply_internal_subset(&subset)?;
                if is_solved {
                    return Ok(true);
                }
                self.known_hidden_sets.insert(subset);
            }
        }

        Ok(false)
    }
}
