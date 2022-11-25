use crate::cache::{Cache, KeyRegistry};
use std::{
    collections::{HashSet, VecDeque},
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

    fn house_keeping(&mut self) -> Option<HashSet<K>> {
        None
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
