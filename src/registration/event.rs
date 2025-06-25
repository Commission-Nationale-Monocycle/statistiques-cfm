use derive_getters::Getters;

#[derive(Debug, Getters, PartialOrd, PartialEq)]
pub struct Event {
    /// The index refers to the column's index, starting from the first event
    index: usize,
    name: String
}

impl Event {
    pub fn new(index: usize, name: String) -> Self {
        Self { index, name }
    }
}
