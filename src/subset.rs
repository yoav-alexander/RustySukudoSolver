use crate::board::Board;

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Subset {
    pub values: Vec<u16>,
    pub positions: Vec<(usize, usize)>,
}

impl Subset {
    pub fn new(values: Vec<u16>, positions: Vec<(usize, usize)>) -> Self {
        Self { values, positions }
    }

    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn valid_in_board<const N: usize>(&self, board: &Board<N>) -> Result<(), String> {
        for (row, col) in self.positions.iter() {
            if !self
                .values
                .iter()
                .all(|v| board.is_possible_value(*row, *col, *v))
            {
                return Err(format!(
                    "Can't set position ({row},{col}) as {:?}\
                     because it's not it the valid options: {:?}.",
                    self.values,
                    board.get_possible_values(*row, *col)
                ));
            }
        }
        Ok(())
    }
}
