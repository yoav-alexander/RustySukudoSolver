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
}
