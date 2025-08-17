use crate::all_equal::AllEqual;
use crate::join::IteratorDebugJoin;
use crate::region::get_all_regions;
use crate::solver::SudokuSolver;
use crate::subset::Subset;
use std::collections::HashMap;
// -=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=-
//                             hidden regions implementations
// -=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=-

impl<const N: usize> SudokuSolver<N> {
    fn get_hidden_sets_in_region(
        &mut self,
        region: Vec<(usize, usize)>,
    ) -> Result<Vec<Subset>, String> {
        let mut digit_position_map = HashMap::new(); // <value, [(row, col)]>
        for &(row, col) in region.iter() {
            let possible_values = self.board.get_possible_values(row, col);
            for value in possible_values {
                digit_position_map
                    .entry(value)
                    .or_insert(Vec::new())
                    .push((row, col));
            }
        }

        let mut digit_position_count_map = vec![Vec::new(); self.board.size() + 1];
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
        if region.contains(&(4, 8)) {
            println!(
                "digit_position_count_map:\n{:}",
                digit_position_count_map.iter().debug_join("\n")
            );
            println!("{}", vec![(4, 8)].all_equal())
        }

        for (index, possible_subset) in digit_position_count_map.into_iter().enumerate().skip(1) {
            if region.contains(&(4, 8)) {
                println!("p0: {:?}", possible_subset);
                println!("len: {:?}", possible_subset.len());
                println!("index: {:?}", index);
            }
            if possible_subset.len() != index {
                continue;
            }

            let (values, possible_positions): (Vec<u16>, Vec<_>) =
                possible_subset.into_iter().unzip();
            if region.contains(&(4, 8)) {
                println!("p1: {:?}", possible_positions);
            }
            if possible_positions.all_equal() {
                if region.contains(&(4, 8)) {
                    println!("pp:\n{:?}", possible_positions);
                }
                let positions = possible_positions.into_iter().next().unwrap();
                subsets.push(Subset::new(values, positions));
            }
        }

        if region.contains(&(4, 8)) {
            println!("subsets:\n{:}", subsets.iter().debug_join("\n"));
        }

        Ok(subsets)
    }

    fn set_hidden_set(&mut self, subset: &Subset) -> Result<bool, String> {
        if subset.size() == 1 {
            // hidden digit - only one possible place for digit in region.
            return self.set(
                subset.positions[0].0,
                subset.positions[0].1,
                subset.values[0],
            );
        }

        if let Err(msg) = subset.valid_in_board(&self.board) {
            panic!("{}", msg)
        }

        for (row, col) in subset.positions.clone() {
            if self.board.get_possible_values(row, col) == subset.values {
                continue;
            }
            self.improved.push((row, col));
            self.board.set_possible_values(row, col, &subset.values);
        }

        Ok(false)
    }

    pub fn enforce_hidden_regions(&mut self) -> Result<bool, String> {
        let regions = get_all_regions(self.board.size());

        for (_, region) in regions {
            let subsets = self.get_hidden_sets_in_region(region)?;
            for subset in subsets {
                if self.known_hidden_sets.contains(&subset) {
                    continue;
                }
                let is_solved = self.set_hidden_set(&subset)?;
                if is_solved {
                    return Ok(true);
                }
                self.known_hidden_sets.insert(subset);
            }
        }

        Ok(false)
    }
}
