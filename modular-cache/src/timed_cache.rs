use crate::{
    cache::{Cache, GetKey, KeyRegistry},
    concurrent_cache::ConcurrentCache,
};
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    time::Instant,
};

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

/// Takes O(n) for finding the keys.
#[derive(Debug)]
pub struct TimedKeyRegistry<K> {
    /// keys ordered by insertion in ASC order, i.e. latest in front, earliest in back
    ordered_keys: VecDeque<TimedKey<K>>,
    max_capacity: usize, // TODO: could also maintain a registry of keys for faster lookup of existing keys -> possibly faster insertion but higher memory footprint
                         // TODO: config - expiration policy, etc
}

impl<K> GetKey<K> for TimedKeyRegistry<K>
where
    K: PartialEq,
{
    /// Takes O(n) for finding the key.
    fn get(&self, key: &K) -> Option<&K> {
        self.ordered_keys
            .iter()
            .find(|tk| &tk.key == key)
            .map(|tk| &tk.key)
    }
}

impl<K> KeyRegistry<K> for TimedKeyRegistry<K>
where
    K: Hash + Eq + PartialEq + Clone,
{
    // type KeyStatsItem = TimedKey<K>;

    fn with_capacity(max_capacity: usize) -> Self {
        Self {
            ordered_keys: VecDeque::with_capacity(max_capacity),
            max_capacity,
        }
    }

    fn clear(&mut self) {
        self.ordered_keys.clear();
    }

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

    // TODO: currently takes O(n) to search for a key. could be improved by a lookup
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

pub type TimedCache<K, V> = Cache<K, TimedKeyRegistry<K>, V>;
pub type ConcurrentTimedCache<K, V> = ConcurrentCache<K, TimedKeyRegistry<K>, V>;

/// Takes O(1) for finding the keys, but higher memory footprint for having the lookup.
pub struct TimedKeyRegistry2<K> {
    key_idx_map: HashMap<K, usize>,
    /// keys ordered by insertion in ASC order, i.e. latest in front, earliest in back
    ordered_keys: VecDeque<TimedKey<K>>,
    max_capacity: usize,
}

impl<K> TimedKeyRegistry2<K>
where
    K: Hash + Eq + PartialEq + Clone,
{
    fn update_indices(&mut self) {
        let key_idx_map_udpated = self
            .ordered_keys
            .iter()
            .enumerate()
            .map(|(idx, tk)| (tk.key.clone(), idx))
            .collect();
        self.key_idx_map = key_idx_map_udpated;
    }
}

impl<K> GetKey<K> for TimedKeyRegistry2<K>
where
    K: Eq + Hash,
{
    /// Takes O(1) for finding the key.
    fn get(&self, key: &K) -> Option<&K> {
        self.key_idx_map.get(key).map(|&idx| {
            let tk = &self.ordered_keys[idx].key;
            if tk == key {
                tk
            } else {
                panic!("invalid state of key registry");
            }
        })
    }
}

impl<K> KeyRegistry<K> for TimedKeyRegistry2<K>
where
    K: Hash + Eq + PartialEq + Clone,
{
    // type KeyStatsItem = TimedKey<K>;

    fn with_capacity(max_capacity: usize) -> Self {
        Self {
            key_idx_map: HashMap::with_capacity(max_capacity),
            ordered_keys: VecDeque::with_capacity(max_capacity),
            max_capacity,
        }
    }

    fn clear(&mut self) {
        self.key_idx_map.clear();
        self.ordered_keys.clear();
    }

    fn len(&self) -> usize {
        // TODO: check also
        if self.key_idx_map.len() != self.ordered_keys.len() {
            panic!("invalid state of key registry");
        }
        self.ordered_keys.len()
    }

    // Takes O(n) in case the key was present and O(1) otherwise.
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
        self.update_indices();

        deleted_key.map(|tk| tk.key)
    }

    // Takes O(n) for re-ordering the lookup.
    fn try_remove(&mut self, key: &K) -> Option<K> {
        // TODO: need to update indices!!!
        let key = self
            .key_idx_map
            .remove(key)
            .and_then(|idx| self.ordered_keys.remove(idx).map(|tk| tk.key));
        if key.is_some() {
            self.update_indices();
        }
        key
    }
}

pub type TimedCacheV2<K, V> = Cache<K, TimedKeyRegistry2<K>, V>;
pub type ConcurrentTimedCacheV2<K, V> = ConcurrentCache<K, TimedKeyRegistry2<K>, V>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn timed_cache_init() {
        let mut cache = TimedCache::<i32, String>::new(Some(4));
        cache.insert(1, "How".to_string());
        cache.insert(2, "Hi".to_string());
        cache.insert(3, "Are".to_string());
        cache.insert(4, "You".to_string());
        cache.insert(5, "Doing".to_string());
        cache.insert(2, "How".to_string());

        assert_eq!(cache.len(), 4);

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2).cloned(), Some("How".to_string()));
        assert_eq!(cache.get(&3).cloned(), Some("Are".to_string()));
        assert_eq!(cache.get(&4).cloned(), Some("You".to_string()));
        assert_eq!(cache.get(&5).cloned(), Some("Doing".to_string()));
    }

    #[test]
    fn concurrent_timed_cache_init() {
        let cache = ConcurrentTimedCache::<i32, String>::new(Some(4));
        cache.insert(1, "How".to_string());
        cache.insert(2, "Hi".to_string());
        cache.insert(3, "Are".to_string());
        cache.insert(4, "You".to_string());
        cache.insert(5, "Doing".to_string());
        cache.insert(2, "How".to_string());

        assert_eq!(cache.len(), 4);

        assert_eq!(cache.get(&1).clone(), None);
        assert_eq!(cache.get(&2).as_deref(), Some(&"How".to_string()));
        assert_eq!(cache.get(&3).as_deref(), Some(&"Are".to_string()));
        assert_eq!(cache.get(&4).as_deref(), Some(&"You".to_string()));
        assert_eq!(cache.get(&5).as_deref(), Some(&"Doing".to_string()));
    }

    #[test]
    fn timed_cache_v2_init() {
        let mut cache = TimedCacheV2::<i32, String>::new(Some(4));
        cache.insert(1, "How".to_string());
        cache.insert(2, "Hi".to_string());
        cache.insert(3, "Are".to_string());
        cache.insert(4, "You".to_string());
        cache.insert(5, "Doing".to_string());
        cache.insert(2, "How".to_string());

        assert_eq!(cache.len(), 4);

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2).cloned(), Some("How".to_string()));
        assert_eq!(cache.get(&3).cloned(), Some("Are".to_string()));
        assert_eq!(cache.get(&4).cloned(), Some("You".to_string()));
        assert_eq!(cache.get(&5).cloned(), Some("Doing".to_string()));
    }

    #[test]
    fn concurrent_timed_cache_v2_init() {
        let cache = Arc::new(ConcurrentTimedCacheV2::<i32, String>::new(Some(4)));

        cache.insert(1, "How".to_string());
        cache.insert(2, "Hi".to_string());

        // await above ones for correct order

        let mut handles = vec![];

        let cache_clone = cache.clone();
        handles.push(std::thread::spawn(move || {
            cache_clone.insert(3, "Are".to_string())
        }));

        let cache_clone = cache.clone();
        handles.push(std::thread::spawn(move || {
            cache_clone.insert(4, "You".to_string())
        }));

        let cache_clone = cache.clone();
        handles.push(std::thread::spawn(move || {
            cache_clone.insert(5, "Doing".to_string())
        }));

        let cache_clone = cache.clone();
        handles.push(std::thread::spawn(move || {
            cache_clone.insert(2, "How".to_string())
        }));

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(cache.len(), 4);

        assert_eq!(cache.get(&1).clone(), None);
        assert_eq!(cache.get(&2).as_deref(), Some(&"How".to_string()));
        assert_eq!(cache.get(&3).as_deref(), Some(&"Are".to_string()));
        assert_eq!(cache.get(&4).as_deref(), Some(&"You".to_string()));
        assert_eq!(cache.get(&5).as_deref(), Some(&"Doing".to_string()));
    }
}
