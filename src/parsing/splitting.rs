use std::{fmt::Display, hash::Hash};

trait DisplayIter: Iterator<Item: Display> {}
impl<A: Iterator<Item: Display>> DisplayIter for A {}


pub struct DisplaySplitter {
    indicator: String,
}

impl DisplaySplitter {
    pub fn new(indicator: impl Display) -> DisplaySplitter {
        DisplaySplitter {
            indicator: indicator.to_string(),
        }
    }

    pub fn split
}
