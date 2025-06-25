use std::cmp::Ordering;
use derive_getters::Getters;

#[derive(Debug, Getters, PartialOrd, PartialEq, Eq, Hash)]
pub struct Event {
    /// The index refers to the column's index, starting from the first event
    index: usize,
    name: String
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
            .then(self.index.cmp(&other.index))
    }
}

impl Event {
    pub fn new(index: usize, name: String) -> Self {
        Self { index, name }
    }
}
