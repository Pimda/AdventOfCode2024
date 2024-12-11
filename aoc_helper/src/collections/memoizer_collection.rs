use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct MemoizerCollection<K, V> {
    map: HashMap<K, V>,
}

impl<K, V> MemoizerCollection<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add_and_return(&mut self, key: K, value: V) -> V
    where
        K: Hash,
        K: Eq,
        V: Clone,
    {
        self.map.entry(key).or_insert(value).clone()
    }

    pub fn add(&mut self, key: K, value: V)
    where
        K: Hash,
        K: Eq,
    {
        self.map.entry(key).or_insert(value);
    }

    pub fn get(&mut self, key: K) -> Option<&V>
    where
        K: Hash,
        K: Eq,
    {
        self.map.get(&key)
    }
}

impl<K, V> Default for MemoizerCollection<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
