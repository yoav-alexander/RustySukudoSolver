use std::fmt::Debug;

pub(crate) trait IteratorDebugJoin {
    fn debug_join(self, sep: &str) -> String;
}

impl<I, T> IteratorDebugJoin for I
where
    I: Iterator<Item = T>,
    T: Debug,
{
    fn debug_join(mut self, sep: &str) -> String {
        self.enumerate()
            .map(|(i, x)| format!("{}. {:?}", i, x))
            .collect::<Vec<String>>()
            .join(sep)
    }
}
