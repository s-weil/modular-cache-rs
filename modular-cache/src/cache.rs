use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait KeyRegistry<K>: Sized {
    // type KeyStatsItem;

    fn with_capacity(max_capacity: usize) -> Self;

    fn init(max_capacity: Option<usize>) -> Self {
        let mc = max_capacity.unwrap_or(usize::MAX);
        Self::with_capacity(mc)
    }

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn clear(&mut self);

    /// Gets the key's value _without_ updating it's statistics.
    /// This is crucial for instance for a
    /// [`LRU cache`](https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU)
    /// and should be considered to not be implemented in this case.
    fn get(&self, key: &K) -> Option<&K>;
    // TODO: rename: 'mut' is misleading
    // Get the key's value and updates it's statistics.
    fn get_mut(&mut self, key: &K) -> Option<&K>;

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
    pub fn new(max_capacity: Option<usize>) -> Self {
        Self {
            store: HashMap::new(),
            key_registry: S::init(max_capacity),
        }
    }

    /// Clears the registry and store, removing all key-value pairs.
    /// Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.key_registry.clear();
        self.store.clear();
    }

    pub fn len(&self) -> usize {
        // TODO: check also key_registry to be synced
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        // TODO: check also key_registry to be synced
        self.store.is_empty()
    }

    /// Get the key's value _without_ updating it's statistics.
    /// Use `get_mut` in case the latter is essential.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.key_registry.get(key).and_then(|k| self.store.get(k))
    }

    // TODO: rename: 'mut' is misleading
    /// Get the key's value and updates it's statistics.
    pub fn get_mut(&mut self, key: &K) -> Option<&V> {
        self.key_registry
            .get_mut(key)
            .and_then(|k| self.store.get(k))
    }

    /// Inserts a key-value pair into the cache.
    /// If the cache did not have this key present, None is returned.
    /// If the cache did have this key present, the value is updated, and the old value is returned.
    /// TODO: remove or keep? The key is not updated, though; this matters for types that can be == without being identical.
    /// See the module-level documentation for more.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(deleted_key) = self.key_registry.add_or_update(key.clone()) {
            self.store.remove(&deleted_key);
        }
        self.store.insert(key, value)
    }

    /// TODO: make sure keys are aligne within registry and store
    /// Removes a key from the cache, returning the stored key and value if the key was previously in the cache.
    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        self.key_registry
            .try_remove(key)
            .and_then(|k| self.store.remove_entry(&k))
    }
}
