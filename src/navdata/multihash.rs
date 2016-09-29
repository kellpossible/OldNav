//! This is a collection which is similar to a hashmap in that you
//! store values associated with a key, but the difference is that
//! there can be multiple values associated with the same key.

use std::collections::HashMap;
use std::hash::Hash;
use std::clone::Clone;
use std::fmt;

/// Not currently used
pub struct MultiHash<K, V> {
    map: HashMap<K, Vec<V>>,
}



impl<K: Hash + Eq + Clone, V> MultiHash<K, V> {
    /// Constructor for `MultiHash`
    pub fn new() -> MultiHash<K, V> {
        MultiHash { map: HashMap::new() }
    }

    /// Insert a new value
    pub fn insert(&mut self, key: K, value: V) {
        if self.contains_key(&key) {
            let new_key = key.clone();
            self.get_mut(&new_key).unwrap().push(value);
        } else {
            let mut new_vec: Vec<V> = Vec::new();
            new_vec.push(value);
            self.map.insert(key, new_vec);
            return;
        }
    }

    /// Get a value out
    pub fn get(&self, key: &K) -> Option<&Vec<V>> {
        return self.map.get(key);
    }

    /// Get a mutable value out
    pub fn get_mut(&mut self, key: &K) -> Option<&mut Vec<V>> {
        return self.map.get_mut(key);
    }

    /// Check whether this hash contains any items associated with a given key
    pub fn contains_key(&self, key: &K) -> bool {
        return self.map.contains_key(&key);
    }
}


impl<K: Hash + Eq, V> fmt::Debug for MultiHash<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "MultiHash: {{n_items: {}}}", self.map.len());

    }
}
