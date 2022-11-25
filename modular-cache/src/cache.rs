use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait KeyRegistry<K> {
    type KeyStatsItem;

    fn init(capacity: usize) -> Self;

    fn len(&self) -> usize;

    // fn get(&mut self, key: &K) -> Option<&mut Self::KeyStatsItem>;

    // return deleted key (if some)
    fn add_or_update(&mut self, key: K) -> Option<K>;

    fn try_remove(&mut self, key: &K) -> Option<K>;

    /// Make sure keys are still valid; return invalidated ones.
    fn house_keeping(&mut self) -> Option<HashSet<K>>;

    // TODO: invalidate key?
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
