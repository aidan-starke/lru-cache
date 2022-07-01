use std::{cmp::Eq, collections::HashMap, hash::Hash};

/// TODO: implement this trait
pub trait LRUCache<K, V> {
    /// Initialize the cache with `max` items stored
    fn initialize(max: usize) -> Self;
    /// Retrieve the value stored by key, if it exists
    fn get(&mut self, key: K) -> Option<&V>;
    /// Store a `value` under `key`
    fn set(&mut self, key: K, value: V);
}

pub struct Cache<K, V> {
    accessed: Vec<K>,
    map: HashMap<K, V>,
    max_size: usize,
}

impl<K, V> LRUCache<K, V> for Cache<K, V>
where
    K: Copy + Eq + Hash,
{
    fn initialize(max: usize) -> Self {
        let map: HashMap<K, V> = HashMap::new();
        let accessed: Vec<K> = Vec::new();

        Cache {
            accessed,
            map,
            max_size: max,
        }
    }

    fn get(&mut self, key: K) -> Option<&V> {
        match self.accessed.contains(&key) {
            true => move_accessed_key_to_end(key, &mut self.accessed),
            false => self.accessed.push(key),
        }

        if self.accessed.len() > self.max_size {
            let lru_key = self.accessed.remove(0);
            self.map.remove(&lru_key);
        }

        self.map.get(&key)
    }

    fn set(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

fn move_accessed_key_to_end<K: Copy + Eq>(accessed_key: K, accessed_arr: &mut Vec<K>) {
    let mut i: usize = 0;
    let mut j: usize = accessed_arr.len() - 1;

    while i < j {
        if accessed_arr[i] == accessed_key && accessed_arr[j] == accessed_key {
            j -= 1;
        } else if accessed_arr[i] == accessed_key && accessed_arr[j] != accessed_key {
            swap_key(i, j, accessed_arr);
            i += 1;
            j -= 1;
        } else {
            i += 1;
        }
    }
}

fn swap_key<K: Copy>(i: usize, j: usize, arr: &mut Vec<K>) {
    let temp: K = arr[j];
    arr[j] = arr[i];
    arr[i] = temp;
}

#[test]
fn cache_clears() {
    let mut cache = Cache::<&str, &str>::initialize(2);

    cache.set("one", "uno");
    cache.set("two", "dos");
    cache.set("three", "tres");
    cache.set("four", "quattro");
    cache.set("five", "cinco");

    cache.get("two");
    cache.get("three");
    cache.get("one");
    cache.get("two");
    cache.get("four");
    cache.get("five");

    assert_eq!(cache.map.len(), 2);
    assert_eq!(cache.accessed.len(), 2);
    assert_eq!(*cache.get("four").unwrap(), "quattro");
    assert_eq!(*cache.get("five").unwrap(), "cinco");
}
