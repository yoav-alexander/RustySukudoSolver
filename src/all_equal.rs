pub trait AllEqual<T> {
    fn all_equal(&self) -> bool;

    fn all_equal_map<B: PartialEq>(&self, f: impl FnMut(&T) -> B) -> bool;
}

impl<T: PartialEq> AllEqual<T> for [T] {
    fn all_equal(&self) -> bool {
        self.iter().all(|x| x == self.first().unwrap())
    }

    fn all_equal_map<B: PartialEq>(&self, f: impl FnMut(&T) -> B) -> bool {
        self.iter()
            .map(f)
            .collect::<Vec<_>>()
            .windows(2)
            .all(|w| w[0] == w[1])
    }
}
