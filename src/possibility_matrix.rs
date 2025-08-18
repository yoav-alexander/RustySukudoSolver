use std::fmt::{Debug, Display, Formatter};

pub struct PossibilityMatrix<const N: usize> {
    size: usize,
    block_size: usize,
    board: [[u16; N]; N],
}

impl<const N: usize> PossibilityMatrix<N> {
    pub fn new() -> PossibilityMatrix<N> {
        Self {
            size: N,
            block_size: N.isqrt(),
            board: [[u16::MAX; N]; N],
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }

    pub fn set(&mut self, row: usize, col: usize, value: u16) {
        assert!(row < self.size && col < self.size);
        self.board[row][col] = 1 << (value - 1);
    }

    pub fn set_possible_values(&mut self, row: usize, col: usize, values: &[u16]) {
        assert!(row < self.size && col < self.size);

        self.board[row][col] = 0;
        for value in values {
            self.board[row][col] |= 1 << (value - 1);
        }
    }

    pub fn constrain_possible_values(&mut self, row: usize, col: usize, values: &[u16]) {
        assert!(row < self.size && col < self.size);
        assert!(!values.is_empty());

        let mut mask = 0;
        for v in values {
            mask |= 1 << (v - 1)
        }
        self.board[row][col] &= mask;
    }

    pub fn remove_value(&mut self, row: usize, col: usize, value: u16) {
        assert!(row < self.size && col < self.size);
        assert!(
            value >= 1 && value <= self.size as u16,
            "Invalid value got {}",
            value
        );
        self.board[row][col] &= !(1 << (value - 1));
    }

    pub fn get_possible_values(&self, row: usize, col: usize) -> Vec<u16> {
        let mut possible_values = vec![];
        for i in 0..self.size {
            if (self.board[row][col] & (1u16 << i)) != 0 {
                possible_values.push((i + 1) as u16);
            }
        }
        possible_values
    }

    pub fn is_possible_value(&self, row: usize, col: usize, value: u16) -> bool {
        assert!(row < self.size && col < self.size && value > 0 && value <= self.size() as u16);
        (self.board[row][col] & (1u16 << (value - 1))) != 0
    }

    pub fn is_cell_resolved(&self, row: usize, col: usize) -> bool {
        // todo uncouple from 9
        (0b0000000111111111 & self.board[row][col]).is_power_of_two()
    }

    pub fn is_board_resolved(&self) -> bool {
        (0..self.size).all(|row| (0..self.size).all(|col| self.is_cell_resolved(row, col)))
    }
}

impl<const N: usize> Debug for PossibilityMatrix<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cell_width = self.size * 2;
        let line_width = (cell_width + 1) * self.block_size() + 1;
        let separator = format!("+{}", "-".repeat(line_width));
        let horizontal_line = format!("{}+", separator.repeat(self.block_size()));

        writeln!(f, "{}", horizontal_line)?;

        for row in 0..self.size {
            write!(f, "| ")?;
            for col in 0..self.size {
                let cell_possible_values = self.get_possible_values(row, col);
                let value_string = if cell_possible_values.is_empty() {
                    " ".repeat(cell_width) // Handle empty cells
                } else {
                    let values: Vec<String> = cell_possible_values
                        .iter()
                        .map(ToString::to_string)
                        .collect();
                    let value_str = values.join(",");
                    format!("{:<width$}", value_str, width = cell_width)
                };

                write!(f, "{} ", value_string)?;
                if (col + 1) % self.block_size() == 0 {
                    write!(f, "| ")?;
                }
            }
            writeln!(f)?;
            if (row + 1) % self.block_size() == 0 {
                writeln!(f, "{}", horizontal_line)?;
            }
        }
        Ok(())
    }
}

impl<const N: usize> Display for PossibilityMatrix<N> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let cell_width = 3;
        let line_width = cell_width * self.block_size();
        let separator = format!("+{}", "-".repeat(line_width));
        let horizontal_line = format!("{}+", separator.repeat(self.block_size()));

        writeln!(f, "{}", horizontal_line)?;
        for row in 0..self.size {
            write!(f, "|")?;
            for col in 0..self.size {
                let cell_possible_values = self.get_possible_values(row, col);
                let value = match cell_possible_values.len() {
                    0 => "!".to_string(),
                    1 => format!("{:}", cell_possible_values[0]),
                    _ => "_".to_string(),
                };

                if (col + 1) % self.block_size() == 0 {
                    write!(f, " {} |", value)?;
                } else {
                    write!(f, " {} ", value)?;
                }
            }
            write!(f, "\n")?;
            if (row + 1) % self.block_size() == 0 {
                writeln!(f, "{}", horizontal_line)?;
            }
        }
        Ok(())
    }
}
