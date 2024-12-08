use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct CountCollection<T> {
    map: HashMap<T, u32>,
}

impl<T> CountCollection<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn count(&self, key: &T) -> u32
    where
        T: Hash,
        T: Eq,
    {
        match self.map.get(key) {
            Some(count) => *count,
            None => 0,
        }
    }

    pub fn add(&mut self, key: T)
    where
        T: Hash,
        T: Eq,
    {
        self.map.entry(key).and_modify(|i| *i += 1).or_insert(1);
    }
}

impl<T> Default for CountCollection<T> {
    fn default() -> Self {
        Self::new()
    }
}
