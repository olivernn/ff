use std::cmp::{Eq, Ord};
use std::hash::Hash;
use std::collections::HashSet;

pub struct MinSet<T: Ord + Hash + Eq> {
    set: HashSet<T>
}

impl<T: Ord + Hash + Eq> MinSet<T> {
    pub fn new() -> MinSet<T> {
        MinSet {
            set: HashSet::new()
        }
    }

    pub fn insert(&mut self, element: T) {
        match self.set.take(&element) {
            Some(ref existing) if existing > &element => {
                self.set.insert(element);
            },

            Some(existing) => {
                self.set.insert(existing);
            }

            None => {
                self.set.insert(element);
            }
        }
    }
}

impl<T: Ord + Hash + Eq> IntoIterator for MinSet<T> {
    type Item = T;
    type IntoIter = ::std::collections::hash_set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.set.into_iter()
    }
}