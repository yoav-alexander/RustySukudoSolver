#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Subset {
    pub values: Vec<usize>,
    pub positions: Vec<(usize, usize)>,
}

impl Subset {
    pub const fn new(values: Vec<usize>, positions: Vec<(usize, usize)>) -> Self {
        Self { values, positions }
    }

    pub fn size(&self) -> usize {
        self.values.len()
    }
}
