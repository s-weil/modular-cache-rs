use std::collections::VecDeque;
use std::time::Instant;
use std::{collections::HashMap, hash::Hash};

/*
TODO:
    - expiration policies
    - lru impplementation
    - concurrent (with std/parkinglot/tokio)
    - test different timed cache: split into lookup and ordered list
    - rescale capacity


*/

#[derive(Clone, Debug)]
pub struct TimedKey<K> {
    key: K,
    instant: Instant,
}

impl<K> TimedKey<K> {
    fn create_now(key: K) -> Self {
        Self {
            key,
            instant: Instant::now(),
        }
    }
}

#[derive(Debug)]
pub struct TimedKeyRegistry<K> {
    /// keys ordered by time
    ordered_keys: VecDeque<TimedKey<K>>,
    max_capacity: usize, // TODO: could also maintain a registry of keys for faster lookup of existing keys -> possibly faster insertion but higher memory footprint
                         // TODO: config - expiration policy, etc
}

impl<K> KeyRegistry<K> for TimedKeyRegistry<K>
where
    K: Hash + Eq + Clone,
{
    type KeyStatsItem = TimedKey<K>;

    fn init(max_capacity: usize) -> Self {
        Self {
            ordered_keys: VecDeque::with_capacity(max_capacity),
            max_capacity,
        }
    }

    // fn get(&mut self, key: &K) -> Option<&mut TimedKey<K>> {
    //     for mut tk in self.ordered_keys.iter_mut() {
    //         if &tk.key == key {
    //             return Some(&mut tk);
    //         }
    //     }
    //     None
    // }

    fn len(&self) -> usize {
        self.ordered_keys.len()
    }

    fn add_or_update(&mut self, key: K) -> Option<K> {
        self.try_remove(&key);
        let timed_key = TimedKey::create_now(key.clone());
        let deleted_key = if self.ordered_keys.len() >= self.max_capacity {
            dbg!("deleting key");
            self.ordered_keys.pop_back()
        } else {
            None
        };
        self.ordered_keys.push_front(timed_key);

        deleted_key.map(|tk| tk.key)
    }

    // TODO: currently takes o(n) to search for a key. could be improved by a lookup
    fn try_remove(&mut self, key: &K) -> Option<K> {
        for (idx, tk) in self.ordered_keys.iter().enumerate() {
            if &tk.key == key {
                self.ordered_keys.remove(idx);
                return Some(key.clone());
            }
        }
        None
    }
}

pub trait KeyRegistry<K> {
    type KeyStatsItem;

    fn init(capacity: usize) -> Self;

    fn len(&self) -> usize;

    // fn get(&mut self, key: &K) -> Option<&mut Self::KeyStatsItem>;

    // return deleted key (if some)
    fn add_or_update(&mut self, key: K) -> Option<K>;

    fn try_remove(&mut self, key: &K) -> Option<K>;
}

// TODO: could also have a trait for cache and then inject LRU, etc
// TODO: generalize with randomstate and buildhasher
// TODO: stats (instant) within `orderd_keys`?
pub struct Cache<K, R, V>
where
    K: Eq + Hash,
    R: KeyRegistry<K>,
{
    store: HashMap<K, V>,
    key_registry: R,
}

impl<K, S, V> Cache<K, S, V>
where
    K: Eq + Hash + Clone,
    S: KeyRegistry<K>,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            store: HashMap::new(),
            key_registry: S::init(capacity),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.store.get(key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(deleted_key) = self.key_registry.add_or_update(key.clone()) {
            self.store.remove(&deleted_key);
        }
        self.store.insert(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        self.key_registry
            .try_remove(key)
            .and_then(|k| self.store.remove_entry(&k))
    }
}

pub type TimedCache<K, V> = Cache<K, TimedKeyRegistry<K>, V>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timed_cache_init() {
        let mut cache = TimedCache::<i32, String>::new(4);
        cache.insert(1, "How".to_string());
        cache.insert(2, "Hi".to_string());
        cache.insert(3, "Are".to_string());
        cache.insert(4, "You".to_string());
        cache.insert(5, "Doing".to_string());
        cache.insert(2, "How".to_string());

        assert_eq!(cache.key_registry.len(), 4);

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2).cloned(), Some("How".to_string()));
        assert_eq!(cache.get(&3).cloned(), Some("Are".to_string()));
        assert_eq!(cache.get(&4).cloned(), Some("You".to_string()));
        assert_eq!(cache.get(&5).cloned(), Some("Doing".to_string()));
    }
}
