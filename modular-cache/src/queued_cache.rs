use crate::{
    cache::{Cache, GetKey, KeyRegistry},
    concurrent_cache::ConcurrentCache,
    key::KeyExtension,
};
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

// TODO: rename to FiFo / Queued

/// Takes O(n) for finding the keys.
#[derive(Debug)]
pub struct QueuedRegistry<KeyExt, K>
where
    KeyExt: KeyExtension<K>,
{
    /// keys ordered by insertion in ASC order, i.e. latest in front, earliest in back
    ordered_keys: VecDeque<KeyExt>,
    max_capacity: usize,
    _phantom_data: std::marker::PhantomData<K>, // TODO: config - expiration policy, etc
}

impl<KeyExt, K> GetKey<K> for QueuedRegistry<KeyExt, K>
where
    KeyExt: KeyExtension<K>,
    K: PartialEq,
{
    /// Takes O(n) for finding the key.
    fn get(&self, key: &K) -> Option<&K> {
        self.ordered_keys
            .iter()
            .find(|tk| tk.key() == key)
            .map(|tk| tk.key())
    }
}

impl<KeyExt, K> KeyRegistry<K> for QueuedRegistry<KeyExt, K>
where
    KeyExt: KeyExtension<K>,
    K: Hash + Eq + PartialEq + Clone,
{
    type KeyExtension = KeyExt;

    fn with_capacity(max_capacity: usize) -> Self {
        Self {
            ordered_keys: VecDeque::with_capacity(max_capacity),
            max_capacity,
            _phantom_data: std::marker::PhantomData::<K>,
        }
    }

    fn clear(&mut self) {
        self.ordered_keys.clear();
    }

    fn len(&self) -> usize {
        self.ordered_keys.len()
    }

    fn add_or_update(&mut self, key: Self::KeyExtension) -> Option<K> {
        self.try_remove(key.key());
        // let timed_key = TimedKey::create_now(key.key().clone());
        let deleted_key = if self.ordered_keys.len() >= self.max_capacity {
            self.ordered_keys.pop_back()
        } else {
            None
        };
        self.ordered_keys.push_front(key);

        deleted_key.map(|tk| tk.key().clone())
    }
    // fn add_or_update(&mut self, key: K) -> Option<K> {
    //     self.try_remove(&key);
    //     let timed_key = TimedKey::create_now(key.clone());
    //     let deleted_key = if self.ordered_keys.len() >= self.max_capacity {
    //         self.ordered_keys.pop_back()
    //     } else {
    //         None
    //     };
    //     self.ordered_keys.push_front(timed_key);

    //     deleted_key.map(|tk| tk.key().clone())
    // }

    // TODO: currently takes O(n) to search for a key. could be improved by a lookup
    fn try_remove(&mut self, key: &K) -> Option<K> {
        let mut find_index = None;

        for (idx, tk) in self.ordered_keys.iter().enumerate() {
            if &tk.key() == &key {
                find_index = Some(idx);
                break;
            }
        }

        if let Some(idx) = find_index {
            self.ordered_keys.remove(idx);
            return Some(key.clone());
        }
        None
    }
}

pub type QueuedCache<K, KeyExt, V> = Cache<K, QueuedRegistry<KeyExt, K>, KeyExt, V>;
pub type ConcurrentQueuedCache<K, KeyExt, V> =
    ConcurrentCache<K, QueuedRegistry<KeyExt, K>, KeyExt, V>;

/// Takes O(1) for finding the keys, but higher memory footprint for having the lookup.
pub struct QueuedLookupRegistry<KeyExt, K>
where
    KeyExt: KeyExtension<K>,
{
    key_idx_map: HashMap<K, usize>,
    /// keys ordered by insertion in DESC order, i.e. latest in back, earliest in front (just as for Vec)
    ordered_keys: VecDeque<KeyExt>,
    max_capacity: usize,
    _phantom_data: std::marker::PhantomData<K>,
}

impl<KeyExt, K> QueuedLookupRegistry<KeyExt, K>
where
    KeyExt: KeyExtension<K>,
    K: Hash + Eq + PartialEq + Clone,
{
    fn update_indices(&mut self) {
        // TODO: if an index was provided, we could just re-index everyhing above/below that idx
        // TODO: check performance. maybe memory swap below?
        let key_idx_map_udpated = self
            .ordered_keys
            .iter()
            .enumerate()
            .map(|(idx, tk)| (tk.key().clone(), idx))
            .collect();
        self.key_idx_map = key_idx_map_udpated;
    }

    // dangerous operation
    fn update_indices_with_shift(&mut self, shift: usize) {
        for (_, idx) in self.key_idx_map.iter_mut() {
            let shifted_idx = *idx - shift;
            *idx = shifted_idx;
        }
    }

    fn delete_oldest(&mut self) -> Option<KeyExt> {
        let first = self.ordered_keys.pop_front();
        if let Some(k) = &first {
            self.key_idx_map.remove(&k.key().clone());
        }
        self.update_indices_with_shift(1);
        first
    }

    fn remove_key(&mut self, key: &K) -> Option<KeyExt> {
        let key = self
            .key_idx_map
            .remove(key)
            .and_then(|idx| self.ordered_keys.remove(idx));
        if key.is_some() {
            self.update_indices();
        }
        key
    }

    fn insert(&mut self, key: KeyExt) {
        let len = self.len();
        self.key_idx_map.insert(key.key().clone(), len);
        // let timed_key = TimedKey::create_now(key);
        self.ordered_keys.push_back(key);
    }
}

impl<KeyExt, K> GetKey<K> for QueuedLookupRegistry<KeyExt, K>
where
    KeyExt: KeyExtension<K>,
    K: Eq + Hash,
{
    /// Takes O(1) for finding the key.
    fn get(&self, key: &K) -> Option<&K> {
        self.key_idx_map.get(key).map(|&idx| {
            let tk = self.ordered_keys[idx].key();
            if tk == key {
                tk
            } else {
                panic!("invalid state of key registry");
            }
        })
    }
}

impl<KeyExt, K> KeyRegistry<K> for QueuedLookupRegistry<KeyExt, K>
where
    KeyExt: KeyExtension<K>,
    K: Hash + Eq + PartialEq + Clone,
{
    type KeyExtension = KeyExt;

    fn with_capacity(max_capacity: usize) -> Self {
        Self {
            key_idx_map: HashMap::with_capacity(max_capacity),
            ordered_keys: VecDeque::with_capacity(max_capacity),
            max_capacity,
            _phantom_data: std::marker::PhantomData::<K>,
        }
    }

    fn clear(&mut self) {
        self.key_idx_map.clear();
        self.ordered_keys.clear();
    }

    fn len(&self) -> usize {
        if self.key_idx_map.len() != self.ordered_keys.len() {
            panic!("invalid state of key registry");
        }
        self.ordered_keys.len()
    }

    // Takes O(n) in case the key is present, or if storage is full, and O(1) otherwise.
    fn add_or_update(&mut self, key: KeyExt) -> Option<K> {
        self.try_remove(key.key());

        let deleted_key = if self.len() >= self.max_capacity {
            self.delete_oldest()
        } else {
            None
        };

        self.insert(key);
        deleted_key.map(|tk| tk.key().clone())
    }
    // // Takes O(n) in case the key is present, or if storage is full, and O(1) otherwise.
    // fn add_or_update(&mut self, key: K) -> Option<K> {
    //     self.try_remove(&key);

    //     let deleted_key = if self.len() >= self.max_capacity {
    //         self.delete_oldest()
    //     } else {
    //         None
    //     };

    //     self.insert(key);
    //     deleted_key.map(|tk| tk.key)
    // }

    // Takes O(n) for re-ordering the lookup.
    fn try_remove(&mut self, key: &K) -> Option<K> {
        let key = self.remove_key(key).map(|tk| tk.key().clone());
        key
    }
}

pub type QueuedLookupCache<K, KeyExt, V> = Cache<K, QueuedLookupRegistry<KeyExt, K>, KeyExt, V>;
pub type ConcurrentQueuedLookupCache<K, KeyExt, V> =
    ConcurrentCache<K, QueuedLookupRegistry<KeyExt, K>, KeyExt, V>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn queued_cache_init() {
        let mut cache = QueuedCache::<i32, i32, String>::new(Some(4));
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
    fn concurrent_queued_cache_init() {
        let cache = ConcurrentQueuedCache::<i32, i32, String>::new(Some(4));
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
    fn queued_lookup_cache_init() {
        let mut cache = QueuedLookupCache::<i32, i32, String>::new(Some(4));
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
    fn concurrent_queued_lookup_cache_init() {
        let cache = Arc::new(ConcurrentQueuedLookupCache::<i32, i32, String>::new(Some(
            4,
        )));

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
