use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct ContainsCollection<T> {
    map: HashMap<T, ()>,
}

impl<T> ContainsCollection<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn contains(&self, key: &T) -> bool
    where
        T: Hash,
        T: Eq,
    {
        self.map.contains_key(key)
    }

    pub fn add_if_not_contains(&mut self, key: T)
    where
        T: Hash,
        T: Eq,
    {
        self.map.entry(key).or_insert(());
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.map.keys()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<T> Default for ContainsCollection<T> {
    fn default() -> Self {
        Self::new()
    }
}
