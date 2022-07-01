use std::{
    cmp::{Eq, Ord},
    collections::HashMap,
    hash::Hash,
};

/// TODO: implement this trait
trait LRUCache<K: Eq + Hash + Ord, V> {
    /// Initialize the cache with `max` items stored
    fn initialize(max: usize) -> Self;
    /// Retrieve the value stored by key, if it exists
    fn get(&self, key: K) -> Option<&V>;
    /// Store a `value` under `key`
    fn set(&mut self, key: K, value: V);
}

impl<K: Eq + Hash + Ord, V> LRUCache<K, V> for HashMap<K, V> {
    fn initialize(max: usize) -> Self {
        let map: HashMap<K, V> = HashMap::with_capacity(max);

        map
    }

    fn get(&self, key: K) -> Option<&V> {
        self.get(&key)
    }

    fn set(&mut self, key: K, value: V) {
        self.insert(key, value);
    }
}
